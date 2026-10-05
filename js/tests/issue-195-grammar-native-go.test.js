// Requirement I195-GRAMMAR-NATIVE-GO: the native merged Go grammar,
// parity/grammars/native/go.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-go 0.25.0 grammar, builds the
// concrete syntax trees of the tree-sitter-go oracle the native grammar
// replaced as the default Go parse. parity/fixtures/native-grammars/go.json holds the
// upstream corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_go.rs
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'go');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-GO',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-go',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native Go grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'source_file');
  for (const rule of [
    'source_file', 'package_clause', 'import_declaration', 'import_specification', 'constant_declaration', 'variable_declaration',
    'function_declaration', 'method_declaration', 'type_parameter_list', 'parameter_list', 'type_alias', 'type_declaration',
    'generic_type', 'pointer_type', 'array_type', 'slice_type', 'structure_type', 'interface_type', 'map_type', 'channel_type',
    'function_type', 'block', 'short_variable_declaration', 'assignment_statement', 'labeled_statement', 'if_statement',
    'for_statement', 'range_clause', 'expression_switch_statement', 'type_switch_statement', 'select_statement',
    'go_statement', 'defer_statement', 'send_statement', 'call_expression', 'selector_expression', 'index_expression',
    'slice_expression', 'type_assertion_expression', 'type_conversion_expression', 'composite_literal', 'function_literal',
    'unary_expression', 'binary_expression', 'raw_string_literal', 'interpreted_string_literal', 'rune_literal',
    'imaginary_literal', 'identifier', 'comment',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // tree-sitter-go has no external scanner.
  assert.doesNotMatch(links, /^\(scanner /mu);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeGoGrammarIsCanonicalLinks'], context.name);
});

test('the native Go grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The corpus sources the oracle parses without an error come first: at
  // this revision, all but the two it recovers from, which are rejections.
  const cases = corpusCases(grammarSourceOf('native-go')).map(({ source }) => source);
  const corpus = cases.filter((source) => !oracleRecovers(source, 'Go'));
  assert.equal(corpus.length, cases.length - 2);
  assert.ok(corpus.length >= 60);
  assert.deepEqual(fixture.matches.slice(0, corpus.length).map(({ source }) => source), corpus);
  assert.deepEqual(fixture.rejections.slice(0, 2).map(({ source }) => source), cases.filter((source) => oracleRecovers(source, 'Go')));
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Go'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Go'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // The renamed `short_variable_declaration` rule keeps the oracle kind `short_var_declaration`.
  assert.equal(fixture.oracleKinds.short_variable_declaration, 'short_var_declaration');
  assert.deepEqual(fixture.divergences, []);
  observe(['nativeGoTreesMatchOracle'], context.name);
});

test('the native Go grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Go'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeGoRejectsInvalidInput'], context.name);
});

test('native Go trees keep every byte of the source, comments and white space included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  // The newline that ends a declaration is a token of the source file, not
  // white space.
  const kinds = leaves(parser.parseTree('// c\npackage main /* d */\nvar x = 1\n').tree)
    .map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    ['comment', '// c'], [null, '\n'], [null, 'package'], [null, ' '], ['package_identifier', 'main'], [null, ' '],
    ['comment', '/* d */'], ['unnamed_token', '\n'], [null, 'var'], [null, ' '], ['identifier', 'x'], [null, ' '], [null, '='],
    [null, ' '], ['integer_literal', '1'], ['unnamed_token', '\n'],
  ]);
  observe(['nativeGoTreesLossless'], context.name);
});

test('native Go trees follow the oracle on a channel type, a type conversion, a generic call and a statement list', (context) => {
  // `chan<- chan int` is a `chan<-` channel type of `chan int`, where the
  // parse that shifts `<-` goes on to a sibling ending where the other's
  // `<- chan int` does; `<-chan int(c)` converts to a `<-chan int` channel
  // type, the reduce/reduce conflict on `int` its precedence 6 wins; and
  // `a[b](c)` converts to the generic type `a[b]`, ahead on the stack's
  // dynamic precedence at `(` and tied with the call where the two merge;
  // and of `x := a\n\ty := b`, the newline is the token that ends the first
  // statement, not white space the other parse skips before the second.
  for (const source of [
    'package main\n\nvar c chan<- chan int\n', 'package main\n\nvar x = <-chan int(c)\n', 'package main\n\nvar x = a[b](c)\n',
    'package main\n\nfunc main() {\n\tx := a\n\ty := b\n}\n',
  ]) {
    const outcome = parser.parseTree(source);
    assert.ok(fixture.matches.some((match) => match.source === source), JSON.stringify(source));
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, [], JSON.stringify(source));
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'Go'), JSON.stringify(source));
  }
  observe(['nativeGoTreesMatchOracle'], context.name);
});

test('the native Go fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
