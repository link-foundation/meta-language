import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = file => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const fixture = readProjectData(new URL(`../../${'parity/fixtures/grammar-aliased-extras.json'}`, import.meta.url));
const leaves = node => node.children ? node.children.flatMap(leaves) : [node];
const nodes = node => [node, ...((node.children ?? []).flatMap(nodes))];

for (const entry of fixture.cases) test(entry.name, () => {
  const parser = compileGrammar(parseGrammarLinks(entry.listing));
  for (const source of entry.reject) assert.equal(parser.parseTree(source).ok, false, source);
  const outcome = parser.parseTree(entry.source);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  assert.deepEqual(outcome.ambiguities, []);
  assert.equal(renderSyntaxTree(outcome.tree), entry.tree);
  assert.equal(leaves(outcome.tree).map(leaf => leaf.text).join(''), entry.source);
  let offset = 0;
  for (const leaf of leaves(outcome.tree)) {
    assert.equal(leaf.start, offset);
    offset += Buffer.byteLength(leaf.text);
    assert.equal(leaf.end, offset);
  }
  if (entry.extra) {
    const extra = nodes(outcome.tree).find(node => node.kind === entry.extra.kind);
    assert.ok(extra?.trivia, 'the declared alias remains an extra');
    for (const [key, value] of Object.entries(entry.extra)) assert.equal(extra[key], value, key);
  }
  if (entry.field) {
    const field = nodes(outcome.tree).find(node => node.field === entry.field.name);
    assert.ok(field);
    for (const key of ['start', 'end', 'text']) assert.equal(field[key], entry.field[key], key);
  }
  if (entry.rows) assert.deepEqual(nativeRows(outcome.tree, entry.source, entry.projection), entry.rows);
});

test('R anonymous scanner extras preserve the existing comment-before-else oracle', () => {
  const fixture = readProjectData(new URL(`../../${'parity/fixtures/native-grammars/r.json'}`, import.meta.url));
  const source = fixture.matches.find(entry => entry.source === '{if(x) y\n# note\nelse z}\n')?.source;
  assert.ok(source, 'the original focused fixture remains present');
  const oracle = oracleSnapshot(source, 'R');
  assert.equal(oracle.recovers, false);
  const outcome = compileGrammar(parseGrammarLinks(read(fixture.grammar))).parseTree(source);
  assert.equal(outcome.ok, true);
  assert.deepEqual(outcome.ambiguities, []);
  assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracle.rows);
  assert.equal(leaves(outcome.tree).map(leaf => leaf.text).join(''), source);
  assert.ok(leaves(outcome.tree).some(leaf => leaf.kind === 'unnamed_token' && leaf.trivia));
});

test('Odin anonymous scanner extras preserve the original invalid-comment recovery', () => {
  const fixture = readProjectData(new URL(`../../${'parity/fixtures/native-grammars/odin.json'}`, import.meta.url));
  const entry = fixture.rejections.find(entry => entry.source === 'package garden\n/*"not a comment*/\n');
  assert.ok(entry, 'the original recovery fixture remains present');
  assert.equal(oracleSnapshot(entry.source, 'Odin').recovers, true);
  const parser = compileGrammar(parseGrammarLinks(read(fixture.grammar)));
  assert.equal(parser.parseTree(entry.source).ok, false);
  const outcome = parser.parseTree(entry.source, { errorRecovery: true });
  assert.equal(outcome.rejection.reason, 'recovered');
  assert.equal(renderSyntaxTree(outcome.tree), entry.recovered);
  assert.equal(leaves(outcome.tree).map(leaf => leaf.text).join(''), entry.source);
});
