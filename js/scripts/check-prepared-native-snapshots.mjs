#!/usr/bin/env node
// CI-only full inventory probe. The caller preloads the file/network guard.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { readProjectData } from './project-data.mjs';
import { compileGrammar, parseGrammarLinks, structuredDataToLino } from '../src/index.js';

const root = resolve(process.env.META_LANGUAGE_TOOLING_ROOT);
const contract = readProjectData(resolve(process.argv[2]));
const inventory = readProjectData(join(root, contract.nativeInventory));
const manifest = readProjectData(join(root, contract.assetManifest));
assert.throws(() => readFileSync(join(root, Object.keys(manifest.assets)[0])), /external grammar asset access denied/u);
assert.throws(() => globalThis.fetch('https://example.invalid/'), /offline execution/u);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const sourceText = tree => tree.type === 'node' ? tree.children.map(sourceText).join('') : tree.text;
const snapshots = [];
const selected = new Set();
for (let index = 3; index < process.argv.length; index += 2) {
  assert.equal(process.argv[index], '--grammar');
  assert.ok(Object.hasOwn(inventory.nativeGrammars, process.argv[index + 1]), 'selected grammar belongs to the actual inventory');
  assert.equal(selected.has(process.argv[index + 1]), false, 'selected grammar is unique');
  selected.add(process.argv[index + 1]);
}
for (const [id, entry] of Object.entries(inventory.nativeGrammars)) {
  if (selected.size && !selected.has(id)) continue;
  const name = id.replace(/^native-/u, '');
  const fixtureFile = `parity/fixtures/native-grammars/${name}.lino`;
  const fixture = readProjectData(join(root, fixtureFile));
  const example = fixture.matches?.find(value => typeof value?.source === 'string' && value.source.length > 0)
    ?? fixture.matches?.find(value => typeof value?.source === 'string');
  assert.ok(example, `prepared grammar has a recorded accepted source: ${id}`);
  const listing = readFileSync(join(root, entry.grammar), 'utf8');
  const outcome = compileGrammar(parseGrammarLinks(listing)).parseTree(example.source);
  assert.equal(outcome.ok, true, `prepared native grammar executes without downloads: ${id}`);
  assert.equal(sourceText(outcome.tree), example.source, `prepared native grammar remains lossless: ${id}`);
  snapshots.push({ id, grammar: entry.grammar, grammarSha256: hash(listing), fixture: fixtureFile,
    sourceSha256: hash(example.source) });
}
assert.equal(snapshots.length, selected.size || Object.keys(inventory.nativeGrammars).length);
process.stdout.write(structuredDataToLino({ schemaVersion: 1, snapshots,
  scope: selected.size ? 'explicit selected grammars' : 'complete registered native inventory',
  externalAssetsDenied: true, networkDenied: true }));
