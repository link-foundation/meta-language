// Requirement I195-GRAMMAR-NATIVE-SCHEME: the native merged Scheme grammar,
// parity/grammars/native/scheme.lino, builds the concrete syntax trees of the
// tree-sitter-scheme oracle that still backs the default Scheme parse.
// parity/fixtures/native-grammars/scheme.json holds the corpus with the oracle
// rows; rust/tests/unit/issue_195_grammar_native_scheme.rs checks the Rust
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
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'scheme');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-SCHEME',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-scheme',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native Scheme grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'program');
  for (const rule of [
    'program', 'comment', 'directive', 'block_comment', 'boolean', 'number', 'character', 'string', 'escape_sequence',
    'symbol', 'keyword', 'list', 'quote', 'quasiquote', 'unquote', 'unquote_splicing', 'syntax', 'quasisyntax',
    'unsyntax', 'unsyntax_splicing', 'vector', 'byte_vector', 'datum_label', 'datum_reference',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|rule) /u);
  assert.doesNotMatch(links, /tree-sitter|grammar\.js|module\.exports|\(regex /u);
  observe(['nativeSchemeGrammarIsCanonicalLinks'], context.name);
});

test('the native Scheme grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  assert.ok(fixture.matches.length >= 100);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Scheme'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Scheme'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // The longer of a number and a symbol wins, the number on a tie; a
  // directive keeps its name without a row.
  const rowsOf = (source) => fixture.matches.find((match) => match.source === source).rows;
  assert.deepEqual(rowsOf('1#a').slice(1), [[1, null, 'number', 1, 0, 2, ''], [1, null, 'symbol', 1, 2, 3, '']]);
  assert.deepEqual(rowsOf('1abc').slice(1), [[1, null, 'symbol', 1, 0, 4, '']]);
  assert.deepEqual(rowsOf('#!r6rs').slice(1), [[1, null, 'directive', 1, 0, 6, ''], [2, null, '#!', 0, 0, 2, '']]);
  observe(['nativeSchemeTreesMatchOracle'], context.name);
});

test('the merged Scheme grammar reads the bytevectors and datum labels of R7RS small', (context) => {
  assert.ok(fixture.divergences.length >= 20);
  for (const { source, reason, rows } of fixture.divergences) {
    assert.match(reason, /tree-sitter-scheme 0\.24\.7/u);
    assert.match(reason, /R7RS small section/u);
    assert.ok(oracleRecovers(source, 'Scheme'), `the oracle recovers from ${JSON.stringify(source)}`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, source);
  }
  const labelled = fixture.divergences.find(({ source }) => source === '#12=(a . #12#)');
  assert.deepEqual(labelled.rows.slice(1, 4), [
    [1, null, 'datum_label', 1, 0, 14, ''], [2, null, 'label', 1, 0, 4, ''], [2, null, 'list', 1, 4, 14, ''],
  ]);
  assert.deepEqual(labelled.rows.at(-2), [3, null, 'datum_reference', 1, 9, 13, '']);
  const bytes = fixture.divergences.find(({ source }) => source === '#u8()');
  assert.deepEqual(bytes.rows.slice(1), [
    [1, null, 'byte_vector', 1, 0, 5, ''], [2, null, '#u8(', 0, 0, 4, ''], [2, null, ')', 0, 4, 5, ''],
  ]);
  observe(['nativeSchemeAcceptsMergedSourceExtensions'], context.name);
});

test('the native Scheme grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 20);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Scheme'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeSchemeRejectsInvalidInput'], context.name);
});

test('native Scheme trees keep every byte of the source, comments and white space included', (context) => {
  for (const source of [...fixture.matches, ...fixture.divergences].map((item) => item.source)) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('#!r6rs ; c\n(define x #u8(1 "a\\n") #| b |# 1abc)').tree)
    .map(({ kind, text }) => [kind, text]);
  assert.deepEqual(kinds, [
    [null, '#!'], ['directive_name', 'r6rs'], ['whitespace', ' '], ['comment_text', '; c'], ['whitespace', '\n'],
    [null, '('], ['symbol', 'define'], ['whitespace', ' '], ['symbol', 'x'], ['whitespace', ' '], [null, '#u8('],
    ['number', '1'], ['whitespace', ' '], [null, '"'], ['string_text', 'a'], ['escape_sequence', '\\n'], [null, '"'],
    [null, ')'], ['whitespace', ' '], [null, '#|'], ['comment_text', ' b '], [null, '|#'], ['whitespace', ' '],
    ['symbol', '1abc'], [null, ')'],
  ]);
  observe(['nativeSchemeTreesLossless'], context.name);
});

test('the native Scheme fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
