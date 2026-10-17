import assert from 'node:assert/strict';
import { test } from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';

const symbol = name => ({ type: 'SYMBOL', name });
const sequence = (...members) => ({ type: 'SEQ', members });
const source = {
  name: 'adjacency', extras: [{ type: 'PATTERN', value: '[ \\t\\r\\n]' }],
  rules: {
    document: sequence(symbol('letter'), { type: 'REPEAT1', content: symbol('letter') }),
    letter: { type: 'PATTERN', value: '[a-z]' },
  },
};
const parserOf = grammar => compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(grammar))));

test('recorded immediate symbol exclusions preserve leading trivia and same-line continuations', () => {
  const decision = { family: 'symbol-pattern-exclusion', rule: 'document', symbol: 'letter', occurrence: 2,
    pattern: '[ \\t\\r]*\\n', count: 1, immediate: true };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(source.rules.document.members[1].content, symbol('letter'));
  const parser = parserOf(transformed);
  for (const text of ['ab', 'a b', '\na b\n', 'a\tb']) assert.equal(parser.parseTree(text).ok, true, text);
  for (const text of ['a\nb', 'a \r\nb']) assert.equal(parser.parseTree(text).ok, false, text);
  for (const change of [{ count: 2 }, { occurrence: 3 }, { symbol: 'missing' }, { immediate: 'yes' }]) {
    assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
  }
});

test('recorded immediate rule exclusions distinguish adjacent prefixes from separated arguments', () => {
  const input = { ...source, rules: { document: sequence(symbol('letter'), symbol('argument')), letter: source.rules.letter,
    argument: sequence({ type: 'STRING', value: '..' }, symbol('letter')) } };
  const decision = { family: 'rule-prefix-exclusion', rule: 'argument', pattern: '\\.{2}', immediate: true };
  const parser = parserOf(transformNativeSource(input, [decision]));
  assert.equal(parser.parseTree('a..b').ok, false);
  assert.equal(parser.parseTree('a ..b').ok, true);
  assert.throws(() => transformNativeSource(input, [{ ...decision, immediate: 1 }]), TypeError);
});

test('recorded selector-dependent following preserves every operator and explicit constant call', () => {
  const input = { name: 'selectors', extras: source.extras, rules: {
    document: sequence(symbol('call'), { type: 'CHOICE', members: [{ type: 'STRING', value: '()' }, { type: 'BLANK' }] }),
    call: sequence(symbol('receiver'), { type: 'FIELD', name: 'operator', content: symbol('operator') },
      { type: 'FIELD', name: 'method', content: { type: 'CHOICE', members: [symbol('identifier'), symbol('constant')] } }),
    receiver: { type: 'STRING', value: 'obj' }, identifier: { type: 'PATTERN', value: '[a-z]+' }, constant: { type: 'PATTERN', value: '[A-Z][a-z]+' },
    operator: { type: 'CHOICE', members: [{ type: 'STRING', value: '.' }, { type: 'IMMEDIATE_TOKEN', content: { type: 'STRING', value: '::' } }] },
  } };
  const decision = { family: 'field-dependent-following', rule: 'call', selectorField: 'operator', selectorRule: 'operator',
    selectorLiteral: '::', field: 'method', symbol: 'constant', followingPattern: '\\(', count: 1 };
  const parser = parserOf(transformNativeSource(input, [decision]));
  for (const text of ['obj.Name', 'obj.name', 'obj::name', 'obj::Name()']) assert.equal(parser.parseTree(text).ok, true, text);
  assert.equal(parser.parseTree('obj::Name').ok, false);
  for (const change of [{ count: 2 }, { selectorLiteral: 'missing' }, { field: 'missing' }, { symbol: 'missing' }]) {
    assert.throws(() => transformNativeSource(input, [{ ...decision, ...change }]), TypeError);
  }
  assert.equal(input.rules.call.type, 'SEQ');
});

test('recorded variant counts retain multiple matching source continuations', () => {
  const branch = literal => ({ type: 'PREC', value: 1, content: sequence({ type: 'STRING', value: literal },
    { type: 'FIELD', name: 'body', content: symbol('letter') }) });
  const input = { ...source, rules: { document: { type: 'CHOICE', members: [branch('a'), branch('b')] }, letter: source.rules.letter } };
  const decision = { family: 'rule-variant-precedence', rule: 'document', fields: { body: 'letter' },
    associativity: 'none', value: 2, count: 2 };
  const changed = transformNativeSource(input, [decision]);
  assert.equal(changed.rules.document.members.length, 2);
  assert.deepEqual(changed.rules.document.members.map(branch => branch.content), input.rules.document.members.map(branch => branch.content));
  assert.deepEqual(changed.rules.document.members.map(branch => branch.value), [2, 2]);
  assert.throws(() => transformNativeSource(input, [{ ...decision, count: 1 }]), TypeError);
  assert.throws(() => transformNativeSource(input, [{ ...decision, count: 0 }]), TypeError);
});
