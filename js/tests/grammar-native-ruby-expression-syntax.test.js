import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';

const read = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = read('parity/fixtures/native-grammars/ruby-expression-syntax-ci.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-ruby');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.ruby);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = read('parity/fixtures/native-grammars/ruby-profile.json');
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('symbol context exclusions preserve declared external references and require exact selections', () => {
  const source = { rules: { operand: { type: 'SEQ', members: [{ type: 'STRING', value: 'a' }, { type: 'SYMBOL', name: '_slash' }] } }, externals: [{ type: 'SYMBOL', name: '_slash' }] };
  const decision = { family: 'symbol-pattern-exclusion', rule: 'operand', symbol: '_slash', pattern: '[ \\t\\r]*\\n', immediate: true, count: 1 };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(transformed.externals, source.externals);
  assert.deepEqual(transformed.rules.operand.members[1].content, source.rules.operand.members[1]);
  assert.equal(transformed.rules.operand.members[1].immediate, true);
  assert.equal(source.rules.operand.members[1].type, 'SYMBOL');
  assert.throws(() => transformNativeSource(source, [{ ...decision, count: 2 }]), /exactly/u);
  assert.throws(() => transformNativeSource(source, [{ ...decision, symbol: '_undeclared' }]), /existing/u);
});

test('Ruby expression syntax regressions retain exact CI inputs and registered decisions', () => {
  assert.equal(entry.id, fixture.source);
  assert.equal(entry.revision, fixture.revision);
  assert.equal(entry.sha256, fixture.sourceSha256);
  assert.equal(entry.corpus.sha256, fixture.corpusSha256);
  assert.equal(fixture.cases.length, 3);
  for (const decision of fixture.decisions) assert.ok(naming.grammars.ruby.sourceTransforms.some(value => JSON.stringify(value) === JSON.stringify(decision)));
  const corpus = read('parity/fixtures/native-grammars/ruby-profile-corpus.json').cases;
  for (const { file, title, source } of fixture.cases) assert.equal(source, corpus.find(value => value.file === file && value.title === title)?.source);
});

for (const { title, source, rows } of [...fixture.cases, ...fixture.controls]) test(`Ruby expression syntax default-budget oracle: ${title ?? source}`, () => {
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

test('Ruby incomplete assignment and regular-expression controls reject and recover losslessly', () => {
  for (const { source } of fixture.rejections) {
    assert.equal(oracleSnapshot(source, fixture.language).recovers, true, source);
    assert.equal(parser.parseTree(source).ok, false, source);
    const actual = parser.parseTree(source, { errorRecovery: true });
    assert.equal(actual.rejection.reason, 'recovered', source);
    assert.equal(text(actual.tree), source, source);
  }
});
