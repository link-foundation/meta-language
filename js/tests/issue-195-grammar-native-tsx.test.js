// Requirement I195-GRAMMAR-NATIVE-TSX: the native merged TSX grammar,
// parity/grammars/native/tsx.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-typescript TSX grammar with its
// external scanner ported to native scanner links, builds the concrete syntax
// trees of the tree-sitter-tsx oracle the native grammar replaced as the
// default TSX parse. parity/fixtures/native-grammars/tsx.json holds the
// upstream corpus with the oracle rows;
// rust/tests/unit/issue_195_grammar_native_tsx.rs checks the Rust executor
// against the same fixture.
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'tsx');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-TSX',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-tsx',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native TSX grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'program');
  for (const rule of [
    'program', 'interface_declaration', 'type_alias_declaration', 'enumeration_declaration', 'abstract_class_declaration',
    'internal_module', 'module', 'ambient_declaration', 'import_alias', 'type_annotation', 'generic_type', 'type_arguments',
    'type_parameters', 'union_type', 'intersection_type', 'conditional_type', 'mapped_type_clause', 'template_literal_type',
    'tuple_type', 'as_expression', 'satisfies_expression', 'non_null_expression', 'instantiation_expression', 'jsx_element',
    'jsx_self_closing_element', 'jsx_opening_element', 'jsx_closing_element', 'jsx_attribute', 'jsx_expression', 'jsx_namespace_name',
    'decorator', 'arrow_function', 'call_expression', 'unary_expression', 'identifier',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // The external scanner of tree-sitter-typescript is ported to native scanner links.
  assert.deepEqual([...links.matchAll(/^\(scanner ([a-z_]+) /gmu)].map(([, name]) => name), [
    'automatic_semicolon', 'template_characters', 'ternary_question_mark', 'html_comment', 'jsx_text',
    'function_signature_automatic_semicolon', 'error_recovery',
  ]);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeTsxGrammarIsCanonicalLinks'], context.name);
});

test('the native TSX grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  const corpus = corpusCases(grammarSourceOf('native-tsx')).map(({ source }) => source);
  assert.ok(corpus.length >= 100);
  assert.deepEqual(fixture.matches.slice(0, corpus.length).map(({ source }) => source), corpus);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'TSX'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'TSX'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  const rowsOf = (source) => fixture.matches.find((match) => match.source === source).rows;
  // Where a unary operand meets `<`, the generated parser forks at a declared
  // conflict and keeps the parse that reduced the operand: `!g<T>()` calls
  // `!g`, and `await g<T>` instantiates `await g`.
  assert.deepEqual(rowsOf('!g<T>();').slice(2, 6), [
    [2, null, 'call_expression', 1, 0, 7, ''], [3, 'function', 'unary_expression', 1, 0, 2, ''],
    [4, 'operator', '!', 0, 0, 1, ''], [4, 'argument', 'identifier', 1, 1, 2, ''],
  ]);
  assert.deepEqual(rowsOf('await g<T>;').slice(2, 6), [
    [2, null, 'instantiation_expression', 1, 0, 10, ''], [3, null, 'await_expression', 1, 0, 7, ''],
    [4, null, 'await', 0, 0, 5, ''], [4, null, 'identifier', 1, 6, 7, ''],
  ]);
  // TSX reads `<div ...>{b}</div>` as a JSX element.
  assert.deepEqual(rowsOf('x = <div className="a">{b}</div>;').slice(5, 7), [
    [3, 'right', 'jsx_element', 1, 4, 32, ''], [4, 'open_tag', 'jsx_opening_element', 1, 4, 23, ''],
  ]);
  observe(['nativeTsxTreesMatchOracle'], context.name);
});

test('the native TSX grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.divergences, []);
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 16);
  // An unclosed JSX element is no TSX.
  assert.ok(entry.rejections.includes('x = <div>;'));
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'TSX'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeTsxRejectsInvalidInput'], context.name);
});

test('native TSX trees keep every byte of the source, comments and white space included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('// a\nconst v = <A.B c={1}>t {y}</A.B>;\n').tree)
    .map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    ['comment', '// a'], [null, '\n'], [null, 'const'], [null, ' '], ['identifier', 'v'], [null, ' '], [null, '='], [null, ' '],
    [null, '<'], ['identifier', 'A'], [null, '.'], ['property_identifier', 'B'], [null, ' '], ['property_identifier', 'c'],
    [null, '='], [null, '{'], ['number', '1'], [null, '}'], [null, '>'], ['jsx_text', 't '], [null, '{'], ['identifier', 'y'],
    [null, '}'], [null, '</'], ['identifier', 'A'], [null, '.'], ['property_identifier', 'B'], [null, '>'], [null, ';'], [null, '\n'],
  ]);
  observe(['nativeTsxTreesLossless'], context.name);
});

test('the native TSX fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
