// Requirement I195-GRAMMAR-NATIVE-ROCQ: the native merged Rocq grammar,
// parity/grammars/native/rocq.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-rocq grammar, builds the concrete syntax
// trees of the tree-sitter-rocq oracle the native grammar replaced as the
// default Rocq parse. parity/fixtures/native-grammars/rocq.json holds the
// upstream corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_rocq.rs
// checks the Rust executor against the same fixture.
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'rocq');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-ROCQ',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-rocq',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native Rocq grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'source_file');
  for (const rule of [
    'source_file', 'sentence', 'attributes', 'require_command', 'import_command', 'evaluation_command', 'theorem_command',
    'definition_command', 'fixpoint_command', 'inductive_command', 'constructor', 'record_command', 'section_command',
    'module_command', 'notation_command', 'ltac_definition', 'proof_block', 'application', 'lambda_function',
    'let_expression', 'match_expression', 'if_expression', 'list_literal', 'binder', 'implicit_binders', 'tactic_sequence',
    'generic_tactic', 'intro_pattern', 'qualified_identifier', 'identifier', 'integer', 'string', 'comment',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // tree-sitter-rocq has no external scanner; a comment nests through its own rule.
  assert.doesNotMatch(links, /^\(scanner /mu);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeRocqGrammarIsCanonicalLinks'], context.name);
});

test('the native Rocq grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The corpus sources the oracle parses without an error come first.
  const corpus = corpusCases(grammarSourceOf('native-rocq'))
    .map(({ source }) => source)
    .filter((source) => !oracleRecovers(source, 'Rocq'));
  assert.ok(corpus.length >= 190);
  assert.deepEqual(fixture.matches.slice(0, corpus.length).map(({ source }) => source), corpus);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Rocq'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Rocq'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // The renamed `identifier` rule keeps the oracle kind `ident`.
  assert.equal(fixture.oracleKinds.identifier, 'ident');
  assert.deepEqual(fixture.divergences, []);
  observe(['nativeRocqTreesMatchOracle'], context.name);
});

test('the native Rocq grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Rocq'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeRocqRejectsInvalidInput'], context.name);
});

test('native Rocq trees keep every byte of the source, comments and white space included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('(* a (* b *) *)\nDefinition f (x : nat) : nat := S x.\n').tree)
    .map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    [null, '(*'], ['unnamed_token', ' a '], [null, '(*'], ['unnamed_token', ' b '], [null, '*)'], ['unnamed_token', ' '],
    [null, '*)'], [null, '\n'], [null, 'Definition'], [null, ' '], ['identifier', 'f'], [null, ' '], [null, '('],
    ['identifier', 'x'], [null, ' '], [null, ':'], [null, ' '], ['identifier', 'nat'], [null, ')'], [null, ' '], [null, ':'],
    [null, ' '], ['identifier', 'nat'], [null, ' '], [null, ':='], [null, ' '], ['identifier', 'S'], [null, ' '],
    ['identifier', 'x'], [null, '.'], [null, '\n'],
  ]);
  observe(['nativeRocqTreesLossless'], context.name);
});

test('native Rocq trees follow the oracle on a byte order mark, an end after a match and an unclosed nested comment', (context) => {
  // A tree-sitter lexer skips a byte order mark at the start of the input, and
  // `end` after the `end` of a match is no keyword of the closed match.
  for (const source of ['﻿Check x.', 'Check match x with y => y end > 0 end.']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'Rocq'), JSON.stringify(source));
  }
  // An unclosed nested comment is no comment.
  assert.equal(parser.parseTree('(* Outer (* Inner *)').ok, false);
  observe(['nativeRocqTreesMatchOracle', 'nativeRocqRejectsInvalidInput'], context.name);
});

test('the native Rocq fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
