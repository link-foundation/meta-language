import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';

const read = path => JSON.parse(readFileSync(new URL(`../../${path}`, import.meta.url), 'utf8'));

test('recorded separator partitioning preserves characters and validates its exact source selection', () => {
  const source = { rules: { source: { type: 'STRING', value: 'x' } }, extras: [{ type: 'PATTERN', value: '[\\s\\u00A0]+' }] };
  const decision = { family: 'split-character-separator', pattern: '[\\s\\u00A0]+', count: 1 };
  const transformed = transformNativeSource(source, [decision]);
  assert.equal(source.extras[0].value, '[\\s\\u00A0]+');
  assert.equal(transformed.extras[0].value, '[\\s\\u00A0]');
  for (const text of [' ', '\t', '\r', '\n', '\u00A0', '\uFEFF', 'x']) assert.equal(new RegExp(`^(?:${source.extras[0].value})$`, 'u').test(text), new RegExp(`^(?:${transformed.extras[0].value})$`, 'u').test(text));
  assert.throws(() => transformNativeSource(source, [{ ...decision, count: 2 }]), /exactly/u);
  assert.throws(() => transformNativeSource(source, [{ ...decision, pattern: '.+' }]), /character class/u);
});

test('CSharp preprocessor directives preserve every pinned conditional and directive tree', () => {
  const naming = read('parity/naming/grammar-name-expansions.json');
  const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
  const imported = importSource(grammarSourceOf('native-csharp'), words, naming.grammars.csharp);
  const parser = compileGrammar(parseGrammarLinks(imported.text));
  const fixture = read('parity/fixtures/native-grammars/csharp-profile.json');
  const corpus = read('parity/fixtures/native-grammars/csharp-profile-corpus.json');
  const titles = ['If, elif and else directives', 'If conditions', 'Define and undefine directives', 'Line directives'];
  const cases = corpus.cases.filter(({ file, title }) => file === 'preprocessor.txt' && titles.includes(title));
  assert.equal(cases.length, titles.length);
  for (const { title, source } of cases) {
    const oracle = oracleSnapshot(source, 'csharp');
    assert.equal(oracle.recovers, false, title);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, title);
    assert.deepEqual(actual.ambiguities, [], title);
    assert.deepEqual(nativeRows(actual.tree, source, fixture), oracle.rows, title);
    const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
    assert.equal(text(actual.tree), source, title);
  }
});
