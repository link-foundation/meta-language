import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import test from 'node:test';
import { transformOptionalCallContext } from '../scripts/native-optional-call-context.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { nativeName } from '../scripts/import-native-grammars.mjs';
import { nativeScannerProfile } from '../scripts/generate-native-scanner-profiles.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const symbol = name => ({ type: 'SYMBOL', name });
const blank = { type: 'BLANK' };
const miniature = () => ({
  name: 'optional_arguments',
  extras: [{ type: 'PATTERN', value: '[ \\t]' }],
  rules: {
    document: symbol('invocation'),
    invocation: { type: 'PREC_LEFT', value: 50, content: { type: 'SEQ', members: [
      { type: 'FIELD', name: 'target', content: symbol('identifier') },
      { type: 'CHOICE', members: [{ type: 'ALIAS', named: true, value: 'arguments', content: symbol('argument') }, blank] },
      { type: 'CHOICE', members: [{ type: 'STRING', value: '!' }, blank] },
    ] } },
    identifier: { type: 'PATTERN', value: '[a-z]+' },
    argument: { type: 'PATTERN', value: '[+-][0-9]+' },
  },
});
const decision = { rule: 'invocation', field: 'target', optionalSymbol: 'argument', pattern: '[ \\t]', count: 1 };
const leaves = node => node.type === 'token' ? [node] : node.children.flatMap(leaves);

test('optional argument contexts preserve aliases, precedence and every later suffix', () => {
  const grammar = miniature();
  const original = structuredClone(grammar.rules.invocation);
  transformOptionalCallContext(grammar, decision);
  const rewritten = grammar.rules.invocation;
  assert.equal(rewritten.type, original.type);
  assert.equal(rewritten.value, original.value);
  assert.equal(rewritten.content.type, 'CHOICE');
  const [present, absent] = rewritten.content.members;
  assert.deepEqual(present.members[0], original.content.members[0]);
  assert.deepEqual(present.members[1], { type: 'NATIVE_PATTERN_LOOKAHEAD', content: blank, pattern: decision.pattern, immediate: true });
  assert.deepEqual(present.members[2], original.content.members[1].members[0]);
  assert.deepEqual(present.members[3], original.content.members[2]);
  assert.deepEqual(absent.members, [original.content.members[0], blank, original.content.members[2]]);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(grammar))));
  for (const source of ['f', 'f!', 'f -1', 'f\t+2!', 'f -1!']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    assert.equal(leaves(outcome.tree).map(({ text }) => text).join(''), source);
  }
  for (const source of ['f-1', 'f+2!', 'f\n-1']) assert.equal(parser.parseTree(source).ok, false, source);
});

test('optional argument contexts validate selectors and exact counts before replacing a rule', () => {
  for (const change of [{ rule: 'missing' }, { field: '' }, { field: 'missing' }, { optionalSymbol: 'missing' }, { pattern: '' }, { count: 0 }, { count: 2 }, { count: 1.5 }]) {
    const grammar = miniature();
    const original = structuredClone(grammar);
    assert.throws(() => transformOptionalCallContext(grammar, { ...decision, ...change }), TypeError);
    assert.deepEqual(grammar, original);
  }
});

const read = file => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const naming = JSON.parse(read('parity/naming/grammar-name-expansions.json'));
const profile = naming.grammars.elixir;
const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
const entry = JSON.parse(read('parity/grammars/sources.json')).sources.find(({ language }) => language === 'elixir');
const upstream = JSON.parse(gunzipSync(readFileSync(new URL(`../../${entry.file}`, import.meta.url))));
const projection = JSON.parse(read('parity/fixtures/native-grammars/elixir-profile.json'));
// These bounded source decisions exercise the generic context operation on
// its pinned optional-call production without rewriting generated artifacts.
const contexts = [
  { family: 'symbol-kind-alias', rule: 'unary_operator', symbol: '_before_unary_op', alias: 'unary_operator_padding', count: 4 },
  ...['_local_call_without_parentheses', '_remote_call_without_parentheses', '_local_call_with_parentheses', '_remote_call_with_parentheses', '_anonymous_call', '_double_call'].map(rule => ({ family: 'rule-precedence', rule, value: 240, associativity: 'left' })),
  { family: 'field-following-context', rule: '_local_call_without_parentheses', field: 'target', pattern: '[ \\t]', count: 1 },
  { family: 'rule-prefix-exclusion', rule: '_capture_expression', pattern: '[0-9]' },
  { family: 'pattern-context-variant', rule: '_expression', symbol: 'unary_operator', pattern: '&[ \\t\\r\\n]*[^0-9 \\t\\r\\n]' },
];
const candidate = transformNativeSource(upstream, contexts);
transformOptionalCallContext(candidate, { rule: '_remote_call_without_parentheses', field: 'target', optionalSymbol: '_call_arguments_without_parentheses', pattern: '[ \\t]', count: 1 });
const imported = importTreeSitterNative(candidate, {
  nameOf: name => nativeName(name, words, profile),
  scanners: nativeScannerProfile(profile.scannerFamilies),
  immediate: entry.scanner.immediate,
});
const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));

test('optional remote argument contexts preserve no-argument calls, spaced arguments and prefix arithmetic', () => {
  for (const source of ['Mod.fun', 'Mod.fun 1', 'Mod.fun(1)', 'Mod.fun/1', 'Mod.fun+1', 'Mod.fun do\n x\nend', '@spec fun(atom) :: string', '@foo+1', '@foo 1+2', '@foo(1)+2', 'foo+1', 'foo -1']) {
    const oracle = oracleSnapshot(source, 'elixir');
    assert.equal(oracle.recovers, false, source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, source);
    assert.deepEqual(actual.ambiguities, [], source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), oracle.rows, source);
    assert.equal(leaves(actual.tree).map(({ text }) => text).join(''), source);
  }
});

test('capture contexts distinguish integer placeholders from complete remote and anonymous operands', () => {
  for (const source of ['&1+2', '&1.foo', '&Mod.fun/1', '&Kernel.>=/2', '& &1 + &2', '& &1.some_fun', '& &1.(&2)', '&fun/1', '&fun(1)', '&1+&2', '&(1+2)', '&(&1 + &2)', '&foo(&1, a, &2)', 'a & b', 'a && b']) {
    const oracle = oracleSnapshot(source, 'elixir');
    assert.equal(oracle.recovers, false, source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, source);
    assert.deepEqual(actual.ambiguities, [], source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), oracle.rows, source);
    assert.equal(leaves(actual.tree).map(({ text }) => text).join(''), source);
  }
});
