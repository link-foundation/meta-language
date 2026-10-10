// The import pipeline of the native merged grammars,
// js/scripts/import-native-grammars.mjs: how it reads the pinned upstream
// test corpora and how js/src/grammar-importers/tree-sitter-native.js turns
// a pinned tree-sitter grammar into a native Links Notation grammar.
import assert from 'node:assert/strict';
import { test } from 'node:test';

import { nativeCorpusFailure } from '../scripts/native-grammar-rows.mjs';
import { rememberedDelimiterScanner } from '../scripts/scanner-families.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { GENERATED, corpusFileCases, mergeConcepts, nativeName, ruleConcept } from '../scripts/import-native-grammars.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const rule = (character, length = 80) => character.repeat(length);

test('the native concept merge keeps stable record order and prunes only unused generated identities', () => {
  const entry = { language: 'fixture' };
  const generated = {
    id: 'grammar.fixture-item',
    phrase: 'fixture item',
    role: 'concept',
    definition: 'A fixture item in a grammar.',
    constraints: ['Only the native fixture grammar defines this construct.', GENERATED],
    sourceAliases: [{ source: 'native:fixture', name: 'item' }],
    formerNames: [],
  };
  const authored = {
    id: 'translation.authored-concept',
    phrase: 'authored concept',
    role: 'concept',
    definition: 'An authored concept remains in place.',
    constraints: ['Hand-authored and not generated.'],
    sourceAliases: [{ source: 'JavaScript source', name: 'authoredConcept' }],
    formerNames: [],
  };
  const register = { concepts: [generated, authored] };
  const imports = [{ entry, rules: [{ name: 'item', sourceName: 'item', concept: generated.id }] }];

  const first = mergeConcepts(register, imports);
  assert.deepEqual(first, register, 're-importing the same source does not reorder the register');
  assert.deepEqual(mergeConcepts(first, imports), first, 'a second import is byte-for-byte idempotent');
  assert.deepEqual(register.concepts, [generated, authored], 'the input register is not mutated');

  const afterRemoval = mergeConcepts(first, [{ entry, rules: [] }]);
  assert.deepEqual(afterRemoval.concepts, [authored], 'unused generated records are removed only after import');
});


test('aliased choice branches preserve child nodes and separate optional suffix reductions', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'types', rules: {
    document: symbol('_type'),
    _type: { type: 'ALIAS', named: true, value: 'type', content: { type: 'CHOICE', members: [
      { type: 'SEQ', members: [symbol('identifier'), { type: 'CHOICE', members: [{ type: 'STRING', value: '?' }, { type: 'BLANK' }] }] },
      { type: 'STRING', value: 'void' },
    ] } },
    identifier: { type: 'PATTERN', value: '[A-Z][A-Za-z]*' },
  } };
  const decision = { family: 'alias-choice-branches', rule: '_type', alias: 'type', helpers: ['_named_type', '_void_type'], suffix: '?' };
  const transformed = transformNativeSource(source, [decision]);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  const outcome = parser.parseTree('Name?');
  assert.equal(outcome.ok, true);
  assert.equal(outcome.tree.children[0].kind, 'type');
  assert.equal(outcome.tree.children[0].children[0].kind, 'identifier');
  assert.equal(outcome.tree.children[1].kind, 'type');
  assert.equal(outcome.tree.children[1].text, '?');
  for (const text of ['Name', 'void']) assert.equal(parser.parseTree(text).ok, true, text);
  for (const text of ['?', 'void?', 'Name??']) assert.equal(parser.parseTree(text).ok, false, text);
  assert.equal(source.rules._type.type, 'ALIAS');
  for (const change of [{ helpers: ['_same', '_same'] }, { helpers: ['_only'] }, { alias: 'absent' }]) {
    assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
  }
});

test('grammar-action external rules keep actions and provenance through the native importer', () => {
  const names = ['opening_tag', 'body', 'closing_tag'];
  const source = { name: 'text_labels', rules: { document: { type: 'SEQ', members: names.map((name) => ({ type: 'SYMBOL', name })) } }, externals: names.map((name) => ({ type: 'SYMBOL', name })) };
  const scanners = rememberedDelimiterScanner({ name: 'labels', startToken: names[0], contentToken: names[1], endToken: names[2], tagPattern: '\\$[^$\\s]*\\$' });
  const imported = importTreeSitterNative(source, { scanners });
  assert.deepEqual(imported.report, { approximations: [], unsupported: [] });
  for (const name of [names[0], names[2]]) {
    const rules = imported.rules.filter((rule) => rule.name === name);
    assert.equal(rules.length, 1);
    assert.equal(rules[0].sourceName, name);
    assert.match(rules[0].fields.join(' '), /\(action /u);
  }
  assert.ok(imported.scanners.every((line) => !line.startsWith('(rule ')));
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
  for (const text of ['$$$$', '$a$é😀$a$', '$a$x$b$']) assert.equal(parser.parseTree(text).ok, text !== '$a$x$b$');
  assert.throws(() => importTreeSitterNative(source, { scanners: `${scanners}(rule document normal empty)\n` }), /duplicates a source rule/u);
});

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

test('corpus language and error attributes survive blank lines in the header', () => {
  const corpus = [rule('='), 'Malformed markup', '', ':error', ':language(xml)', '', rule('='), '<r>', rule('-'), '(document)', ''].join('\n');
  assert.deepEqual(corpusFileCases(corpus), [
    { title: 'Malformed markup', source: '<r>', language: 'xml', attributes: ['error'] },
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

test('contextual keyword exclusions preserve command prefixes and all source productions', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const literal = (value) => ({ type: 'STRING', value });
  const source = { name: 'keywords', rules: {
    source: { type: 'CHOICE', members: [symbol('good'), symbol('pipeline')] },
    good: { type: 'SEQ', members: [{ type: 'ALIAS', named: false, value: 'when', content: { type: 'PREC', value: 1, content: { type: 'PATTERN', value: '[wW][hH][eE][nN]' } } }, literal('('), literal(')')] },
    pipeline: { type: 'SEQ', members: [symbol('word'), literal('!')] },
    word: { type: 'PATTERN', value: '[A-Za-z-]+' },
  } };
  const original = structuredClone(source);
  const decision = { family: 'symbol-keyword-exclusion', rule: 'source', symbol: 'pipeline', keywordRules: ['good'], wordRule: 'word' };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(transformed.rules.good, original.rules.good);
  assert.deepEqual(transformed.rules.pipeline, original.rules.pipeline);
  assert.equal(transformed.rules.source.members.length, original.rules.source.members.length);
  assert.deepEqual(transformed.rules.source.members[1].content, original.rules.source.members[1]);
  const imported = importTreeSitterNative(transformed);
  assert.deepEqual(imported.report.unsupported, []);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
  for (const text of ['when()', 'WhEn()', 'other!', 'when-extra!']) assert.equal(parser.parseTree(text).ok, true, text);
  for (const text of ['when!', 'WhEn!']) assert.equal(parser.parseTree(text).ok, false, text);
  for (const change of [{ symbol: 'absent' }, { keywordRules: ['word'] }, { wordRule: 'absent' }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('dynamic rule priorities and lifted lexical aliases retain their original bodies', () => {
  const pattern = { type: 'PATTERN', value: '[aA]' };
  const source = { rules: { source: { type: 'TOKEN', content: { type: 'ALIAS', named: false, value: 'a', content: { type: 'PREC', value: 1, content: pattern } } } } };
  const original = structuredClone(source);
  const lifted = transformNativeSource(source, [{ family: 'lift-token-aliases', rules: ['source'] }]);
  assert.deepEqual(source, original);
  assert.deepEqual(lifted.rules.source, { type: 'ALIAS', named: false, value: 'a', content: { type: 'TOKEN', content: original.rules.source.content.content } });
  const prioritized = transformNativeSource(source, [{ family: 'rule-dynamic-precedence', rule: 'source', value: 1 }]);
  assert.deepEqual(prioritized.rules.source, { type: 'PREC_DYNAMIC', value: 1, content: original.rules.source });
  for (const decision of [{ family: 'lift-token-aliases', rules: ['absent'] }, { family: 'rule-dynamic-precedence', rule: 'source', value: 0 }]) assert.throws(() => transformNativeSource(source, [decision]), TypeError);
});

test('end boundary reconciliation retains the sentinel and accepts only the empty input boundary', () => {
  const source = { name: 'sentinel', rules: { document: { type: 'SEQ', members: [
    { type: 'STRING', value: 'value' },
    { type: 'CHOICE', members: [{ type: 'STRING', value: ';' }, { type: 'STRING', value: '\0' }] },
  ] } } };
  const original = structuredClone(source);
  const decision = { family: 'literal-end-boundary', rule: 'document', literal: '\0' };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(transformed.rules.document.members[1].members[0], original.rules.document.members[1].members[0]);
  assert.deepEqual(transformed.rules.document.members[1].members[1].content, original.rules.document.members[1].members[1]);
  const imported = importTreeSitterNative(transformed);
  assert.deepEqual(imported.report.unsupported, []);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
  for (const text of ['value', 'value;', 'value\0']) assert.equal(parser.parseTree(text).ok, true, JSON.stringify(text));
  for (const text of ['value!', 'value;!', 'value\0!']) assert.equal(parser.parseTree(text).ok, false, JSON.stringify(text));
  for (const change of [{ rule: 'absent' }, { literal: 'absent' }, { literal: '' }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('symbol preferences keep all productions and settle a shared complete span', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'preference', rules: {
    document: { type: 'CHOICE', members: [symbol('first'), symbol('second')] },
    first: symbol('word'), second: symbol('word'), word: { type: 'STRING', value: 'a' },
  } };
  const original = structuredClone(source);
  const decision = { family: 'symbol-dynamic-precedence', rules: ['second'], symbol: 'word', value: 1 };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(transformed.rules.document, original.rules.document);
  assert.deepEqual(transformed.rules.second.content, original.rules.second);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  const result = parser.parseTree('a');
  assert.equal(result.ok, true);
  assert.equal(result.tree.children[0].kind, 'second');
  for (const change of [{ rules: ['absent'] }, { symbol: 'absent' }, { rules: ['word'] }, { value: 0 }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('a complete context preference preserves longer fallback productions', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'boundary', rules: {
    document: { type: 'SEQ', members: [symbol('_replacement'), { type: 'STRING', value: ')' }] },
    _replacement: { type: 'CHOICE', members: [symbol('guard'), symbol('expression')] },
    guard: { type: 'CHOICE', members: [{ type: 'STRING', value: 'a' }, { type: 'STRING', value: 'a;b' }] },
    expression: { type: 'STRING', value: 'a' },
  } };
  const original = structuredClone(source);
  const decision = { family: 'complete-context-variant', rule: '_replacement', symbol: 'expression', boundary: ')' };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(transformed.rules._replacement.content, original.rules._replacement);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  assert.equal(parser.parseTree('a)').tree.children[0].kind, 'expression');
  assert.equal(parser.parseTree('a;b)').tree.children[0].kind, 'guard');
  assert.equal(parser.parseTree('a;!').ok, false);
  for (const change of [{ rule: 'absent' }, { symbol: 'document' }, { boundary: '' }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('input boundary preferences skip extras, preserve fields and emit no boundary leaf', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'input_boundary', extras: [{ type: 'PATTERN', value: '\\s+' }], rules: {
    document: { type: 'CHOICE', members: [
      { type: 'FIELD', name: 'preferred', content: symbol('short') }, symbol('long'),
    ] },
    short: { type: 'STRING', value: 'a' },
    long: { type: 'STRING', value: 'ab' },
  } };
  const transformed = transformNativeSource(source, [{ family: 'complete-context-variant', rule: 'document', field: 'preferred', boundary: null }]);
  assert.deepEqual(transformed.rules.document.content, source.rules.document);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  const tree = parser.parseTree(' a\n').tree;
  assert.equal(tree.children.find((child) => child.kind === 'short').field, 'preferred');
  const leaves = (node) => node.type === 'node' ? node.children.flatMap(leaves) : [node];
  assert.ok(leaves(tree).every((leaf) => leaf.end > leaf.start));
  assert.equal(parser.parseTree('ab').ok, true);
  assert.equal(parser.parseTree('ac').ok, false);
});

test('context keyword guards do not duplicate the source word token or its lexer candidates', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'context_words', word: 'identifier', extras: [{ type: 'PATTERN', value: '\\s+' }], rules: {
    document: { type: 'CHOICE', members: [symbol('declaration'), symbol('identifier')] },
    declaration: { type: 'SEQ', members: [{ type: 'STRING', value: 'def' }, symbol('identifier')] },
    identifier: { type: 'PATTERN', value: '[a-z]+' },
  } };
  for (const family of ['complete-context-variant', 'keyword-context-variant']) for (const guards of [{ includedKeywords: ['def'] }, { excludedKeywords: ['def'] }]) {
    const transformed = transformNativeSource(source, [{ family, rule: 'document', symbol: 'declaration', boundary: null, wordRule: 'identifier', ...guards }]);
    const imported = importTreeSitterNative(transformed);
    assert.equal(imported.rules.find((rule) => rule.name === 'identifier').kind, 'token');
    assert.deepEqual(imported.keywords, importTreeSitterNative(source).keywords);
    const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
    assert.equal(parser.parseTree('def name').ok, true);
    assert.equal(parser.parseTree('name').ok, true);
  }
});

test('pattern context preferences retain the complete original fallback choice', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'context_patterns', rules: {
    document: { type: 'CHOICE', members: [symbol('first'), symbol('second')] },
    first: { type: 'CHOICE', members: [{ type: 'STRING', value: 'ab' }, { type: 'STRING', value: 'xy' }, { type: 'STRING', value: 'z' }] },
    second: { type: 'CHOICE', members: [{ type: 'STRING', value: 'ab' }, { type: 'STRING', value: 'xy' }, { type: 'STRING', value: 'q' }] },
  } };
  const decision = { family: 'pattern-context-variant', rule: 'document', symbol: 'second', pattern: 'a' };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(transformed.rules.document.content, source.rules.document);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  assert.equal(parser.parseTree('ab').tree.children[0].kind, 'second');
  assert.equal(parser.parseTree('xy').tree.children[0].kind, 'first');
  for (const text of ['z', 'q']) assert.equal(parser.parseTree(text).ok, true);
  assert.equal(parser.parseTree('!').ok, false);
  for (const change of [{ pattern: '' }, { pattern: null }, { symbol: 'absent' }, { includedKeywords: ['a'] }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('pattern word boundaries retain named tokens and reject longer identifier prefixes', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'keyword_boundaries', word: 'identifier', extras: [{ type: 'PATTERN', value: '\\s+' }], rules: {
    document: { type: 'SEQ', members: [symbol('select_keyword'), symbol('identifier')] },
    select_keyword: { type: 'PATTERN', value: '[sS][eE][lL][eE][cC][tT]' },
    identifier: { type: 'PATTERN', value: '[a-zA-Z_é][a-zA-Z0-9_é]*' },
  } };
  const decision = { family: 'rule-word-boundary', rules: ['select_keyword'], wordRule: 'identifier', continuationPattern: '[a-zA-Z0-9_é]' };
  const transformed = transformNativeSource(source, [decision]);
  const imported = importTreeSitterNative(transformed);
  assert.equal(imported.rules.find(({ name }) => name === 'select_keyword').kind, 'token');
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
  for (const text of ['SELECT name', 'select é']) {
    const outcome = parser.parseTree(text);
    assert.equal(outcome.ok, true);
    assert.equal(outcome.tree.children[0].kind, 'select_keyword');
    assert.equal(outcome.tree.children[0].type, 'token');
  }
  for (const text of ['SELECTname', 'SELECT1', 'SELECT_', 'SELECTé']) assert.equal(parser.parseTree(text).ok, false);
  for (const change of [{ rules: ['absent'] }, { rules: ['select_keyword', 'select_keyword'] }, { continuationPattern: '' }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('literal continuations preserve operator rows and avoid splitting a longer token', () => {
  const source = { name: 'operator_boundaries', extras: [{ type: 'PATTERN', value: '\\s+' }], rules: {
    document: { type: 'SEQ', members: [{ type: 'STRING', value: '<' }, { type: 'STRING', value: 'x' }, { type: 'STRING', value: '>' }, { type: 'STRING', value: '=' }, { type: 'STRING', value: 'y' }] },
  } };
  const transformed = transformNativeSource(source, [{ family: 'literal-continuation-exclusion', rules: ['document'], literal: '>', continuationPattern: '=' }]);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  assert.equal(parser.parseTree('<x>=y').ok, false);
  const outcome = parser.parseTree('<x> =y');
  assert.equal(outcome.ok, true);
  assert.equal(outcome.tree.children.find(({ text }) => text === '>').text, '>');
});

test('field ordering and contextual literals preserve every original field production', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'fields', rules: {
    document: { type: 'FIELD', name: 'value', content: { type: 'CHOICE', members: [symbol('first'), symbol('second')] } },
    first: { type: 'STRING', value: 'a' }, second: { type: 'STRING', value: 'b' },
  } };
  const ordered = transformNativeSource(source, [{ family: 'field-ordered-variants', rule: 'document', field: 'value', variants: ['second', 'first'] }]);
  assert.deepEqual(ordered.rules.document.content.members, [...source.rules.document.content.members].reverse());
  const transformed = transformNativeSource(ordered, [{ family: 'field-literal-alternative', rule: 'document', field: 'value', literal: 'c', alias: 'first' }]);
  assert.deepEqual(transformed.rules.document.content.members[1], ordered.rules.document.content);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  for (const text of ['a', 'b', 'c']) assert.equal(parser.parseTree(text).ok, true);
  assert.equal(parser.parseTree('d').ok, false);
  assert.throws(() => transformNativeSource(source, [{ family: 'field-ordered-variants', rule: 'document', field: 'value', variants: ['second', 'absent'] }]), TypeError);
});

test('a trivia pattern run retains one terminator and consumes successive terminators losslessly', () => {
  const source = { name: 'terminators', rules: { document: { type: 'SEQ', members: [{ type: 'STRING', value: 'a' }, { type: 'PATTERN', value: '\\n' }] } } };
  const transformed = transformNativeSource(source, [{ family: 'pattern-trivia-run', rule: 'document', pattern: '\\n' }]);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  for (const text of ['a\n', 'a\n\n']) {
    const outcome = parser.parseTree(text);
    assert.equal(outcome.ok, true);
    assert.equal(outcome.tree.end, text.length);
  }
  assert.equal(parser.parseTree('a').ok, false);
  assert.throws(() => transformNativeSource(source, [{ family: 'pattern-trivia-run', rule: 'document', pattern: 'x' }]), TypeError);
});

test('optional suffix preference requires a value without changing the original prefix', () => {
  const symbol = (name) => ({ type: 'SYMBOL', name });
  const source = { name: 'attachment', rules: {
    document: { type: 'SEQ', members: [symbol('_prefix'), { type: 'CHOICE', members: [symbol('arguments'), { type: 'BLANK' }] }] },
    _prefix: { type: 'CHOICE', members: [
      { type: 'SEQ', members: [{ type: 'STRING', value: ' ' }, { type: 'CHOICE', members: [symbol('word'), { type: 'BLANK' }] }] },
      symbol('parenthesized'),
    ] },
    word: { type: 'STRING', value: 'a' },
    arguments: { type: 'STRING', value: '(b)' },
    parenthesized: { type: 'STRING', value: ' (b)' },
  } };
  const original = structuredClone(source);
  const decision = { family: 'optional-suffix-context', rule: 'document', prefix: '_prefix', suffix: 'arguments', helper: '_attachment_prefix', requiredSymbol: 'word', value: 1 };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(source, original);
  assert.deepEqual(transformed.rules._prefix, original.rules._prefix);
  assert.deepEqual(transformed.rules.document.content, original.rules.document);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  for (const text of [' ', ' a', ' a(b)', ' (b)']) assert.equal(parser.parseTree(text).ok, true, JSON.stringify(text));
  assert.equal(parser.parseTree(' (b)').tree.children[0].kind, 'parenthesized');
  assert.equal(parser.parseTree(' a(b)').tree.children.at(-1).kind, 'arguments');
  for (const change of [{ prefix: 'absent' }, { helper: '_prefix' }, { requiredSymbol: 'arguments' }, { value: 0 }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('a named choice alias preserves the selected production beneath its boundary', () => {
  const source = { name: 'choice_alias', rules: {
    document: { type: 'ALIAS', named: true, value: 'statement', content: { type: 'CHOICE', members: [{ type: 'SYMBOL', name: '_read' }, { type: 'SYMBOL', name: '_write' }] } },
    _read: { type: 'SYMBOL', name: 'selection' },
    _write: { type: 'SYMBOL', name: 'insertion' },
    selection: { type: 'SEQ', members: [{ type: 'STRING', value: 'select' }, { type: 'SYMBOL', name: 'number' }] },
    insertion: { type: 'SEQ', members: [{ type: 'STRING', value: 'insert' }, { type: 'SYMBOL', name: 'number' }] },
    number: { type: 'PATTERN', value: '[0-9]+' },
  } };
  const decision = { family: 'alias-choice-rule', rule: 'document', alias: 'statement', helper: '_statement' };
  const transformed = transformNativeSource(source, [decision]);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  const nodes = (tree) => tree.type === 'node' ? [tree.kind, ...tree.children.flatMap(nodes)] : [];
  for (const [input, production] of [['select2', 'selection'], ['insert3', 'insertion']]) {
    const result = parser.parseTree(input);
    assert.equal(result.ok, true);
    assert.deepEqual(nodes(result.tree), ['document', 'statement', production]);
  }
  assert.throws(() => transformNativeSource(source, [{ ...decision, alias: 'absent' }]), /exactly one/u);
  assert.equal(Object.hasOwn(source.rules, '_statement'), false);
});

test('contextual keyword exclusions preserve quoted and longer identifier alternatives', () => {
  const source = { name: 'aliases', extras: [{ type: 'PATTERN', value: '\\s+' }], rules: {
    document: { type: 'SEQ', members: [{ type: 'STRING', value: '*' }, { type: 'SYMBOL', name: 'identifier' }] },
    identifier: { type: 'CHOICE', members: [{ type: 'SYMBOL', name: 'word' }, { type: 'SEQ', members: [{ type: 'STRING', value: '"' }, { type: 'SYMBOL', name: 'word' }, { type: 'STRING', value: '"' }] }] },
    word: { type: 'PATTERN', value: '[A-Za-z]+' },
    keyword_from: { type: 'PATTERN', value: '[fF][rR][oO][mM]' },
  } };
  const decision = { family: 'symbol-keyword-exclusion', rule: 'document', symbol: 'identifier', keywordRules: ['keyword_from'], keywordPatterns: true, wordRule: 'word' };
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformNativeSource(source, [decision])))));
  for (const input of ['* FROM', '* fRoM']) assert.equal(parser.parseTree(input).ok, false, input);
  for (const input of ['* FROMage', '* "FROM"', '* named']) assert.equal(parser.parseTree(input).ok, true, input);
});

test('renamed scanner-only tokens retain their original concrete kinds', () => {
  const source = { name: 'comments', externals: [{ type: 'SYMBOL', name: 'comment_open' }], rules: { document: { type: 'SYMBOL', name: 'comment_open' } } };
  const imported = importTreeSitterNative(source, { nameOf: (name) => name === 'comment_open' ? 'comment_opening' : name,
    scanners: '(scanner comments (tokens comment_opening) (operations (consume (literal %23)) (emit comment_opening)))\n' });
  assert.deepEqual(imported.kinds, [{ name: 'comment_opening', sourceName: 'comment_open' }]);
  const grammar = parseGrammarLinks(renderTreeSitterNative(imported));
  assert.deepEqual(grammar.kinds[0].sourceNames, [{ source: 'tree-sitter', name: 'comment_open' }]);
  assert.equal(compileGrammar(grammar).parseTree('#').ok, true);
});

test('source prefix exclusions retain fallback productions and word continuations', () => {
  const source = { name: 'commands', rules: { document: { type: 'SEQ', members: [{ type: 'SYMBOL', name: 'command' }, { type: 'STRING', value: '(' }, { type: 'STRING', value: ')' }] }, command: { type: 'SYMBOL', name: 'identifier' }, identifier: { type: 'PATTERN', value: '[A-Za-z_][A-Za-z0-9_]*' } } };
  const before = structuredClone(source);
  const decision = { family: 'rule-prefix-exclusion', rule: 'command', pattern: '[iI][fF]', continuationPattern: '[A-Za-z0-9_]' };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(source, before);
  assert.deepEqual(transformed.rules.command.content, source.rules.command);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  for (const input of ['other()', 'ifx()', 'IF1()']) assert.equal(parser.parseTree(input).ok, true, input);
  for (const input of ['if()', 'IF()']) assert.equal(parser.parseTree(input).ok, false, input);
  for (const change of [{ rule: 'absent' }, { pattern: '' }, { continuationPattern: '' }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('literal and pattern prefix exclusions preserve character token identities', () => {
  const source = { name: 'references', rules: { document: { type: 'REPEAT1', content: { type: 'CHOICE', members: [{ type: 'STRING', value: '$' }, { type: 'PATTERN', value: '[^()]' }] } } } };
  const transformed = transformNativeSource(source, [
    { family: 'literal-prefix-exclusion', rule: 'document', pattern: '$', excludedPattern: '\\$\\{' },
    { family: 'pattern-prefix-exclusion', rule: 'document', pattern: '[^()]', excludedPattern: '\\$\\{' },
  ]);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  for (const input of ['$plain', 'a$b', 'é😀']) assert.equal(parser.parseTree(input).ok, true, input);
  for (const input of ['${broken', 'a${broken']) assert.equal(parser.parseTree(input).ok, false, input);
  for (const family of ['literal-prefix-exclusion', 'pattern-prefix-exclusion']) assert.throws(() => transformNativeSource(source, [{ family, rule: 'document', pattern: 'absent', excludedPattern: 'x' }]), TypeError);
});

test('named aliases of choice symbols retain the concrete child below the alias', () => {
  const source = {
    name: 'aliases', supertypes: ['expression'],
    rules: {
      source: { type: 'ALIAS', named: true, value: 'target', content: { type: 'SYMBOL', name: 'expression' } },
      expression: { type: 'CHOICE', members: [{ type: 'SYMBOL', name: 'identifier' }, { type: 'SYMBOL', name: 'number' }] },
      identifier: { type: 'PATTERN', value: '[a-z]+' }, number: { type: 'PATTERN', value: '[0-9]+' },
    },
  };
  const transformed = transformNativeSource(source, [{ family: 'alias-choice-rule', rule: 'source', alias: 'target', helper: '_target_content' }]);
  assert.deepEqual(transformed.rules._target_content, source.rules.expression);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  for (const [input, kind] of [['abc', 'identifier'], ['123', 'number']]) {
    const result = parser.parseTree(input);
    assert.equal(result.ok, true);
    assert.equal(result.tree.children[0].kind, 'target');
    assert.equal(result.tree.children[0].children[0].kind, kind);
  }
});

test('complete alternative preferences keep the original fallback and every production', () => {
  const source = {
    name: 'alternatives', rules: {
      source: { type: 'CHOICE', members: [
        { type: 'ALIAS', named: true, value: 'short', content: { type: 'SYMBOL', name: 'opening' } },
        { type: 'SEQ', members: [{ type: 'SYMBOL', name: 'opening' }, { type: 'SYMBOL', name: 'closing' }] },
      ] },
      opening: { type: 'STRING', value: 'x' }, closing: { type: 'STRING', value: 'y' },
    },
  };
  const decision = { family: 'complete-alternative-preference', rule: 'source', symbol: 'opening' };
  const transformed = transformNativeSource(source, [decision]);
  assert.deepEqual(transformed.rules.source.content, source.rules.source);
  assert.deepEqual(transformed.rules.source.preferred, source.rules.source.members[1]);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  assert.equal(parser.parseTree('xy').ok, true);
  assert.equal(parser.parseTree('x').ok, true);
  assert.equal(parser.parseTree('y').ok, false);
  for (const change of [{ symbol: 'missing' }, { rule: 'missing' }, { symbol: null }]) assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
});

test('distributed choice aliases preserve hidden nodes and named literal leaves', () => {
  const source = { name: 'aliases', rules: {
    document: { type: 'ALIAS', named: true, value: 'target', content: { type: 'CHOICE', members: [{ type: 'SYMBOL', name: '_name' }, { type: 'STRING', value: '_' }] } },
    _name: { type: 'ALIAS', named: true, value: 'qualified_name', content: { type: 'PATTERN', value: '[a-z]+' } },
  } };
  const decision = { family: 'alias-choice-alternatives', rule: 'document', alias: 'target', count: 1 };
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformNativeSource(source, [decision])))));
  const named = parser.parseTree('grove');
  assert.equal(named.ok, true);
  assert.equal(named.tree.children[0].kind, 'target');
  assert.equal(named.tree.children[0].children[0].kind, 'qualified_name');
  const literal = parser.parseTree('_');
  assert.equal(literal.ok, true);
  assert.equal(literal.tree.children[0].type, 'token');
  assert.equal(literal.tree.children[0].kind, 'target');
  assert.throws(() => transformNativeSource(source, [{ ...decision, count: 2 }]), /production count/);
});

test('reviewed lexical prefix guards keep the original named token identity', () => {
  const source = { name: 'guarded_words', rules: {
    document: { type: 'SYMBOL', name: 'identifier' },
    identifier: { type: 'PATTERN', value: '[a-z]+' },
  } };
  const decision = { family: 'pattern-prefix-exclusion', rule: 'identifier', pattern: '[a-z]+', excludedPattern: 'if(?![a-z])', preserveLexicalIdentity: true };
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformNativeSource(source, [decision])))));
  assert.equal(parser.parseTree('if').ok, false);
  for (const word of ['grove', 'iffy']) {
    const result = parser.parseTree(word);
    assert.equal(result.ok, true);
    assert.equal(result.tree.children[0].type, 'token');
    assert.equal(result.tree.children[0].kind, 'identifier');
    assert.equal(result.tree.children[0].text, word);
  }
});
