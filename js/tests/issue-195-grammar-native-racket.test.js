// Requirement I195-GRAMMAR-NATIVE-RACKET: the native merged Racket grammar,
// parity/grammars/native/racket.lino, builds the concrete syntax trees of the
// tree-sitter-racket oracle the native grammar replaced as the default Racket parse.
// parity/fixtures/native-grammars/racket.json holds the corpus with the oracle
// rows; rust/tests/unit/issue_195_grammar_native_racket.rs checks the Rust
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'racket');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-RACKET',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-racket',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native Racket grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'program');
  for (const rule of [
    'program', 'dot', 'comment', 'block_comment', 'datum_comment', 'boolean', 'string', 'byte_string', 'here_string',
    'here_terminator', 'here_line', 'here_end', 'regular_expression', 'escape_sequence', 'number', 'character', 'symbol', 'keyword',
    'box', 'list', 'vector', 'structure', 'hash', 'graph', 'quote', 'quasiquote', 'syntax', 'quasisyntax', 'unquote',
    'unquote_splicing', 'unsyntax', 'unsyntax_splicing', 'extension', 'language_name',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|rule) /u);
  // A renamed rule keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeRacketGrammarIsCanonicalLinks'], context.name);
});

test('the native Racket grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  assert.ok(fixture.matches.length >= 100);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Racket'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Racket'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // A here string ends at the line equal to its terminator, so a second here
  // string may reuse the first one's terminator as a line; a leading byte
  // order mark has no row.
  const rowsOf = (source) => fixture.matches.find((match) => match.source === source).rows;
  assert.deepEqual(rowsOf('#<<A\nB\nA\n#<<B\nA\nB').slice(1), [
    [1, null, 'here_string', 1, 0, 8, ''], [2, null, '#<<', 0, 0, 3, ''],
    [1, null, 'here_string', 1, 9, 17, ''], [2, null, '#<<', 0, 9, 12, ''],
  ]);
  assert.deepEqual(rowsOf('﻿a'), [[0, null, 'program', 1, 3, 4, ''], [1, null, 'symbol', 1, 3, 4, '']]);
  assert.deepEqual(rowsOf('(a . b)').slice(1), [
    [1, null, 'list', 1, 0, 7, ''], [2, null, '(', 0, 0, 1, ''], [2, null, 'symbol', 1, 1, 2, ''],
    [2, null, 'dot', 1, 3, 4, ''], [2, null, 'symbol', 1, 5, 6, ''], [2, null, ')', 0, 6, 7, ''],
  ]);
  observe(['nativeRacketTreesMatchOracle'], context.name);
});

test('the merged Racket grammar reads the line feeds the Racket Reference quotes', (context) => {
  assert.ok(fixture.divergences.length >= 20);
  for (const { source, reason, rows } of fixture.divergences) {
    assert.match(reason, /tree-sitter-racket 0\.25\.0/u);
    assert.match(reason, /Racket Reference section 1\.3\./u);
    assert.ok(oracleRecovers(source, 'Racket'), `the oracle recovers from ${JSON.stringify(source)}`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, source);
  }
  const rowsOf = (source) => fixture.divergences.find((divergence) => divergence.source === source).rows;
  assert.deepEqual(rowsOf('#\\\n'), [[0, null, 'program', 1, 0, 3, ''], [1, null, 'character', 1, 0, 3, '']]);
  assert.deepEqual(rowsOf('a\\\nb'), [[0, null, 'program', 1, 0, 4, ''], [1, null, 'symbol', 1, 0, 4, '']]);
  observe(['nativeRacketAcceptsMergedSourceExtensions'], context.name);
});

test('the native Racket grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 20);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Racket'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  // The oracle's here string scanner compares only the first character of a
  // line with the terminator (strcmp over 32-bit code points); the native
  // grammar ends a here string only at a line equal to its terminator.
  for (const source of ['#<<EOF\nab\nEOFx', '#<<EOF\nabc\nEXX']) {
    assert.equal(oracleRecovers(source, 'Racket'), false, JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
  }
  observe(['nativeRacketRejectsInvalidInput'], context.name);
});

test('native Racket trees keep every byte of the source, comments and white space included', (context) => {
  for (const source of [...fixture.matches, ...fixture.divergences].map((item) => item.source)) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('#lang racket ; c\n(define x #hash((a . "b\\n")) #| b |# #<<E\nx\nE\n1.5t3)').tree)
    .map(({ kind, text }) => [kind, text]);
  assert.deepEqual(kinds, [
    [null, '#lang '], ['language_name', 'racket'], ['whitespace', ' '], ['comment', '; c'], ['whitespace', '\n'],
    [null, '('], ['symbol', 'define'], ['whitespace', ' '], ['symbol', 'x'], ['whitespace', ' '],
    ['hash_prefix', '#hash'], [null, '('], [null, '('], ['symbol', 'a'], ['whitespace', ' '], ['dot', '.'],
    ['whitespace', ' '], [null, '"'], ['string_text', 'b'], ['escape_sequence', '\\n'], [null, '"'], [null, ')'],
    [null, ')'], ['whitespace', ' '], [null, '#|'], ['comment_text', ' b '], [null, '|#'], ['whitespace', ' '],
    [null, '#<<'], ['here_terminator', 'E'], ['here_newline', '\n'], ['here_line', 'x'], ['here_newline', '\n'],
    ['here_end', 'E'], ['whitespace', '\n'], ['number', '1.5t3'], [null, ')'],
  ]);
  observe(['nativeRacketTreesLossless'], context.name);
});

test('the native Racket fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
