import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { grammarSourceOf, importSource } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { applySourceContextPreference } from '../scripts/native-source-preferences.mjs';

const read = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = read('parity/fixtures/native-grammars/csharp-pattern-context-ci.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-csharp');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.csharp);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = read('parity/fixtures/native-grammars/csharp-profile.json');
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('outer source contexts retain previous preferences and recognize syntactic suffixes across extras', () => {
  const symbol = name => ({ type: 'SYMBOL', name });
  const literal = value => ({ type: 'STRING', value });
  const grammar = { name: 'source_context', extras: [{ type: 'PATTERN', value: '[ \\t\\n]' }], rules: {
    document: { type: 'SEQ', members: [symbol('piece'), literal(')'), { type: 'CHOICE', members: [literal('{'), literal('name')] }] },
    piece: { type: 'NATIVE_COMPLETE_CONTEXT_VARIANT', preferred: symbol('type_value'), boundary: ')', content: symbol('value') },
    value: literal('item'), type_value: literal('item'), constant_value: literal('item'),
  } };
  const decision = { family: 'symbol-complete-context', rules: ['piece'], symbol: 'value', preferredSymbol: 'constant_value',
    outerContext: true, count: 1, boundaries: [{ type: 'SEQ', members: [literal(')'), literal('{')] }] };
  for (const change of [{ count: 2 }, { outerContext: 'yes' }, { boundaries: [symbol('absent')] }]) {
    const before = structuredClone(grammar);
    assert.throws(() => applySourceContextPreference(grammar, { ...decision, ...change }), TypeError);
    assert.deepEqual(grammar, before);
  }
  const previous = structuredClone(grammar.rules.piece);
  assert.equal(applySourceContextPreference(grammar, decision), true);
  assert.deepEqual(grammar.rules.piece.content, previous, 'the complete existing preference remains as fallback');
  const compiled = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(grammar))));
  const firstKind = source => {
    const result = compiled.parseTree(source);
    assert.equal(result.ok, true, source);
    assert.deepEqual(result.ambiguities, [], source);
    return result.tree.children[0].children[0].kind;
  };
  for (const source of ['item){', 'item) {', 'item)\n{']) assert.equal(firstKind(source), 'constant_value');
  for (const source of ['item)name', 'item) name']) assert.equal(firstKind(source), 'type_value');
});

test('CSharp enclosing pattern contexts retain exact second-run CI trees and source bytes', () => {
  for (const decision of fixture.sourceTransforms) {
    assert.ok(naming.grammars.csharp.sourceTransforms.some(other => JSON.stringify(other) === JSON.stringify(decision)), 'context policies must be registered source data');
  }
  assert.equal(entry.id, fixture.grammarSource);
  assert.equal(entry.revision, fixture.revision);
  assert.equal(entry.sha256, fixture.sourceSha256);
  assert.deepEqual(entry.corpus, fixture.corpus);
  assert.deepEqual(fixture.cases.map(({ id }) => id), [1, 2, 3]);
  const corpus = read('parity/fixtures/native-grammars/csharp-profile-corpus.json').cases;
  for (const { file, title, source, rows } of fixture.cases) {
    assert.equal(corpus.find(item => item.file === file && item.title === title)?.source, source, title);
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
