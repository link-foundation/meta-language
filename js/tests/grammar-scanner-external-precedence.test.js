import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { renderSyntaxTree } from '../src/grammar-runtime/syntax-tree.js';
import { nativeRows } from '../scripts/native-grammar-rows.mjs';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/scanner-external-precedence.json', import.meta.url), 'utf8'));
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

for (const entry of fixture) test(entry.name, () => {
  const parser = compileGrammar(parseGrammarLinks(entry.listing));
  for (const input of entry.accept) {
    const result = parser.parseTree(input);
    assert.equal(result.ok, true, input);
    assert.deepEqual(result.ambiguities, [], input);
    assert.equal(renderSyntaxTree(result.tree), entry.trees[input], input);
    assert.equal(text(result.tree), input);
    assert.equal(result.tree.start, 0);
    assert.equal(result.tree.end, Buffer.byteLength(input));
  }
  for (const input of entry.reject) assert.equal(parser.parseTree(input).ok, false, input);
});

test('the oracle projection retains actual composite extra flags without a configured kind list', () => {
  const input = '#define VALUE\nx';
  const tree = compileGrammar(parseGrammarLinks(fixture[2].listing)).parseTree(input).tree;
  const rows = nativeRows(tree, input);
  assert.deepEqual(rows.find(row => row[2] === 'directive'), [1, null, 'directive', 1, 0, 14, 'X']);
  assert.deepEqual(rows.find(row => row[2] === 'name'), [2, null, 'name', 1, 8, 13, '']);
});
