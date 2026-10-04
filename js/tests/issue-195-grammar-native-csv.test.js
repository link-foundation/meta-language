// Requirement I195-GRAMMAR-NATIVE-CSV: the native merged CSV grammar,
// parity/grammars/native/csv.lino, builds the concrete syntax trees of the
// tree-sitter-csv oracle the native grammar replaced as the default CSV parse.
// parity/fixtures/native-grammars/csv.json holds the corpus with the oracle
// rows; rust/tests/unit/issue_195_grammar_native_csv.rs checks the Rust
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'csv');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-CSV',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-csv',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native CSV grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'document');
  for (const rule of ['document', 'row', 'field', 'number', 'float', 'boolean', 'quoted', 'text']) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|rule) /u);
  // A renamed rule keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeCsvGrammarIsCanonicalLinks'], context.name);
});

test('the native CSV grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  assert.ok(fixture.matches.length >= 100);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'CSV'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'CSV'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // A typed field keeps its kind, and a boolean shows its keyword.
  const typed = fixture.matches.find(({ source }) => source === '1.5,true,0x1F\n');
  assert.deepEqual(typed.rows.filter((row) => row[0] >= 3).map((row) => row.slice(2, 6)), [
    ['float', 1, 0, 3], ['boolean', 1, 4, 8], ['true', 0, 4, 8], ['number', 1, 9, 13],
  ]);
  // The spaces after a closing quote belong to no row.
  const spaced = fixture.matches.find(({ source }) => source === '"a" ,b');
  assert.deepEqual(spaced.rows.filter((row) => row[0] === 2).map((row) => row.slice(2, 6)), [
    ['field', 1, 0, 3], [',', 0, 4, 5], ['field', 1, 5, 6],
  ]);
  observe(['nativeCsvTreesMatchOracle'], context.name);
});

test('the merged CSV grammar reads an empty last field the oracle recovers from', (context) => {
  assert.ok(fixture.divergences.length >= 8);
  for (const { source, reason, rows } of fixture.divergences) {
    assert.match(reason, /tree-sitter-csv f6bf6e3/u);
    assert.match(reason, /RFC 4180/u);
    assert.ok(oracleRecovers(source, 'CSV'), `the oracle recovers from ${JSON.stringify(source)}`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, source);
  }
  const empty = fixture.divergences.find(({ source }) => source === 'a,');
  assert.deepEqual(empty.rows, [
    [0, null, 'document', 1, 0, 2, ''], [1, null, 'row', 1, 0, 2, ''], [2, null, 'field', 1, 0, 1, ''],
    [3, null, 'text', 1, 0, 1, ''], [2, null, ',', 0, 1, 2, ''], [2, null, 'field', 1, 2, 2, ''],
    [3, null, 'text', 1, 2, 2, ''],
  ]);
  observe(['nativeCsvAcceptsMergedSourceExtensions'], context.name);
});

test('the native CSV grammar rejects invalid quoting the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'CSV'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeCsvRejectsInvalidInput'], context.name);
});

test('native CSV trees keep every byte of the source, line breaks and blank lines included', (context) => {
  for (const source of [...fixture.matches, ...fixture.divergences].map((item) => item.source)) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree(' "a""b" , 1\r\n\r\ntrue,\n').tree).map(({ kind, text }) => [kind, text]);
  assert.deepEqual(kinds, [
    ['text', ' "a""b"'], ['blank_space', ' '], [null, ','], ['number', ' 1'], ['newline', '\r\n\r\n'], [null, 'true'],
    [null, ','], ['text', ''], ['newline', '\n'],
  ]);
  observe(['nativeCsvTreesLossless'], context.name);
});

test('the native CSV fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
