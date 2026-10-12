import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import test from 'node:test';
import { transformFieldExpressionContext } from '../scripts/native-field-expression-context.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { nativeName } from '../scripts/import-native-grammars.mjs';
import { nativeScannerProfile } from '../scripts/generate-native-scanner-profiles.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const symbol = name => ({ type: 'SYMBOL', name });
const field = (name, rule) => ({ type: 'FIELD', name, content: symbol(rule) });
const miniature = () => ({ name: 'field_context', rules: {
  expression: { type: 'CHOICE', members: [symbol('atom'), symbol('binary'), symbol('call'), symbol('unary')] },
  atom: { type: 'PATTERN', value: '[a-z]+' },
  binary: { type: 'SEQ', members: [field('left', 'expression'), { type: 'STRING', value: '+' }, field('right', 'expression')] },
  call: { type: 'CHOICE', members: [symbol('bare_call'), symbol('remote')] },
  bare_call: { type: 'SEQ', members: [symbol('atom'), symbol('argument')] },
  argument: symbol('expression'),
  remote: { type: 'SEQ', members: [field('left', 'expression'), { type: 'STRING', value: '.' }, symbol('atom'), { type: 'CHOICE', members: [symbol('argument'), { type: 'BLANK' }] }] },
  unary: { type: 'CHOICE', members: [
    { type: 'PREC', value: 10, content: { type: 'SEQ', members: [{ type: 'STRING', value: '-' }, field('operand', 'expression')] } },
    { type: 'PREC_DYNAMIC', value: -1, content: { type: 'PREC', value: 30, content: { type: 'SEQ', members: [{ type: 'STRING', value: '@' }, field('operand', 'expression')] } } },
  ] },
} });
const miniatureDecision = {
  clones: [
    { rule: 'expression', helper: '_field_expression', references: { call: '_field_call', unary: '_field_unary' }, exclude: { binary: 1 } },
    { rule: 'expression', helper: '_atomic_expression', exclude: { binary: 1, call: 1, unary: 1 } },
    { rule: 'call', helper: '_field_call', references: { remote: '_field_remote' }, exclude: { bare_call: 1 } },
    { rule: 'remote', helper: '_field_remote', exclude: { argument: 1 } },
    { rule: 'unary', helper: '_field_unary', precedences: { values: [30], count: 1 }, fields: [{ field: 'operand', symbol: 'expression', helper: '_atomic_expression', count: 1 }] },
  ],
  fields: [
    { rule: 'remote', field: 'left', symbol: 'expression', helper: '_field_expression', count: 1 },
    { rule: 'unary', field: 'operand', symbol: 'expression', helper: '_atomic_expression', precedence: 30, count: 1 },
  ],
};

test('field expression contexts preserve original kinds and scope exact operand fields', () => {
  const grammar = miniature();
  const original = structuredClone(grammar);
  transformFieldExpressionContext(grammar, miniatureDecision);
  assert.deepEqual(grammar.rules.expression, original.rules.expression);
  assert.deepEqual(grammar.rules.binary, original.rules.binary);
  assert.deepEqual(grammar.rules.call, original.rules.call);
  assert.deepEqual(grammar.rules.argument, original.rules.argument);
  assert.deepEqual(grammar.rules._field_expression.members, [symbol('atom'),
    { type: 'ALIAS', named: true, value: 'call', content: symbol('_field_call') },
    { type: 'ALIAS', named: true, value: 'unary', content: symbol('_field_unary') }]);
  assert.deepEqual(grammar.rules._field_remote.members.at(-1), { type: 'CHOICE', members: [{ type: 'BLANK' }] });
  assert.deepEqual(grammar.rules.unary.members[0], original.rules.unary.members[0]);
  assert.deepEqual(grammar.rules.unary.members[1].content.content.members[1], field('operand', '_atomic_expression'));
  assert.equal(grammar.rules._field_unary.members.length, 1);
});

test('field expression contexts validate all identities and occurrence counts before mutation', () => {
  const changes = [
    decision => { decision.clones[0].helper = 'visible'; },
    decision => { decision.clones[0].helper = 'expression'; },
    decision => { decision.clones[0].references.call = '_missing'; },
    decision => { decision.clones[0].exclude.binary = 2; },
    decision => { decision.clones[1].exclude.atom = 1; },
    decision => { decision.clones[4].precedences.count = 2; },
    decision => { decision.clones[4].fields[0].count = 2; },
    decision => { decision.fields[0].field = 'missing'; },
    decision => { decision.fields[1].precedence = 10; decision.fields[1].count = 2; },
  ];
  for (const change of changes) {
    const grammar = miniature();
    const original = structuredClone(grammar);
    const decision = structuredClone(miniatureDecision);
    change(decision);
    assert.throws(() => transformFieldExpressionContext(grammar, decision), TypeError);
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
const candidate = transformNativeSource(upstream, profile.sourceTransforms.filter(({ family }) => family !== 'field-expression-context'));
const postfix = {
  call: '_postfix_call', _call_without_parentheses: '_postfix_unparenthesized_call', _call_with_parentheses: '_postfix_parenthesized_call',
  _local_call_just_do_block: '_postfix_block_call', _remote_call_without_parentheses: '_postfix_remote_reference',
  _local_call_with_parentheses: '_postfix_local_parenthesized_call', _remote_call_with_parentheses: '_postfix_remote_parenthesized_call',
  _anonymous_call: '_postfix_anonymous_call', _double_call: '_postfix_repeated_call',
};
const attribute = {
  call: '_attribute_call', _call_without_parentheses: '_attribute_unparenthesized_call', _call_with_parentheses: '_attribute_parenthesized_call',
  _local_call_without_parentheses: '_attribute_local_unparenthesized_call', _local_call_just_do_block: '_attribute_block_call',
  _local_call_with_parentheses: '_attribute_local_parenthesized_call', _double_call: '_attribute_repeated_call',
};
const postfixAttribute = {
  ...attribute,
  call: '_postfix_attribute_call',
  _call_without_parentheses: '_postfix_attribute_unparenthesized_call',
  _call_with_parentheses: '_postfix_attribute_parenthesized_call',
};
const references = map => ({ ...map, unary_operator: '_postfix_unary_operand' });
const contextDecision = {
  clones: [
    ...Object.entries(postfix).map(([rule, helper]) => ({ rule, helper, references: references(postfix),
      ...(rule === '_call_without_parentheses' ? { exclude: { _local_call_without_parentheses: 1 } } : {}),
      ...(rule === '_remote_call_without_parentheses' ? { exclude: { _call_arguments_without_parentheses: 1 } } : {}),
    })),
    ...Object.entries(attribute).map(([rule, helper]) => ({ rule, helper, references: attribute,
      ...(rule === '_call_without_parentheses' ? { exclude: { _remote_call_without_parentheses: 1 } } : {}),
      ...(['_call_with_parentheses', '_double_call'].includes(rule) ? { exclude: { _remote_call_with_parentheses: 1, _anonymous_call: 1 } } : {}),
    })),
    ...['call', '_call_without_parentheses', '_call_with_parentheses'].map(rule => ({ rule, helper: postfixAttribute[rule], references: postfixAttribute,
      ...(rule === '_call_without_parentheses' ? { exclude: { _local_call_without_parentheses: 1, _remote_call_without_parentheses: 1 } } : {}),
      ...(rule === '_call_with_parentheses' ? { exclude: { _remote_call_with_parentheses: 1, _anonymous_call: 1 } } : {}),
    })),
    { rule: '_expression', helper: '_postfix_expression', references: references(postfix), exclude: { binary_operator: 1 } },
    { rule: '_expression', helper: '_attribute_expression', references: attribute, exclude: { binary_operator: 1, dot: 1, access_call: 1 } },
    { rule: '_expression', helper: '_postfix_attribute_expression', references: references(postfixAttribute), exclude: { binary_operator: 1, dot: 1, access_call: 1 } },
    { rule: 'unary_operator', helper: '_postfix_unary_operand', precedences: { values: [220, 235], count: 2 }, fields: [{ field: 'operand', symbol: '_expression', helper: '_postfix_attribute_expression', precedence: 220, count: 1 }] },
  ],
  fields: [
    ...['_remote_dot', 'dot', '_anonymous_dot'].map(rule => ({ rule, field: 'left', symbol: '_expression', helper: '_postfix_expression', count: 1 })),
    { rule: 'access_call', field: 'target', symbol: '_expression', helper: '_postfix_expression', count: 1 },
    { rule: 'unary_operator', field: 'operand', symbol: '_expression', helper: '_attribute_expression', precedence: 220, count: 1 },
  ],
};
transformFieldExpressionContext(candidate, contextDecision);
const imported = importTreeSitterNative(candidate, {
  nameOf: name => nativeName(name, words, profile), scanners: nativeScannerProfile(profile.scannerFamilies), immediate: entry.scanner.immediate,
});
const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
const leaves = node => node.type === 'token' ? [node] : node.children.flatMap(leaves);

test('postfix operand projections preserve remote calls, grouping, capture placeholders and attribute precedence', () => {
  for (const source of [
    '1 + Mod.inner_fun 1', 'Mod.outer_fun mid_fun inner_fun.(a)', '@mod.foo', '&1.foo', '-mod.foo', '(a+b).foo', 'foo(1).bar', 'foo x.foo',
    'a+b[c]', 'outer Mod.inner arg do\n a\nend', '@foo(1).bar', '@foo Mod.fun', '@spec fun(atom) :: string', '@foo+1', '@foo[1]',
    '@Mod.foo 1', '@foo value.bar', '@foo().bar + 1', '@foo(1) + 2', 'foo() do\n x\nend.bar',
    '@-1', '@!true', '@not false', '@&1', '@foo -1', '@foo !true', '@foo(1)(2).bar', '@foo.bar.baz', '@((1+2)).bar', '@1+2', '@-mod.foo',
    '@spec fun(Keyword.t()) :: String.t()', '@spec foo() :: String.t()', '@type name :: String.t()',
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
