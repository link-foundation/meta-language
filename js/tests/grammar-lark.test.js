// JavaScript twins of rust/tests/unit/grammar_import_lark.rs and
// rust/tests/integration/grammar_import_lark.rs, with the same expectations.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { deserializeGrammar, serializeGrammar } from '../src/grammar.js';
import { GrammarImportError } from '../src/grammar-importers/common.js';
import { importLark } from '../src/grammar-importers/lark.js';
import { renderGrammarRule } from './support/render-grammar-expression.js';

const covering = readFileSync(
  new URL('../../rust/tests/fixtures/grammar/lark/covering.lark', import.meta.url),
  'utf8',
);

test('imports covering lark fixture', () => {
  const grammar = importLark(covering);

  assert.equal(grammar.sourceFormat, 'lark');
  assert.equal(grammar.startRule()?.name, 'start');
  assert.deepEqual(
    grammar.ruleNames(),
    ['start', 'item', 'trailer', 'WORD', 'NUMBER', 'WS', '_ignore'],
  );
  assert.equal(grammar.rule('start').kind, 'normal');
  assert.equal(grammar.rule('item').kind, 'silent');
  assert.equal(grammar.ruleDocs.get('item'), 'inline');
  assert.equal(grammar.rule('WORD').kind, 'token');
  assert.equal(grammar.ruleDocs.get('_ignore'), '%ignore');

  assert.deepEqual(grammar.rule('start').expression, {
    kind: 'seq',
    items: [
      { kind: 'ref', name: 'item' },
      {
        kind: 'repeat0',
        item: {
          kind: 'seq',
          items: [{ kind: 'literal', value: ',' }, { kind: 'ref', name: 'item' }],
        },
      },
      { kind: 'optional', item: { kind: 'ref', name: 'trailer' } },
    ],
  });
  assert.deepEqual(grammar.rule('item').expression, {
    kind: 'choice',
    items: [
      { kind: 'ref', name: 'WORD' },
      { kind: 'repeat', item: { kind: 'ref', name: 'NUMBER' }, min: 2, max: 2 },
    ],
    ordered: false,
  });
  assert.deepEqual(grammar.rule('WORD').expression, {
    kind: 'charClass',
    items: [{ kind: 'range', start: 'A', end: 'Z' }],
    negated: false,
  });
  assert.deepEqual(grammar.rule('trailer').expression, {
    kind: 'capture',
    label: 'regex',
    item: { kind: 'literal', value: '[a-z]+' },
  });
  assert.deepEqual(grammar.undefinedNonterminals(), []);

  // The runtime-neutral rendering used by the shared parity fixtures.
  assert.equal(
    renderGrammarRule(grammar.rule('start')),
    'normal seq(ref(item), repeat0(seq(literal(","), ref(item))), optional(ref(trailer)))',
  );
});

test('prefers conventional start rule over first lark rule', () => {
  const grammar = importLark(`
other: "x"
start: other
`);

  assert.equal(grammar.startRule()?.name, 'start');
});

test('malformed lark reports parse error', () => {
  assert.throws(() => importLark('start: [unterminated'), (error) =>
    error instanceof GrammarImportError && error.kind === 'parse' && error.format === 'lark');
});

test('unresolved lark import reports unsupported error', () => {
  assert.throws(() => importLark('%import common.WS'), (error) =>
    error instanceof GrammarImportError &&
    error.kind === 'unsupported' &&
    error.format === 'lark' &&
    error.construct === '%import');
});

// JavaScript has no links encoder for grammars; the serialized grammar
// document is its round-trip representation.
test('imported lark grammar survives serialization round trip', () => {
  const grammar = importLark(covering);

  const restored = deserializeGrammar(serializeGrammar(grammar));

  assert.deepEqual(restored.normalized(), grammar.normalized());
});
