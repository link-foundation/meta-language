import assert from 'node:assert/strict';
import test from 'node:test';
import { nativeRows } from '../scripts/native-grammar-rows.mjs';

// These controls exercise the existing JavaScript projection without invoking
// a grammar, an oracle process or recovery search. The Rust row helper mirrors
// the same explicit trees and expected rows.
const projection = {
  hidden: ['hidden_missing'], anonymous: ['line_break'], extras: [],
  oracleKinds: { native_document: 'document', native_group: 'group', native_name: 'identifier', native_identifier: 'identifier' },
};
const token = (text, start, end, kind = null, extra = {}) => ({ type: 'token', kind, start, end, text, ...extra });
const missing = (kind, position, literal = false) => ({ type: 'missing', kind, start: position, end: position, literal });
const node = (kind, start, end, children) => ({ type: 'node', kind, start, end, children });
const rebuild = tree => tree.type === 'node' ? tree.children.map(rebuild).join('') : tree.type === 'missing' ? '' : tree.text;

test('native recovery rows preserve error bytes, named and literal missing leaves, and hidden missing projections', () => {
  const source = ' α?β\n';
  const tree = node('native_document', 0, 7, [
    token(' ', 0, 1, null, { trivia: true }),
    node('native_group', 1, 6, [
      token('α', 1, 3, 'native_name', { field: 'word' }),
      { type: 'error', start: 3, end: 4, text: '?' },
      missing('native_identifier', 4), missing(';', 4, true), missing("';", 4), missing(null, 4),
      missing('hidden_missing', 4), missing('line_break', 4),
      token('β', 4, 6, 'native_name'),
    ]),
    token('\n', 6, 7, null, { trivia: true }),
  ]);
  assert.deepEqual(nativeRows(tree, source, projection), [
    [0, null, 'document', 1, 1, 7, ''],
    [1, null, 'group', 1, 1, 6, ''],
    [2, 'word', 'identifier', 1, 1, 3, ''],
    [2, null, 'ERROR', 1, 3, 4, 'E'],
    [2, null, 'identifier', 1, 4, 4, 'M'],
    [2, null, ';', 0, 4, 4, 'M'],
    [2, null, ';', 0, 4, 4, 'M'],
    [2, null, null, 1, 4, 4, 'M'],
    [2, null, 'identifier', 1, 4, 6, ''],
  ]);
  assert.equal(rebuild(tree), source);
});

test('native missing-only recovery keeps empty source and zero-width rows', () => {
  const tree = node('native_document', 0, 0, [missing('native_identifier', 0), missing(']', 0, true), missing('hidden_missing', 0)]);
  assert.deepEqual(nativeRows(tree, '', projection), [
    [0, null, 'document', 1, 0, 0, ''],
    [1, null, 'identifier', 1, 0, 0, 'M'],
    [1, null, ']', 0, 0, 0, 'M'],
  ]);
  assert.equal(rebuild(tree), '');
});

test('native error-only recovery retains NUL, Unicode and CRLF source bytes', () => {
  const source = '\0é\r\n';
  const tree = node('native_document', 0, 5, [{ type: 'error', start: 0, end: 5, text: source }]);
  assert.deepEqual(nativeRows(tree, source, projection), [
    [0, null, 'document', 1, 0, 5, ''],
    [1, null, 'ERROR', 1, 0, 5, 'E'],
  ]);
  assert.equal(rebuild(tree), source);
});
