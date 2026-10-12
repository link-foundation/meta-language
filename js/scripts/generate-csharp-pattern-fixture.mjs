#!/usr/bin/env node
// Only reported CI sources and their bounded pattern controls.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource } from './import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from './native-grammar-rows.mjs';

const read = file => JSON.parse(readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8'));
const argument = process.argv.indexOf('--fixture');
const fixtureName = argument < 0 ? 'csharp-pattern-ci' : process.argv[argument + 1];
if (!/^csharp-pattern(?:-[a-z]+)*-ci$/u.test(fixtureName ?? '')) throw new TypeError('select a CSharp pattern CI fixture name');
const target = new URL(`../../parity/fixtures/native-grammars/${fixtureName}.json`, import.meta.url);
const fixture = read(`parity/fixtures/native-grammars/${fixtureName}.json`);
const naming = read('parity/naming/grammar-name-expansions.json');
for (const decision of fixture.sourceTransforms) {
  assert.ok(naming.grammars.csharp.sourceTransforms.some(value => JSON.stringify(value) === JSON.stringify(decision)), 'all pattern preferences must be registered source data');
}
const source = grammarSourceOf('native-csharp');
const imported = importSource(source, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.csharp);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = read('parity/fixtures/native-grammars/csharp-profile.json');
const corpus = read('parity/fixtures/native-grammars/csharp-profile-corpus.json').cases;
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const snapshot = entry => {
  const oracle = oracleSnapshot(entry.source, fixture.language);
  assert.equal(oracle.recovers, false, entry.title ?? entry.source);
  const parsed = parser.parseTree(entry.source);
  assert.equal(parsed.ok, true, entry.title ?? entry.source);
  assert.deepEqual(parsed.ambiguities, [], entry.title ?? entry.source);
  assert.deepEqual(nativeRows(parsed.tree, entry.source, projection), oracle.rows, entry.title ?? entry.source);
  assert.equal(text(parsed.tree), entry.source);
  return { ...entry, rows: oracle.rows };
};
const cases = fixture.selections.map(selection => {
  const found = corpus.find(entry => entry.file === selection.file && entry.title === selection.title);
  assert.ok(found, 'each CI reproduction comes from the pinned corpus');
  return snapshot({ ...selection, source: found.source });
});
const controls = fixture.controls?.map(entry => snapshot(typeof entry === 'string' ? { source: entry } : { source: entry.source }));
const result = { ...fixture, grammarSource: source.id, revision: source.revision, sourceSha256: source.sha256, corpus: source.corpus, cases, ...(controls ? { controls } : {}) };
const expected = `${JSON.stringify(result, null, 2)}\n`;
if (process.argv.includes('--check')) assert.equal(readFileSync(target, 'utf8'), expected, 'bounded pattern fixture drift');
else writeFileSync(target, expected);
