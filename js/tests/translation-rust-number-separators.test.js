import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { tokenize } from '../src/translation/lexer.js';
import { acceptRustNumberDigitBoundary } from '../src/translation/rust-number-rules.js';
import { translateProgram } from '../src/program-translation.js';
import rustCases from './fixtures/translation-cases/frontend-rust.mjs';
import javascriptCases from './fixtures/translation-cases/frontend-javascript.mjs';

const policy = readProjectData(new URL('../src/translation/data/rust-number-literals.lino', import.meta.url));
const first = source => tokenize(source, 'Rust').tokens[0];

test('shared Rust numeric boundary admits only complete supported integer suffixes after separators', () => {
  assert.deepEqual(policy.integerSuffixes, ['u8', 'i8', 'u16', 'i16', 'u32', 'i32', 'u64', 'i64', 'u128', 'i128', 'usize', 'isize']);
  for (const suffix of policy.integerSuffixes) {
    for (const separators of ['_', '__', '___']) {
      const raw = '12_345' + separators + suffix;
      assert.deepEqual(first(raw), { kind: 'number', value: '12345', suffix, raw, start: 0, end: raw.length });
      assert.equal(acceptRustNumberDigitBoundary(false, true, suffix, false), true);
      assert.equal(acceptRustNumberDigitBoundary(false, true, suffix, true), false);
    }
  }
  for (const suffix of ['', 'u', 'u7', 'u64extra', 'U64', 'f80', 'n', 'unknown']) {
    assert.equal(acceptRustNumberDigitBoundary(false, true, suffix, false), false, suffix);
  }
  for (const floating of [false, true]) for (const continued of [false, true]) {
    assert.equal(acceptRustNumberDigitBoundary(floating, false, '', continued), true);
  }
  assert.equal(acceptRustNumberDigitBoundary(true, true, 'f64', false), true);
});

test('Rust separated suffixes retain integer precision, source offsets and emitted executable values', () => {
  const source = 'pub fn exact() -> u64 { 9_007_199_254_740_992_u64 }\npub fn negative() -> i64 { -9_i64 }\npub fn largest() -> u128 { 340282366920938463463374607431768211455_u128 }\nfn main() {}';
  const tokens = tokenize(source, 'Rust').tokens;
  const exact = tokens.find(token => token.raw === '9_007_199_254_740_992_u64');
  assert.equal(exact.value, '9007199254740992');
  assert.equal(source.slice(exact.start, exact.end), exact.raw);
  assert.equal(exact.suffix, 'u64');
  const negative = tokens.findIndex(token => token.raw === '9_i64');
  assert.equal(tokens[negative - 1].value, '-');
  const result = translateProgram(source, 'Rust', 'JavaScript');
  assert.equal(result.diagnostic, null);
  const context = {};
  runInNewContext(result.code, context);
  assert.equal(context.exact(), 9_007_199_254_740_992n);
  assert.equal(context.negative(), -9n);
  assert.equal(context.largest(), (1n << 128n) - 1n);
});

test('the actual complete runtime-shell native test file tokenizes without changing its source', () => {
  const path = new URL('../../rust/tests/unit/translation_rust_runtime_shell.rs', import.meta.url);
  const bytes = readFileSync(path);
  const before = createHash('sha256').update(bytes).digest('hex');
  const source = bytes.toString('utf8');
  assert.ok(source.includes('9007199254740992_u64'));
  const { tokens } = tokenize(source, 'Rust');
  const token = tokens.find(token => token.raw === '9007199254740992_u64');
  assert.deepEqual({ value: token.value, suffix: token.suffix }, { value: '9007199254740992', suffix: 'u64' });
  assert.equal(tokens.at(-1).kind, 'eof');
  assert.equal(tokens.at(-1).end, source.length);
  assert.equal(createHash('sha256').update(readFileSync(path)).digest('hex'), before);
});

test('protected Rust unsuffixed and incomplete suffix refusals and JavaScript numeric rejections remain intact', () => {
  for (const raw of ['1_', '1__', '1_u', '1_u7', '1_u64extra', '1_u64_', '1_u64_foo', '1_U64', '1_unknown']) {
    assert.throws(() => tokenize(raw, 'Rust'), error => error.kind === 'syntax' && error.message.includes('malformed number'), raw);
  }
  const fixture = readFileSync(new URL('./fixtures/translation-cases/frontend-rust.mjs', import.meta.url), 'utf8');
  assert.ok(fixture.includes("lexMalformedNumber: fn('1_')"));
  for (const raw of ['1_', '1__0', '1_u64', '9_007_199_254_740_992_u64', '1_0n_', '0xff_u8', '08', '1.5n', '3in', '0x_1', '1e_2']) {
    assert.throws(() => tokenize(raw, 'JavaScript'), error => error.kind === 'syntax', raw);
  }
  for (const [raw, value] of [['1_000', '1000'], ['0xff', '255'], ['0b10n', '2'], ['1_000n', '1000'], ['1.25e2', '125']]) {
    assert.equal(tokenize(raw, 'JavaScript').tokens[0].value, value);
  }
  assert.equal(first('12E+2_f64').value, '1200');
  assert.equal(first('1_f32').floating, true);
  assert.throws(() => tokenize('1e999_f64', 'Rust'), error => error.kind === 'syntax' && error.message.includes('out of range'));
});

test('existing protected Rust and JavaScript malformed numeric fixture sources keep their lexer errors', () => {
  const source = rustCases['frontend-rust-lexMalformedNumber'][1];
  assert.throws(() => tokenize(source, 'Rust'), error => error.kind === 'syntax' && error.reason === 'malformed number 1_'
    && error.span.start === source.indexOf('1_') && error.span.end === source.indexOf('1_') + 2);
  for (const name of ['radix-empty-hex', 'radix-bad-binary', 'radix-bad-octal', 'exponent-bigint']) {
    const source = javascriptCases['frontend-javascript-error-' + name][1];
    assert.throws(() => tokenize(source, 'JavaScript'), error => error.kind === 'syntax' && error.message.startsWith('malformed number'), name);
  }
});
