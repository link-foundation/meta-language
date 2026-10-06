// Requirement I195-GRAMMAR-NATIVE-JAVA: the native merged Java grammar,
// parity/grammars/native/java.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-java 0.23.5 grammar, builds the
// concrete syntax trees of the tree-sitter-java oracle the native grammar
// replaced as the default Java parse. parity/fixtures/native-grammars/java.json holds the
// upstream corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_java.rs
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'java');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-JAVA',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-java',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native Java grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'program');
  for (const rule of [
    'program', 'package_declaration', 'import_declaration', 'module_declaration', 'class_declaration', 'record_declaration',
    'interface_declaration', 'enumeration_declaration', 'annotation_type_declaration', 'class_body', 'field_declaration',
    'method_declaration', 'constructor_declaration', 'formal_parameters', 'type_parameters', 'generic_type', 'array_type',
    'block', 'local_variable_declaration', 'if_statement', 'for_statement', 'enhanced_for_statement', 'try_statement',
    'switch_expression', 'yield_statement', 'lambda_expression', 'method_reference', 'method_invocation',
    'object_creation_expression', 'binary_expression', 'instance_of_expression', 'ternary_expression', 'cast_expression',
    'annotation', 'element_value_pair', 'string_literal', 'identifier', 'line_comment', 'block_comment',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // tree-sitter-java has no external scanner.
  assert.doesNotMatch(links, /^\(scanner /mu);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeJavaGrammarIsCanonicalLinks'], context.name);
});

test('the native Java grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The corpus sources the oracle parses without an error come first: at
  // this revision, every one of them.
  const cases = corpusCases(grammarSourceOf('native-java')).map(({ source }) => source);
  const corpus = cases.filter((source) => !oracleRecovers(source, 'Java'));
  assert.equal(corpus.length, cases.length);
  assert.ok(corpus.length >= 100);
  assert.deepEqual(fixture.matches.slice(0, corpus.length).map(({ source }) => source), corpus);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Java'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Java'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // The renamed `instance_of_expression` rule keeps the oracle kind `instanceof_expression`.
  assert.equal(fixture.oracleKinds.instance_of_expression, 'instanceof_expression');
  assert.deepEqual(fixture.divergences, []);
  observe(['nativeJavaTreesMatchOracle'], context.name);
});

test('the native Java grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Java'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeJavaRejectsInvalidInput'], context.name);
});

test('native Java trees keep every byte of the source, comments and white space included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('// c\nclass A { /* d */ int x = 1; }\n').tree)
    .map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    ['line_comment', '// c'], [null, '\n'], [null, 'class'], [null, ' '], ['identifier', 'A'], [null, ' '], [null, '{'],
    [null, ' '], ['block_comment', '/* d */'], [null, ' '], [null, 'int'], [null, ' '], ['identifier', 'x'], [null, ' '],
    [null, '='], [null, ' '], ['decimal_integer_literal', '1'], [null, ';'], [null, ' '], [null, '}'], [null, '\n'],
  ]);
  observe(['nativeJavaTreesLossless'], context.name);
});

test('native Java trees follow the oracle on a generic type, a method reference and an annotation argument', (context) => {
  // `A<B> c;` forks at the declared conflict of a generic type and a primary
  // expression, and the generic type's dynamic precedence keeps it; `b` of
  // `b::m` is reduced alone to a type and to a primary expression, a
  // declared reduce/reduce conflict the rule defined first wins; and
  // `v = 1` closes an element value pair of precedence 2 against an
  // assignment of 1.
  for (const source of ['class A { void f() { A<B> c; } }\n', 'class A { void f() { a = b::m; } }\n', '@A(v = 1) class C {}\n']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, [], JSON.stringify(source));
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'Java'), JSON.stringify(source));
  }
  observe(['nativeJavaTreesMatchOracle'], context.name);
});

test('the native Java fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
