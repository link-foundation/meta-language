// Requirement I195-GRAMMAR-NATIVE-MAKE: the native merged Make grammar,
// parity/grammars/native/make.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-make 1.1.1 grammar, builds the
// concrete syntax trees of the tree-sitter-make oracle the native grammar
// replaced as the default Make parse. parity/fixtures/native-grammars/make.json holds the
// upstream corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_make.rs
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'make');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-MAKE',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-make',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

// The kind of each leaf, the kind of the node it is a child of, its text and whether it is trivia there.
function placed(tree, parent = null) {
  if (tree.type !== 'node') return [[tree.kind ?? null, parent, tree.text, Boolean(tree.trivia)]];
  return tree.children.flatMap((child) => placed(child, tree.kind));
}

test('the native Make grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'makefile');
  for (const rule of [
    'makefile', 'rule', 'recipe', 'recipe_line', 'search_path_assignment', 'recipe_prefix_assignment', 'variable_assignment',
    'shell_assignment', 'define_directive', 'include_directive', 'search_path_directive', 'export_directive', 'remove_export_directive',
    'override_directive', 'remove_definition_directive', 'private_directive', 'conditional', 'else_if_directive', 'else_directive',
    'if_equal_directive', 'if_not_equal_directive', 'if_defined_directive', 'if_not_defined_directive', 'variable_reference',
    'substitution_reference', 'automatic_variable', 'function_call', 'arguments', 'shell_function', 'list', 'paths', 'concatenation',
    'string', 'archive', 'shell_text_with_split', 'text', 'word', 'comment', 'raw_line',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // tree-sitter-make has no external scanner.
  assert.doesNotMatch(links, /^\(scanner /mu);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(make /u);
  observe(['nativeMakeGrammarIsCanonicalLinks'], context.name);
});

test('the native Make grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The oracle parses every corpus source at this revision without an error
  // but the three custom `.RECIPEPREFIX` cases its corpus marks TODO, which
  // the native grammar rejects with the other invalid input.
  const all = corpusCases(grammarSourceOf('native-make')).map(({ source }) => source);
  assert.equal(all.length, 103);
  const cases = all.filter((source) => !oracleRecovers(source, 'Make'));
  assert.equal(cases.length, 100);
  assert.deepEqual(fixture.matches.slice(0, cases.length).map(({ source }) => source), cases);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Make'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Make'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  assert.deepEqual(fixture.divergences, []);
  observe(['nativeMakeTreesMatchOracle'], context.name);
});

test('the native Make grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  // A rule whose recipe line lacks the tab, and a target `(` of an unclosed
  // reference: the lexer takes the line break after the colon as a token and
  // the `(` as the literal, as tree-sitter does, before the line is read.
  for (const source of ['a:\nb\n', 'a = $(b\n']) assert.ok(entry.rejections.includes(source), JSON.stringify(source));
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Make'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeMakeRejectsInvalidInput'], context.name);
});

test('native Make trees keep every byte of the source, line breaks included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  // A comment is trivia before the node after it, which the rows put in the
  // makefile as the oracle does; the reference and the text after it are one
  // value, and a recipe line keeps its `@` and the shell text, as in the oracle.
  assert.deepEqual(placed(parser.parseTree('a = $(b) x\n# c\nall:\n\t@echo $$y\n').tree).filter(([kind]) => kind !== null), [
    ['word', 'variable_assignment', 'a', false], ['unnamed_token', 'variable_assignment', ' ', false],
    ['unnamed_token', 'variable_assignment', ' ', false], ['word', 'variable_reference', 'b', false],
    ['unnamed_token', 'text', ' x', false], ['unnamed_token', 'variable_assignment', '\n', false],
    ['comment', 'targets', '# c', true], ['word', 'targets', 'all', false], ['unnamed_token', 'recipe', '\n', false],
    ['unnamed_token', 'shell_text', 'echo ', false], ['word', 'variable_reference', 'y', false], ['unnamed_token', 'recipe', '\n', false],
  ]);
  observe(['nativeMakeTreesLossless'], context.name);
});

test('the native Make fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
