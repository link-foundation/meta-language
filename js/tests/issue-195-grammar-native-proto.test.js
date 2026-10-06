// Requirement I195-GRAMMAR-NATIVE-PROTO: the native merged Protocol Buffers grammar,
// parity/grammars/native/proto.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-proto 0.6.0 grammar, builds the
// concrete syntax trees of the tree-sitter-proto oracle the native grammar
// replaced as the default Protocol Buffers parse. parity/fixtures/native-grammars/proto.json holds the
// upstream corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_proto.rs
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'proto');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-PROTO',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-proto',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

// The kind of each leaf, the kind of the node it is a child of, its text and whether it is trivia there.
function placed(tree, parent = null) {
  if (tree.type !== 'node') return [[tree.kind ?? null, parent, tree.text, Boolean(tree.trivia)]];
  return tree.children.flatMap((child) => placed(child, tree.kind));
}

test('the native Protocol Buffers grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'source_file');
  for (const rule of [
    'source_file', 'syntax', 'edition', 'import', 'package', 'option', 'option_name', 'enumeration', 'enumeration_body', 'enumeration_field',
    'message', 'message_body', 'extend', 'group', 'field', 'field_options', 'one_of', 'one_of_field', 'map_field', 'key_type', 'type',
    'reserved', 'extensions', 'ranges', 'range', 'reserved_field_names', 'service', 'remote_procedure_call', 'constant', 'block_literal',
    'extension_name', 'identifier', 'full_identifier', 'boolean', 'integer_literal', 'float_literal', 'string', 'escape_sequence', 'comment',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // tree-sitter-proto has no external scanner.
  assert.doesNotMatch(links, /^\(scanner /mu);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(proto /u);
  observe(['nativeProtoGrammarIsCanonicalLinks'], context.name);
});

test('the native Protocol Buffers grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The oracle parses every corpus source at this revision without an error.
  const cases = corpusCases(grammarSourceOf('native-proto')).map(({ source }) => source);
  assert.equal(cases.length, 40);
  assert.deepEqual(fixture.matches.slice(0, cases.length).map(({ source }) => source), cases);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Protocol Buffers'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Protocol Buffers'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  assert.deepEqual(fixture.divergences, []);
  observe(['nativeProtoTreesMatchOracle'], context.name);
});

test('the native Protocol Buffers grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Protocol Buffers'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeProtoRejectsInvalidInput'], context.name);
});

test('native Protocol Buffers trees keep every byte of the source, line breaks included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  // A comment is trivia of the node around it; the escape sequence and the
  // text between quotes are leaves of the string, and adjacent strings are one
  // string, as in the oracle.
  assert.deepEqual(placed(parser.parseTree('option a = "x\\n" "y"; // c\n').tree).filter(([kind]) => kind !== null), [
    ['identifier', 'option', 'a', false], ['unnamed_token', 'string', 'x', false], ['escape_sequence', 'string', '\\n', false],
    ['unnamed_token', 'string', 'y', false], ['comment', 'source_file', '// c', true],
  ]);
  observe(['nativeProtoTreesLossless'], context.name);
});

test('the native Protocol Buffers fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
