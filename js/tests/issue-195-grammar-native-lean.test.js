// Requirement I195-GRAMMAR-NATIVE-LEAN: the native merged Lean grammar,
// parity/grammars/native/lean.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-lean4 grammar with its external layout
// scanner ported to a native scanner link, builds the concrete syntax trees of
// the tree-sitter-lean4 oracle the native grammar replaced as the default Lean
// parse. parity/fixtures/native-grammars/lean.json holds the upstream corpus
// with the oracle rows; rust/tests/unit/issue_195_grammar_native_lean.rs checks
// the Rust executor against the same fixture.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { compileGrammar, parseGrammarLinks, renderGrammarLinks } from '../src/index.js';
import {
  NATIVE_GRAMMARS,
  buildNativeGrammarFixture,
  fixturePath,
  renderFixture,
} from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';
import { corpusCases, grammarSourceOf } from '../scripts/import-native-grammars.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'lean');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-LEAN',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-lean',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native Lean grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'module');
  for (const rule of [
    'module', 'import', 'command', 'set_option', 'namespace', 'section', 'open', 'variable', 'universe', 'hash_command',
    'declaration', 'attributes', 'definition', 'instance_declaration', 'structure', 'inductive', 'constructor', 'explicit_binder',
    'implicit_binder', 'instance_binder', 'application', 'projection', 'explicit', 'fun', 'quantifier', 'by', 'tactic',
    'match', 'match_arm', 'do', 'do_let', 'anonymous_constructor', 'structure_instance', 'string', 'identifier', 'comment',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // The external layout scanner of tree-sitter-lean4 is ported to a native scanner link.
  assert.deepEqual([...links.matchAll(/^\(scanner ([a-z_]+) /gmu)].map(([, name]) => name), ['layout']);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeLeanGrammarIsCanonicalLinks'], context.name);
});

test('the native Lean grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The corpus sources the oracle parses without an error come first.
  const corpus = corpusCases(grammarSourceOf('native-lean'))
    .map(({ source }) => source)
    .filter((source) => !oracleRecovers(source, 'Lean'));
  assert.ok(corpus.length >= 200);
  assert.deepEqual(fixture.matches.slice(0, corpus.length).map(({ source }) => source), corpus);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Lean'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Lean'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // `pp` is a name of level 0 right that one parse ends before a projection
  // `.all` and the other after `all`; right associativity keeps the dotted name.
  assert.deepEqual(fixture.matches.find(({ source }) => source === 'set_option pp.all true\n').rows.slice(3, 6), [
    [2, 'name', 'identifier', 1, 11, 13, ''], [2, 'name', '.', 0, 13, 14, ''], [2, 'name', 'identifier', 1, 14, 17, ''],
  ]);
  observe(['nativeLeanTreesMatchOracle'], context.name);
});

test('the merged Lean grammar reads the explicit functions of Theorem Proving in Lean 4', (context) => {
  assert.deepEqual(fixture.divergences.map(({ source }) => source), entry.divergences.map(({ source }) => source));
  assert.ok(fixture.divergences.length >= 1);
  for (const { source, reason, rows } of fixture.divergences) {
    assert.match(reason, /tree-sitter-lean4 0\.3\.0/u);
    assert.match(reason, /Theorem Proving in Lean 4 section/u);
    assert.ok(oracleRecovers(source, 'Lean'), `the oracle recovers from ${JSON.stringify(source)}`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, source);
  }
  // `#check @foo.bar` projects `bar` out of the explicit `@foo`.
  assert.deepEqual(fixture.divergences[0].rows.slice(-7), [
    [1, null, 'hash_command', 1, 56, 71, ''], [2, null, 'projection', 1, 63, 71, ''],
    [3, 'term', 'explicit', 1, 63, 67, ''], [4, null, '@', 0, 63, 64, ''], [4, null, 'identifier', 1, 64, 67, ''],
    [3, null, '.', 0, 67, 68, ''], [3, 'name', 'identifier', 1, 68, 71, ''],
  ]);
  observe(['nativeLeanAcceptsMergedSourceExtensions'], context.name);
});

test('the native Lean grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 40);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Lean'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeLeanRejectsInvalidInput'], context.name);
});

test('native Lean trees keep every byte of the source, comments and white space included', (context) => {
  for (const source of [...fixture.matches, ...fixture.divergences].map((item) => item.source)) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('-- a\ndef f (x : Nat) : Nat := x.succ /- b -/\n').tree)
    .map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    ['comment', '-- a'], [null, '\n'], [null, 'def'], [null, ' '], ['identifier', 'f'], [null, ' '], [null, '('],
    ['identifier', 'x'], [null, ' '], [null, ':'], [null, ' '], ['identifier', 'Nat'], [null, ')'], [null, ' '], [null, ':'],
    [null, ' '], ['identifier', 'Nat'], [null, ' '], [null, ':='], ['unnamed_token', ' '], ['identifier', 'x'], [null, '.'],
    ['identifier', 'succ'], [null, ' '], ['comment', '/- b -/'], ['unnamed_token', ''], [null, '\n'],
  ]);
  observe(['nativeLeanTreesLossless'], context.name);
});

test('explicit Lean applications attach each argument to the preceding application', () => {
  const source = '#check @plant Tree rain\n';
  const outcome = parser.parseTree(source);
  assert.equal(outcome.ok, true);
  assert.deepEqual(outcome.ambiguities, []);
  const rows = nativeRows(outcome.tree, source, fixture);
  assert.deepEqual(rows.filter((row) => row[2] === 'application').map((row) => [row[4], row[5]]), [[7, 23], [7, 18]]);
  assert.deepEqual(rows.filter((row) => row[2] === 'explicit').map((row) => [row[4], row[5]]), [[7, 13]]);
});

test('the native Lean fixture is current', () => {
  const actual = buildNativeGrammarFixture(entry);
  for (const key of ['matches', 'divergences', 'rejections']) {
    for (const [index, observed] of actual[key].entries()) {
      if (JSON.stringify(observed) !== JSON.stringify(fixture[key][index])) console.error('native fixture change:', JSON.stringify({ grammar: entry.id, key, index, observed }));
    }
  }
  assert.equal(renderFixture(actual), read(fixturePath(entry)));
});
