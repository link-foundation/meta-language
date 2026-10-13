import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { pathToFileURL } from 'node:url';
import { publicDeclarationConsumers, checkPublicDeclarationContracts } from '../scripts/check-public-declaration-contracts.mjs';
import { eraseTypeScript } from '../src/index.js';

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
