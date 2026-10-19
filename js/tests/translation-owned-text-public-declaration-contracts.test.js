import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath, pathToFileURL } from 'node:url';
import vm from 'node:vm';
import { publicDeclarationConsumers } from '../scripts/check-public-declaration-contracts.mjs';
import { readProjectData } from '../scripts/project-data.mjs';

// An explicit private producer path permits pre-integration checks. CI always
// uses the package's real public module and the same generated consumer source.
const modulePath = process.env.META_LANGUAGE_OWNED_TEXT_PUBLIC_MODULE
  ?? fileURLToPath(new URL('../src/index.js', import.meta.url));
const moduleUrl = pathToFileURL(modulePath);
const publicApi = await import(moduleUrl.href);
const { emitRust } = await import(new URL('./translation/emit-rust.js', moduleUrl));
const fixture = readProjectData(new URL('../../experiments/rust-owned-local-state-fixture.lino', import.meta.url));

async function runConsumer(name, context) {
  const consumer = publicDeclarationConsumers(modulePath).find(entry => entry.name === name);
  assert.ok(consumer, 'the actual declared owned text consumer exists: ' + name);
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-owned-text-public-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, publicApi.eraseTypeScript(consumer.source).code);
  return import(pathToFileURL(file).href);
}

function observe(code, name, observations) {
  const runtime = {};
  vm.runInNewContext(code, runtime, { timeout: 1000 });
  for (const [text, count, flag, expected] of observations) {
    assert.equal(runtime[name](text, BigInt(count), flag), expected, name);
  }
}

for (const entry of fixture.cases) {
  for (const [selection, model, key] of [
    ['moves', fixture.models.moves, 'javascript'],
    ['copies', fixture.models.copies, 'replayJavaScript'],
  ]) {
    test('the exact ' + selection + ' owned text consumer retains ' + entry.name + ' and actual definition replay', async context => {
      const consumer = await runConsumer('valid-owned-text-' + selection + '-' + entry.name, context);
      const evidence = consumer.contractEvidence;
      const program = publicApi.decodeTypedProgramLinks(evidence.network, evidence.root);
      const declaration = program.declarations.get(entry.name);
      assert.equal(declaration.sourceLocalState.model, model);
      assert.deepEqual(declaration.sourceSignature.parameters[1].type, { kind: 'fixed', bits: 32, signed: true });
      assert.equal(evidence.code, entry[key]);
      observe(evidence.code, entry.name, entry.observations);
      const definitions = emitRust(program).definitions.join('\n\n');
      assert.ok(evidence.rustCode.includes(definitions), 'replay uses actual emitted complete definitions');
      const replay = consumer.reimportActualRustDefinitions(definitions);
      assert.equal(replay.program.declarations.get(entry.name).sourceLocalState.model, fixture.models.copies);
      observe(replay.code, entry.name, entry.observations);
      assert.ok(evidence.network.links().every(link => !link.metadata().definition?.includes(entry.source)));
    });
  }
}

test('the exact public owned text link edit changes the replacement while preserving owners and signed width', async context => {
  const consumer = await runConsumer('valid-source-free-owned-text-value-edit', context);
  const evidence = consumer.contractEvidence;
  const entry = fixture.cases.find(value => value.name === 'select');
  const packet = publicApi.createTypedProgramLinks(entry.source, 'Rust', { rustOwnedLocalStateModel: fixture.models.moves });
  const original = publicApi.decodeTypedProgramLinks(packet.network, packet.root).declarations.get('select');
  const edited = publicApi.decodeTypedProgramLinks(evidence.network, evidence.root).declarations.get('select');
  assert.deepEqual(edited.sourceLocalState, original.sourceLocalState);
  assert.deepEqual(edited.sourceSignature, original.sourceSignature);
  observe(evidence.originalCode, 'select', entry.observations);
  const observations = [['seed', '0', true, 'edited'], ['Ω😀', '-2147483648', false, '7']];
  observe(evidence.code, 'select', observations);
  const definitions = emitRust(evidence.program).definitions.join('\n\n');
  assert.ok(evidence.rustCode.includes(definitions));
  observe(consumer.reimportActualRustDefinitions(definitions).code, 'select', observations);
});

test('the exact unregistered public owned text model retains its version refusal', async context => {
  await assert.rejects(runConsumer('reject-unregistered-owned-text-model-version', context), error => {
    assert.equal(error.kind, 'unsupported');
    assert.equal(error.reason, 'Rust owned local state model: select the original move profile or canonical String copy profile');
    assert.equal(error.span, null);
    return true;
  });
});

test('the exact declared owned text selector requires actual Rust source', async context => {
  const consumer = await runConsumer('valid-owned-text-selection-type-with-wrong-source-language', context);
  assert.throws(() => consumer.createWrongLanguagePacket(), {
    name: 'TypeError', message: 'a Rust owned local state model requires Rust source',
  });
});
