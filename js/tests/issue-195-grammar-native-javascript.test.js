// Requirement I195-GRAMMAR-NATIVE-JAVASCRIPT: the native merged JavaScript
// grammar, parity/grammars/native/javascript.lino, which
// js/scripts/import-native-grammars.mjs imports from the pinned
// tree-sitter-javascript grammar with its external scanner ported to native
// scanner links, builds the concrete syntax trees of the
// tree-sitter-javascript oracle the native grammar replaced as the default
// JavaScript parse. parity/fixtures/native-grammars/javascript.json holds the
// upstream corpus with the oracle rows;
// rust/tests/unit/issue_195_grammar_native_javascript.rs checks the Rust
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
import { corpusCases, grammarSourceOf } from '../scripts/import-native-grammars.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'javascript');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));
// The one upstream corpus case the oracle recovers from: two object literals
// in a row are no expression.
const ORACLE_ERROR = 'Extra complex literals in expressions';

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-JAVASCRIPT',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-javascript',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native JavaScript grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'program');
  for (const rule of [
    'program', 'import_statement', 'export_statement', 'function_declaration', 'generator_function_declaration',
    'class_declaration', 'class_body', 'method_definition', 'field_definition', 'lexical_declaration',
    'variable_declaration', 'if_statement', 'for_statement', 'for_in_statement', 'while_statement', 'try_statement',
    'switch_statement', 'return_statement', 'arrow_function', 'call_expression', 'member_expression',
    'assignment_expression', 'binary_expression', 'ternary_expression', 'await_expression', 'template_string',
    'template_substitution', 'regular_expression', 'string', 'number', 'comment', 'jsx_element',
    'jsx_self_closing_element', 'jsx_expression', 'object_pattern', 'array_pattern', 'spread_element',
    'private_property_identifier', 'optional_chain', 'identifier',
  ]) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // The external scanner of tree-sitter-javascript is ported to native scanner links.
  assert.deepEqual([...links.matchAll(/^\(scanner ([a-z_]+) /gmu)].map(([, name]) => name), [
    'automatic_semicolon', 'template_characters', 'ternary_question_mark', 'html_comment', 'jsx_text',
  ]);
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) {
    assert.match(line, /^\((?:grammar|extra|conflict|precedences|scanner|rule|kind) /u);
  }
  // A renamed rule or an aliased kind keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeJavaScriptGrammarIsCanonicalLinks'], context.name);
});

test('the native JavaScript grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  const corpus = corpusCases(grammarSourceOf('native-javascript'));
  const accepted = corpus.filter(({ title }) => title !== ORACLE_ERROR).map(({ source }) => source);
  assert.ok(accepted.length >= 110);
  assert.deepEqual(fixture.matches.slice(0, accepted.length).map(({ source }) => source), accepted);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'JavaScript'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'JavaScript'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  const rowsOf = (source) => fixture.matches.find((match) => match.source === source).rows;
  // The scanner inserts a semicolon at a line break, which the default tree
  // leaves out as the oracle does.
  assert.deepEqual(rowsOf('a\nb\n'), [
    [0, null, 'program', 1, 0, 4, ''], [1, null, 'expression_statement', 1, 0, 1, ''], [2, null, 'identifier', 1, 0, 1, ''],
    [1, null, 'expression_statement', 1, 2, 3, ''], [2, null, 'identifier', 1, 2, 3, ''],
  ]);
  // The scanner reads the characters of a template up to a substitution.
  assert.deepEqual(rowsOf('x = `a${b}c`;').slice(6), [
    [4, null, '`', 0, 4, 5, ''], [4, null, 'string_fragment', 1, 5, 6, ''], [4, null, 'template_substitution', 1, 6, 10, ''],
    [5, null, '${', 0, 6, 8, ''], [5, null, 'identifier', 1, 8, 9, ''], [5, null, '}', 0, 9, 10, ''],
    [4, null, 'string_fragment', 1, 10, 11, ''], [4, null, '`', 0, 11, 12, ''], [2, null, ';', 0, 12, 13, ''],
  ]);
  // An HTML-like comment (ECMA-262 annex B.1.1) is an extra the scanner reads.
  assert.deepEqual(rowsOf('<!-- c\nx;').slice(0, 2), [[0, null, 'program', 1, 0, 9, ''], [1, null, 'html_comment', 1, 0, 6, 'X']]);
  observe(['nativeJavaScriptTreesMatchOracle'], context.name);
});

test('the native JavaScript grammar rejects invalid input the oracle recovers from', (context) => {
  assert.deepEqual(fixture.divergences, []);
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  // The upstream corpus case the oracle recovers from is a rejection too.
  const errors = corpusCases(grammarSourceOf('native-javascript')).filter(({ title }) => title === ORACLE_ERROR);
  assert.equal(errors.length, 1);
  for (const { source } of errors) assert.ok(entry.rejections.includes(source), JSON.stringify(source));
  assert.ok(fixture.rejections.length >= 20);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'JavaScript'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  // The zero-width automatic semicolon before a line break is scanned in the
  // parse state the keyword `class` after it is lexed in, so a bare `class` is
  // not read as an identifier.
  for (const source of ['x\nclass', '\nfunction foo() {}\nclass']) {
    assert.ok(oracleRecovers(source, 'JavaScript'), JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
  }
  // The string fragment `//`, of lexical precedence 1, is no comment of
  // precedence 0 after the opening quote, though the comment is longer: the
  // error is where it is, not at the end of a string the comment ran to.
  for (const [source, offset] of [['x = a || 0 .1 ;', 12], ['y = "//" ; x = a || 0 .1 ;', 23], ["y = '/*' ; x = a || 0 .1 ;", 23]]) {
    assert.equal(parser.parseTree(source).rejection?.offset, offset, JSON.stringify(source));
    const recovered = parser.parseTree(source, { errorRecovery: true, recovery: 'accept' }).tree;
    const repairs = leaves(recovered).filter(({ type }) => type !== 'token').map(({ type, start, end }) => `${type}@${start}-${end}`);
    assert.deepEqual(repairs, [`missing@${offset}-${offset}`, `error@${offset}-${offset + 1}`], JSON.stringify(source));
  }
  // An error after the string leaves the string as it is.
  const after = parser.parseTree('x = a || 0 .1 ; y = "//" ; z ;', { errorRecovery: true, recovery: 'accept' }).tree;
  assert.deepEqual(leaves(after).filter(({ type }) => type !== 'token').map(({ type, start }) => `${type}@${start}`), ['missing@12', 'error@12']);
  // A comment right after a closing quote stays a comment.
  for (const source of ['x = "a"// c\n', 'x = "a"/* c */;']) {
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, JSON.stringify(source));
    assert.deepEqual(leaves(outcome.tree).filter(({ kind }) => kind === 'comment').map(({ start }) => start), [7], JSON.stringify(source));
  }
  observe(['nativeJavaScriptRejectsInvalidInput'], context.name);
});

test('native JavaScript trees keep every byte of the source, comments and white space included', (context) => {
  for (const { source } of fixture.matches) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree('// a\nconst s = `t${x}` /* b */;\nlet r = /c+/g, j = <p>hi {y}</p>\n').tree)
    .map(({ kind, text }) => [kind ?? null, text]);
  assert.deepEqual(kinds, [
    ['comment', '// a'], [null, '\n'], [null, 'const'], [null, ' '], ['identifier', 's'], [null, ' '], [null, '='],
    [null, ' '], [null, '`'], ['string_fragment', 't'], [null, '${'], ['identifier', 'x'], [null, '}'], [null, '`'],
    [null, ' '], ['comment', '/* b */'], [null, ';'], [null, '\n'], [null, 'let'], [null, ' '], ['identifier', 'r'],
    [null, ' '], [null, '='], [null, ' '], [null, '/'], ['regular_expression_pattern', 'c+'], [null, '/'],
    ['regular_expression_flags', 'g'], [null, ','], [null, ' '], ['identifier', 'j'], [null, ' '], [null, '='],
    [null, ' '], [null, '<'], ['identifier', 'p'], [null, '>'], ['jsx_text', 'hi '], [null, '{'], ['identifier', 'y'],
    [null, '}'], [null, '</'], ['identifier', 'p'], [null, '>'], ['unnamed_token', ''], [null, '\n'],
  ]);
  observe(['nativeJavaScriptTreesLossless'], context.name);
});

test('the native JavaScript fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
