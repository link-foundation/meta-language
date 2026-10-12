import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import test from 'node:test';
import { transformArgumentSuffixContext } from '../scripts/native-argument-suffix-context.mjs';
import { transformOptionalCallContext } from '../scripts/native-optional-call-context.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { nativeName } from '../scripts/import-native-grammars.mjs';
import { nativeScannerProfile } from '../scripts/generate-native-scanner-profiles.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const symbol = name => ({ type: 'SYMBOL', name });
const blank = { type: 'BLANK' };
const miniature = () => ({ name: 'argument_suffix', rules: {
  document: symbol('_expression'),
  _expression: { type: 'CHOICE', members: [symbol('identifier'), symbol('call'), symbol('group')] },
  call: { type: 'CHOICE', members: [symbol('_optional_call'), symbol('_required_call')] },
  _optional_call: { type: 'PREC_RIGHT', value: 50, content: { type: 'SEQ', members: [symbol('identifier'), symbol('arguments'), { type: 'CHOICE', members: [symbol('suffix'), blank] }] } },
  _required_call: { type: 'SEQ', members: [symbol('identifier'), symbol('suffix')] },
  arguments: symbol('_expression'),
  group: { type: 'SEQ', members: [{ type: 'STRING', value: '(' }, symbol('_expression'), { type: 'STRING', value: ')' }] },
  suffix: { type: 'STRING', value: '!' },
  identifier: { type: 'PATTERN', value: '[a-z]+' },
} });
const decision = {
  argumentRule: 'arguments', expressionRule: '_expression', suffixRule: 'suffix', expressionHelper: '_argument_expression',
  ruleHelpers: { call: '_argument_call', _optional_call: '_argument_optional_call' },
  excludedRules: ['_required_call'], expressionCount: 1, expressionCallCount: 1,
  suffixCounts: { call: 0, _optional_call: 1 }, exclusionCounts: { _required_call: 1 },
};

test('argument suffix contexts copy call definitions and retain parentheses in their original context', () => {
  const grammar = miniature();
  const original = structuredClone(grammar);
  transformArgumentSuffixContext(grammar, decision);
  for (const [name, body] of Object.entries(original.rules)) {
    if (name !== 'arguments') assert.deepEqual(grammar.rules[name], body, name);
  }
  assert.deepEqual(grammar.rules.arguments, symbol('_argument_expression'));
  assert.deepEqual(grammar.rules._argument_call, { type: 'CHOICE', members: [symbol('_argument_optional_call')] });
  assert.deepEqual(grammar.rules._argument_expression.members, [symbol('identifier'), { type: 'ALIAS', named: true, value: 'call', content: symbol('_argument_call') }, symbol('group')]);
  assert.equal(grammar.rules._argument_optional_call.type, 'PREC_RIGHT');
  assert.equal(grammar.rules._argument_optional_call.value, 50);
  assert.deepEqual(grammar.rules._argument_optional_call.content.members, [symbol('identifier'), symbol('arguments'), blank]);
  assert.deepEqual(grammar.rules.group, original.rules.group);
});

test('argument suffix contexts reject changed selectors and counts without partially adding helpers', () => {
  const changes = [
    { argumentRule: 'missing' }, { expressionRule: 'missing' }, { suffixRule: 'missing' },
    { expressionHelper: 'visible' }, { expressionHelper: '_expression' }, { expressionHelper: '_argument_call' },
    { ruleHelpers: { call: '_same', _optional_call: '_same' } }, { excludedRules: ['missing'] },
    { expressionCount: 2 }, { expressionCallCount: 2 }, { suffixCounts: { call: 0, _optional_call: 2 } },
    { exclusionCounts: { _required_call: 2 } }, { suffixCounts: { call: 0 } },
  ];
  for (const change of changes) {
    const grammar = miniature();
    const original = structuredClone(grammar);
    assert.throws(() => transformArgumentSuffixContext(grammar, { ...decision, ...change }), TypeError);
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
const calls = ['_local_call_without_parentheses', '_remote_call_without_parentheses', '_local_call_with_parentheses', '_remote_call_with_parentheses', '_anonymous_call', '_double_call'];
const candidate = transformNativeSource(upstream, [
  ...calls.map(rule => ({ family: 'rule-precedence', rule, value: 240, associativity: rule.includes('without_parentheses') ? 'right' : 'left' })),
  { family: 'field-following-context', rule: '_local_call_without_parentheses', field: 'target', pattern: '[ \\t]', count: 1 },
]);
transformOptionalCallContext(candidate, { rule: '_remote_call_without_parentheses', field: 'target', optionalSymbol: '_call_arguments_without_parentheses', pattern: '[ \\t]', count: 1 });
const ruleHelpers = {
  call: '_bare_argument_call',
  _call_without_parentheses: '_bare_argument_unparenthesized_call',
  _call_with_parentheses: '_bare_argument_parenthesized_call',
  _local_call_without_parentheses: '_bare_argument_local_unparenthesized_call',
  _remote_call_without_parentheses: '_bare_argument_remote_unparenthesized_call',
  _local_call_with_parentheses: '_bare_argument_local_parenthesized_call',
  _remote_call_with_parentheses: '_bare_argument_remote_parenthesized_call',
  _anonymous_call: '_bare_argument_anonymous_call',
  _double_call: '_bare_argument_repeated_call',
};
transformArgumentSuffixContext(candidate, {
  argumentRule: '_call_arguments_without_parentheses', expressionRule: '_expression', suffixRule: 'do_block', expressionHelper: '_bare_argument_expression',
  ruleHelpers, excludedRules: ['_local_call_just_do_block'], expressionCount: 2, expressionCallCount: 1,
  suffixCounts: Object.fromEntries(Object.keys(ruleHelpers).map(name => [name, ['call', '_call_without_parentheses', '_call_with_parentheses', '_anonymous_call'].includes(name) ? 0 : name === '_remote_call_without_parentheses' ? 2 : 1])),
  exclusionCounts: { _local_call_just_do_block: 1 },
});
const imported = importTreeSitterNative(candidate, {
  nameOf: name => nativeName(name, words, profile), scanners: nativeScannerProfile(profile.scannerFamilies), immediate: entry.scanner.immediate,
});
const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
const leaves = node => node.type === 'token' ? [node] : node.children.flatMap(leaves);

test('bare argument call projection attaches blocks to the enclosing call and preserves explicit grouping', () => {
  for (const source of [
    'outer_fun inner_fun arg do\n a\nend',
    'outer inner(arg) do\n a\nend',
    'outer(inner arg do\n a\nend)',
    'outer (inner arg do\n a\nend)',
    'outer (inner arg) do\n a\nend',
    'outer inner arg, other do\n a\nend',
    'outer inner(arg do\n a\nend)',
    'outer inner arg do\n other next value\nend',
    'Mod.outer inner arg do\n a\nend',
  ]) {
    const oracle = oracleSnapshot(source, 'elixir');
    assert.equal(oracle.recovers, false, source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, source);
    assert.deepEqual(actual.ambiguities, [], source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), oracle.rows, source);
    assert.equal(leaves(actual.tree).map(({ text }) => text).join(''), source);
  }
});
