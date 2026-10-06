// Requirement I195-GRAMMAR-NATIVE-RUST: the native merged Rust grammar,
// parity/grammars/native/rust.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-rust grammar with its external scanner
// ported to native scanner links, builds the concrete syntax trees of the
// tree-sitter-rust oracle the native grammar replaced as the default Rust
// parse. parity/fixtures/native-grammars/rust.json holds the upstream corpus
// with the oracle rows; rust/tests/unit/issue_195_grammar_native_rust.rs
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'rust');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-RUST',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-rust',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native Rust grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'source_file');
  for (const rule of [
    'source_file', 'function_item', 'structure_item', 'enumeration_item', 'implementation_item', 'trait_item',
    'module_item', 'use_declaration', 'let_declaration', 'macro_definition', 'macro_invocation', 'attribute_item',
    'match_expression', 'if_expression', 'closure_expression', 'binary_expression', 'call_expression',
    'await_expression', 'try_expression', 'string_literal', 'raw_string_literal', 'character_literal', 'line_comment',
    'block_comment', 'lifetime', 'generic_type', 'pattern', 'identifier',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // The external scanner of tree-sitter-rust is ported to native scanner links.
  assert.deepEqual([...links.matchAll(/^\(scanner ([a-z_]+) /gmu)].map(([, name]) => name), [
    'strings', 'raw_strings', 'floats', 'block_comments', 'line_documentation', 'error_sentinel',
  ]);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|conflict|scanner|rule|kind) /u);
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeRustGrammarIsCanonicalLinks'], context.name);
});

test('the native Rust grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  const corpus = corpusCases(grammarSourceOf('native-rust'));
  const accepted = corpus.filter(({ file }) => file !== 'error.txt').map(({ source }) => source);
  assert.ok(accepted.length >= 150);
  assert.deepEqual(fixture.matches.slice(0, accepted.length).map(({ source }) => source), accepted);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Rust'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Rust'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  const rowsOf = (source) => fixture.matches.find((match) => match.source === source).rows;
  // A postfix `.await` and `?` nest as the oracle's precedences settle them.
  assert.deepEqual(rowsOf('x.await?;'), [
    [0, null, 'source_file', 1, 0, 9, ''], [1, null, 'expression_statement', 1, 0, 9, ''],
    [2, null, 'try_expression', 1, 0, 8, ''], [3, null, 'await_expression', 1, 0, 7, ''],
    [4, null, 'identifier', 1, 0, 1, ''], [4, null, '.', 0, 1, 2, ''], [4, null, 'await', 0, 2, 7, ''],
    [3, null, '?', 0, 7, 8, ''], [2, null, ';', 0, 8, 9, ''],
  ]);
  // A macro invocation followed by `;` in a block is an expression statement:
  // `_expression_except_range` reduces it at level 1, where a declaration
  // statement of level 0 and an empty statement conflict.
  assert.deepEqual(rowsOf('fn f() { m!(x); }').slice(8, 11), [
    [3, null, '{', 0, 7, 8, ''], [3, null, 'expression_statement', 1, 9, 15, ''],
    [4, null, 'macro_invocation', 1, 9, 14, ''],
  ]);
  // The scanner's doc comment content is the kind an alias names, which keeps
  // its tree-sitter name `doc_comment` in the default tree.
  assert.deepEqual(rowsOf('//! i\n/// d\nfn f() {}\n').slice(0, 11), [
    [0, null, 'source_file', 1, 0, 22, ''], [1, null, 'line_comment', 1, 0, 6, 'X'], [2, null, '//', 0, 0, 2, ''],
    [2, 'inner', 'inner_doc_comment_marker', 1, 2, 3, ''], [3, null, '!', 0, 2, 3, ''],
    [2, 'doc', 'doc_comment', 1, 3, 6, ''], [1, null, 'line_comment', 1, 6, 12, 'X'], [2, null, '//', 0, 6, 8, ''],
    [2, 'outer', 'outer_doc_comment_marker', 1, 8, 9, ''], [3, null, '/', 0, 8, 9, ''],
    [2, 'doc', 'doc_comment', 1, 9, 12, ''],
  ]);
  observe(['nativeRustTreesMatchOracle'], context.name);
});

test('the native Rust grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.divergences, []);
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  // The upstream corpus cases the oracle recovers from are rejections too.
  const errors = corpusCases(grammarSourceOf('native-rust')).filter(({ file }) => file === 'error.txt');
  assert.ok(errors.length >= 3);
  for (const { source } of errors) assert.ok(entry.rejections.includes(source), JSON.stringify(source));
  assert.ok(fixture.rejections.length >= 30);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Rust'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeRustRejectsInvalidInput'], context.name);
});

test('native Rust trees keep every byte of the source, comments and white space included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('/// d\n#[a] fn f<\'a>(x: &\'a str) /* b /* c */ */ { r#"s"#; "\\n"; } // e\n').tree)
    .map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    [null, '//'], [null, '/'], ['documentation_comment', ' d\n'], [null, '#'], [null, '['], ['identifier', 'a'],
    [null, ']'], [null, ' '], [null, 'fn'], [null, ' '], ['identifier', 'f'], [null, '<'], [null, "'"],
    ['identifier', 'a'], [null, '>'], [null, '('], ['identifier', 'x'], [null, ':'], [null, ' '], [null, '&'],
    [null, "'"], ['identifier', 'a'], [null, ' '], ['primitive_type', 'str'], [null, ')'], [null, ' '], [null, '/*'],
    ['unnamed_token', ' b /* c */ '], [null, '*/'], [null, ' '], [null, '{'], [null, ' '], ['unnamed_token', 'r#"'],
    ['string_content', 's'], ['unnamed_token', '"#'], [null, ';'], [null, ' '], ["'\"", '"'],
    ['escape_sequence', '\\n'], ["'\"", '"'], [null, ';'], [null, ' '], [null, '}'], [null, ' '], [null, '//'],
    ['unnamed_token', ' e'], [null, '\n'],
  ]);
  observe(['nativeRustTreesLossless'], context.name);
});

test('the native Rust fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
