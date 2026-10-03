// Requirement I195-GRAMMAR-NATIVE-DIFF: the native merged diff grammar,
// parity/grammars/native/diff.lino, builds the concrete syntax trees of the
// tree-sitter-diff oracle the native grammar replaced as the default diff parse.
// parity/fixtures/native-grammars/diff.json holds the corpus with the oracle
// rows; rust/tests/unit/issue_195_grammar_native_diff.rs checks the Rust
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
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'diff');
const fixture = JSON.parse(read(fixturePath(entry)));
const links = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(links));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-DIFF',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-diff',
    fixtureFile: fixturePath(entry),
    assertions,
    testName,
  });
}

function leaves(tree) {
  return tree.type === 'node' ? tree.children.flatMap(leaves) : [tree];
}

test('the native diff grammar is a canonical Links Notation grammar', (context) => {
  const grammar = parseGrammarLinks(links);
  assert.equal(renderGrammarLinks(grammar), links);
  assert.equal(grammar.start, 'source');
  for (const rule of ['source', 'block', 'command', 'hunks', 'hunk', 'changes', 'location', 'addition', 'deletion', 'context']) {
    assert.ok(grammar.rules.has(rule), rule);
  }
  // Every line is one link; no foreign grammar is embedded as a string.
  for (const line of links.trimEnd().split('\n')) assert.match(line, /^\((?:grammar|extra|rule) /u);
  // A renamed rule keeps its tree-sitter name only as a source-name alias.
  assert.doesNotMatch(links.replaceAll(/ \(source-names(?: \([^()]*\))+\)/gu, ''), /tree-sitter|grammar\.js\b|module\.exports|\(regex /u);
  observe(['nativeDiffGrammarIsCanonicalLinks'], context.name);
});

test('the native diff grammar builds the tree-sitter oracle rows of every corpus source', (context) => {
  assert.equal(fixture.matches.length, entry.matches.length);
  assert.ok(fixture.matches.length >= 100);
  for (const { source, rows } of fixture.matches) {
    const label = JSON.stringify(source);
    assert.equal(oracleRecovers(source, 'Diff'), false, `the oracle accepts ${label}`);
    assert.deepEqual(oracleRows(source, 'Diff'), rows, `the oracle rows of ${label} are current`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, label);
    assert.deepEqual(outcome.ambiguities, [], label);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, label);
  }
  // A git patch is a block: the command, its headers and the hunks of one file.
  const patch = fixture.matches.find(({ source }) => source.startsWith('diff --git a/x b/y\nsimilarity index 90%'));
  assert.deepEqual(patch.rows.filter((row) => row[1] !== null || row[0] <= 2).map((row) => row.slice(0, 3)), [
    [0, null, 'source'], [1, null, 'block'], [2, null, 'command'], [2, null, 'similarity'], [2, null, 'file_change'],
    [2, null, 'file_change'], [2, null, 'index'], [2, null, 'old_file'], [2, null, 'new_file'], [2, null, 'hunks'],
    [4, 'location', 'location'], [4, 'changes', 'changes'],
  ]);
  observe(['nativeDiffTreesMatchOracle'], context.name);
});

test('the merged diff grammar reads GNU and git unified diff lines the oracle recovers from', (context) => {
  assert.ok(fixture.divergences.length >= 8);
  for (const { source, reason, rows } of fixture.divergences) {
    assert.match(reason, /tree-sitter-diff 0\.1\.0/u);
    assert.match(reason, /GNU diffutils|git|source rule/u);
    assert.ok(oracleRecovers(source, 'Diff'), `the oracle recovers from ${JSON.stringify(source)}`);
    const outcome = parser.parseTree(source);
    assert.ok(outcome.ok, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, source);
  }
  // A hunk line that starts with a space is context, keyword or not.
  const context_ = fixture.divergences.find(({ source }) => source === ' new x\n');
  assert.deepEqual(context_.rows, [[0, null, 'source', 1, 0, 7, ''], [1, null, 'context', 1, 0, 6, '']]);
  observe(['nativeDiffAcceptsMergedSourceExtensions'], context.name);
});

test('the native diff grammar rejects invalid diffs the oracle recovers from', (context) => {
  assert.deepEqual(fixture.rejections.map(({ source }) => source), entry.rejections);
  assert.ok(fixture.rejections.length >= 10);
  for (const { source } of fixture.rejections) {
    assert.ok(oracleRecovers(source, 'Diff'), JSON.stringify(source));
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, false, JSON.stringify(source));
    assert.ok(outcome.rejection, JSON.stringify(source));
  }
  observe(['nativeDiffRejectsInvalidInput'], context.name);
});

test('native diff trees keep every byte of the source, line breaks and blank lines included', (context) => {
  for (const source of [...fixture.matches, ...fixture.divergences].map((item) => item.source)) {
    const tree = parser.parseTree(source).tree;
    assert.equal(leaves(tree).map(({ text }) => text).join(''), source, JSON.stringify(source));
  }
  const kinds = leaves(parser.parseTree(' \n--- a b\n@@ -1 +1 @@ f\n-x \r\n').tree).map(({ kind, text }) => [kind, text]);
  assert.deepEqual(kinds, [
    ['newline', ' \n'], [null, '---'], [null, ' '], ['word', 'a'], [null, ' '], ['word', 'b'], ['newline', '\n'],
    [null, '@@'], [null, ' '], ['line_range', '-1'], [null, ' '], ['line_range', '+1'], [null, ' '], [null, '@@'],
    ['anything', ' f'], ['newline', '\n'], [null, '-'], ['anything', 'x '], ['newline', '\r\n'],
  ]);
  observe(['nativeDiffTreesLossless'], context.name);
});

test('the native diff fixture is current', () => {
  assert.equal(renderFixture(buildNativeGrammarFixture(entry)), read(fixturePath(entry)));
});
