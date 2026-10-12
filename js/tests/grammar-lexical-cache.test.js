import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource, corpusCases } from '../scripts/import-native-grammars.mjs';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = file => JSON.parse(readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8'));
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-ruby');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.ruby);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = { ...read('parity/fixtures/native-grammars/ruby-profile.json'), oracleKinds: nativeOracleKinds(imported.text) };
const corpus = corpusCases(entry);
const recorded = read('parity/fixtures/native-grammars/ruby-profile-corpus.json').cases;
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

for (const title of ['relational', 'minus, call, exponential, range', 'method call with reserved keyword args', 'method calls with splat argument']) {
  test(`Ruby fixed lexical matching preserves the default budget: ${title}`, () => {
    const selected = corpus.find(value => value.file === 'expressions.txt' && value.title === title);
    assert.ok(selected);
    assert.equal(selected.source, recorded.find(value => value.file === selected.file && value.title === title)?.source);
    const oracle = oracleSnapshot(selected.source, 'Ruby');
    assert.equal(oracle.recovers, false);
    const actual = parser.parseTree(selected.source);
    assert.equal(actual.ok, true, JSON.stringify(actual.rejection));
    assert.deepEqual(actual.ambiguities, []);
    assert.deepEqual(nativeRows(actual.tree, selected.source, projection), oracle.rows);
    assert.equal(text(actual.tree), selected.source);
  });
}

test('fixed lexical cache retains neighboring Ruby token contexts and invalid boundaries', () => {
  for (const source of ['a == b\n', 'a -1\n', 'foo if: true\n', 'foo(*bar)\n', 'foo(x, *)\n', 'foo(**)\n']) {
    const oracle = oracleSnapshot(source, 'Ruby');
    assert.equal(oracle.recovers, false, source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), oracle.rows, source);
    assert.deepEqual(actual.ambiguities, []);
    assert.equal(text(actual.tree), source);
  }
  const invalid = 'foo(*\n';
  assert.equal(oracleSnapshot(invalid, 'Ruby').recovers, true);
  assert.equal(parser.parseTree(invalid).ok, false);
  const recovered = parser.parseTree(invalid, { errorRecovery: true });
  assert.equal(recovered.rejection.reason, 'recovered');
  assert.equal(text(recovered.tree), invalid);
});
