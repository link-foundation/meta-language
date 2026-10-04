// Requirement I195-GRAMMAR-NATIVE-C: the native merged C grammar,
// parity/grammars/native/c.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-c grammar, builds the concrete syntax
// trees of the tree-sitter-c oracle the native grammar replaced as the
// default C parse. parity/fixtures/native-grammars/c.json holds the upstream
// corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_c.rs
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
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'c');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-C',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-c',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native C grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'translation_unit');
  for (const rule of [
    'translation_unit', 'function_definition', 'declaration', 'type_definition', 'preprocessor_include',
    'preprocessor_definition', 'preprocessor_if', 'structure_specifier', 'enumeration_specifier', 'compound_statement',
    'if_statement', 'for_statement', 'while_statement', 'do_statement', 'return_statement', 'expression',
    'binary_expression', 'call_expression', 'conditional_expression', 'string_literal', 'character_literal',
    'concatenated_string', 'comment', 'identifier', 'primitive_type',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|conflict|rule) /u);
  // A renamed rule keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeCGrammarIsCanonicalLinks'], context.name);
});

test('the native C grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  assert.ok(fixture.matches.length >= 100);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'C'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'C'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // A keyword is an identifier only where the keyword itself cannot stand,
  // as the oracle's lexer prefers a keyword wherever the parser accepts it.
  const rowsOf = (source) => fixture.matches.find((match) => match.source === source).rows;
  assert.deepEqual(rowsOf('x = typedef;').slice(3), [
    [3, 'left', 'identifier', 1, 0, 1, ''], [3, 'operator', '=', 0, 2, 3, ''],
    [3, 'right', 'identifier', 1, 4, 11, ''], [2, null, ';', 0, 11, 12, ''],
  ]);
  assert.deepEqual(rowsOf('#include <stdio.h>\n').slice(1), [
    [1, null, 'preproc_include', 1, 0, 19, ''], [2, null, '#include', 0, 0, 8, ''],
    [2, 'path', 'system_lib_string', 1, 9, 18, ''],
  ]);
  // The oracle's conflicts are settled as its generated parser settles them:
  // `a;` is a statement, not a declaration (a declared conflict of silent
  // rules); `#if` outranks a directive; a right-associative case statement
  // keeps the statements after it.
  for (const source of ['{ a; }', '#if A\n#endif\n', '{ case 1: a; b; }', 'int x = sizeof(char * ());']) {
    assert.deepEqual(parser.parseTree(source).ambiguities, [], JSON.stringify(source));
  }
  observe(['nativeCTreesMatchOracle'], context.name);
});

test('the native C grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.divergences, []);
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 20);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'C'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  // A keyword is not an identifier where the keyword may begin a statement.
  for (const source of ['typedef;', 'if;', 'struct;', 'while;']) assert.ok(entry.rejections.includes(source), source);
  observe(['nativeCRejectsInvalidInput'], context.name);
});

test('native C trees keep every byte of the source, comments and white space included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('#include <stdio.h> /* c */\nint main(void) { return a ? "s\\n" : \'c\'; } // d\n').tree)
    .map(({ kind, text }) => [kind, text]);
  assert.deepEqual(kinds, [
    ["'#include", '#include'], [null, ' '], ['system_library_string', '<stdio.h>'], [null, ' '], ['comment', '/* c */'],
    ['unnamed_token', '\n'], ['primitive_type', 'int'], [null, ' '], ['identifier', 'main'], [null, '('],
    ['primitive_type', 'void'], [null, ')'], [null, ' '], [null, '{'], [null, ' '], [null, 'return'], [null, ' '],
    ['identifier', 'a'], [null, ' '], [null, '?'], [null, ' '], [null, '"'], ['string_content', 's'],
    ['escape_sequence', '\\n'], [null, '"'], [null, ' '], [null, ':'], [null, ' '], [null, "'"], ['character', 'c'],
    [null, "'"], [null, ';'], [null, ' '], [null, '}'], [null, ' '], ['comment', '// d'], [null, '\n'],
  ]);
  observe(['nativeCTreesLossless'], context.name);
});

test('the native C fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
