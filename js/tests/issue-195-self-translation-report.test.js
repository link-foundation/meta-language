import assert from 'node:assert/strict';
import test from 'node:test';
import { compareRustDefinitions, rustFunctionDefinitions, sourceCoverage } from '../scripts/generate-self-translation-report.mjs';
import { selfTranslate } from '../src/self-translation.js';

test('self-translation report reads individual UTF-8 function ranges and exact CST names', () => {
  const source = '// café 😀\nfn first() -> i64 { 1 }\nfn café() -> i64 { 2 }\npub fn r#type() { }\n';
  assert.deepEqual([...rustFunctionDefinitions(source)], [
    ['first', 'fn first() -> i64 { 1 }'],
    ['café', 'fn café() -> i64 { 2 }'],
    ['r#type', 'pub fn r#type() { }'],
  ]);
});

test('self-translation report compares each function independently', () => {
  const translated = '// heading\nfn first() -> i64 { 1 }\nfn second() -> i64 { 2 }\nfn third() -> i64 { 3 }\n';
  const counterpart = 'fn first() -> i64 {\n  1\n}\nfn second() -> i64 { 9 }\nfn unrelated() {}\n';
  const report = compareRustDefinitions(translated, counterpart);
  assert.equal(report.functions, 3);
  assert.equal(report.matched, 2);
  assert.equal(report.identical, 1);
  const standalone = compareRustDefinitions(translated, null);
  assert.equal(standalone.functions, 3);
  assert.equal(standalone.matched, 0);
  assert.equal(standalone.identical, 0);
});

test('self-translation report accounts for actual translated, carried and UTF-8 comment source bytes', () => {
  const source = '\r\n// café 😀\r\nexport function add(a, b) { return a + b; }\n\nclass Unsupported {}\n';
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  const coverage = sourceCoverage(source, translation.items);
  assert.equal(coverage.sourceBytes, Buffer.byteLength(source));
  assert.equal(coverage.itemBytes + coverage.layoutBytes, coverage.sourceBytes);
  assert.equal(coverage.unrepresentedBytes, 0);
  assert.deepEqual(coverage.unrepresentedRanges, []);
  assert.ok(coverage.bytesByStatus.translated > 0);
  assert.ok(coverage.bytesByStatus.carried >= Buffer.byteLength('class Unsupported {}'));
});

test('self-translation report exposes omitted source even when there are no carried items', () => {
  const source = 'const café = 1;\n';
  const empty = sourceCoverage(source, []);
  assert.equal(empty.unrepresentedBytes, Buffer.byteLength(source));
  assert.deepEqual(empty.unrepresentedRanges, [{ start: 0, end: Buffer.byteLength(source) }]);
  const partial = sourceCoverage('a(); b();', [{ start: 0, end: 4, status: 'translated' }]);
  assert.equal(partial.unrepresentedBytes, 5);
  assert.deepEqual(partial.unrepresentedRanges, [{ start: 4, end: 9 }]);
  const layout = sourceCoverage('\r\n \t\u2003', []);
  assert.equal(layout.layoutBytes, layout.sourceBytes);
  assert.equal(layout.unrepresentedBytes, 0);
});

test('self-translation report rejects overlapping, out-of-bounds and split UTF-8 item ranges', () => {
  for (const items of [
    [{ start: -1, end: 2, status: 'translated' }],
    [{ start: 0, end: 6, status: 'translated' }],
    [{ start: 0, end: 1, status: 'translated' }],
    [{ start: 0, end: 2, status: 'translated' }, { start: 1, end: 3, status: 'carried' }],
    [{ start: 0, end: 0, status: 'translated' }],
    [{ start: 0, end: 2, status: 'unknown' }],
  ]) assert.throws(() => sourceCoverage('é();', items), /invalid self-translation source range|unknown self-translation item status/u);
});
