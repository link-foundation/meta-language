import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { importSource } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

test('prefix boundaries reuse a declared scanner or production without replacing its source policy', () => {
  const source = { name: 'boundary', rules: { document: { type: 'STRING', value: 'x' } }, externals: [{ type: 'SYMBOL', name: '_operator_line' }] };
  const result = transformNativeSource(source, [{ family: 'rule-prefix-exclusion', rule: 'document', prefixSymbol: '_operator_line', immediate: true }]);
  assert.deepEqual(result.rules.document, {
    type: 'NATIVE_PREFIX_EXCLUSION', content: source.rules.document,
    prefixes: { type: 'SYMBOL', name: '_operator_line' }, immediate: true,
  });
  assert.deepEqual(result.externals, source.externals);
  for (const change of [{}, { prefixSymbol: '_missing' }, { prefixSymbol: '_operator_line', pattern: 'x' }, { pattern: '' }]) {
    assert.throws(() => transformNativeSource(source, [{ family: 'rule-prefix-exclusion', rule: 'document', ...change }]), TypeError);
  }
  assert.deepEqual(source.rules.document, { type: 'STRING', value: 'x' });
});

const read = file => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const naming = JSON.parse(read('parity/naming/grammar-name-expansions.json'));
const entry = JSON.parse(read('parity/grammars/sources.json')).sources.find(source => source.language === 'elixir');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.elixir);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = { ...JSON.parse(read('parity/fixtures/native-grammars/elixir-profile-sources.json')), oracleKinds: nativeOracleKinds(imported.text) };
const leaves = node => node.type === 'token' ? [node] : node.children.flatMap(leaves);

test('registered Elixir expression boundaries preserve operator continuations and complete block operands', () => {
  for (const source of [
    'a\n..\nb', 'x\nnot in\n[y]', 'x\nnot in[y]', 'a\n^^^\nb', 'a\n&&\nb', 'a\n!==\nb',
    'a\nnotinput', 'x\n+y', 'fn\n->1\nend', 'foo do: 1', 'foo dog',
    'foo do end..bar do end//baz do end',
  ]) {
    const oracle = oracleSnapshot(source, 'Elixir');
    assert.equal(oracle.recovers, false, source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, source);
    assert.deepEqual(actual.ambiguities, [], source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), oracle.rows, source);
    assert.equal(leaves(actual.tree).map(({ text }) => text).join(''), source);
  }
});
