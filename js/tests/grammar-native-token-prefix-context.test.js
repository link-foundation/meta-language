import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { transformTokenPrefixExclusion } from '../scripts/native-token-prefix-context.mjs';
import { nativeName, scannerText, sourceText } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const token = content => ({ type: 'TOKEN', content });
const precedence = content => ({ type: 'PREC', value: -2, content });
const input = { name: 'token_prefix', rules: {
  document: { type: 'STRING', value: 'x' },
  comment: token(precedence({ type: 'PATTERN', value: '#[^\\n]*' })),
  _prefix: { type: 'STRING', value: '#{' },
} };

test('token prefix context retains lexical identity and every precedence wrapper', () => {
  const changed = structuredClone(input);
  transformTokenPrefixExclusion(changed, { rule: 'comment', prefixSymbol: '_prefix', immediate: true, count: 1 });
  assert.equal(changed.rules.comment.type, 'TOKEN');
  assert.equal(changed.rules.comment.content.type, 'PREC');
  assert.equal(changed.rules.comment.content.value, -2);
  assert.deepEqual(changed.rules.comment.content.content.content, input.rules.comment.content.content);
  const imported = importTreeSitterNative(changed);
  assert.equal(imported.rules.find(({ sourceName }) => sourceName === 'comment').kind, 'token');
  assert.match(renderTreeSitterNative(imported), /\(rule comment token \(lexicalPrecedence -2 \(seq \(not \(immediateToken \(and/u);
  for (const change of [{ count: 0 }, { count: 2 }, { rule: 'missing' }, { prefixSymbol: 'missing' }, { immediate: 'yes' }, { pattern: '#' }]) {
    const copy = structuredClone(input);
    assert.throws(() => transformTokenPrefixExclusion(copy, { rule: 'comment', prefixSymbol: '_prefix', immediate: true, count: 1, ...change }), TypeError);
    assert.deepEqual(copy, input);
  }
});

const naming = read('parity/naming/grammar-name-expansions.json');
const expansions = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
const decisions = naming.grammars.ruby;
const entry = read('parity/grammars/sources.json').sources.find(({ id }) => id === 'tree-sitter-ruby');
const source = transformNativeSource(JSON.parse(sourceText(entry)), decisions.sourceTransforms);
transformTokenPrefixExclusion(source, { rule: 'comment', prefixSymbol: '_interpolation_start', immediate: true, count: 1 });
const imported = importTreeSitterNative(source, { nameOf: name => nativeName(name, expansions, decisions),
  wordRule: 'word_characters', scanners: scannerText(entry), immediate: entry.scanner.immediate });
const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
const projection = read('parity/fixtures/native-grammars/ruby-profile.json');
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('active interpolation prefixes cannot be swallowed by a comment after an input-ended literal', () => {
  const source = 'x="a",%\n#{';
  assert.equal(oracleSnapshot(source, 'ruby').recovers, true);
  assert.equal(parser.parseTree(source).ok, false);
  const recovered = parser.parseTree(source, { errorRecovery: true });
  assert.equal(recovered.rejection.reason, 'recovered');
  assert.equal(text(recovered.tree), source);
});

test('ordinary comments and comments within interpolation code retain their independent trees', () => {
  for (const source of ['#{ root comment\nx=1\n', 'x="#{1 #{ comment\n}"', 'x="a",%\n#{1}']) {
    const oracle = oracleSnapshot(source, 'ruby');
    assert.equal(oracle.recovers, false, source);
    const native = parser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, projection), oracle.rows, source);
    assert.equal(text(native.tree), source);
  }
});
