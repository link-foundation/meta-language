import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { compareRustDefinitions, createModuleContext, measureSelfTranslationModule, partitionSelfTranslationReportModules, readModuleHeaderBindings, rustFunctionDefinitions, sourceCoverage } from '../scripts/generate-self-translation-report.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { partitionGroupFiles, partitionMeasuredWorkloads, partitionTests, testFiles } from '../scripts/test-groups.mjs';
import { createReportMeasurementWorker } from '../scripts/self-translation-report-worker-host.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { selfTranslate } from '../src/self-translation.js';

test('report checkout preserves the complete workload and removes unrelated archives', () => {
  const repository = fileURLToPath(new URL('../../', import.meta.url));
  const workflow = '.github/workflows/ci.yml';
  const git = (...args) => execFileSync('git', args, { cwd: repository, encoding: 'utf8' });
  const reportJob = source => parseWorkflow(source).find(job => job.id === 'self-translation-report');
  const current = reportJob(readFileSync(join(repository, workflow), 'utf8'));
  const previous = reportJob(git('show', `1e0cecca0cb83797c8b3263e5576705db95cb492:${workflow}`));
  const checkout = current.steps[0].text;
  const sparse = /^ {10}sparse-checkout-cone-mode: false\n {10}sparse-checkout: \|\n((?: {12}\S[^\n]*\n)+)/mu.exec(checkout);
  assert.ok(sparse, 'the exact report checkout uses non-cone sparse rules');
  assert.equal(checkout.replace(sparse[0], ''), previous.steps[0].text);
  assert.equal(current.header, previous.header);
  assert.deepEqual(current.steps.slice(1), previous.steps.slice(1));
  const scratch = mkdtempSync(join(tmpdir(), 'meta-language-report-checkout-'));
  try {
    const rules = join(scratch, 'patterns');
    writeFileSync(rules, sparse[1].replace(/^ {12}/gmu, ''));
    const tracked = git('ls-files', '-z');
    const files = tracked.split('\0').filter(Boolean);
    const selected = new Set(execFileSync('git', [
      'sparse-checkout', 'check-rules', '-z', '--no-cone', '--rules-file', rules,
    ], { cwd: repository, encoding: 'utf8', input: tracked }).split('\0').filter(Boolean));
    for (const prefix of ['js/src/', 'js/scripts/', 'rust/src/', 'scripts/', 'parity/grammars/']) {
      const required = files.filter(file => file.startsWith(prefix));
      assert.ok(required.length > 0, prefix);
      for (const file of required) assert.ok(selected.has(file), file);
    }
    for (const file of [
      'js/package.json', 'js/package-lock.json', 'parity/self-translation/decorators.lino',
      'parity/fixtures/ci-39615628279-workload-costs.lino',
      'parity/fixtures/ci-45076523061-report-workload-costs.lino',
      'parity/fixtures/ci-45104457483-report-workload-costs.lino',
      'parity/fixtures/ci-45215384144-report-workload-costs.lino',
      'parity/fixtures/ci-45296878458-report-workload-costs.lino',
    ]) assert.ok(selected.has(file), file);
    for (const file of selected) {
      assert.doesNotMatch(file, /^(?:dev\/log|experiments|parity\/naming)\//u);
      if (file.startsWith('parity/fixtures/')) {
        assert.match(file, /^parity\/fixtures\/ci-[^/]+workload-costs\.lino$/u);
      }
      if (file.startsWith('parity/self-translation/')) {
        assert.equal(file, 'parity/self-translation/decorators.lino');
      }
    }
    assert.ok(selected.size < files.length);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});

test('recent report timings balance all complete module workloads across the same sixteen shards', () => {
  const repository = fileURLToPath(new URL('../../', import.meta.url));
  const git = (args, options = {}) => execFileSync('git', ['--no-replace-objects', ...args], {
    cwd: repository, maxBuffer: 2 * 1024 * 1024, ...options,
  });
  function readSources(revision) {
    const files = git(['ls-tree', '-r', '--name-only', revision, 'js/src'])
      .toString().trimEnd().split('\n').filter(file => file.endsWith('.js')).sort();
    const blobs = git(['cat-file', '--batch'], {
      input: files.map(file => `${revision}:${file}\n`).join(''),
      maxBuffer: 32 * 1024 * 1024,
    });
    const bytes = new Map();
    let offset = 0;
    for (const file of files) {
      const headerEnd = blobs.indexOf(10, offset);
      const header = /^[a-f0-9]+ blob ([0-9]+)$/u.exec(blobs.toString('ascii', offset, headerEnd));
      assert.ok(header, file);
      const end = headerEnd + 1 + Number(header[1]);
      assert.equal(blobs[end], 10, file);
      bytes.set(file, blobs.subarray(headerEnd + 1, end));
      offset = end + 1;
    }
    assert.equal(offset, blobs.length);
    return { files, bytes };
  }
  // Replay the old assignments on the source tree used to evaluate the scheduling change.
  const comparisonRevision = 'b964c412ed4efd5a0b9d04002780b7306f0e92b4';
  const { files: historicalSources, bytes: historicalBytes } = readSources(comparisonRevision);
  const historicalInput = { readSource: file => historicalBytes.get(file) };
  const record = readProjectData(new URL('../../parity/fixtures/ci-45076523061-report-workload-costs.lino', import.meta.url));
  assert.deepEqual(record.logs.map(row => row.number), Array.from({ length: 16 }, (_, index) => index + 1));
  const elapsed = new Map(record.entries.map(row => [row.file, row.milliseconds]));
  const baseline = readProjectData(new URL('../../parity/fixtures/ci-39615628279-workload-costs.lino', import.meta.url));
  const priorEntries = new Map([...baseline.entries, ...record.entries].map(row => [row.file, row]));
  const previous = partitionMeasuredWorkloads(historicalSources, 16, historicalInput);
  const historical = partitionMeasuredWorkloads(historicalSources, 16, {
    ...historicalInput, costs: { ...baseline, entries: [...priorEntries.values()] },
  });
  const longest = shards => Math.max(...shards.map(shard => shard.reduce((sum, file) => sum + (elapsed.get(file) ?? 0), 0)));
  assert.deepEqual(previous.flat().sort(), historicalSources);
  assert.deepEqual(historical.flat().sort(), historicalSources);
  assert.ok(longest(previous) >= 30 * 60 * 1000, 'recent elapsed samples reproduce the uneven prior assignment');
  assert.ok(longest(historical) <= 18 * 60 * 1000, 'the recent elapsed samples are spread across the sixteen owners');

  const pipeline = readProjectData(new URL('../../parity/fixtures/ci-45104457483-report-workload-costs.lino', import.meta.url));
  assert.deepEqual(pipeline.logs.map(row => row.number), record.logs.map(row => row.number));
  const unfinished = pipeline.logs.flatMap(row => row.unfinished);
  assert.deepEqual(unfinished, ['js/src/typed-program-links.js', 'js/src/typescript-erasure-rules.js']);
  for (const row of pipeline.entries) {
    assert.ok(!unfinished.includes(row.file));
    elapsed.set(row.file, row.milliseconds);
    priorEntries.set(row.file, row);
  }
  assert.ok(longest(historical) >= 30 * 60 * 1000, 'overlap-adjusted samples expose the prior overloaded shard');

  const latest = readProjectData(new URL('../../parity/fixtures/ci-45215384144-report-workload-costs.lino', import.meta.url));
  assert.deepEqual(latest.logs.map(row => row.number), record.logs.map(row => row.number));
  const cancelled = latest.logs.filter(row => row.conclusion === 'cancelled');
  assert.deepEqual(cancelled.map(row => row.number), [5]);
  const [unfinishedModule] = cancelled[0].unfinished;
  assert.deepEqual(cancelled[0].unfinished, ['js/src/typed-program-expression-projection.js']);
  const sampledSources = [...latest.entries.map(row => row.file), unfinishedModule].sort();
  const { files: sampledRevision, bytes: sampledBytes } = readSources(latest.head);
  assert.deepEqual(sampledSources, sampledRevision);
  const sampledInput = { readSource: file => sampledBytes.get(file) };
  const preceding = partitionMeasuredWorkloads(sampledSources, 16, {
    ...sampledInput, costs: { ...baseline, entries: [...priorEntries.values()] },
  });
  assert.deepEqual(preceding.flat().sort(), sampledSources);
  assert.ok(longest(preceding) <= 20 * 60 * 1000, 'overlap-adjusted samples leave headroom within the deadline');

  for (const row of latest.entries) {
    elapsed.set(row.file, row.milliseconds);
    priorEntries.set(row.file, row);
  }
  const updated = partitionMeasuredWorkloads(sampledSources, 16, {
    ...sampledInput, costs: { ...baseline, entries: [...priorEntries.values()] },
  });
  assert.deepEqual(updated.flat().sort(), sampledSources);
  assert.ok(longest(preceding) >= 28 * 60 * 1000, 'the cancelled shard accumulated nearly the full deadline before its final module');
  assert.ok(longest(updated) <= 17 * 60 * 1000, 'updated costs distribute the expensive modules across the sixteen owners');

  const complete = readProjectData(new URL('../../parity/fixtures/ci-45296878458-report-workload-costs.lino', import.meta.url));
  assert.equal(git(['rev-parse', `${complete.revision}^2`]).toString().trim(), complete.head);
  assert.deepEqual(complete.logs.map(row => row.number), record.logs.map(row => row.number));
  for (const row of complete.logs) {
    assert.equal(row.conclusion, 'success');
    assert.deepEqual(row.unfinished, []);
  }
  const { files: completeSources, bytes: completeBytes } = readSources(complete.revision);
  assert.equal(completeSources.length, 382);
  assert.deepEqual(complete.entries.map(row => row.file).sort(), completeSources);
  for (const row of complete.entries) {
    const bytes = completeBytes.get(row.file);
    assert.equal(bytes.length, row.bytes, row.file);
    assert.equal(createHash('sha256').update(bytes).digest('hex'), row.sha256, row.file);
    elapsed.set(row.file, row.milliseconds);
  }
  const completeInput = { readSource: file => completeBytes.get(file) };
  const beforeComplete = partitionMeasuredWorkloads(completeSources, 16, {
    ...completeInput, costs: { ...baseline, entries: [...priorEntries.values()] },
  });
  const current = partitionSelfTranslationReportModules(completeSources, 16, completeInput);
  assert.deepEqual(beforeComplete.flat().sort(), completeSources);
  assert.deepEqual(current.flat().sort(), completeSources);
  assert.ok(longest(beforeComplete) >= 27 * 60 * 1000, 'complete timings expose the overloaded prior shard');
  assert.ok(longest(current) <= 19 * 60 * 1000, 'complete timings balance all sixteen shards within the available time');
});

test('recent translation test timings balance both jobs in each original three-way partition', () => {
  const { translation } = partitionTests(testFiles()).groups;
  const record = readProjectData(new URL('../../parity/fixtures/ci-45076523061-translation-test-workload-costs.lino', import.meta.url));
  assert.deepEqual(record.logs.map(row => row.number), [1, 2, 3, 4, 5, 6]);
  const elapsed = new Map(record.entries.map(row => [row.file, row.milliseconds]));
  const workload = files => files.reduce((sum, file) => sum + (elapsed.get('js/tests/' + file) ?? 0), 0);
  const old = Array.from({ length: 6 }, (_, index) => translation.filter((_, offset) => offset % 6 === index));
  const current = Array.from({ length: 6 }, (_, index) => partitionGroupFiles(translation, `${index + 1}/6`));
  for (let index = 0; index < 3; index += 1) {
    assert.deepEqual([...current[index], ...current[index + 3]].sort(), partitionGroupFiles(translation, `${index + 1}/3`));
  }
  assert.deepEqual(current.flat().sort(), translation);
  assert.ok(Math.max(...current.map(workload)) < 10 * 60 * 1000);
  assert.ok(Math.max(...old.map(workload)) > 11 * 60 * 1000);
});

test('one CI measurement worker preserves full source coverage, hashes and both independent comparisons', async () => {
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

test('report import resolution checks callable candidates and avoids dependencies of carried classes', () => {
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
  const source = modules.get('consumer.js');
  const translation = selfTranslate(source, 'JavaScript', 'Rust', context);
  assert.equal(translation.items.find(item => source.slice(item.start, item.end).includes('function twice')).status, 'translated');
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

test('self-translation report accounts for actual translated, carried and UTF-8 comment source bytes', () => {
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
