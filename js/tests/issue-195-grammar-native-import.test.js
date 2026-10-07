// The import pipeline of the native merged grammars,
// js/scripts/import-native-grammars.mjs: how it reads the pinned upstream
// test corpora and how js/src/grammar-importers/tree-sitter-native.js turns
// a pinned tree-sitter grammar into a native Links Notation grammar.
import assert from 'node:assert/strict';
import { test } from 'node:test';

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
