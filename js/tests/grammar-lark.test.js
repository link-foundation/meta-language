// JavaScript twins of rust/tests/unit/grammar_import_lark.rs and
// rust/tests/integration/grammar_import_lark.rs, with the same expectations.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { readProjectData } from '../scripts/project-data.mjs';
import { deserializeGrammar, serializeGrammar } from '../src/grammar.js';
import { GrammarImportError } from '../src/grammar-importers/common.js';
import { createGrammarParser, emitLark, importLark } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';
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
    kind: 'token',
    item: {
      kind: 'repeat1',
      item: { kind: 'charClass', items: [{ kind: 'range', start: 'a', end: 'z' }], negated: false },
    },
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

const regexFixture = 'parity/fixtures/lark-regex-terminals.json';
const regexCases = readProjectData(new URL('../../' + regexFixture, import.meta.url));

for (const [group, requirementId, assertion] of [
  ['cases', 'I195-I233-LARK-REGEX-TERMINALS', 'regexTerminalsMatchTheirLanguage'],
  ['flags', 'I195-I233-LARK-REGEX-FLAGS', 'supportedRegexFlagsChangeMatching'],
]) {
  test('Lark regex ' + group + ' match before and after emission', (context) => {
    for (const { source, accept, reject } of regexCases[group]) {
      const grammar = importLark(source);
      for (const inputGrammar of [grammar, importLark(emitLark(grammar).source)]) {
        const parser = createGrammarParser(inputGrammar);
        for (const [inputs, expected] of [[accept, true], [reject, false]]) {
          for (const input of inputs) {
            assert.equal(parser.parseTree(input).ok, expected, source + ' ' + JSON.stringify(input));
          }
        }
      }
    }
    recordIssue195Observations({
      requirementId, suffix: 'behavior', fixtureId: requirementId.toLowerCase(), fixtureFile: regexFixture,
      assertions: [assertion], testName: context.name,
    });
  });
}

test('Lark refuses unsupported and malformed regexes at their UTF-8 byte positions', (context) => {
  for (const [source, diagnostic] of regexCases.diagnostics) {
    assert.throws(() => importLark(source), {
      message: 'lark import parse error: ' + diagnostic,
    });
  }
  recordIssue195Observations({
    requirementId: 'I195-I233-LARK-REGEX-DIAGNOSTICS', suffix: 'behavior',
    fixtureId: 'i233-lark-regex-diagnostics', fixtureFile: regexFixture,
    assertions: ['unsupportedRegexReportsSourceByte'], testName: context.name,
  });
});
