// Requirement I195-GRAMMAR-NATIVE-SOLIDITY: the native merged Solidity grammar,
// parity/grammars/native/solidity.lino, which js/scripts/import-native-grammars.mjs
// imports from the pinned tree-sitter-solidity 1.2.13 grammar, builds the
// concrete syntax trees of the tree-sitter-solidity oracle the native grammar
// replaced as the default Solidity parse. parity/fixtures/native-grammars/solidity.json holds the
// upstream corpus with the oracle rows; rust/tests/unit/issue_195_grammar_native_solidity.rs
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'solidity');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-SOLIDITY',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-solidity',
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

test('the native Solidity grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'source_file');
  for (const rule of [
    'source_file', 'pragma_directive', 'import_directive', 'contract_declaration', 'interface_declaration', 'library_declaration',
    'error_declaration', 'structure_declaration', 'enumeration_declaration', 'event_definition', 'using_directive', 'assembly_statement',
    'yul_block', 'yul_function_definition', 'block_statement', 'if_statement', 'for_statement', 'while_statement', 'do_while_statement',
    'revert_statement', 'try_statement', 'catch_clause', 'emit_statement', 'state_variable_declaration', 'modifier_definition',
    'constructor_definition', 'function_definition', 'call_arguments', 'expression', 'ternary_expression', 'binary_expression',
    'call_expression', 'parenthesized_expression', 'mapping', 'primitive_type', 'identifier', 'comment',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // tree-sitter-solidity has no external scanner.
  assert.doesNotMatch(links, /^\(scanner /mu);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports/u);
  observe(['nativeSolidityGrammarIsCanonicalLinks'], context.name);
});

test('the native Solidity grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  // The oracle parses every corpus source at this revision without an error.
  const all = corpusCases(grammarSourceOf('native-solidity')).map(({ source }) => source);
  assert.equal(all.length, 125);
  const cases = all.filter((source) => !oracleRecovers(source, 'Solidity'));
  assert.equal(cases.length, 125);
  assert.deepEqual(fixture.matches.slice(0, cases.length).map(({ source }) => source), cases);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Solidity'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Solidity'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  assert.deepEqual(fixture.divergences, []);
  observe(['nativeSolidityTreesMatchOracle'], context.name);
});

test('the native Solidity grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Solidity'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeSolidityRejectsInvalidInput'], context.name);
});

test('native Solidity trees keep every byte of the source', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  // A comment is trivia before the node after it, as in the oracle, and a
  // number keeps its digits as a token of the literal.
  assert.deepEqual(placed(parser.parseTree('// é\ncontract C { uint x = 1; }\n').tree).filter(([kind]) => kind !== null), [
    ['comment', 'contract_declaration', '// é', true], ['identifier', 'contract_declaration', 'C', false],
    ['identifier', 'state_variable_declaration', 'x', false], ['unnamed_token', 'number_literal', '1', false],
  ]);
  observe(['nativeSolidityTreesLossless'], context.name);
});

test('the native Solidity fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});

test('native Solidity authored arithmetic and assignment grouping regressions match the oracle', () => {
  for (const source of ["contract Plant {function build() public { target = new Seed{value: 7}(4); }}\n","contract Math {function sum() public returns(int) {return 9+4-2;}}\n","contract Math {function sum() public returns(int) {return 9-4+2;}}\n"]) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true);
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'Solidity'));
  }
});
