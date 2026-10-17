#!/usr/bin/env node
import { structuredDataToLino } from '../src/structured-data-links.js';
import { readProjectData, resolveProjectDataPath } from './project-data.mjs';
// Select only reported Ruby CI sources and frozen contextual controls.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource } from './import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from './native-grammar-rows.mjs';

const read = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const argument = process.argv.indexOf('--fixture');
const fixtureName = argument < 0 ? 'ruby-heredoc-call-context-ci' : process.argv[argument + 1];
if (!/^ruby-[a-z]+(?:-[a-z]+)*-ci$/u.test(fixtureName ?? '')) throw new TypeError('select a Ruby source context CI fixture name');
const target = new URL(`../../parity/fixtures/native-grammars/${fixtureName}.json`, import.meta.url);
const fixture = read(`parity/fixtures/native-grammars/${fixtureName}.json`);
const naming = read('parity/naming/grammar-name-expansions.json');
for (const decision of fixture.sourceTransforms) {
  assert.ok(naming.grammars.ruby.sourceTransforms.some(other => JSON.stringify(other) === JSON.stringify(decision)), 'Ruby context preferences must be registered source data');
}
const entry = grammarSourceOf('native-ruby');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.ruby);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = read('parity/fixtures/native-grammars/ruby-profile.json');
const corpus = read('parity/fixtures/native-grammars/ruby-profile-corpus.json').cases;
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const snapshot = item => {
  const oracle = oracleSnapshot(item.source, fixture.language);
  assert.equal(oracle.recovers, false, item.title ?? item.source);
  const parsed = parser.parseTree(item.source);
  assert.equal(parsed.ok, true, item.title ?? item.source);
  assert.deepEqual(parsed.ambiguities, [], item.title ?? item.source);
  assert.deepEqual(nativeRows(parsed.tree, item.source, projection), oracle.rows, item.title ?? item.source);
  assert.equal(text(parsed.tree), item.source);
  return { ...item, rows: oracle.rows };
};
const cases = fixture.cases.map(({ file, title }) => {
  const item = corpus.find(other => other.file === file && other.title === title);
  assert.ok(item, 'each ambiguity reproduction is retained verbatim from the pinned corpus');
  return snapshot(item);
});
const controls = fixture.controls.map(({ rows: _rows, ...item }) => snapshot(item));
const result = { ...fixture, grammarSource: entry.id, revision: entry.revision, sourceSha256: entry.sha256, corpus: entry.corpus, cases, controls };
const rendered = structuredDataToLino(result);
if (process.argv.includes('--check')) assert.equal(readFileSync(resolveProjectDataPath(target), 'utf8'), rendered, 'Ruby context fixture drift');
else writeFileSync(resolveProjectDataPath(target), rendered);
