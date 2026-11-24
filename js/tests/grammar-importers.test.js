import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import {
  GrammarImportError,
  createGrammarParser,
  deserializeGrammar,
  emitPeggy,
  emitPest,
  importAbnf,
  importBnf,
  importEbnf,
  importPest,
  importTreeSitterJson,
  parseWithGrammar,
  serializeGrammar,
} from '../src/index.js';

const corpus = readProjectData(new URL('../../parity/fixtures/grammar-importers.json', import.meta.url));

const importers = {
  abnf: importAbnf,
  bnf: importBnf,
  ebnf: importEbnf,
  pest: importPest,
  'tree-sitter-json': importTreeSitterJson,
};

const establishedFixtures = [
  ['abnf', ['postal-address.abnf', 'numeric-terminals.abnf', 'incremental.abnf']],
  ['bnf', ['arithmetic.bnf', 'list.bnf', 'postal-address.bnf']],
  ['ebnf', ['arithmetic.ebnf', 'number.ebnf', 'iso14977-sample.ebnf']],
  ['pest', ['arithmetic.pest', 'json.pest', 'predicates.pest']],
  ['tree-sitter-json', ['covering.json', 'json.json']],
];

test('shared grammar importer corpus parses and rejects the same texts', () => {
  for (const fixture of corpus.cases) {
    const grammar = importers[fixture.format](fixture.source);
    assert.equal(grammar.sourceFormat, expectedSourceFormat(fixture.format), fixture.format);
    assert.equal(grammar.startRule()?.name, fixture.start, fixture.format);
    assert.deepEqual(grammar.ruleNames().slice(0, fixture.rules.length), fixture.rules, fixture.format);
    assert.deepEqual(grammar.undefinedNonterminals(), [], fixture.format);

    for (const source of fixture.accepts) {
      assert.doesNotThrow(() => parseWithGrammar(grammar, source), `${fixture.format}: ${source}`);
    }
    for (const source of fixture.rejects) {
      assert.throws(() => parseWithGrammar(grammar, source), `${fixture.format}: ${source}`);
    }

    const emitted = emitPeggy(grammar);
    assert.match(emitted, new RegExp(`^start = ${fixture.start}$`, 'm'), fixture.format);

    const restored = deserializeGrammar(serializeGrammar(grammar));
    assert.deepEqual(restored.normalized(), grammar.normalized(), fixture.format);
    for (const source of fixture.accepts) {
      assert.doesNotThrow(() => parseWithGrammar(restored, source), fixture.format);
    }
  }
});

test('JavaScript imports every established Rust fixture for the five shared formats', () => {
  for (const [format, files] of establishedFixtures) {
    const directory = format === 'pest' ? 'peg' : format.replace('-json', '');
    for (const file of files) {
      const source = readFileSync(resolveProjectDataPath(new URL(
        `../../rust/tests/fixtures/grammar/${directory}/${file}`,
        import.meta.url,
      )), 'utf8');
      const grammar = importers[format](source);
      assert.ok(grammar.startRule(), `${format}/${file} has a start rule`);
      assert.deepEqual(grammar.undefinedNonterminals(), [], `${format}/${file} resolves every rule`);
    }
  }
});

test('ABNF preserves repetition, incremental alternatives, case, and numeric terminals', () => {
  const grammar = importAbnf(`
token = *2%x30-39
method = "get"
method =/ %s"POST"
exact = 2%x41.42
letter = ALPHA
`);

  assert.deepEqual(grammar.rule('token').expression, {
    kind: 'repeat',
    item: { kind: 'charRange', start: '0', end: '9' },
    min: 0,
    max: 2,
  });
  assert.deepEqual(grammar.rule('method').expression, {
    kind: 'choice',
    items: [
      { kind: 'literalInsensitive', value: 'get' },
      { kind: 'literal', value: 'POST' },
    ],
    ordered: false,
  });
  assert.deepEqual(grammar.rule('exact').expression, {
    kind: 'repeat',
    item: { kind: 'literal', value: 'AB' },
    min: 2,
    max: 2,
  });
  assert.ok(grammar.rule('ALPHA'));
});

test('BNF and EBNF preserve empty alternatives and reject missing references', () => {
  assert.deepEqual(importBnf('<start> ::= | "x"').rule('start').expression, {
    kind: 'choice',
    items: [{ kind: 'empty' }, { kind: 'literal', value: 'x' }],
    ordered: false,
  });
  assert.deepEqual(importEbnf('start = [ "x" ], { "y" } ;').rule('start').expression, {
    kind: 'seq',
    items: [
      { kind: 'optional', item: { kind: 'literal', value: 'x' } },
      { kind: 'repeat0', item: { kind: 'literal', value: 'y' } },
    ],
  });
  assert.throws(() => importBnf('<start> ::= <missing>'), GrammarImportError);
  assert.throws(() => importEbnf('start = missing ;'), GrammarImportError);
  assert.throws(() => importEbnf("digit = #'[0-9]' ;"), /unsupported.*inline regex/i);
  assert.throws(() => importEbnf('token = ? special ? ;'), /unsupported.*special sequence/i);
});

test('pest preserves predicates, counted repetition, modifiers, and unsupported constructs', () => {
  const grammar = importPest(`
start = { &"a" ~ !"b" ~ ANY }
exact = _{ "x"{2} }
range = @{ "y"{1,3} }
`);
  assert.equal(grammar.rule('exact').kind, 'silent');
  assert.equal(grammar.rule('range').kind, 'atomic');
  assert.deepEqual(grammar.rule('start').expression, {
    kind: 'seq',
    items: [
      { kind: 'and', item: { kind: 'literal', value: 'a' } },
      { kind: 'not', item: { kind: 'literal', value: 'b' } },
      { kind: 'any' },
    ],
  });
  assert.throws(() => importPest('start = { PUSH("x") }'), /unsupported.*Push/i);
  assert.throws(() => importPest('start = { missing }'), /missing/);
});

const pestFixture = 'parity/fixtures/pest-built-ins.lino';
const pestCases = readProjectData(new URL('../../' + pestFixture, import.meta.url));

function observePest(requirementId, context, assertions) {
  recordIssue195Observations({
    requirementId, suffix: 'behavior', fixtureId: requirementId.toLowerCase(), fixtureFile: pestFixture,
    assertions, testName: context.name,
  });
}

function assertPestCases(cases) {
  for (const [source, accepted, rejected] of cases) {
    const grammar = importPest(source);
    assert.deepEqual(grammar.undefinedNonterminals(), [], source);
    const restored = deserializeGrammar(serializeGrammar(grammar));
    const emitted = emitPest(restored).source;
    for (const candidate of [restored, importPest(emitted)]) {
      const parser = createGrammarParser(candidate);
      for (const input of accepted) assert.equal(parser.parseTree(input).ok, true, `${source} accepts ${input}`);
      for (const input of rejected) assert.equal(parser.parseTree(input).ok, false, `${source} rejects ${input}`);
    }
  }
}

test('pest resolves ASCII and pinned Unicode properties and matches absolute input edges', (context) => {
  assert.equal(pestCases.pestVersion, '2.9.2');
  assertPestCases(pestCases.builtinCases);
  const { grammar, source, spans } = pestCases.anchorSearch;
  const anchored = createGrammarParser(importPest(grammar));
  assert.deepEqual(anchored.findAll(source).map(({ start, end }) => [start, end]), spans);
  observePest('I195-I234-PEST-BUILT-IN-RULES', context, ['builtInRulesExecute', 'anchorsUseAbsolutePositions']);
});

test('pest reports consistent undefined and unsupported rule diagnostics', (context) => {
  for (const [source, message] of pestCases.diagnostics) {
    assert.throws(() => importPest(source), { name: 'GrammarImportError', message });
  }
  observePest('I195-I234-PEST-RULE-DIAGNOSTICS', context, ['pestRuleDiagnosticsUseExactMessages']);
});

test('pest inserts defined trivia between sequence and repetition items across rule calls', (context) => {
  assertPestCases(pestCases.triviaCases);
  observePest('I195-I234-PEST-IMPLICIT-TRIVIA', context, ['implicitTriviaTracksAtomicContext']);
});

test('tree-sitter JSON preserves fields, aliases, precedence, tokens, extras, and source order', () => {
  const grammar = importTreeSitterJson(JSON.stringify({
    name: 'covering',
    rules: {
      document: { type: 'FIELD', name: 'value', content: { type: 'SYMBOL', name: 'item' } },
      item: { type: 'ALIAS', named: true, value: 'name', content: { type: 'PATTERN', value: '[a-z_]' } },
      token: { type: 'IMMEDIATE_TOKEN', content: { type: 'STRING', value: '!' } },
      precedence: { type: 'PREC_LEFT', value: 2, content: { type: 'STRING', value: 'p' } },
      empty: { type: 'BLANK' },
    },
    extras: [{ type: 'PATTERN', value: '[ \\t]' }],
  }));

  assert.deepEqual(grammar.ruleNames(), [
    'document', 'item', 'token', 'precedence', 'empty', '_extras',
  ]);
  assert.equal(grammar.rule('token').kind, 'token');
  assert.equal(grammar.rule('document').expression.label, 'value');
  assert.equal(grammar.rule('item').expression.label, 'alias:name');
  assert.equal(grammar.rule('precedence').expression.label, 'prec_left=2');
  assert.throws(() => importTreeSitterJson('{"rules":'), GrammarImportError);
  assert.throws(
    () => importTreeSitterJson('{"rules":{"start":{"type":"UNKNOWN"}}}'),
    /unsupported.*UNKNOWN/i,
  );
});

function expectedSourceFormat(format) {
  return format === 'pest' ? 'peg' : format.replace('-json', '');
}
