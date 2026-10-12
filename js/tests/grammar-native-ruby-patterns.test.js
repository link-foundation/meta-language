import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource, sourceText } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';

const read = file => JSON.parse(readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8'));
const fixture = read('parity/fixtures/native-grammars/ruby-pattern-ci.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-ruby');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.ruby);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = read('parity/fixtures/native-grammars/ruby-profile.json');
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('Ruby pattern clause boundaries retain source productions and identifier spellings', () => {
  const source = JSON.parse(sourceText(entry));
  const transformed = transformNativeSource(source, fixture.decisions);
  assert.equal(transformed.rules._statement.type, 'NATIVE_PREFIX_EXCLUSION');
  assert.deepEqual(transformed.rules._statement.content, source.rules._statement);
  for (const name of ['in_clause', 'identifier', '_call', '_method_name']) assert.deepEqual(transformed.rules[name], source.rules[name]);
  assert.deepEqual(source, JSON.parse(sourceText(entry)), 'source transformations do not mutate their pinned input');
});

test('Ruby pattern selections preserve six exact pinned CI sources', () => {
  assert.equal(entry.id, fixture.source);
  assert.equal(entry.revision, fixture.revision);
  assert.equal(entry.sha256, fixture.sourceSha256);
  assert.equal(entry.corpus.sha256, fixture.corpusSha256);
  assert.equal(fixture.cases.length, 6);
  for (const decision of fixture.decisions) assert.ok(naming.grammars.ruby.sourceTransforms.some(value => JSON.stringify(value) === JSON.stringify(decision)), 'the tested pattern boundary is a registered production decision');
  const corpus = read('parity/fixtures/native-grammars/ruby-profile-corpus.json').cases;
  for (const { file, title, source } of fixture.cases) assert.equal(source, corpus.find(value => value.file === file && value.title === title)?.source);
});

for (const { title, source, rows } of [...fixture.cases, ...fixture.controls]) test(`Ruby pattern default-budget oracle: ${title ?? source}`, () => {
  const label = title ?? source;
  const oracle = oracleSnapshot(source, fixture.language);
  assert.equal(oracle.recovers, false, label);
  assert.deepEqual(oracle.rows, rows, label);
  const actual = parser.parseTree(source);
  assert.equal(actual.ok, true, `${label}: ${JSON.stringify(actual.rejection)}`);
  assert.deepEqual(actual.ambiguities, [], label);
  assert.deepEqual(nativeRows(actual.tree, source, projection), rows, label);
  assert.equal(text(actual.tree), source, label);
});

test('Ruby invalid pattern controls still reject and recover losslessly', () => {
  for (const { source } of fixture.rejections) {
    assert.equal(oracleSnapshot(source, fixture.language).recovers, true, source);
    assert.equal(parser.parseTree(source).ok, false, source);
    const actual = parser.parseTree(source, { errorRecovery: true });
    assert.equal(actual.rejection.reason, 'recovered', source);
    assert.equal(text(actual.tree), source, source);
  }
});
