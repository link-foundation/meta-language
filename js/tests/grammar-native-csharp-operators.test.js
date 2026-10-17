import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource, sourceText } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = file => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = readData('parity/fixtures/native-grammars/csharp-operator-ci.json');
const projection = readData('parity/fixtures/native-grammars/csharp-profile.json');
const naming = readData('parity/naming/grammar-name-expansions.json');
const sourceEntry = grammarSourceOf('native-csharp');
const decisions = structuredClone(naming.grammars.csharp);
const key = ({ family, rule, rules, symbol, occurrence }) => [family, rule, rules?.join(','), symbol, occurrence].join('|');
const recorded = new Set(fixture.sourceTransforms.map(key));
decisions.sourceTransforms = decisions.sourceTransforms.filter(decision => !recorded.has(key(decision)));
decisions.sourceTransforms.push(...fixture.sourceTransforms);
const imported = importSource(sourceEntry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), decisions);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

function agreesWithOracle(source, rows = oracleSnapshot(source, fixture.language).rows) {
  const result = parser.parseTree(source);
  assert.equal(result.ok, true, source);
  assert.deepEqual(result.ambiguities, [], source);
  assert.deepEqual(nativeRows(result.tree, source, projection), rows, source);
  assert.equal(text(result.tree), source);
}

test('C# cast and initializer reproductions retain exact CI sources and independent rows', () => {
  const corpus = readData('parity/fixtures/native-grammars/csharp-profile-corpus.json').cases;
  assert.deepEqual(fixture.corpus, sourceEntry.corpus);
  assert.deepEqual(fixture.cases.map(({ id }) => id), [8, 23]);
  const original = JSON.parse(sourceText(sourceEntry));
  const first = original.rules.binary_expression.members[0].content.members;
  assert.equal(first[0].name, fixture.sourceTransforms[0].sourceField);
  assert.deepEqual(first[0].content, { type: 'SYMBOL', name: fixture.sourceTransforms[0].symbol });
  assert.equal(first[1].content.value, fixture.sourceTransforms[0].sourceOperator);
  for (const { file, title, source, rows } of fixture.cases) {
    assert.equal(corpus.find(entry => entry.file === file && entry.title === title)?.source, source, title);
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, title);
    assert.deepEqual(oracle.rows, rows, title);
    agreesWithOracle(source, rows);
  }
});

test('C# operand preferences preserve arithmetic, conjunction, generic calls and identifier spellings', () => {
  for (const source of [
    'bool c = a && b;', 'int x = (a) + b;', 'var x = (a + b) * c;',
    'var x = (int) b + c;', 'var x = (a) && b + c;', 'var x = (a+b) && c;',
    'var x = ((a)) && b;', 'var x = Fn<T>(1);', 'var x = a < b > (c);',
    'var x = (Café) && b;', 'var x = (Namespace.Type) && b;',
    'var x = (@type) && b;', 'var x = (\\u0061) && b;',
  ]) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, source);
    agreesWithOracle(source, oracle.rows);
  }
});
