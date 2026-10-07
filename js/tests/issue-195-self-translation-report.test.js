import assert from 'node:assert/strict';
import test from 'node:test';
import { compareRustDefinitions, rustFunctionDefinitions } from '../scripts/generate-self-translation-report.mjs';

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
