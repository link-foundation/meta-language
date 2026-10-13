import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { pathToFileURL } from 'node:url';
import vm from 'node:vm';
import { publicDeclarationConsumers } from '../scripts/check-public-declaration-contracts.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, eraseTypeScript, LinkNetwork } from '../src/index.js';

async function runConsumer(name, context) {
  const consumer = publicDeclarationConsumers().find(consumer => consumer.name === name);
  assert.ok(consumer, `declared public consumer ${name} exists`);
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-selected-model-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, eraseTypeScript(consumer.source).code);
  return import(pathToFileURL(file).href);
}

function runtime(code) {
  const context = {};
  vm.runInNewContext(code, context, { timeout: 1000 });
  return context;
}

for (const language of ['Rust', 'JavaScript']) {
  test(`the exact ${language} declaration-loop consumer preserves owned targets and executes after source-free edits`, async context => {
    const { contractEvidence } = await runConsumer(`valid-selected-${language}-declaration-loop-profile`, context);
    const { network, root } = contractEvidence;
    const declaration = decodeTypedProgramLinks(network, root).declarations.get('run');
    assert.equal(declaration.sourceLoop.model, 'closed declaration scalar loop 1');
    assert.deepEqual(declaration.sourceLoop.continuation, { root: 'crate', path: ['run'] });
    assert.match(contractEvidence.rustCode, /pub fn run\(/u);
    const original = runtime(contractEvidence.code);
    assert.equal(original.run(0, 2), 2);
    assert.equal(original.run(0, 20_000), 20_000);
    assert.ok(Object.is(original.run(-0, -1), -0));
    const literals = network.links().filter(link => link.metadata().term === 'string value' && link.metadata().definition === '1');
    assert.equal(literals.length, 1);
    literals[0].metadata().definition = '2';
    const edited = LinkNetwork.fromLino(network.toLino());
    const code = emitTypedProgramLinks(edited, root, 'JavaScript').code;
    assert.equal(runtime(code).run(0, 3), 4);
    const reverse = createTypedProgramLinks(code, 'JavaScript', { javaScriptScalarLoopModel: 'closed declaration scalar loop 1' });
    assert.equal(runtime(emitTypedProgramLinks(reverse.network, reverse.root, 'JavaScript').code).run(0, 3), 4);
    assert.deepEqual(decodeTypedProgramLinks(reverse.network, reverse.root).declarations.get('run').sourceLoop.continuation,
      { root: 'crate', path: ['run'] });
  });

  test(`the exact unregistered ${language} declaration-loop version refuses at runtime`, async context => {
    await assert.rejects(runConsumer(`reject-unregistered-${language}-declaration-loop-version`, context), /scalar loop model/iu);
  });
}

test('the declaration-loop public selection preserves the existing model and default capture refusals', () => {
  const source = 'fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{run};}}fn main(){}';
  assert.throws(() => createTypedProgramLinks(source, 'Rust'), /mutable parameter/u);
  const fixture = readProjectData(new URL('../../parity/fixtures/rust-scalar-loops.lino', import.meta.url));
  for (const entry of fixture.bindingCaptureRefusals) {
    assert.throws(() => createTypedProgramLinks(entry.source, entry.language,
      { [entry.language === 'Rust' ? 'rustScalarLoopModel' : 'javaScriptScalarLoopModel']: 'closed primitive scalar loop 1' }), error => {
      assert.equal(error.reason, entry.reason);
      assert.deepEqual(error.span, entry.span);
      return true;
    });
  }
});

test('the exact closed borrowed-call consumer retains source headers and actual checked call targets', async context => {
  const { contractEvidence } = await runConsumer('valid-closed-borrowed-Rust-call-interface', context);
  const program = decodeTypedProgramLinks(contractEvidence.network, contractEvidence.root);
  assert.equal(program.declarations.get('caller').body.fn, 'leaf');
  assert.deepEqual(program.declarations.get('leaf').sourceSignature.parameters[0].wrappers, ['reference']);
  assert.match(contractEvidence.rustCode, /pub fn leaf\(value: &f64\) -> f64/u);
  assert.match(contractEvidence.rustCode, /pub fn caller\(value: f64\) -> f64/u);
  assert.match(contractEvidence.rustCode, /crate::leaf\(&\(/u);
  assert.match(contractEvidence.legacyCode, /pub fn leaf\(value: f64\) -> f64/u);
  assert.equal(runtime(contractEvidence.code).caller(3), 4);
});

test('the exact closed borrowed-call value edit changes the caller while retaining owned interfaces', async context => {
  const { contractEvidence } = await runConsumer('valid-source-free-closed-borrowed-call-value-edit', context);
  const program = decodeTypedProgramLinks(contractEvidence.network, contractEvidence.root);
  assert.equal(program.declarations.get('caller').body.fn, 'leaf');
  assert.deepEqual(program.declarations.get('leaf').sourceSignature.parameters[0].wrappers, ['reference']);
  assert.match(contractEvidence.rustCode, /pub fn leaf\(value: &f64\) -> f64/u);
  assert.match(contractEvidence.rustCode, /crate::leaf\(&\(/u);
  assert.equal(runtime(contractEvidence.originalCode).caller(3), 4);
  assert.equal(runtime(contractEvidence.changedCode).caller(3), 5);
});

test('the exact unregistered borrowed-call version refuses and the old leaf-only model retains its boundary', async context => {
  await assert.rejects(runConsumer('reject-unregistered-closed-borrowed-call-version', context), /Rust source header emission model/u);
  const packet = createTypedProgramLinks('fn leaf(value:&f64)->f64{*value+1.0}fn caller(value:f64)->f64{leaf(&value)}fn main(){}', 'Rust',
    { rustSourceSignatureModel: 'Rust source function signature 1' });
  assert.throws(() => emitTypedProgramLinks(packet.network, packet.root, 'Rust',
    { rustSourceHeaderEmissionModel: 'primitive borrowed Rust function header 1' }), /calls and effects/u);
  assert.throws(() => emitTypedProgramLinks(packet.network, packet.root, 'JavaScript',
    { rustSourceHeaderEmissionModel: 'closed primitive borrowed Rust call interface 1' }), /requires Rust target/u);
});

test('the exact qualified binary64 consumer preserves actual owners and canonical meaning after a value edit', async context => {
  const { contractEvidence } = await runConsumer('valid-standard-qualified-binary64-owner-value-edit', context);
  const declaration = decodeTypedProgramLinks(contractEvidence.network, contractEvidence.root).declarations.get('add_one');
  assert.equal(declaration.sourcePrimitiveOwners.model, 'standard primitive Rust binary64 owner 1');
  assert.deepEqual(declaration.sourcePrimitiveOwners.parameters[0].sourcePath, ['std', 'primitive', 'f64']);
  assert.deepEqual(declaration.sourcePrimitiveOwners.parameters[0].resolvedPath, ['std', 'primitive', 'f64']);
  assert.deepEqual(declaration.sourcePrimitiveOwners.result.sourcePath, ['std', 'primitive', 'f64']);
  assert.deepEqual(declaration.sourceSignature.parameters[0].owner.path, ['f64']);
  assert.deepEqual(declaration.sourceSignature.result.owner.path, ['f64']);
  assert.equal(runtime(contractEvidence.originalCode).add_one(3), 4);
  assert.equal(runtime(contractEvidence.changedCode).add_one(3), 5);
  assert.match(contractEvidence.rustCode, /pub fn add_one\(value: f64\) -> f64/u);
});

test('the exact imported binary64 consumer preserves its alias proof and primitive scalar observations', async context => {
  const { contractEvidence } = await runConsumer('valid-standard-imported-binary64-owner', context);
  const declaration = decodeTypedProgramLinks(contractEvidence.network, contractEvidence.root).declarations.get('keep');
  assert.deepEqual(declaration.sourcePrimitiveOwners.namespace.imports, [{ name: 'Real', target: ['std', 'primitive', 'f64'] }]);
  assert.deepEqual(declaration.sourcePrimitiveOwners.parameters[0].sourcePath, ['Real']);
  assert.deepEqual(declaration.sourcePrimitiveOwners.parameters[0].resolvedPath, ['std', 'primitive', 'f64']);
  assert.equal(declaration.sourcePrimitiveOwners.parameters[0].alias, 'Real');
  assert.deepEqual(declaration.sourceSignature.parameters[0].owner.path, ['f64']);
  const values = runtime(contractEvidence.code);
  assert.equal(values.keep(2), 2);
  assert.ok(Object.is(values.keep(-0), -0));
  assert.ok(Number.isNaN(values.keep(NaN)));
});

test('the exact unregistered binary64 owner version refuses and wrong-language/default selections stay explicit', async context => {
  await assert.rejects(runConsumer('reject-unregistered-standard-binary64-owner-version', context), /primitive owner model/iu);
  assert.throws(() => createTypedProgramLinks('function keep(value){return value;}', 'JavaScript',
    { rustPrimitiveOwnerModel: 'standard primitive Rust binary64 owner 1' }), /requires Rust source/u);
  assert.throws(() => createTypedProgramLinks('fn keep(value:std::primitive::f64)->std::primitive::f64{value}fn main(){}', 'Rust'));
});

test('the exact scoped-binding consumer reimports every complete module and retains executable loop edits', async context => {
  const { contractEvidence } = await runConsumer('valid-complete-repeated-JavaScript-scoped-binding-output', context);
  assert.equal(contractEvidence.codes.length, 4);
  for (const code of contractEvidence.codes) {
    const values = runtime(code);
    assert.equal(values.run(0, 2), 2);
    assert.equal(values.run(0, 20_000), 20_000);
    assert.ok(Object.is(values.run(-0, -1), -0));
  }
  assert.equal(runtime(contractEvidence.editedCode).run(0, 3), 4);
  assert.equal(runtime(contractEvidence.replayCode).run(0, 3), 4);
  const declaration = decodeTypedProgramLinks(contractEvidence.network, contractEvidence.root).declarations.get('run');
  assert.equal(declaration.sourceLoop.model, 'closed declaration scalar loop 1');
  assert.deepEqual(declaration.sourceLoop.continuation, { root: 'crate', path: ['run'] });
});

test('the exact unregistered scoped-binding version refuses and selected aliases require a JavaScript target', async context => {
  await assert.rejects(runConsumer('reject-unregistered-JavaScript-scoped-binding-emission-version', context), /scoped binding.*model/iu);
  const packet = createTypedProgramLinks('fn keep(value:f64)->f64{value}fn main(){}', 'Rust');
  assert.throws(() => emitTypedProgramLinks(packet.network, packet.root, 'Rust',
    { javaScriptScopedBindingEmissionModel: 'owned JavaScript scoped bindings 1' }), /requires JavaScript target/u);
});

test('default complete-module replay retains its reserved synthesized binding refusal', () => {
  const source = 'fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{run};}}fn main(){}';
  const original = createTypedProgramLinks(source, 'Rust', { rustScalarLoopModel: 'closed declaration scalar loop 1' });
  const firstCode = emitTypedProgramLinks(original.network, original.root, 'JavaScript').code;
  const firstReplay = createTypedProgramLinks(firstCode, 'JavaScript', { javaScriptScalarLoopModel: 'closed declaration scalar loop 1' });
  const secondCode = emitTypedProgramLinks(firstReplay.network, firstReplay.root, 'JavaScript').code;
  assert.throws(() => createTypedProgramLinks(secondCode, 'JavaScript',
    { javaScriptScalarLoopModel: 'closed declaration scalar loop 1' }), error => {
    assert.equal(error.reason, "reserved identifier: ml_main_ignored1 uses the translator's reserved ml_ prefix");
    assert.deepEqual(error.span, { start: 390, end: 406 });
    return true;
  });
});
