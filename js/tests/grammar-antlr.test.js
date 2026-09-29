// Ports rust/tests/unit/grammar_import_antlr.rs and
// rust/tests/integration/grammar_import_antlr.rs with the same expectations.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { GrammarBuilder as G, deserializeGrammar, serializeGrammar } from '../src/grammar.js';
import { importAntlr } from '../src/grammar-importers/antlr.js';
import { GrammarImportError } from '../src/grammar-importers/common.js';
import { renderGrammarRule } from './support/render-grammar-expression.js';

const fixture = (file) => readFileSync(
  new URL(`../../rust/tests/fixtures/grammar/antlr/${file}`, import.meta.url),
  'utf8',
);
const range = (start, end) => ({ kind: 'range', start, end });
const character = (value) => ({ kind: 'char', value });

test('imports arithmetic ANTLR fixture', () => {
  const grammar = importAntlr(fixture('arithmetic.g4'));

  assert.equal(grammar.sourceFormat, 'antlr');
  assert.equal(grammar.startRule()?.name, 'expr');
  assert.deepEqual(grammar.ruleNames(), ['expr', 'term', 'factor', 'INT', 'ID', 'WS']);
  assert.equal(grammar.rule('expr').kind, 'normal');
  assert.equal(grammar.rule('INT').kind, 'token');
  assert.equal(grammar.rule('ID').kind, 'token');
  assert.equal(grammar.rule('WS').kind, 'token');
  assert.equal(grammar.rule('WS').doc, '-> skip');

  assert.deepEqual(grammar.rule('expr').expression, {
    kind: 'seq',
    items: [
      G.ref('term'),
      G.repeat0({
        kind: 'seq',
        items: [
          { kind: 'choice', items: [G.literal('+'), G.literal('-')], ordered: false },
          G.ref('term'),
        ],
      }),
    ],
  });
  assert.deepEqual(grammar.rule('ID').expression, {
    kind: 'seq',
    items: [
      G.charClass([range('a', 'z'), range('A', 'Z'), character('_')]),
      G.repeat0(G.charClass([range('a', 'z'), range('A', 'Z'), character('_'), range('0', '9')])),
    ],
  });
});

test('lowers covering ANTLR constructs', () => {
  const grammar = importAntlr(fixture('covering.g4'));

  assert.equal(grammar.sourceFormat, 'antlr');
  assert.equal(grammar.startRule()?.name, 'entry');
  assert.deepEqual(
    grammar.ruleNames(),
    ['entry', 'item', 'literalRange', 'TOKEN', 'DIGIT', 'COMMENT', 'ACTIONED'],
  );
  assert.equal(grammar.rule('DIGIT').kind, 'silent');
  assert.deepEqual(
    grammar.rule('TOKEN').expression,
    G.charClass([range('a', 'z'), range('0', '9'), character('_')]),
  );
  assert.deepEqual(grammar.rule('literalRange').expression, G.charRange('a', 'z'));
  assert.equal(grammar.rule('COMMENT').doc, '-> channel(HIDDEN)');
  assert.equal(grammar.rule('ACTIONED').doc, 'dropped predicate; dropped action; -> type(ID)');

  assert.deepEqual(grammar.rule('entry').expression, {
    kind: 'seq',
    items: [
      G.capture('name', G.ref('ID')),
      G.capture('values', G.capture('non_greedy', G.repeat0(G.ref('item')))),
      G.ref('literalRange'),
      G.ref('item'),
    ],
  });
  assert.deepEqual(grammar.rule('item').expression, {
    kind: 'choice',
    items: [G.any(), G.charClass([character(';')], true), G.not(G.literal('x'))],
    ordered: false,
  });
});

test('skips lexer mode declarations', () => {
  const grammar = importAntlr(fixture('lexer-mode.g4'));

  assert.equal(grammar.startRule()?.name, 'STRING_TEXT');
  assert.deepEqual(grammar.ruleNames(), ['STRING_TEXT']);
  assert.deepEqual(
    grammar.rule('STRING_TEXT').expression,
    G.repeat1(G.charClass([character('"')], true)),
  );
});

test('unresolved references remain visible on imported grammar', () => {
  const grammar = importAntlr('grammar Missing; start : missing ;');

  assert.deepEqual(grammar.rule('start').expression, G.ref('missing'));
  assert.ok(grammar.undefinedNonterminals().includes('missing'));
});

test('skips inline comments before sequence boundaries', () => {
  const grammar = importAntlr(`grammar Comments;
         start : 'a' // first branch
             | ('b' /* group end */) // second branch
             ;`);

  assert.deepEqual(grammar.rule('start').expression, {
    kind: 'choice',
    items: [G.literal('a'), G.literal('b')],
    ordered: false,
  });
});

test('malformed ANTLR reports parse error', () => {
  assert.throws(
    () => importAntlr('grammar Bad; start : ( missing ;'),
    (error) => error instanceof GrammarImportError && error.kind === 'parse' &&
      error.format === 'antlr',
  );
});

test('unsupported rule prelude reports unsupported error', () => {
  assert.throws(
    () => importAntlr('grammar Bad; rule locals [int value] : \'x\' ;'),
    (error) => error instanceof GrammarImportError && error.kind === 'unsupported' &&
      error.format === 'antlr' && error.construct === 'rule prelude locals',
  );
});

test('imported ANTLR grammar survives a serialization round trip', () => {
  // The Rust integration test round-trips the grammar through links; the
  // JavaScript grammar round-trips through its serialized form.
  const grammar = importAntlr(fixture('covering.g4'));
  const restored = deserializeGrammar(serializeGrammar(grammar));

  assert.deepEqual(restored.normalized(), grammar.normalized());
});

// Rendered rules and error messages recorded from the Rust importer by
// experiments/antlr-parity.
test('renders and rejects ANTLR sources exactly like the Rust importer', () => {
  const grammar = importAntlr(fixture('covering.g4'));
  assert.deepEqual(grammar.ruleNames().map((name) => renderGrammarRule(grammar.rule(name))), [
    'normal seq(capture("name", ref(ID)), capture("values", capture("non_greedy", repeat0(ref(item)))), ref(literalRange), ref(item))',
    'normal choice(any, notClass(char(";")), not(literal("x")))',
    'normal range("a", "z")',
    'token class(range("a", "z"), range("0", "9"), char("_"))',
    'silent class(range("0", "9"))',
    'token seq(literal("//"), repeat0(notClass(char("\\r"), char("\\n"))))',
    'token literal("a")',
  ]);

  const errors = [
    ['grammar Bad; start : ( missing ;', 'antlr import parse error: expected \')\' at byte 31'],
    ['grammar Bad; start : \'ab\'..\'z\' ;', 'antlr import parse error: range start "ab" must contain one character at byte 21'],
    ['grammar Bad; start : # ;', 'antlr import parse error: unexpected character \'#\' at byte 21'],
    ['grammar Empty;', 'antlr import parse error: ANTLR grammar does not contain rules'],
    ['grammar Bad; rule [int x] : \'x\' ;', 'antlr import unsupported construct: rule arguments'],
    ['grammar Bad; start : \'é日\' § ;', 'antlr import parse error: unexpected character \'§\' at byte 29'],
    ['grammar Bad; start : \'x\u0301\'..\'z\' ;', 'antlr import parse error: range start "x\\u{301}" must contain one character at byte 21'],
    ['lexer grammar Esc;\nA : \'\\uD800\' ;', 'antlr import parse error: invalid unicode escape at byte 23'],
    ['grammar D;\nr : \'r\' ;\nmode INSIDE', 'antlr import parse error: expected \';\' after directive at byte 0'],
  ];
  for (const [source, message] of errors) {
    assert.throws(() => importAntlr(source), (error) => error.message === message, source);
  }
});
