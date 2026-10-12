import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = file => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const fixture = readProjectData(new URL(`../../${'parity/fixtures/grammar-extra-projection.json'}`, import.meta.url));
const leaves = node => node.children ? node.children.flatMap(leaves) : [node];

for (const entry of fixture.cases) test(entry.name, () => {
  const parser = compileGrammar(parseGrammarLinks(entry.listing));
  for (const source of entry.reject) assert.equal(parser.parseTree(source).ok, false, source);
  const outcome = parser.parseTree(entry.source);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  assert.deepEqual(outcome.ambiguities, []);
  assert.equal(renderSyntaxTree(outcome.tree), entry.tree);
  const tokens = leaves(outcome.tree);
  assert.equal(tokens.map(leaf => leaf.text).join(''), entry.source);
  assert.deepEqual(tokens.map(({ kind, start, end, trivia, text }) => [kind, start, end, Boolean(trivia), text]), entry.tokens);
  for (const recovery of entry.recover ?? []) {
    const outcome = parser.parseTree(recovery.source, { errorRecovery: true });
    assert.equal(outcome.rejection.reason, 'recovered');
    assert.equal(renderSyntaxTree(outcome.tree), recovery.tree);
    assert.equal(leaves(outcome.tree).map(leaf => leaf.text).join(''), recovery.source);
  }
});

test('Make continuation extras match the unchanged independent oracle rows', () => {
  const fixture = readProjectData(new URL(`../../${'parity/fixtures/native-grammars/make.json'}`, import.meta.url));
  const source = 'target: foo \\\n\tbar | baz \\\nfoobar\n';
  const entry = fixture.matches.find(entry => entry.source === source);
  assert.ok(entry, 'the original CI source stays in the fixture');
  const oracle = oracleSnapshot(source, 'Make');
  assert.equal(oracle.recovers, false);
  assert.deepEqual(oracle.rows, entry.rows);
  const outcome = compileGrammar(parseGrammarLinks(read(fixture.grammar))).parseTree(source);
  assert.equal(outcome.ok, true);
  assert.deepEqual(outcome.ambiguities, []);
  assert.deepEqual(nativeRows(outcome.tree, source, fixture), entry.rows);
  assert.equal(leaves(outcome.tree).map(leaf => leaf.text).join(''), source);
  assert.deepEqual(leaves(outcome.tree).filter(leaf => leaf.text === '\\\n').map(({ kind, start, end, trivia }) => ({ kind, start, end, trivia })), [
    { kind: null, start: 12, end: 14, trivia: true },
    { kind: null, start: 25, end: 27, trivia: true },
  ]);
});

test('Pascal roots retain leading separators while their children keep syntax spans', () => {
  const fixture = readProjectData(new URL(`../../${'parity/fixtures/native-grammars/pascal.json'}`, import.meta.url));
  assert.equal(fixture.rootIncludesLeadingTrivia, true);
  const parser = compileGrammar(parseGrammarLinks(read(fixture.grammar)));
  for (const source of ['\nvar x: Integer;', '\r\nvar x: Integer;', ' \t\nvar x: Integer;', 'var x: Integer;']) {
    const oracle = oracleSnapshot(source, 'Pascal');
    assert.equal(oracle.recovers, false);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracle.rows);
    assert.equal(leaves(outcome.tree).map(leaf => leaf.text).join(''), source);
    assert.equal(outcome.tree.start, 0);
    assert.equal(oracle.rows[0][4], 0);
  }
});

test('Odin anonymous scanner extras retain the original default recovery rows', () => {
  const fixture = readProjectData(new URL(`../../${'parity/fixtures/native-grammars/odin.json'}`, import.meta.url));
  const inventory = readProjectData(new URL(`../../${'parity/language-grammar-inventory.json'}`, import.meta.url));
  const source = inventory.languages.find(entry => entry.name === 'Odin').recoverySource;
  const expected = readProjectData(new URL(`../../${'parity/fixtures/native-default-cst-expected.json'}`, import.meta.url)).languages.Odin;
  assert.equal(oracleSnapshot(source, 'Odin').recovers, true);
  const outcome = compileGrammar(parseGrammarLinks(read(fixture.grammar))).parseTree(source, { errorRecovery: true, recovery: 'accept' });
  assert.equal(outcome.ok, true);
  assert.deepEqual(nativeRows(outcome.tree, source, fixture), expected.recovery);
  assert.equal(leaves(outcome.tree).map(leaf => leaf.text).join(''), source);
});
