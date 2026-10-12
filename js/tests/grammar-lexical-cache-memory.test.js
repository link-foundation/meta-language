import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { releaseLexicalCaches, retainLexicalCache } from '../src/grammar-runtime/lexical-cache-memory.js';
import { acceptLexicalCacheReservation } from '../src/translation/frontend-rules.js';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = file => JSON.parse(readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8'));
const fixture = read('parity/fixtures/lexical-cache-memory.json');
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('optional lexical cache reservations preserve the unchanged memory limit', () => {
  for (const { cells, count, limit, accepted } of fixture.reservations) {
    assert.equal(acceptLexicalCacheReservation(cells, count, limit), accepted);
  }
});

test('evicting shared lexical caches releases only discarded optional cells', () => {
  const memory = { cells: 6, limit: 12 };
  const first = { entries: new WeakMap(), cells: 0, size: 0 };
  const nested = { entries: new WeakMap(), cells: 0, size: 0 };
  const expression = {};
  first.entries.set(expression, new Map([['first', ['matched']]]));
  nested.entries.set(expression, new Map([['nested', ['matched']]]));
  assert.equal(retainLexicalCache(memory, first, 3), true);
  assert.equal(retainLexicalCache(memory, nested, 3), true);
  assert.equal(memory.cells, 12);
  assert.equal(retainLexicalCache(memory, nested, 4), true);
  assert.equal(memory.cells, 10);
  assert.equal(first.cells, 0);
  assert.equal(nested.cells, 4);
  assert.equal(first.entries.has(expression), false);
  assert.equal(nested.entries.has(expression), false);
  assert.deepEqual([...memory.lexicalCaches], [nested]);
  releaseLexicalCaches(memory);
  assert.equal(memory.cells, 6, 'mandatory rule cells stay charged');
  assert.equal(retainLexicalCache(memory, first, 7), false);
  assert.equal(memory.cells, 6);
  assert.equal(memory.lexicalCaches.size, 0);
});

test('long lexical extras preserve the tree when optional caches exhaust a small budget', () => {
  const parser = compileGrammar(parseGrammarLinks(fixture.grammar));
  const source = `#${'a'.repeat(fixture.commentLength)}\nx;`;
  const expected = parser.parseTree(source);
  const actual = parser.parseTree(source, { memoryLimit: fixture.memoryLimit });
  assert.equal(expected.ok, true);
  assert.equal(actual.ok, true, JSON.stringify(actual.rejection));
  assert.deepEqual(actual, expected);
  assert.equal(text(actual.tree), source);
  assert.equal(parser.parseTree(source, { memoryLimit: 0 }).rejection.reason, 'memoryBudget');
  assert.equal(parser.parseTree(source.slice(0, -1), { memoryLimit: fixture.memoryLimit }).rejection.reason, 'syntax');
});

test('the original Zig diagnostic-comment CI input fits the unchanged default memory budget', () => {
  const selected = read('parity/fixtures/native-grammars/zig-corpus.json').cases
    .find(value => value.file.endsWith('undef_arith_is_illegal.zig'));
  assert.ok(selected);
  assert.equal(Buffer.byteLength(selected.source), 495499);
  const grammar = readFileSync(new URL('../../parity/grammars/native/zig.lino', import.meta.url), 'utf8');
  const projection = read('parity/fixtures/native-grammars/zig.json');
  const parser = compileGrammar(parseGrammarLinks(grammar));
  const expected = oracleSnapshot(selected.source, 'Zig');
  assert.equal(expected.recovers, false);
  const actual = parser.parseTree(selected.source);
  assert.equal(actual.ok, true, JSON.stringify(actual.rejection));
  assert.deepEqual(actual.ambiguities, []);
  assert.deepEqual(nativeRows(actual.tree, selected.source, projection), expected.rows);
  assert.equal(text(actual.tree), selected.source);
});
