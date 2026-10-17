import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { pathToFileURL } from 'node:url';
import vm from 'node:vm';
import { publicDeclarationConsumers } from '../scripts/check-public-declaration-contracts.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { decodeTypedProgramLinks, eraseTypeScript } from '../src/index.js';
import { emitRust } from '../src/translation/emit-rust.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-type-namespace.lino', import.meta.url));

async function runConsumer(name, context) {
  const consumer = publicDeclarationConsumers().find(entry => entry.name === name);
  assert.ok(consumer, `the real strict public consumer ${name} exists`);
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-type-namespace-public-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, eraseTypeScript(consumer.source).code);
  return import(pathToFileURL(file).href);
}

function output(code) {
  const lines = [];
  vm.runInNewContext(code, { console: { log: (...values) => lines.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return lines;
}

for (const entry of fixture.cases) {
  test(`the exact ${entry.id} namespace consumer retains header owners and actual emitted-definition replay`, async context => {
    const consumer = await runConsumer(`valid-type-namespace-owner-${entry.id}`, context);
    const evidence = consumer.contractEvidence;
    const program = decodeTypedProgramLinks(evidence.network, evidence.root);
    const declaration = program.declarations.get(entry.functionName);
    assert.deepEqual(declaration.params[0].type, entry.expected.parameterType);
    assert.deepEqual(declaration.ret, entry.expected.resultType);
    assert.deepEqual(declaration.sourceTypeNamespace.parameters, [entry.expected.parameterSpelling]);
    assert.deepEqual(declaration.sourceTypeNamespace.result, entry.expected.resultSpelling);
    assert.deepEqual(declaration.sourceSignature.parameters[0].owner.path, entry.expected.parameterOwner);
    assert.deepEqual(declaration.sourceSignature.result.owner.path, entry.expected.resultOwner);
    assert.deepEqual(output(evidence.code), entry.output);
    if (entry.callFunction) assert.equal(program.declarations.get(entry.callFunction).body.fn, entry.callTarget);
    const emitted = emitRust(program);
    const definitions = emitted.definitions.join('\n\n');
    assert.ok(evidence.rustCode.includes(definitions));
    const replay = consumer.reimportActualRustDefinitions(definitions);
    const target = name => emitted.mappings.find(mapping => mapping.source === name)?.target ?? name;
    const mappedType = type => type.kind === 'data' ? { kind: 'data', name: target(type.name) } : type;
    const replayDeclaration = replay.program.declarations.get(target(entry.functionName));
    assert.deepEqual(replayDeclaration.params[0].type, mappedType(entry.expected.parameterType));
    assert.deepEqual(replayDeclaration.ret, mappedType(entry.expected.resultType));
    assert.equal(replayDeclaration.sourceTypeNamespace.model, fixture.typeNamespaceModel);
    assert.deepEqual(output(replay.code), []);
    if (entry.callFunction) assert.equal(replay.program.declarations.get(target(entry.callFunction)).body.fn, target(entry.callTarget));
    assert.ok(!evidence.network.toLino().includes(encodeURIComponent(entry.source)));
  });
}

test('the exact public namespace value edit changes execution while keeping the distinct type and call owners', async context => {
  const consumer = await runConsumer('valid-source-free-type-namespace-value-edit', context);
  const evidence = consumer.contractEvidence;
  const program = decodeTypedProgramLinks(evidence.network, evidence.root);
  assert.equal(program.declarations.get('nested.value').body.fn, 'nested.f64');
  assert.deepEqual(program.declarations.get('nested.header').params[0].type, { kind: 'data', name: 'f64' });
  assert.deepEqual(program.declarations.get('nested.header').sourceTypeNamespace.parameters, [['f64']]);
  assert.deepEqual(output(evidence.code), ['false', 'true']);
});

test('the exact unregistered public namespace model receives its runtime refusal', async context => {
  await assert.rejects(runConsumer('reject-unregistered-type-namespace-source-version', context), error => {
    assert.equal(error.reason, 'Rust type namespace model: select the legacy model or resolved Rust type namespace 1');
    assert.equal(error.span, null);
    return true;
  });
});

test('the exact declared namespace selector requires actual Rust source', async context => {
  const consumer = await runConsumer('valid-type-namespace-selection-type-with-wrong-source-language', context);
  assert.throws(() => consumer.createWrongLanguagePacket(), {
    name: 'TypeError', message: 'a Rust type namespace model requires Rust source',
  });
});

test('the exact declared namespace and empty-option composition retains the independent-profile refusal', async context => {
  const consumer = await runConsumer('valid-type-namespace-independent-empty-option-profile-types', context);
  assert.throws(() => consumer.createComposedEmptyOptionPacket(), error => {
    assert.equal(error.reason, 'Rust empty option source model: the closed empty interpretation requires its independent source profile');
    assert.equal(error.span, null);
    return true;
  });
});
