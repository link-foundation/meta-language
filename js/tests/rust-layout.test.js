// Issue #217: emitted Rust that does not fit the width is laid out one item
// per line, the way rustfmt would, identically to the Rust root
// (`translation::rust_layout`).

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { RUST_WIDTH, wrapRust } from '../src/translation/rust-layout.js';

test('a short line is kept', () => {
  assert.equal(wrapRust('let x = vec![1, 2];'), 'let x = vec![1, 2];');
});

test('a long list goes one item per line', () => {
  const words = Array.from({ length: 30 }, (_, index) => `"word${index}"`).join(', ');
  const wrapped = wrapRust(`    let words = vec![${words}];`);
  assert.ok(wrapped.startsWith('    let words = vec![\n        "word0",\n'));
  assert.ok(wrapped.endsWith('\n        "word29",\n    ];'));
  assert.ok(wrapped.split('\n').every((line) => Array.from(line).length <= RUST_WIDTH));
});

test('a parenthesized expression never becomes a tuple', () => {
  const line = '    let total = (alpha_value_long_name + beta_value_long_name + gamma_value_long_name + delta_value) * 2;';
  assert.equal(
    wrapRust(line),
    '    let total = (\n        alpha_value_long_name + beta_value_long_name + gamma_value_long_name + delta_value\n    ) * 2;',
  );
});

test('brackets and commas inside literals are text', () => {
  const line = `    let text = "${'a, (b) [c], '.repeat(12)}";`;
  assert.equal(wrapRust(line), line);
});
