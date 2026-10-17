import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { pathToFileURL } from 'node:url';
import vm from 'node:vm';
import { publicDeclarationConsumers } from '../scripts/check-public-declaration-contracts.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { createTypedProgramLinks, decodeTypedProgramLinks, eraseTypeScript } from '../src/index.js';
import { emitRust } from '../src/translation/emit-rust.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-empty-options.lino', import.meta.url));

async function runConsumer(name, context) {
  const consumer = publicDeclarationConsumers().find(consumer => consumer.name === name);
  assert.ok(consumer, `the real strict public consumer ${name} exists`);
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-empty-option-public-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, eraseTypeScript(consumer.source).code);
  return import(pathToFileURL(file).href);
}

function execute(code, name) {
  return vm.runInNewContext(`${code}\n${name}();`, {}, { timeout: 1000 });
}

for (const entry of fixture.cases) {
  test(`the exact ${entry.id} empty-option consumer retains owned facts and reimports actual Rust definitions`, async context => {
    const consumer = await runConsumer(`valid-empty-option-owner-${entry.id}`, context);
    const { contractEvidence } = consumer;
    const program = decodeTypedProgramLinks(contractEvidence.network, contractEvidence.root);
    const declaration = program.declarations.get(entry.functionName);
    assert.deepEqual(declaration.sourceEmptyOption, entry.expected);
    assert.deepEqual(declaration.ret, fixture.type);
    assert.deepEqual(declaration.body.type, fixture.type);
    assert.equal(declaration.body.k, 'none');
    assert.equal(declaration.body.absence, 'undefined');
    assert.equal(execute(contractEvidence.code, entry.functionName), undefined);
    const definitions = emitRust(program).definitions.join('\n\n');
    assert.ok(contractEvidence.rustCode.includes(definitions));
    const replay = consumer.reimportActualRustDefinitions(definitions);
    assert.equal(execute(replay.code, entry.emittedFunctionName), undefined);
    const reversed = decodeTypedProgramLinks(replay.network, replay.root).declarations.get(entry.emittedFunctionName);
    assert.deepEqual(reversed.ret, fixture.type);
    assert.equal(reversed.sourceEmptyOption.sourceName, entry.emittedFunctionName);
    assert.equal(reversed.sourceEmptyOption.model, fixture.model);
    assert.ok(!contractEvidence.network.toLino().includes(encodeURIComponent(entry.source)));
  });
}

test('the exact empty-option declaration rename changes owned links and the executable binding through Rust reimport', async context => {
  const consumer = await runConsumer('valid-source-free-empty-option-declaration-rename', context);
  const { contractEvidence } = consumer;
  const program = decodeTypedProgramLinks(contractEvidence.network, contractEvidence.root);
  assert.ok(program.declarations.has('vacant'));
  assert.ok(!program.declarations.has('empty'));
  const declaration = program.declarations.get('vacant');
  assert.deepEqual(declaration.ret, fixture.type);
  assert.equal(declaration.sourceEmptyOption.sourceName, 'vacant');
  assert.deepEqual(declaration.sourceEmptyOption.namespace.declarationNames, ['vacant']);
  assert.equal(execute(contractEvidence.originalCode, 'empty'), undefined);
  assert.equal(execute(contractEvidence.code, 'vacant'), undefined);
  assert.throws(() => execute(contractEvidence.code, 'empty'), /empty is not defined/u);
  const definitions = emitRust(program).definitions.join('\n\n');
  assert.ok(contractEvidence.rustCode.includes(definitions));
  assert.match(definitions, /pub fn vacant\(\) -> Option<f64>/u);
  const replay = consumer.reimportActualRustDefinitions(definitions);
  assert.equal(execute(replay.code, 'vacant'), undefined);
  assert.throws(() => execute(replay.code, 'empty'), /empty is not defined/u);
});

test('the exact unregistered empty-option model version refuses at runtime', async context => {
  await assert.rejects(runConsumer('reject-unregistered-empty-option-source-version', context), error => {
    assert.equal(error.reason, 'Rust empty option source model: select the legacy model or closed empty primitive Rust option 1');
    assert.equal(error.span, null);
    return true;
  });
});

test('the exact declared wrong-language consumer receives the Rust-source selection diagnostic', async context => {
  const consumer = await runConsumer('valid-empty-option-selection-type-with-wrong-source-language', context);
  assert.throws(() => consumer.createWrongLanguagePacket(), {
    name: 'TypeError', message: 'a Rust empty option source model requires Rust source',
  });
});

test('the exact declared independent-profile compositions refuse without changing the existing default diagnostic', async context => {
  const consumer = await runConsumer('valid-empty-option-independent-profile-composition-types', context);
  for (const create of [consumer.createComposedSignaturePacket, consumer.createComposedPrimitivePacket]) {
    assert.throws(create, error => {
      assert.equal(error.reason, 'Rust empty option source model: the closed empty interpretation requires its independent source profile');
      assert.equal(error.span, null);
      return true;
    });
  }
  assert.throws(() => createTypedProgramLinks(fixture.cases[0].source, 'Rust'), error => {
    assert.equal(error.reason, 'type Option<…>: generic types are outside the portable core');
    assert.deepEqual(error.span, { start: 12, end: 18 });
    return true;
  });
});
