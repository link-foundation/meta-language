// Normal test discovery executes every original isolated prototype assertion.
import '../../experiments/rust-scalar-loop-prototype.test.mjs';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { pathToFileURL } from 'node:url';
import vm from 'node:vm';
import { publicDeclarationConsumers } from '../scripts/check-public-declaration-contracts.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks, eraseTypeScript, LinkNetwork } from '../src/index.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { checkRustScalarLoopProof } from '../src/translation/rust-scalar-loops.js';
import { acceptScalarLoopExpressionTypeAnnotation } from '../src/translation/scalar-loop-rules.js';

test('shared scalar loop annotation admission distinguishes inference from malformed present types', () => {
  for (const [present, nullValue, kind, expected] of [
    [false, false, '', true],
    [true, true, '', true],
    [true, false, 'float', true],
    [true, false, 'bool', true],
    [true, false, '', false],
    [true, false, 'string', false],
    [true, false, 'array', false],
    [true, false, 'unit', false],
  ]) assert.equal(acceptScalarLoopExpressionTypeAnnotation(present, nullValue, kind), expected);
});

test('actual scalar loop snapshots retain null, omitted and typed annotations through checking and source-free replay', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/rust-scalar-loops.lino', import.meta.url));
  const source = fixture.cases[1].javaScriptDefinitions;
  const original = parseJavaScript(source, { scalarLoopModel: fixture.model });
  assert.equal(original.items[0].body.then.k, 'let');
  assert.equal(Object.hasOwn(original.items[0].body.then, 'type'), true);
  assert.equal(original.items[0].body.then.type, null);
  for (const annotation of [undefined, null, { kind: 'float' }]) {
    const surface = JSON.parse(JSON.stringify(original));
    const binding = surface.items[0].body.then;
    if (annotation === undefined) delete binding.type;
    else binding.type = annotation;
    const restoredSurface = JSON.parse(JSON.stringify(surface));
    assert.equal(Object.hasOwn(restoredSurface.items[0].body.then, 'type'), annotation !== undefined);
    if (annotation !== undefined) assert.deepEqual(restoredSurface.items[0].body.then.type, annotation);
    checkRustScalarLoopProof(restoredSurface.items[0], 'JavaScript');
    const packet = encodeTypedProgramLinks(checkProgram(restoredSurface));
    const network = LinkNetwork.fromLino(packet.network.toLino());
    const program = decodeTypedProgramLinks(network, packet.root);
    assert.equal(program.declarations.get('swap').sourceLoop.sourceName, 'swap');
    const runtime = {};
    vm.runInNewContext(emitTypedProgramLinks(network, packet.root, 'JavaScript').code, runtime, { timeout: 1000 });
    assert.equal(runtime.swap(1, 2, 1, 0), 2);
    assert.equal(Object.is(runtime.swap(0, -0, 1, 0), -0), true);
  }
});

test('scalar loop snapshot proofs reject malformed annotations and retain scalar kind checks', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/rust-scalar-loops.lino', import.meta.url));
  const original = parseJavaScript(fixture.cases[1].javaScriptDefinitions, { scalarLoopModel: fixture.model });
  for (const annotation of [false, 0, '', [], {}, { kind: null }, { kind: 1 }, { kind: 'string' }, { kind: 'array', element: { kind: 'float' } }]) {
    const item = JSON.parse(JSON.stringify(original.items[0]));
    item.body.then.type = annotation;
    assert.throws(() => checkRustScalarLoopProof(item, 'JavaScript'), /a loop expression has a non-scalar value type/u);
  }
});

async function runDeclaredConsumer(name, context) {
  const consumer = publicDeclarationConsumers().find(consumer => consumer.name === name);
  assert.ok(consumer, `declared consumer ${name} exists`);
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-scalar-loop-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const filename = join(directory, 'consumer.mjs');
  writeFileSync(filename, eraseTypeScript(consumer.source).code);
  return import(pathToFileURL(filename).href);
}

for (const language of ['Rust', 'JavaScript']) {
  test(`the exact declared ${language} scalar loop consumer emits and executes from public reloaded links`, async context => {
    const { contractEvidence } = await runDeclaredConsumer(`valid-selected-${language}-scalar-loop-profile`, context);
    assert.equal(contractEvidence.language, 'JavaScript');
    assert.equal(contractEvidence.declaration, true);
    assert.equal(contractEvidence.root, true);
    assert.equal(contractEvidence.proof, true);
    assert.match(contractEvidence.rustCode, /pub fn advance\(/u);
    const runtime = {};
    vm.runInNewContext(contractEvidence.code, runtime, { timeout: 1000 });
    assert.equal(runtime.advance(3, 5, 0), 8);
    assert.equal(runtime.advance(0, 20_000, 0), 20_000);
  });

  test(`the exact declared unknown ${language} scalar loop model also fails at runtime`, async context => {
    await assert.rejects(runDeclaredConsumer(`reject-unknown-${language}-scalar-loop-profile`, context),
      error => /scalar loop model/iu.test(error.reason ?? error.message));
  });
}

test('public scalar loop model selections reject the opposite source language before source import', () => {
  assert.throws(() => createTypedProgramLinks('fn main(){}', 'Rust',
    { javaScriptScalarLoopModel: 'closed primitive scalar loop 1' }), /requires JavaScript source/u);
  assert.throws(() => createTypedProgramLinks('function keep(){return 1;}', 'JavaScript',
    { rustScalarLoopModel: 'closed primitive scalar loop 1' }), /requires Rust source/u);
});

test('the authentic public scalar loop executes a direct divisor-link edit after LiNo reload', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/rust-scalar-loops.lino', import.meta.url));
  const entry = fixture.cases.find(entry => entry.name === fixture.originalName);
  assert.equal(Buffer.byteLength(fixture.actualSource), 512);
  const packet = createTypedProgramLinks(entry.source, 'Rust', { rustScalarLoopModel: fixture.model });
  const network = LinkNetwork.fromLino(packet.network.toLino());
  const execute = () => {
    const runtime = {};
    vm.runInNewContext(emitTypedProgramLinks(network, packet.root, 'JavaScript').code, runtime, { timeout: 1000 });
    return runtime[entry.name](65536, 1, 0);
  };
  assert.equal(execute(), 256);
  const divisors = network.links().filter(link => link.metadata().definition === '256');
  assert.equal(divisors.length, 1);
  divisors[0].setMetadata(divisors[0].metadata().withDefinition('128'));
  assert.equal(execute(), 512);
  const actualJavaScript = emitTypedProgramLinks(network, packet.root, 'JavaScript').code;
  const reverse = createTypedProgramLinks(actualJavaScript, 'JavaScript', { javaScriptScalarLoopModel: fixture.model });
  const replay = LinkNetwork.fromLino(reverse.network.toLino());
  const runtime = {};
  vm.runInNewContext(emitTypedProgramLinks(replay, reverse.root, 'JavaScript').code, runtime, { timeout: 1000 });
  assert.equal(runtime[entry.name](65536, 1, 0), 512);
  assert.ok(emitTypedProgramLinks(replay, reverse.root, 'Rust').code.includes(`pub fn ${entry.name}(`));
});

test('CI checks shared scalar loop rules and actual automated fixtures after earlier failures', () => {
  const workflow = readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const jobs = parseWorkflow(workflow);
  for (const command of ['node js/scripts/generate-scalar-loop-rules.mjs --check',
    'node experiments/generate-rust-scalar-loop-fixture.mjs --check']) {
    const gates = jobs.flatMap(job => job.steps).filter(step => step.run.trim() === command);
    assert.equal(gates.length, 1, command);
    assert.equal(gates[0].afterFailure, true, command);
  }
});
