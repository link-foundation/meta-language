import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { translateProgram } from '../src/program-translation.js';
import { parseRust } from '../src/translation/rust.js';

test('Rust binary64 decimal and exponent literals retain IEEE-754 arithmetic in JavaScript', () => {
  const source = `pub fn add(x: f64, y: f64) -> f64 { x + y }
pub fn coefficient() -> f64 { 1.25e-3f64 }
pub fn divide(x: f64, y: f64) -> f64 { x / y }
pub fn negative_zero() -> f64 { -0f64 }
fn main() {}`;
  const parsed = parseRust(source);
  assert.equal(parsed.items[0].params[0].type.kind, 'float');
  assert.equal(parsed.items[1].body.type.kind, 'float');
  const translated = translateProgram(source, 'Rust', 'JavaScript');
  assert.equal(translated.diagnostic, null);
  const context = {};
  runInNewContext(translated.code, context);
  assert.equal(context.add(0.1, 0.2), 0.1 + 0.2);
  assert.equal(context.coefficient(), 0.00125);
  assert.equal(context.divide(1, 0), Infinity);
  assert.equal(Object.is(context.negative_zero(), -0), true);
});

test('Rust unsuffixed floating literals use binary64 rather than machine integers', () => {
  const source = 'pub fn value() -> f64 { 2.5e2 } fn main() {}';
  const translated = translateProgram(source, 'Rust', 'JavaScript');
  assert.equal(translated.diagnostic, null);
  const context = {};
  runInNewContext(translated.code, context);
  assert.equal(context.value(), 250);
});

test('binary32 remains distinct until per-operation binary32 rounding is implemented', () => {
  const result = translateProgram('pub fn value() -> f32 { 0.1f32 } fn main() {}', 'Rust', 'JavaScript');
  assert.equal(result.diagnostic.kind, 'unsupported');
  assert.match(result.diagnostic.message, /Rust type f32/u);
});
