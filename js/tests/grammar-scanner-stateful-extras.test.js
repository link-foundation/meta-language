import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import test from 'node:test';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { renderSyntaxTree } from '../src/grammar-runtime/syntax-tree.js';

const fixtures = readProjectData(new URL('../../parity/fixtures/scanner-stateful-extras.json', import.meta.url));
const leaves = (node) => node.type === 'token' ? [node] : node.children.flatMap(leaves);
for (const entry of fixtures) test(entry.name, () => {
  const parser = compileGrammar(parseGrammarLinks(entry.listing));
  for (const input of entry.reject) assert.equal(parser.parseTree(input).ok, false, input);
  for (const input of entry.accept) {
    const outcome = parser.parseTree(input);
    assert.equal(outcome.ok, true, input);
    assert.equal(renderSyntaxTree(outcome.tree), entry.trees[input]);
    assert.equal(leaves(outcome.tree).map(({ text }) => text).join(''), input);
    let offset = 0;
    for (const token of leaves(outcome.tree)) {
      assert.equal(token.start, offset);
      offset += Buffer.byteLength(token.text);
      assert.equal(token.end, offset);
    }
  }
});

test('external extra spans exclude their skipped prefix without losing bytes', () => {
  const entry = fixtures.find(({ name }) => name === 'external extras retain skipped trivia and state');
  const outcome = compileGrammar(parseGrammarLinks(entry.listing)).parseTree(entry.accept[0]);
  const drop = leaves(outcome.tree).find(({ kind }) => kind === 'drop');
  assert.equal(drop.start, 3);
  assert.equal(drop.end, 9);
  assert.equal(drop.text, 'é😀');
});
