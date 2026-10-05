// Requirement I195-GRAMMAR-NATIVE-TYPESCRIPT: the native merged TypeScript
// grammar, parity/grammars/native/typescript.lino, which
// js/scripts/import-native-grammars.mjs imports from the pinned
// tree-sitter-typescript grammar with its external scanner ported to native
// scanner links, builds the concrete syntax trees of the
// tree-sitter-typescript oracle the native grammar replaced as the default
// TypeScript parse. parity/fixtures/native-grammars/typescript.json holds the
// upstream corpus with the oracle rows;
// rust/tests/unit/issue_195_grammar_native_typescript.rs checks the Rust
// executor against the same fixture.
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'typescript');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-TYPESCRIPT',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-typescript',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native TypeScript grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'program');
  for (const rule of [
    'program', 'interface_declaration', 'type_alias_declaration', 'enumeration_declaration', 'abstract_class_declaration',
    'internal_module', 'module', 'ambient_declaration', 'import_alias', 'type_annotation', 'generic_type', 'type_arguments',
    'type_parameters', 'union_type', 'intersection_type', 'conditional_type', 'mapped_type_clause', 'template_literal_type',
    'tuple_type', 'as_expression', 'satisfies_expression', 'non_null_expression', 'instantiation_expression', 'type_assertion',
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
  observe(['nativeTypeScriptGrammarIsCanonicalLinks'], context.name);
});

test('the native TypeScript grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  const corpus = corpusCases(grammarSourceOf('native-typescript')).map(({ source }) => source);
  assert.ok(corpus.length >= 100);
  assert.deepEqual(fixture.matches.slice(0, corpus.length).map(({ source }) => source), corpus);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'TypeScript'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'TypeScript'), rows, `the oracle rows of ${label} are current`);
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
  // TypeScript reads `<string>y` as a type assertion, which TSX reads as JSX.
  assert.deepEqual(rowsOf('let x = <string>y;')[6].slice(1, 3), ['value', 'type_assertion']);
  observe(['nativeTypeScriptTreesMatchOracle'], context.name);
});

test('the native TypeScript grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.divergences, []);
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 15);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'TypeScript'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeTypeScriptRejectsInvalidInput'], context.name);
});

test('native TypeScript trees keep every byte of the source, comments and white space included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('// a\nlet v: Array<number> = <number[]>w!;\ntype L = `x${T}`;\n').tree)
    .map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    ['comment', '// a'], [null, '\n'], [null, 'let'], [null, ' '], ['identifier', 'v'], [null, ':'], [null, ' '],
    ['type_identifier', 'Array'], [null, '<'], [null, 'number'], [null, '>'], [null, ' '], [null, '='], [null, ' '],
    [null, '<'], [null, 'number'], [null, '['], [null, ']'], [null, '>'], ['identifier', 'w'], [null, '!'], [null, ';'],
    [null, '\n'], [null, 'type'], [null, ' '], ['type_identifier', 'L'], [null, ' '], [null, '='], [null, ' '], [null, '`'],
    ['string_fragment', 'x'], [null, '${'], ['type_identifier', 'T'], [null, '}'], [null, '`'], [null, ';'], [null, '\n'],
  ]);
  observe(['nativeTypeScriptTreesLossless'], context.name);
});

test('native TypeScript trees follow the oracle on an identifier `as` after a keyword and after a line break', (context) => {
  // A tree-sitter lexer lexes the keyword `as` only in the parse states that
  // take it: not after `return` or an `if` condition, and not after the
  // automatic semicolon its scanner scans before a line break.
  for (const source of ['return as ;', 'if ( x ) as ( 1 ) ;', 'a\nas ( t ) ;']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'TypeScript'), JSON.stringify(source));
  }
  observe(['nativeTypeScriptTreesMatchOracle'], context.name);
});

test('the native TypeScript fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
