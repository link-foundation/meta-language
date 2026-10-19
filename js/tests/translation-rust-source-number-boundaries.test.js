import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { LinkNetwork, createTypedProgramLinks, emitTypedProgramLinks } from '../src/index.js';
import { acceptRustNumberDigitBoundary } from '../src/translation/rust-number-rules.js';
import { acceptRustSourceWhitespace, acceptRustSourceNumberDigitBoundary } from '../src/translation/rust-source-whitespace-rules.js';
import { tokenize } from '../src/translation/lexer.js';

const whitespace = [9, 10, 11, 12, 13, 32, 133, 8206, 8207, 8232, 8233];
const suffixes = ['u8', 'i8', 'u16', 'i16', 'u32', 'i32', 'u64', 'i64', 'u128', 'i128', 'usize', 'isize'];

test('Rust source number boundaries reuse the unchanged original rule only for proven Rust whitespace', () => {
  for (const suffix of suffixes) for (const unit of whitespace) {
    assert.equal(acceptRustSourceWhitespace(unit), true);
    assert.equal(acceptRustSourceNumberDigitBoundary(false, true, suffix, false, unit), true);
    assert.equal(acceptRustNumberDigitBoundary(false, true, suffix, false, unit), unit < 128);
  }
  for (const unit of [128, 160, 233, 955, 769, 55349, 0xfeff, NaN, Infinity]) {
    assert.equal(acceptRustNumberDigitBoundary(false, true, 'u64', false, unit), false);
    assert.equal(acceptRustSourceNumberDigitBoundary(false, true, 'u64', false, unit), false);
  }
  for (const suffix of ['', 'u', 'u7', 'u64extra', 'U64', 'unknown']) for (const unit of whitespace) {
    assert.equal(acceptRustSourceNumberDigitBoundary(false, true, suffix, false, unit), false);
  }
  for (const unit of whitespace) assert.equal(acceptRustSourceNumberDigitBoundary(false, true, 'u64', true, unit), false);
});

test('all legal Rust whitespace separates suffixed exact integers through public source-free Links replay', () => {
  for (const unit of whitespace) {
    const separator = String.fromCharCode(unit);
    const source = `pub fn exact()->u64{9_007_199_254_740_992_u64${separator}}fn main(){}`;
    const token = tokenize(source, 'Rust').tokens.find(token => token.raw === '9_007_199_254_740_992_u64');
    assert.equal(token.value, '9007199254740992');
    assert.equal(source.slice(token.start, token.end), token.raw);
    const packet = createTypedProgramLinks(source, 'Rust');
    const emitted = emitTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root, 'JavaScript');
    const context = {};
    vm.runInNewContext(emitted.code, context, { timeout: 1000 });
    assert.equal(context.exact(), 9007199254740992n);
  }
  for (const literal of ['1_', '1_u64é', '1_u64λ', '1_u64\u0301', '1_u64𝔘', '1_u64\u00a0']) {
    assert.throws(() => tokenize(literal, 'Rust'), error => error.kind === 'syntax');
  }
});
