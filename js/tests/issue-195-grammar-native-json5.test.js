// Requirement I195-GRAMMAR-NATIVE-JSON5: the native merged JSON5 grammar,
// parity/grammars/native/json5.lino, builds the concrete syntax trees of the
// tree-sitter-json5-orchard oracle that still backs the default JSON5 parse.
// parity/fixtures/native-grammars/json5.json holds the corpus with the oracle
// rows; rust/tests/unit/issue_195_grammar_native_json5.rs checks the Rust
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'json5');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-JSON5',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-json5',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native JSON5 grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'file');
  for (const rule of ['file', 'object', 'member', 'identifier', 'array', 'string', 'number', 'null', 'true', 'false', 'comment']) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|rule) /u);
  assert.doesNotMatch(links, /tree-sitter|grammar\.js|module\.exports|\(regex /u);
  observe(['nativeJson5GrammarIsCanonicalLinks'], context.name);
});

test('the native JSON5 grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  assert.ok(fixture.matches.length >= 100);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'JSON5'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'JSON5'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // An unquoted name is an identifier field, and a comment after the value is an extra row.
  const member = fixture.matches.find(({ source }) => source === '{a: 1}');
  assert.deepEqual(member.rows.filter((row) => row[0] === 3).map((row) => row.slice(1, 6)), [
    ['name', 'identifier', 1, 1, 2], [null, ':', 0, 2, 3], ['value', 'number', 1, 4, 5],
  ]);
  const commented = fixture.matches.find(({ source }) => source === '[1,]//c\n');
  assert.deepEqual(commented.rows.at(-1), [1, null, 'comment', 1, 4, 7, 'X']);
  observe(['nativeJson5TreesMatchOracle'], context.name);
});

test('the merged JSON5 grammar reads the white space, names and escapes of the JSON5 specification', (context) => {
  assert.ok(fixture.divergences.length >= 20);
  for (const { source, reason, rows } of fixture.divergences) {
    assert.match(reason, /tree-sitter-json5-orchard 0\.1\.0/u);
    assert.match(reason, /JSON5 1\.0\.0 section/u);
    assert.ok(oracleRecovers(source, 'JSON5'), `the oracle recovers from ${JSON.stringify(source)}`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, source);
  }
  const escaped = fixture.divergences.find(({ source }) => source === '{\\u0061:1}');
  assert.deepEqual(escaped.rows.filter((row) => row[0] === 3)[0], [3, 'name', 'identifier', 1, 1, 7, '']);
  observe(['nativeJson5AcceptsMergedSourceExtensions'], context.name);
});

test('the native JSON5 grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 20);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'JSON5'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeJson5RejectsInvalidInput'], context.name);
});

test('native JSON5 trees keep every byte of the source, comments and white space included', (context) => {
  for (const source of [...fixture.matches, ...fixture.divergences].map((item) => item.source)) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('﻿{a: \'x\', // c\n"b":[0x1F,],}').tree)
    .filter(({ kind, trivia }) => kind !== null || !trivia)
    .map(({ kind, text }) => [kind, text]);
  assert.deepEqual(kinds, [
    [null, '{'], ['identifier', 'a'], [null, ':'], ['string', '\'x\''], [null, ','], ['comment', '// c'],
    ['string', '"b"'], [null, ':'], [null, '['], ['number', '0x1F'], [null, ','], [null, ']'], [null, ','], [null, '}'],
  ]);
  observe(['nativeJson5TreesLossless'], context.name);
});

test('the native JSON5 fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
