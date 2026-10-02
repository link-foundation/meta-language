// Requirement I195-GRAMMAR-NATIVE-INI: the native merged INI grammar,
// parity/grammars/native/ini.lino, builds the concrete syntax trees of the
// tree-sitter-ini oracle that still backs the default INI parse.
// parity/fixtures/native-grammars/ini.json holds the corpus with the oracle
// rows; rust/tests/unit/issue_195_grammar_native_ini.rs checks the Rust
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'ini');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-INI',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-ini',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native INI grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'document');
  for (const rule of ['document', 'section', 'section_name', 'setting', 'comment']) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|rule) /u);
  assert.doesNotMatch(links, /tree-sitter|grammar\.js|module\.exports|\(regex /u);
  observe(['nativeIniGrammarIsCanonicalLinks'], context.name);
});

test('the native INI grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  assert.ok(fixture.matches.length >= 50);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.deepEqual(oracleRows(source, 'INI'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // A comment the oracle parses as an extra is an X row wherever it stands.
  const commented = fixture.matches.find(({ source }) => source === '[a]\nk=v\n; c\n[b]\n');
  assert.deepEqual(commented.rows.filter((row) => row[2] === 'comment'), [[1, null, 'comment', 1, 8, 12, 'X']]);
  observe(['nativeIniTreesMatchOracle'], context.name);
});

test('the merged INI grammar accepts a last comment line without a line break', (context) => {
  assert.ok(fixture.divergences.length > 0);
  for (const { source, reason, rows } of fixture.divergences) {
    assert.match(reason, /configparser/u);
    assert.ok(oracleRecovers(source, 'INI'), `the oracle recovers from ${JSON.stringify(source)}`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, source);
  }
  assert.deepEqual(fixture.divergences[0].rows, [
    [0, null, 'document', 1, 0, 3, ''], [1, null, 'comment', 1, 0, 3, 'X'], [2, null, 'text', 1, 1, 3, ''],
  ]);
  observe(['nativeIniAcceptsMergedSourceExtensions'], context.name);
});

test('the native INI grammar rejects invalid INI the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'INI'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeIniRejectsInvalidInput'], context.name);
});

test('native INI trees keep every byte of the source, line breaks and comment markers included', (context) => {
  for (const source of [...fixture.matches, ...fixture.divergences].map((item) => item.source)) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree(' \n; c\nk = v\r\n').tree).map(({ kind, text }) => [kind, text]);
  assert.deepEqual(kinds, [
    ['blank_space', ' '], ['newline', '\n'], ['comment_marker', ';'], ['text', ' c'], ['newline', '\n'],
    ['setting_name', 'k'], [null, ' '], [null, '='], ['setting_value', ' v\r'], ['newline', '\n'],
  ]);
  observe(['nativeIniTreesLossless'], context.name);
});

test('the native INI fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
