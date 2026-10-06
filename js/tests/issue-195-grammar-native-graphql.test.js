// Requirement I195-GRAMMAR-NATIVE-GRAPHQL: the native merged GraphQL grammar,
// parity/grammars/native/graphql.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-graphql 0.3.0 grammar, builds the
// concrete syntax trees of the tree-sitter-graphql oracle the native grammar
// replaced as the default GraphQL parse. parity/fixtures/native-grammars/graphql.json holds the
// upstream corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_graphql.rs
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'graphql');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-GRAPHQL',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-graphql',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

// The kind of the node each comma leaf is a child of, and whether it is trivia there.
function commas(tree, parent = null) {
  if (tree.type !== 'node') return tree.kind === 'comma' ? [[parent, Boolean(tree.trivia)]] : [];
  return tree.children.flatMap((child) => commas(child, tree.kind));
}

test('the native GraphQL grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'source_file');
  for (const rule of [
    'document', 'definition', 'executable_definition', 'operation_definition', 'operation_type', 'variable_definitions',
    'variable_definition', 'selection_set', 'selection', 'field', 'alias', 'arguments', 'argument', 'value', 'variable',
    'string_value', 'integer_value', 'float_value', 'boolean_value', 'null_value', 'enumeration_value', 'list_value',
    'object_value', 'object_field', 'fragment_spread', 'fragment_definition', 'inline_fragment', 'type_condition',
    'directives', 'directive', 'directive_definition', 'schema_definition', 'object_type_definition',
    'enumeration_type_definition', 'input_object_type_definition', 'type_extension', 'named_type', 'list_type',
    'non_null_type', 'name', 'comment', 'comma', 'description',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // tree-sitter-graphql has no external scanner.
  assert.doesNotMatch(links, /^\(scanner /mu);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(graphql /u);
  observe(['nativeGraphqlGrammarIsCanonicalLinks'], context.name);
});

test('the native GraphQL grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The oracle parses every corpus source at this revision without an error.
  const cases = corpusCases(grammarSourceOf('native-graphql')).map(({ source }) => source);
  assert.equal(cases.length, 4);
  assert.deepEqual(fixture.matches.slice(0, cases.length).map(({ source }) => source), cases);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'GraphQL'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'GraphQL'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  assert.deepEqual(fixture.divergences, []);
  observe(['nativeGraphqlTreesMatchOracle'], context.name);
});

test('the native GraphQL grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'GraphQL'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeGraphqlRejectsInvalidInput'], context.name);
});

test('native GraphQL trees keep every byte of the source, line breaks included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  // A comma is an extra, but the token of a variable definition and of an
  // object field that names it: tree-sitter takes an extra token only where
  // the parse state has no action on it.
  assert.deepEqual(commas(parser.parseTree('query Q($a: Int, $b: Int) { f(o: {a: 1, b: 2}, x: [1, 2]) }').tree), [
    ['variable_definition', false], ['object_field', false], ['argument', true], ['value', true],
  ]);
  assert.deepEqual(commas(parser.parseTree(',{ a, b }').tree), [['selection_set', true], ['field', true]]);
  observe(['nativeGraphqlTreesLossless'], context.name);
});

test('the native GraphQL fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
