// Requirement I195-GRAMMAR-NATIVE-JSON: the native merged JSON grammar,
// parity/grammars/native/json.lino, builds the concrete syntax trees of the
// tree-sitter-json oracle the native grammar replaced as the default JSON parse.
// parity/fixtures/native-grammars/json.json holds the corpus with the oracle
// rows; rust/tests/unit/issue_195_grammar_native_json.rs checks the Rust
// executor against the same fixture.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import {
  compileGrammar,
  parseGrammarLinks,
  percentDecodeLinksText,
  renderGrammarLinks,
} from '../src/index.js';
import {
  NATIVE_GRAMMARS,
  buildNativeGrammarFixture,
  fixturePath,
  renderFixture,
} from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'json');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-JSON',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-json',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native JSON grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'document');
  for (const rule of ['document', 'object', 'pair', 'array', 'string', 'number', 'true', 'false', 'null', 'comment']) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|rule) /u);
  // A renamed rule keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports/u);
  observe(['nativeJsonGrammarIsCanonicalLinks'], context.name);
});

test('the native JSON grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.deepEqual(oracleRows(source, 'JSON'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  observe(['nativeJsonTreesMatchOracle'], context.name);
});

test('the merged JSON grammar accepts what RFC 8259 accepts and the oracle recovers from', (context) => {
  assert.ok(fixture.divergences.length > 0);
  for (const { source, reason, rows } of fixture.divergences) {
    assert.match(reason, /RFC 8259/u);
    assert.ok(oracleRecovers(source, 'JSON'), `the oracle recovers from ${JSON.stringify(source)}`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, source);
  }
  assert.deepEqual(fixture.divergences[0].rows, [[0, null, 'document', 1, 0, 4, ''], [1, null, 'number', 1, 0, 4, '']]);
  observe(['nativeJsonAcceptsMergedSourceExtensions'], context.name);
});

test('the native JSON grammar rejects invalid JSON the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'JSON'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeJsonRejectsInvalidInput'], context.name);
});

test('native JSON trees keep every byte of the source, a byte order mark included', (context) => {
  for (const source of [...fixture.matches, ...fixture.divergences].map((item) => item.source)) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const [mark] = leaves(parser.parseTree('\ufeff[]').tree);
  assert.deepEqual([mark.kind, mark.text, mark.start, mark.end], ['byte_order_mark', '\ufeff', 0, 3]);
  // TextDecoder drops a leading U+FEFF unless told not to; the Links form keeps it.
  assert.equal(percentDecodeLinksText('%EF%BB%BF'), '\ufeff');
  assert.match(links, /\(literal %EF%BB%BF\)/u);
  observe(['nativeJsonTreesLossless'], context.name);
});

test('the native JSON fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
