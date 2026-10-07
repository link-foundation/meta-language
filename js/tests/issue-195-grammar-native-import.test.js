// The import pipeline of the native merged grammars,
// js/scripts/import-native-grammars.mjs: how it reads the pinned upstream
// test corpora and how js/src/grammar-importers/tree-sitter-native.js turns
// a pinned tree-sitter grammar into a native Links Notation grammar.
import assert from 'node:assert/strict';
import { test } from 'node:test';

import { nativeCorpusFailure } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { corpusFileCases, nativeName, ruleConcept } from '../scripts/import-native-grammars.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const rule = (character, length = 80) => character.repeat(length);

test('an alias of an optional hidden rule preserves its child node and builds no absent node', () => {
  const grammar = {
    name: 'optional_body',
    rules: {
      document: { type: 'SEQ', members: [
        { type: 'ALIAS', named: true, value: 'block', content: { type: 'CHOICE', members: [
          { type: 'SYMBOL', name: '_body' }, { type: 'BLANK' },
        ] } },
        { type: 'STRING', value: '!' },
      ] },
      _body: { type: 'SYMBOL', name: 'return_statement' },
      return_statement: { type: 'SEQ', members: [
        { type: 'STRING', value: 'return' }, { type: 'SYMBOL', name: 'number' },
      ] },
      number: { type: 'PATTERN', value: '[0-9]+' },
    },
  };
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(grammar))));
  const nodes = (tree) => tree.type === 'node' ? [tree.kind, ...tree.children.flatMap(nodes)] : [];
  assert.deepEqual(nodes(parser.parseTree('return1!').tree), ['document', 'block', 'return_statement']);
  assert.deepEqual(nodes(parser.parseTree('!').tree), ['document']);
});

test('a corpus case ends at its longest line of dashes, as in tree-sitter test', () => {
  const corpus = [
    rule('='),
    'Cargo script frontmatter',
    rule('='),
    '',
    '---cargo',
    '[dependencies]',
    '---',
    '',
    'fn main() {}',
    '',
    rule('-'),
    '',
    '(source_file (frontmatter) (function_item))',
    '',
    rule('='),
    'Plain',
    rule('='),
    'x',
    '---',
    '',
    '(source_file)',
    '',
  ].join('\n');
  assert.deepEqual(corpusFileCases(corpus), [
    { title: 'Cargo script frontmatter', source: '\n---cargo\n[dependencies]\n---\n\nfn main() {}\n' },
    { title: 'Plain', source: 'x' },
  ]);
});

test('of equally long lines of dashes, the last divides, and a case without one is all source', () => {
  const corpus = [rule('='), 'Twice', rule('='), 'a', '---', 'b', '---', '(x)', rule('='), 'Open', rule('='), 'c', ''].join('\n');
  assert.deepEqual(corpusFileCases(corpus), [
    { title: 'Twice', source: 'a\n---\nb' },
    { title: 'Open', source: 'c\n' },
  ]);
});

test('an upstream rule named like an object member is named from its words', () => {
  // tree-sitter-lean has a rule `constructor`; only reviewed decisions rename it.
  const decisions = { names: { _declaration: 'declaration_choice' } };
  assert.equal(nativeName('constructor', new Map(), decisions), 'constructor');
  assert.equal(nativeName('toString', new Map(), {}), 'to_string');
  assert.equal(nativeName('_declaration', new Map(), decisions), 'declaration_choice');
  assert.equal(ruleConcept({ name: 'constructor', sourceName: 'constructor' }, { concepts: {} }), 'grammar.constructor');
});

// A string whose text and closing quote an external scanner reads, as
// tree-sitter-rust's string_content and string_close are.
const QUOTED = {
  name: 'quoted',
  rules: {
    source: { type: 'REPEAT', content: { type: 'SYMBOL', name: 'string' } },
    string: { type: 'SEQ', members: [{ type: 'STRING', value: '"' }, { type: 'SYMBOL', name: 'string_content' }, { type: 'SYMBOL', name: '_close' }] },
  },
  extras: [{ type: 'PATTERN', value: '[ ]' }],
  externals: [{ type: 'SYMBOL', name: 'string_content' }, { type: 'SYMBOL', name: '_close' }],
};
const QUOTES = '(scanner quotes (tokens string_content _close) (operations '
  + '(if (valid string_content) (then (while (not (next (literal %22))) (do advance)) (emit string_content))) '
  + '(if (valid _close) (then (consume (literal %22)) (emit _close))) fail))\n';
const tokens = (tree) => (tree.type === 'token' ? (tree.trivia ? [] : [[tree.kind, tree.text]]) : tree.children.flatMap(tokens));

test('a native scanner reads the externals, from the current byte where the upstream scanner does', () => {
  const imported = importTreeSitterNative(QUOTED, { scanners: QUOTES, immediate: ['string_content', '_close'] });
  const text = renderTreeSitterNative(imported);
  const lines = text.split('\n');
  assert.equal(lines[2], QUOTES.trim(), 'the scanner comes before the rules');
  // The hidden _close is a token tree-sitter leaves unnamed.
  assert.equal(lines[4], '(rule string normal (seq (literal %22) (alias string_content (immediateToken (ref string_content))) (alias unnamed_token (immediateToken (ref _close)))))');
  const outcome = compileGrammar(parseGrammarLinks(text)).parseTree('"a b" "c"');
  assert.ok(outcome.ok);
  assert.deepEqual(tokens(outcome.tree), [
    [null, '"'], ['string_content', 'a b'], ['unnamed_token', '"'],
    [null, '"'], ['string_content', 'c'], ['unnamed_token', '"'],
  ]);
  // Without immediateToken the extras before the text would be skipped.
  const skipping = renderTreeSitterNative(importTreeSitterNative(QUOTED, { scanners: QUOTES }));
  assert.deepEqual(tokens(compileGrammar(parseGrammarLinks(skipping)).parseTree('" a"').tree)[1], ['string_content', 'a']);
  assert.throws(
    () => importTreeSitterNative(QUOTED, { scanners: QUOTES, immediate: ['string'] }),
    /the immediate external string is no scanner token/u,
  );
});

test('a renamed node kind that only an alias names keeps its tree-sitter name', () => {
  const commented = (second) => ({
    name: 'commented',
    rules: {
      source: { type: 'REPEAT', content: { type: 'CHOICE', members: [
        { type: 'ALIAS', named: true, value: 'doc_comment', content: { type: 'SYMBOL', name: '_line' } },
        { type: 'ALIAS', named: true, value: second, content: { type: 'SYMBOL', name: '_block' } },
      ] } },
      _line: { type: 'PATTERN', value: '#[a-z]*' },
      _block: { type: 'PATTERN', value: '![a-z]*' },
    },
    extras: [],
  });
  const nameOf = (name) => name.replace('doc_', 'documentation_');
  const text = renderTreeSitterNative(importTreeSitterNative(commented('doc_comment'), { nameOf }));
  assert.match(text, /^\(kind documentation_comment \(source-names \(tree-sitter doc_comment\)\)\)$/mu);
  const grammar = parseGrammarLinks(text);
  assert.deepEqual(grammar.kinds, [{ name: 'documentation_comment', sourceNames: [{ source: 'tree-sitter', name: 'doc_comment' }] }]);
  assert.deepEqual(tokens(compileGrammar(grammar).parseTree('#a!b').tree).map(([kind]) => kind), ['documentation_comment', 'documentation_comment']);
  // An alias kept as is needs no kind link.
  assert.doesNotMatch(renderTreeSitterNative(importTreeSitterNative(commented('doc_comment'))), /^\(kind /mu);
  // Two upstream kinds that read the same natively are refused.
  assert.throws(
    () => importTreeSitterNative(commented('documentation_comment'), { nameOf }),
    /upstream names read the same natively: documentation_comment \(doc_comment, documentation_comment\)/u,
  );
});

test('a hidden immediate token is a token tree-sitter leaves unnamed', () => {
  // tree-sitter-lean's `_string_content`: token.immediate of a hidden rule.
  const hidden = (name) => ({
    name: 'immediate',
    rules: {
      source: { type: 'REPEAT', content: { type: 'SYMBOL', name: 'string' } },
      string: { type: 'SEQ', members: [{ type: 'STRING', value: '"' }, { type: 'SYMBOL', name }, { type: 'STRING', value: '"' }] },
      [name]: { type: 'IMMEDIATE_TOKEN', content: { type: 'PATTERN', value: '[^"]+' } },
    },
    extras: [{ type: 'PATTERN', value: '[ ]' }],
  });
  const kinds = (name) => tokens(compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(hidden(name))))).parseTree('" a"').tree);
  assert.deepEqual(kinds('_text'), [[null, '"'], ['unnamed_token', ' a'], [null, '"']]);
  // A visible one keeps its name.
  assert.deepEqual(kinds('text'), [[null, '"'], ['text', ' a'], [null, '"']]);
});


test('nullable pattern extras preserve the content field and leave the pinned source untouched', () => {
  const alias = { type: 'ALIAS', named: true, value: 'comment_content', content: { type: 'PATTERN', value: '[a-z]*' } };
  const block = { type: 'STRING', value: '[[x]]' };
  const source = { rules: { comment: { type: 'CHOICE', members: [
    { type: 'FIELD', name: 'content', content: alias }, block,
  ] } } };
  const original = structuredClone(source);
  const decision = { family: 'nullable-pattern-extras', rule: 'comment', pattern: '[a-z]*', extraAlternative: 1, helper: '_block_extra', whitespace: '[ ]*' };
  const result = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(result.rules._block_extra, block);
  const [extras, content] = result.rules.comment.members[0].members;
  assert.equal(extras.type, 'REPEAT');
  assert.equal(extras.content.members[1].type, 'ALIAS');
  assert.equal(extras.content.members[1].value, 'comment');
  assert.equal(extras.content.members[1].content.name, '_block_extra');
  assert.equal(content.type, 'FIELD');
  assert.equal(content.name, 'content');
  assert.equal(content.content.content.type, 'IMMEDIATE_TOKEN');
  assert.deepEqual(content.content.content.content, alias.content);
  for (const change of [
    { family: 'unknown' }, { helper: 'visible' }, { helper: 'comment' },
    { extraAlternative: 3 }, { pattern: 'absent' },
  ]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('reviewed precedence replaces an outer level without changing the production or pinned source', () => {
  const body = { type: 'STRING', value: 'x' };
  const source = { rules: { source: { type: 'PREC_LEFT', value: 2, content: body } } };
  const original = structuredClone(source);
  const decision = { family: 'rule-precedence', rule: 'source', associativity: 'right', value: -1 };
  const result = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(result.rules.source, { type: 'PREC_RIGHT', value: -1, content: body });
  for (const change of [{ rule: 'absent' }, { associativity: 'invalid' }, { value: 1.5 }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('ordered variants choose the reviewed tree and retain fallback language alternatives', () => {
  const sym = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'variants', rules: {
    source: { type: 'CHOICE', members: [sym('first'), sym('second'), sym('other')] },
    first: { type: 'STRING', value: 'x' }, second: { type: 'PATTERN', value: '[xy]' }, other: { type: 'STRING', value: 'z' },
  } };
  const original = structuredClone(source);
  const decision = { family: 'ordered-variants', rule: 'source', variants: ['first', 'second'] };
  const result = transformNativeSource(source, [decision]);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(result))));
  for (const text of ['x', 'y', 'z']) {
    const outcome = parser.parseTree(text);
    assert.equal(outcome.ok, true, text);
    assert.deepEqual(outcome.ambiguities, []);
    assert.equal(outcome.tree.children[0].kind, text === 'x' ? 'first' : text === 'y' ? 'second' : 'other');
  }
  assert.deepEqual(source, original);
  for (const variants of [['first'], ['first', 'absent'], ['first', 'first']]) assert.throws(() => transformNativeSource(source, [{ ...decision, variants }]), TypeError);
});

test('a reviewed field variant preserves fields and aliases while keeping the original alternative', () => {
  const sym = (name) => ({ type: 'SYMBOL', name });
  const source = { rules: {
    invocation: { type: 'SEQ', members: [{ type: 'FIELD', name: 'target', content: { type: 'CHOICE', members: [sym('identifier'), sym('member')] } }, { type: 'STRING', value: '()' }] },
    expression: { type: 'CHOICE', members: [sym('invocation'), sym('identifier')] },
  } };
  const original = structuredClone(source);
  const decision = { family: 'field-choice-variant', rule: 'invocation', helper: '_member_invocation', field: 'target', symbol: 'member', alias: 'invocation', insertInto: 'expression' };
  const result = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(result.rules.invocation, original.rules.invocation);
  assert.deepEqual(result.rules._member_invocation.members[0], { type: 'FIELD', name: 'target', content: sym('member') });
  assert.deepEqual(result.rules.expression.members.at(-1), { type: 'ALIAS', named: true, value: 'invocation', content: sym('_member_invocation') });
  for (const change of [{ helper: 'visible' }, { field: 'absent' }, { symbol: 'absent' }, { insertInto: 'invocation' }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('context variants preserve kinds, source productions and opaque operand contexts', () => {
  const sym = (name) => ({ type: 'SYMBOL', name });
  const source = { supertypes: ['expression'], rules: {
    source: sym('expression'), expression: { type: 'CHOICE', members: [sym('term'), sym('prefix')] },
    term: { type: 'STRING', value: 'x' }, prefix: { type: 'SEQ', members: [{ type: 'STRING', value: '!' }, sym('expression')] },
  } };
  const original = structuredClone(source);
  const decision = { family: 'context-rule-variants', rules: ['expression', 'term', 'prefix'], insertInto: ['source'], prefix: '_context_', opaqueRules: ['prefix'] };
  const result = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(result.rules.expression, source.rules.expression);
  assert.deepEqual(result.rules._context_prefix, source.rules.prefix);
  assert.deepEqual(result.rules._context_expression.members[0], { type: 'ALIAS', named: true, value: 'term', content: sym('_context_term') });
  assert.deepEqual(result.rules.source, sym('_context_expression'));
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(result))));
  for (const text of ['x', '!x', '!!x']) {
    const outcome = parser.parseTree(text);
    assert.equal(outcome.ok, true);
    assert.deepEqual(outcome.ambiguities, []);
    assert.equal(tokens(outcome.tree).map(([, value]) => value).join(''), text);
    const kinds = (tree) => tree.type === 'node' ? [tree.kind, ...tree.children.flatMap(kinds)] : [];
    assert.ok(kinds(outcome.tree).includes('term'));
  }
  for (const change of [{ prefix: 'visible' }, { rules: ['expression', 'expression'] }, { insertInto: ['absent'] }, { opaqueRules: ['absent'] }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('optional literal preference preserves the nullable branch and validates the selection', () => {
  const source = { rules: { field: { type: 'SEQ', members: [{ type: 'CHOICE', members: [{ type: 'STRING', value: 'modifier' }, { type: 'BLANK' }] }, { type: 'STRING', value: 'x' }] } } };
  const original = structuredClone(source);
  const decision = { family: 'prefer-optional-literal', rule: 'field', literal: 'modifier', value: 1 };
  const result = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(result.rules.field.members[0].members[1], { type: 'BLANK' });
  assert.equal(result.rules.field.members[0].members[0].type, 'PREC_DYNAMIC');
  for (const change of [{ literal: 'absent' }, { rule: 'absent' }, { value: 0 }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});


test('corpus failure diagnostics identify changed rows and unequal row counts', () => {
  assert.equal(nativeCorpusFailure('case', { actual: [[1], [2]], expected: [[1], [3]] }), 'case: first difference 1: {"actual":[2],"expected":[3],"actualLength":2,"expectedLength":2}');
  assert.equal(nativeCorpusFailure('case', { actual: [[1]], expected: [[1], [2]] }), 'case: first difference 1: {"actual":null,"expected":[2],"actualLength":1,"expectedLength":2}');
  assert.equal(nativeCorpusFailure('case', { actual: false, expected: true, message: 'rejected' }), 'case: {"actual":false,"expected":true,"message":"rejected"}');
});

test('alias pattern priorities keep the alias and distinguish explicit tokens from generic extras', () => {
  const pattern = { type: 'PATTERN', value: '[ab]+' };
  const source = { rules: { source: { type: 'ALIAS', named: true, value: 'directive', content: pattern } } };
  const original = structuredClone(source);
  const decision = { family: 'alias-pattern-precedence', rules: ['source'], alias: 'directive', value: 1 };
  const result = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(result.rules.source, { type: 'ALIAS', named: true, value: 'directive', content: { type: 'TOKEN', content: { type: 'PREC', value: 1, content: pattern } } });
  for (const change of [{ rules: ['absent'] }, { alias: 'absent' }, { value: 0.5 }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('field-selected variant precedence keeps unselected alternatives and all fields', () => {
  const sym = (name) => ({ type: 'SYMBOL', name });
  const branch = (lhs) => ({ type: 'PREC_LEFT', value: 1, content: { type: 'SEQ', members: [
    { type: 'FIELD', name: 'lhs', content: sym(lhs) }, { type: 'FIELD', name: 'operator', content: sym('less') }, { type: 'FIELD', name: 'rhs', content: sym('expression') },
  ] } });
  const source = { rules: { binary: { type: 'CHOICE', members: [branch('expression'), branch('reference')] } } };
  const original = structuredClone(source);
  const decision = { family: 'rule-variant-precedence', rule: 'binary', fields: { lhs: 'reference', operator: 'less' }, associativity: 'right', value: 1, dynamic: 1 };
  const result = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(result.rules.binary.members[0], original.rules.binary.members[0]);
  assert.equal(result.rules.binary.members[1].type, 'PREC_DYNAMIC');
  assert.equal(result.rules.binary.members[1].content.type, 'PREC_RIGHT');
  assert.deepEqual(result.rules.binary.members[1].content.content, original.rules.binary.members[1].content);
  for (const change of [{ fields: { operator: 'less' } }, { fields: { lhs: 'absent' } }, { value: 1.5 }, { dynamic: -1 }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});
