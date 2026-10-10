import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { selfTranslate } from '../src/self-translation.js';

test('a resource-limited executor module retains all source bytes and restores exactly', () => {
  const source = readFileSync(new URL('../src/grammar-runtime/executor.js', import.meta.url), 'utf8');
  const bytes = Buffer.from(source, 'utf8');
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  assert.ok(translated.items.length > 0);
  let cursor = 0;
  for (const item of translated.items) {
    assert.ok(item.start >= cursor && item.end > item.start && item.end <= bytes.length);
    assert.match(bytes.subarray(cursor, item.start).toString('utf8'), /^\s*$/u);
    cursor = item.end;
    if (item.term === 'ERROR') assert.equal(item.status, 'carried');
  }
  assert.match(bytes.subarray(cursor).toString('utf8'), /^\s*$/u);
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
});

test('restoration preserves an absent final newline and rejects appended code', () => {
  const source = '/** @param {number} value @returns {number} */\nfunction identity(value) { return value; }';
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
  const edited = selfTranslate(translated.code + '\nconst EXTRA: f64 = 2.0;\n', 'Rust', 'JavaScript');
  assert.notEqual(edited.code, source);
  assert.ok(edited.items.some(item => item.term !== 'comment' && item.status !== 'provenance'));
});
