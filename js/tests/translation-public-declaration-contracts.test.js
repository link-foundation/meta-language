import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { pathToFileURL } from 'node:url';
import vm from 'node:vm';
import { publicDeclarationConsumers, checkPublicDeclarationContracts } from '../scripts/check-public-declaration-contracts.mjs';
import { createTypedProgramLinks, emitTypedProgramLinks, eraseTypeScript } from '../src/index.js';

test('the exact declared public consumer executes with frozen RDF statements and both formal archive shapes', async (context) => {
  const source = publicDeclarationConsumers()[0].source;
  const erased = eraseTypeScript(source).code;
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-public-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, erased);
  const consumer = await import(pathToFileURL(file).href);
  assert.deepEqual(consumer.contractEvidence, {
    tripleCount: 1, sourceCount: 1, preservedDefinition: true, copiedDefinition: true,
    blankScope: 'typed consumer', literalDatatype: 'http://www.w3.org/1999/02/22-rdf-syntax-ns#langString',
    literalLanguage: 'en', predicateKind: 'identifier',
  });
});

test('required compiler checks fail explicitly when no executable was supplied or available', () => {
  assert.throws(() => checkPublicDeclarationContracts(), /existing TypeScript executable/u);
  assert.throws(() => checkPublicDeclarationContracts('/__missing_meta_language_typescript_compiler__'),
    /Cannot execute the required TypeScript compiler/u);
});

test('the declared combined Rust source profiles execute through the public entry point', async (context) => {
  const source = publicDeclarationConsumers().find(({ name }) =>
    name === 'valid-combined-guarded-completion-and-vector-profiles').source;
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-profile-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, eraseTypeScript(`${source}\nexport { packet, definition };`).code);
  const { packet, definition } = await import(pathToFileURL(file).href);
  assert.ok(packet.network.link(packet.root));
  assert.ok(definition.network.link(definition.root));
});

test('the declared numeric consumer verifies a Rust and JavaScript correspondence', async (context) => {
  const source = publicDeclarationConsumers().find(({ name }) =>
    name === 'valid-shared-numeric-function-correspondence').source;
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-numeric-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, eraseTypeScript(source).code);
  const { contractEvidence } = await import(pathToFileURL(file).href);
  assert.deepEqual(contractEvidence, {
    matched: true, model: 'closed binary floating point function',
  });
});

async function runHeaderConsumer(name, context) {
  const consumer = publicDeclarationConsumers().find(consumer => consumer.name === name);
  assert.ok(consumer, `declared header consumer ${name} exists`);
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-header-consumer-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const file = join(directory, 'consumer.mjs');
  writeFileSync(file, eraseTypeScript(consumer.source).code);
  return import(pathToFileURL(file).href);
}

test('the exact declared borrowed header consumer preserves ordered lifetimes and executes scalar values', async context => {
  const { contractEvidence } = await runHeaderConsumer('valid-selected-borrowed-Rust-source-header', context);
  assert.equal(contractEvidence.language, 'Rust');
  assert.equal(contractEvidence.root, true);
  assert.equal(contractEvidence.signature, true);
  assert.match(contractEvidence.rustCode, /pub fn choose<'a, 'b>\(flag: &'a bool, value: &'b f64\) -> f64/u);
  assert.match(contractEvidence.rustCode, /let flag = \*flag;/u);
  assert.match(contractEvidence.rustCode, /let value = \*value;/u);
  assert.match(contractEvidence.legacyCode, /pub fn choose\(flag: bool, value: f64\) -> f64/u);
  const runtime = {};
  vm.runInNewContext(contractEvidence.code, runtime, { timeout: 1000 });
  assert.equal(runtime.choose(true, 2), 2);
  assert.equal(runtime.choose(false, 2), -2);
  assert.ok(Object.is(runtime.choose(true, -0), -0));
});

test('the exact declared scalar-link edit changes behavior while the selected borrowed header remains intact', async context => {
  const { contractEvidence } = await runHeaderConsumer('valid-source-free-borrowed-Rust-header-value-edit', context);
  assert.equal(contractEvidence.language, 'Rust');
  assert.equal(contractEvidence.root, true);
  assert.equal(contractEvidence.signature, true);
  assert.match(contractEvidence.rustCode, /pub fn add_one\(value: &f64\) -> f64/u);
  assert.match(contractEvidence.rustCode, /let value = \*value;/u);
  const original = {}, changed = {};
  vm.runInNewContext(contractEvidence.originalCode, original, { timeout: 1000 });
  vm.runInNewContext(contractEvidence.changedCode, changed, { timeout: 1000 });
  assert.equal(original.add_one(3), 4);
  assert.equal(changed.add_one(3), 5);
});

test('the exact declared unknown header model fails at runtime and selected headers reject other targets', async context => {
  await assert.rejects(runHeaderConsumer('reject-unknown-Rust-source-header-emission-model', context), /Rust source header emission model/u);
  const packet = createTypedProgramLinks('fn keep(value:&f64)->f64{*value}fn main(){}', 'Rust',
    { rustSourceSignatureModel: 'Rust source function signature 1' });
  assert.throws(() => emitTypedProgramLinks(packet.network, packet.root, 'JavaScript',
    { rustSourceHeaderEmissionModel: 'primitive borrowed Rust function header 1' }), /requires Rust target/u);
});
