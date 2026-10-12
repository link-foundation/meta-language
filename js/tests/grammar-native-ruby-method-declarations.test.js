import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource, sourceText } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';

const read = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = read('parity/fixtures/native-grammars/ruby-method-declaration-context-ci.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-ruby');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.ruby);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = read('parity/fixtures/native-grammars/ruby-profile.json');
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const exact = ({ source, rows, title }) => {
  const oracle = oracleSnapshot(source, fixture.language);
  assert.equal(oracle.recovers, false, title ?? source);
  assert.deepEqual(oracle.rows, rows, title ?? source);
  const actual = parser.parseTree(source);
  assert.equal(actual.ok, true, `${title ?? source}: ${JSON.stringify(actual.rejection)}`);
  assert.deepEqual(actual.ambiguities, [], title ?? source);
  assert.deepEqual(nativeRows(actual.tree, source, projection), rows, title ?? source);
  assert.equal(text(actual.tree), source, title ?? source);
};

test('completed method declarations preserve every original statement and method production', () => {
  const original = JSON.parse(sourceText(entry));
  const transformed = transformNativeSource(original, fixture.sourceTransforms);
  assert.deepEqual(transformed.rules._statement.content, original.rules._statement, 'the full original statement remains the fallback');
  for (const name of ['method', 'singleton_method', '_method_rest', '_method_name', 'parameters', 'bare_parameters', '_expression']) {
    assert.deepEqual(transformed.rules[name], original.rules[name], `${name} retains all source alternatives`);
  }
  const stale = { ...fixture.sourceTransforms[0], count: 2 };
  assert.throws(() => transformNativeSource(original, [stale]), /exactly/u);
  assert.deepEqual(original, JSON.parse(sourceText(entry)), 'source pins remain unchanged');
});

test('the exact combined Ruby parameter CI source fits its unchanged default parser budget', () => {
  for (const decision of fixture.sourceTransforms) {
    assert.ok(naming.grammars.ruby.sourceTransforms.some(other => JSON.stringify(other) === JSON.stringify(decision)), 'preferences must be registered source data');
  }
  assert.equal(entry.id, fixture.grammarSource);
  assert.equal(entry.revision, fixture.revision);
  assert.equal(entry.sha256, fixture.sourceSha256);
  assert.deepEqual(entry.corpus, fixture.corpus);
  assert.equal(fixture.cases.length, 1);
  const item = fixture.cases[0];
  const corpus = read('parity/fixtures/native-grammars/ruby-profile-corpus.json').cases;
  assert.equal(corpus.find(other => other.file === item.file && other.title === item.title)?.source, item.source);
  assert.equal(Buffer.byteLength(item.source), 317);
  exact(item);
});

test('isolated parameters and expression-valued, operator, singleton, member and rescued definitions retain exact trees', () => {
  assert.equal(fixture.controls.length, 23);
  assert.deepEqual(fixture.controls.slice(0, 13).map(({ source }) => source), fixture.cases[0].source.trim().split('\n\n'));
  for (const item of fixture.controls) exact(item);
});
