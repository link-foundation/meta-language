import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { automatedTranslationTest } from './support/automated-translation-test.js';
import { compareRustDefinitions, createModuleContext, measureReportModules, measureSelfTranslationModule, partitionReportModules, readModuleHeaderBindings, rustFunctionDefinitions, sourceCoverage } from '../scripts/generate-self-translation-report.mjs';
import { createReportMeasurementWorker } from '../scripts/self-translation-report-worker-host.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { partitionGroupFiles, partitionMeasuredWorkloads, partitionTests, testFiles } from '../scripts/test-groups.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { selfTranslate } from '../src/self-translation.js';

test('report scheduling retains every source across eight balanced shards', () => {
  const costs = readProjectData(new URL('../../parity/fixtures/ci-45080642295-report-costs.lino', import.meta.url));
  const elapsed = new Map(costs.entries.map(row => [row.file, row.milliseconds]));
  const files = [...elapsed.keys()].sort();
  const shards = partitionMeasuredWorkloads(files, 8, { costs });
  assert.equal(costs.logs.length, 8);
  assert.deepEqual(shards.flat().sort(), files);
  const loads = shards.map(shard => shard.reduce((sum, file) => sum + elapsed.get(file), 0));
  assert.ok(Math.max(...loads) < 1_500_000, 'recorded costs leave a margin in the existing deadline');
});

test('report scheduling separates shard 2 import and measurement work while keeping every module', () => {
  const costs = readProjectData(new URL('../../parity/fixtures/ci-45080642295-report-costs.lino', import.meta.url));
  const files = costs.entries.map(row => row.file).sort();
  const previous = partitionMeasuredWorkloads(files, 8, { costs });
  const expected = previous.map(shard => [...shard]);
  for (const [module, destination] of [
    ['js/src/grammar-emitters/antlr.js', 7],
    ['js/src/program-translation.js', 7],
    ['js/src/translation/emit-rust.js', 8],
  ]) {
    const old = expected.find(shard => shard.includes(module));
    old.splice(old.indexOf(module), 1);
    expected[destination - 1].push(module);
  }
  const shards = partitionReportModules(files);
  assert.deepEqual(shards, expected.map(shard => shard.sort()));
  assert.deepEqual(shards.flat().sort(), files);
});

test('translation scheduling retains every test across six balanced shards', () => {
  const files = partitionTests(testFiles()).groups.translation;
  const shards = Array.from({ length: 6 }, (_, index) => partitionGroupFiles(files, `${index + 1}/6`));
  assert.deepEqual(shards.flat().sort(), files);
  const costs = readProjectData(new URL('../../parity/fixtures/ci-45080642295-translation-costs.lino', import.meta.url));
  const elapsed = new Map(costs.entries.map(row => [row.file, row.milliseconds]));
  const loads = shards.map(shard => shard.reduce((sum, file) => sum + (elapsed.get('js/tests/' + file) ?? 0), 0));
  assert.ok(Math.max(...loads) < 600_000, 'balance the observed workload within the existing deadline');
});

test('one CI measurement worker preserves full source coverage, hashes and both independent comparisons', automatedTranslationTest, async () => {
  const module = 'js/src/translation/primitive-value-rules.js';
  const context = { moduleDirectory: [], imports: {} };
  const decoratorsText = readFileSync(new URL('../../parity/self-translation/decorators.lino', import.meta.url), 'utf8');
  const worker = createReportMeasurementWorker(decoratorsText);
  try {
    const expected = measureSelfTranslationModule(module, DecoratorSet.fromLino(decoratorsText), context);
    const actual = await worker.measure(module, context);
    const { milliseconds: first, ...stableExpected } = expected;
    const { milliseconds: second, ...stableActual } = actual;
    assert.ok(first >= 0 && second >= 0);
    assert.deepEqual(stableActual, stableExpected);
    assert.equal(actual.coverage.unrepresentedBytes, 0);
    assert.ok(actual.functions > 0);
    assert.ok(actual.handWrittenLines > 0);
    const repeated = await worker.measure(module, context);
    assert.equal(repeated.sourceSha256, actual.sourceSha256);
    await assert.rejects(worker.measure('../outside.js', context), /inside js\/src/u);
  } finally { await worker.close(); }
});

test('two report measurements retain complete results and failures in source order', async () => {
  const modules = ['first', 'second', 'third', 'fourth'].map(name => `js/src/${name}.js`);
  const resolved = [];
  const finished = [];
  const { promise, resolve } = Promise.withResolvers();
  const context = path => {
    resolved.push(`js/src/${path}`);
    return { path };
  };
  const firstWorker = async (module, { path }) => {
    await promise;
    finished.push(module);
    return { path };
  };
  const secondWorker = async (module, { path }) => {
    finished.push(module);
    if (module === modules[1]) throw new Error('retained measurement failure');
    if (module === modules[3]) resolve();
    return { path };
  };
  const results = await measureReportModules(modules, context, [firstWorker, secondWorker]);
  assert.deepEqual(resolved, modules);
  assert.deepEqual(finished, [modules[1], modules[2], modules[3], modules[0]]);
  assert.deepEqual(results, [
    { module: modules[0], row: { path: 'first.js' } },
    { module: modules[1], error: 'retained measurement failure' },
    { module: modules[2], row: { path: 'third.js' } },
    { module: modules[3], row: { path: 'fourth.js' } },
  ]);
});

test('report import resolution checks callable candidates and avoids dependencies of carried classes', async (testContext) => {
  const modules = new Map([
    ['consumer.js', "import { Box } from './class.js';import { plus as add } from './provider.js';export function twice(value){return add(value,value);}"],
    ['class.js', "import { hidden } from './unused.js';export class Box { read(){return hidden();} }"],
    ['provider.js', '/** @param {number} left @param {number} right @returns {number} */ export function plus(left,right){return left+right;}'],
    ['unused.js', 'export function hidden(){return 3;}'],
  ]);
  const reads = [];
  const context = createModuleContext(name => { reads.push(name); return modules.get(name) ?? null; })('consumer.js');
  assert.deepEqual(context.imports['./class.js'], []);
  assert.deepEqual(context.imports['./provider.js'].map(signature => signature.name), ['plus']);
  assert.equal(reads.includes('unused.js'), false);
  await testContext.test('the resolved callable enables actual translation', automatedTranslationTest, () => {
    const source = modules.get('consumer.js');
    const translation = selfTranslate(source, 'JavaScript', 'Rust', context);
    assert.equal(translation.items.find(item => source.slice(item.start, item.end).includes('function twice')).status, 'translated');
  });
});

test('lexical report headers agree with independent grammar headers despite opaque and nested syntax', () => {
  const sources = [
    "import { plus as add, count, } from './provider.js';export function result(){return add(count,count);}",
    "import Default from './default.js';import * as namespace from './namespace.js';import './effects.js';export { plus } from './forward.js';export default function original(){}",
    "// import { fake } from './comment.js'\nconst text=\"export function fake(){}\";const template=`import { fake } from './template.js' ${ { value:'}' }.value }`;const pattern=/export [(){}]/;export const real=3;export async function next(){return real;}",
    "class Box { read(){return import('./dynamic.js');} }function nested(){const value={export:3,import:4};return value;}export class Carried{};export function* generator(){yield 1;}export const first=1,second=2;",
    "import /* between */ { plus /* name */ as add } from /* source */ './provider.js'\nexport\nfunction second(){return add(1,2)}",
    "import { café } from './unicode.js';export function café(){return 1;}",
  ];
  for (const source of sources) assert.deepEqual(readModuleHeaderBindings(source), readModuleHeaderBindings(source, true), source);
});

test('self-translation report reads individual UTF-8 function ranges and exact CST names', () => {
  const source = '// café 😀\nfn first() -> i64 { 1 }\nfn café() -> i64 { 2 }\npub fn r#type() { }\n';
  assert.deepEqual([...rustFunctionDefinitions(source)], [
    ['first', 'fn first() -> i64 { 1 }'],
    ['café', 'fn café() -> i64 { 2 }'],
    ['r#type', 'pub fn r#type() { }'],
  ]);
});

test('self-translation report compares each function independently', () => {
  const translated = '// heading\nfn first() -> i64 { 1 }\nfn second() -> i64 { 2 }\nfn third() -> i64 { 3 }\n';
  const counterpart = 'fn first() -> i64 {\n  1\n}\nfn second() -> i64 { 9 }\nfn unrelated() {}\n';
  const report = compareRustDefinitions(translated, counterpart);
  assert.equal(report.functions, 3);
  assert.equal(report.matched, 2);
  assert.equal(report.identical, 1);
  const standalone = compareRustDefinitions(translated, null);
  assert.equal(standalone.functions, 3);
  assert.equal(standalone.matched, 0);
  assert.equal(standalone.identical, 0);
});

test('self-translation report accounts for actual translated, carried and UTF-8 comment source bytes', automatedTranslationTest, () => {
  const source = '\r\n// café 😀\r\nexport function add(a, b) { return a + b; }\n\nclass Unsupported {}\n';
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  const coverage = sourceCoverage(source, translation.items);
  assert.equal(coverage.sourceBytes, Buffer.byteLength(source));
  assert.equal(coverage.itemBytes + coverage.layoutBytes, coverage.sourceBytes);
  assert.equal(coverage.unrepresentedBytes, 0);
  assert.deepEqual(coverage.unrepresentedRanges, []);
  assert.ok(coverage.bytesByStatus.translated > 0);
  assert.ok(coverage.bytesByStatus.carried >= Buffer.byteLength('class Unsupported {}'));
});

test('self-translation report exposes omitted source even when there are no carried items', () => {
  const source = 'const café = 1;\n';
  const empty = sourceCoverage(source, []);
  assert.equal(empty.unrepresentedBytes, Buffer.byteLength(source));
  assert.deepEqual(empty.unrepresentedRanges, [{ start: 0, end: Buffer.byteLength(source) }]);
  const partial = sourceCoverage('a(); b();', [{ start: 0, end: 4, status: 'translated' }]);
  assert.equal(partial.unrepresentedBytes, 5);
  assert.deepEqual(partial.unrepresentedRanges, [{ start: 4, end: 9 }]);
  const layout = sourceCoverage('\r\n \t\u2003', []);
  assert.equal(layout.layoutBytes, layout.sourceBytes);
  assert.equal(layout.unrepresentedBytes, 0);
});

test('self-translation report rejects overlapping, out-of-bounds and split UTF-8 item ranges', () => {
  for (const items of [
    [{ start: -1, end: 2, status: 'translated' }],
    [{ start: 0, end: 6, status: 'translated' }],
    [{ start: 0, end: 1, status: 'translated' }],
    [{ start: 0, end: 2, status: 'translated' }, { start: 1, end: 3, status: 'carried' }],
    [{ start: 0, end: 0, status: 'translated' }],
    [{ start: 0, end: 2, status: 'unknown' }],
  ]) assert.throws(() => sourceCoverage('é();', items), /invalid self-translation source range|unknown self-translation item status/u);
});
