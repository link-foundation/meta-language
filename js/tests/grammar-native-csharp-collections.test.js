import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = file => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = readData('parity/fixtures/native-grammars/csharp-collection-ci.json');
const projection = readData('parity/fixtures/native-grammars/csharp-profile.json');
const naming = readData('parity/naming/grammar-name-expansions.json');
const decisions = structuredClone(naming.grammars.csharp);
const collectionExpressionPreference = fixture.sourceTransforms[0];
decisions.sourceTransforms = decisions.sourceTransforms.filter(({ family, symbol }) => family !== collectionExpressionPreference.family || symbol !== collectionExpressionPreference.symbol);
decisions.sourceTransforms.push(collectionExpressionPreference);
const imported = importSource(grammarSourceOf('native-csharp'), new Map(naming.words.map(({ word, replacement }) => [word, replacement])), decisions);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('the six C# collection CI reproductions retain their pinned upstream sources and independent rows', () => {
  const corpus = readData('parity/fixtures/native-grammars/csharp-profile-corpus.json').cases;
  assert.deepEqual(fixture.corpus, grammarSourceOf('native-csharp').corpus);
  assert.deepEqual(fixture.cases.map(({ id }) => id), [9, 10, 11, 12, 13, 14]);
  for (const { file, title, source, rows } of fixture.cases) {
    assert.equal(corpus.find(entry => entry.file === file && entry.title === title)?.source, source, title);
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, title);
    assert.deepEqual(oracle.rows, rows, title);
    const result = parser.parseTree(source);
    assert.equal(result.ok, true, title);
    assert.deepEqual(result.ambiguities, [], title);
    assert.deepEqual(nativeRows(result.tree, source, projection), rows, title);
    assert.equal(text(result.tree), source, title);
  }
});

test('the collection preference preserves conditional element binding and ordinary element access', () => {
  for (const source of ['var result = owner?[1, 2];', 'var result = owner[1];', 'var combined = [1, ..others, 2,];']) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, source);
    const result = parser.parseTree(source);
    assert.equal(result.ok, true, source);
    assert.deepEqual(result.ambiguities, [], source);
    assert.deepEqual(nativeRows(result.tree, source, projection), oracle.rows, source);
    assert.equal(text(result.tree), source);
  }
});
