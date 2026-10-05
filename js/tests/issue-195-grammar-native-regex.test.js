// Requirement I195-GRAMMAR-NATIVE-REGEX: the native merged Regex grammar,
// parity/grammars/native/regex.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-regex 0.25.0 grammar, builds the
// concrete syntax trees of the tree-sitter-regex oracle the native grammar
// replaced as the default Regex parse. parity/fixtures/native-grammars/regex.json holds the
// upstream corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_regex.rs
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'regex');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-REGEX',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-regex',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native Regex grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'pattern');
  for (const rule of [
    'pattern', 'alternation', 'term', 'any_character', 'start_assertion', 'end_assertion', 'boundary_assertion',
    'non_boundary_assertion', 'lookaround_assertion', 'lookahead_assertion', 'lookbehind_assertion', 'pattern_character',
    'character_class', 'posix_character_class', 'class_range', 'class_character', 'anonymous_capturing_group',
    'named_capturing_group', 'non_capturing_group', 'inline_flags_group', 'flags', 'zero_or_more', 'one_or_more', 'optional',
    'count_quantifier', 'backreference_escape', 'named_group_backreference', 'decimal_escape', 'character_class_escape',
    'unicode_character_escape', 'unicode_property_value_expression', 'control_escape', 'identity_escape', 'group_name',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // tree-sitter-regex has no external scanner.
  assert.doesNotMatch(links, /^\(scanner /mu);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeRegexGrammarIsCanonicalLinks'], context.name);
});

test('the native Regex grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The oracle parses every corpus source at this revision without an error.
  const cases = corpusCases(grammarSourceOf('native-regex')).map(({ source }) => source);
  assert.ok(cases.length >= 35);
  assert.deepEqual(fixture.matches.slice(0, cases.length).map(({ source }) => source), cases);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Regex'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Regex'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // A brace or a bracket that opens no quantifier or POSIX class is a
  // character, as ECMAScript reads it, where the oracle's lexer recovers.
  assert.deepEqual(fixture.divergences.map(({ source }) => source), ['a{', '[[:alpha:]']);
  for (const { source, rows } of fixture.divergences) {
    assert.ok(oracleRecovers(source, 'Regex'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, JSON.stringify(source));
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, JSON.stringify(source));
  }
  observe(['nativeRegexTreesMatchOracle'], context.name);
});

test('the native Regex grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Regex'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeRegexRejectsInvalidInput'], context.name);
});

test('native Regex trees keep every byte of the source, line breaks included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  // The line break between two terms is an extra of the pattern.
  const kinds = leaves(parser.parseTree('a\n(?:b)+?').tree).map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    ['pattern_character', 'a'], [null, '\n'], [null, '(?:'], ['pattern_character', 'b'], [null, ')'], [null, '+'], ['lazy', '?'],
  ]);
  observe(['nativeRegexTreesLossless'], context.name);
});

test('the native Regex fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
