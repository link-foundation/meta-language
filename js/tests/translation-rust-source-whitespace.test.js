import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { LinkNetwork, createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/index.js';
import { tokenize } from '../src/translation/lexer.js';
import { acceptRustSourceWhitespace, acceptRustSourceByteOrderMark } from '../src/translation/rust-source-whitespace-rules.js';
import { readProjectData } from '../scripts/project-data.mjs';

const units = [9, 10, 11, 12, 13, 32, 133, 8206, 8207, 8232, 8233];
const excluded = [0, 8, 14, 31, 0xa0, 0x1680, ...Array.from({ length: 11 }, (_, index) => 0x2000 + index), 0x202f, 0x205f, 0x3000, 0xfeff];
const source = separator => `pub${separator}fn identity(value:i32)->i32{value}\nfn main(){}`;

function replay(text) {
  const packet = createTypedProgramLinks(text, 'Rust');
  const network = LinkNetwork.fromLino(packet.network.toLino());
  assert.equal(decodeTypedProgramLinks(network, packet.root).declarations.get('identity').span.start, text.indexOf('fn'));
  return emitTypedProgramLinks(network, packet.root, 'JavaScript');
}

test('Rust source whitespace is exactly the eleven normative UTF-16 code units', () => {
  const policy = readProjectData(new URL('../src/translation/data/rust-source-whitespace.lino', import.meta.url));
  assert.deepEqual(policy.codeUnits, units);
  assert.deepEqual(policy.byteOrderMark, { codeUnit: 65279, position: 'first source code unit only', count: 1 });
  const expected = new Set(units);
  for (let unit = 0; unit < 65536; unit += 1) {
    assert.equal(acceptRustSourceWhitespace(unit), expected.has(unit), `U+${unit.toString(16)}`);
    assert.equal(acceptRustSourceByteOrderMark(unit, true), unit === 0xfeff);
    assert.equal(acceptRustSourceByteOrderMark(unit, false), false);
  }
  for (const unit of [-1, 65536, NaN, Infinity, -Infinity, 9.5, '9', null]) {
    assert.equal(acceptRustSourceWhitespace(unit), false);
    assert.equal(acceptRustSourceByteOrderMark(unit, true), false);
  }
});

test('every Rust whitespace form retains original offsets and executable public Links replay', () => {
  for (const unit of units) {
    const text = '\ufeff// 😀 original offset\n' + source(String.fromCharCode(unit));
    const { tokens } = tokenize(text, 'Rust');
    assert.equal(tokens[0].value, 'pub');
    assert.equal(tokens[0].start, text.indexOf('pub'));
    const declaration = tokens.find(token => token.value === 'fn');
    assert.equal(declaration.start, text.indexOf('fn'));
    for (const token of tokens) assert.equal(text.slice(token.start, token.end), token.raw);
    assert.equal(tokens.at(-1).end, text.length);
    const context = {};
    vm.runInNewContext(replay(text).code, context, { timeout: 1000 });
    assert.equal(context.identity(7n), 7n);
  }
});

test('non-Rust whitespace and every noninitial byte order mark remain actual source tokens', () => {
  for (const unit of excluded) {
    const separator = String.fromCharCode(unit);
    const text = source(separator);
    const token = tokenize(text, 'Rust').tokens.find(token => token.raw === separator);
    assert.equal(token?.kind, 'punct', `U+${unit.toString(16)} is not discarded`);
    assert.equal(token.start, 3);
    assert.throws(() => createTypedProgramLinks(text, 'Rust'), `U+${unit.toString(16)} cannot separate Rust tokens`);
  }
  for (const text of ['\ufeff\ufeff' + source(' '), ' \ufeff' + source(' '), '// initial line\n\ufeff' + source(' ')]) {
    assert.throws(() => createTypedProgramLinks(text, 'Rust'));
  }
  const literal = tokenize('"\ufeff\u0085\u200e\u00a0"', 'Rust').tokens[0];
  assert.equal(literal.kind, 'string');
  assert.equal(literal.value, '\ufeff\u0085\u200e\u00a0');
  const comment = tokenize('// \ufeff\u0085\u200e\u00a0\n' + source(' '), 'Rust').comments[0];
  assert.equal(comment.text, '// \ufeff\u0085\u200e\u00a0');
});

test('JavaScript, Lean and Rocq retain their existing whitespace interpretation', () => {
  for (const language of ['JavaScript', 'Lean', 'Rocq']) {
    for (const unit of [...units, ...excluded]) {
      const separator = String.fromCharCode(unit);
      const { tokens } = tokenize(separator + 'name', language);
      if (/\s/u.test(separator)) {
        assert.equal(tokens[0].value, 'name');
        assert.equal(tokens[0].start, 1);
      } else {
        assert.equal(tokens[0].raw, separator);
        assert.equal(tokens[0].start, 0);
      }
    }
  }
});
