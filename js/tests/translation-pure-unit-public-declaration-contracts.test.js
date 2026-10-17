import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath, pathToFileURL } from 'node:url';
import vm from 'node:vm';
import { publicDeclarationConsumers } from '../scripts/check-public-declaration-contracts.mjs';
import { readProjectData } from '../scripts/project-data.mjs';

// An explicit producer path checks the actual public index before integration.
// CI uses this package's index and exactly the same declared consumer sources.
const modulePath = process.env.META_LANGUAGE_PURE_UNIT_PUBLIC_MODULE
  ?? fileURLToPath(new URL('../src/index.js', import.meta.url));
const moduleUrl = pathToFileURL(modulePath);
const publicApi = await import(moduleUrl.href);
const { emitRust } = await import(new URL('./translation/emit-rust.js', moduleUrl));
const privateFixture = readProjectData(new URL('../../parity/fixtures/rust-pure-unit-calls.lino', import.meta.url));
const publicFixture = readProjectData(new URL('../../parity/fixtures/rust-public-pure-unit-calls.lino', import.meta.url));
const consumers = new Map(publicDeclarationConsumers(modulePath).map(entry => [entry.name, entry]));

async function runConsumer(name, context) {
  const consumer = consumers.get(name);
  assert.ok(consumer, 'the actual declared Unit consumer exists: ' + name);
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-pure-unit-public-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, publicApi.eraseTypeScript(consumer.source).code);
  return import(pathToFileURL(file).href);
}

function execute(code) {
  const lines = [];
  const context = vm.createContext({ console: { log: (...values) => lines.push(values.map(String).join(' ')) } });
  vm.runInContext(code, context, { timeout: 1000 });
  return { context, output: lines.join('\n') };
}

function checkCalls(code, names, expected) {
  const runtime = execute(code);
  const calls = [];
  for (const name of names) {
    const original = runtime.context[name];
    assert.equal(typeof original, 'function', 'the actual owned helper is executable: ' + name);
    runtime.context[name] = (...arguments_) => {
      calls.push(name);
      return original(...arguments_);
    };
  }
  vm.runInContext('main();', runtime.context, { timeout: 1000 });
  assert.deepEqual(calls, expected);
  return runtime.output;
}

function checkOwners(program, entry, model, visibility) {
  assert.deepEqual(program.sourcePureUnitCalls, { model, language: 'Rust' });
  assert.deepEqual([...program.declarations.keys()], entry.names);
  assert.deepEqual(program.main.effects.map(effect => effect.value.fn), entry.calls);
  for (const name of entry.names) {
    const declaration = program.declarations.get(name);
    assert.deepEqual(declaration.params, []);
    assert.deepEqual(declaration.ret, { kind: 'unit' });
    assert.deepEqual(declaration.body, { k: 'unit', type: { kind: 'unit' } });
    assert.equal(declaration.sourcePureUnit.model, model);
    if (visibility === 'private') assert.equal(Object.hasOwn(declaration.sourcePureUnit, 'visibility'), false);
    else {
      const fact = declaration.sourcePureUnit.visibility;
      assert.equal(fact.form, 'public');
      assert.equal(entry.source.slice(fact.span.start, fact.span.end), 'pub');
    }
  }
}

function checkActualPublicReplay(consumer, evidence, entry) {
  let program = evidence.program;
  for (let cycle = 0; cycle < 3; cycle++) {
    const emitted = emitRust(program);
    const replay = consumer.reimportActualRustDefinitions(emitted.definitions.join('\n\n'));
    program = replay.program;
    checkOwners(program, { ...entry, source: emitted.definitions.join('\n\n'), calls: [] }, publicFixture.model, 'public');
    assert.equal(checkCalls(replay.code, entry.names, []), entry.output);
  }
}

for (const entry of privateFixture.cases) {
  test('the exact private Unit consumer retains ' + entry.id + ' and its actual definition replay boundary', async context => {
    const consumer = await runConsumer('valid-pure-unit-private-' + entry.id, context);
    const evidence = consumer.contractEvidence;
    checkOwners(evidence.program, entry, privateFixture.model, 'private');
    assert.equal(checkCalls(evidence.code, entry.names, entry.calls), entry.output);
    const definitions = emitRust(evidence.program).definitions.join('\n\n');
    assert.ok(evidence.rustCode.includes(definitions));
    if (entry.names.length) {
      assert.throws(() => consumer.reimportActualRustDefinitions(definitions), error => {
        assert.equal(error.kind, 'unsupported');
        assert.match(error.reason, /only flat private pure Unit functions/u);
        return true;
      });
    } else {
      const replay = consumer.reimportActualRustDefinitions(definitions);
      checkOwners(replay.program, entry, privateFixture.model, 'private');
      assert.equal(checkCalls(replay.code, [], []), entry.output);
    }
  });
}

for (const entry of publicFixture.cases) {
  test('the exact public Unit consumer retains ' + entry.id + ' and three actual complete definition replays', async context => {
    const consumer = await runConsumer('valid-pure-unit-public-' + entry.id, context);
    const evidence = consumer.contractEvidence;
    checkOwners(evidence.program, entry, publicFixture.model, 'public');
    assert.equal(checkCalls(evidence.code, entry.names, entry.calls), entry.output);
    assert.ok(evidence.rustCode.includes(emitRust(evidence.program).definitions.join('\n\n')));
    checkActualPublicReplay(consumer, evidence, entry);
  });
}

for (const entry of publicFixture.generated) {
  test('the exact complete JavaScript Unit consumer retains ' + entry.id + ' and its actual emitted definition set', async context => {
    const consumer = await runConsumer('valid-pure-unit-actual-JavaScript-' + entry.id, context);
    const evidence = consumer.contractEvidence;
    assert.equal(evidence.originalSource, entry.original);
    assert.equal(execute(evidence.originalSource).output, entry.output);
    const actual = emitRust(evidence.originalProgram);
    assert.equal(actual.definitions.join('\n\n'), entry.definitions);
    assert.equal(actual.text, entry.fullEmittedRust);
    assert.equal(evidence.actualRustCode, actual.text);
    assert.equal(entry.source, actual.definitions.join('\n\n') + '\nfn main(){}');
    checkOwners(evidence.program, entry, publicFixture.model, 'public');
    assert.equal(checkCalls(evidence.code, entry.names, []), entry.output);
    checkActualPublicReplay(consumer, evidence, entry);
    assert.throws(() => publicApi.createTypedProgramLinks(actual.text, 'Rust'), /function without a result: ml_main/u);
    assert.throws(() => publicApi.createTypedProgramLinks(actual.text, 'Rust', { rustPureUnitCallModel: publicFixture.model }),
      /attributes require a separate compilation context proof/u);
    const wrapper = actual.text.slice(actual.text.lastIndexOf('fn main()'));
    assert.throws(() => publicApi.createTypedProgramLinks(wrapper, 'Rust'), /std::env::args: the standard library is outside the portable core/u);
    assert.throws(() => publicApi.createTypedProgramLinks(wrapper, 'Rust', { rustPureUnitCallModel: publicFixture.model }));
    for (const invalid of publicFixture.javaScriptInvalid) {
      assert.throws(() => publicApi.createTypedProgramLinks(invalid.source, 'JavaScript'), error => {
        assert.equal(error.kind, invalid.kind);
        assert.equal(error.reason, invalid.reason);
        assert.deepEqual(error.span ?? null, invalid.span);
        return true;
      });
    }
  });
}

for (const selection of ['private', 'public']) {
  test('the exact unregistered ' + selection + ' Unit model retains its public runtime refusal', async context => {
    await assert.rejects(runConsumer('reject-unregistered-' + selection + '-pure-unit-model-version', context), error => {
      assert.equal(error.kind, 'unsupported');
      assert.equal(error.reason, 'Rust pure Unit-call model: select the legacy model, closed pure Rust unit calls 1 or closed public pure Rust unit calls 1');
      assert.equal(error.span, null);
      return true;
    });
  });

  test('the exact declared ' + selection + ' Unit selector requires actual Rust source', async context => {
    const consumer = await runConsumer('valid-' + selection + '-pure-unit-selection-type-with-wrong-source-language', context);
    assert.throws(() => consumer.createWrongLanguagePacket(), {
      name: 'TypeError', message: 'a Rust pure Unit-call model requires Rust source',
    });
  });
}
