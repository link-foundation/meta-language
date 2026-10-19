import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { tokenize } from '../src/translation/lexer.js';
import { parseRust } from '../src/translation/rust.js';

const tokenFacts = result => result.tokens.map(({ kind, value, raw }) => [kind, value, raw]);

test('Rust character tokens retain original absolute UTF16 spans and string token values', () => {
  const prefix = '// 😀 original UTF16 source\r\n';
  const cases = [
    ["'{'", 'char', '123'], ["'}'", 'char', '125'], ["'\"'", 'char', '34'],
    ["'α'", 'char', '945'], ["'😀'", 'char', '128512'],
    [String.raw`'\u{1_F600___}'`, 'char', '128512'],
    [String.raw`'\x7F'`, 'char', '127'], [String.raw`'\''`, 'char', '39'],
    ["b'}'", 'byteChar', '125'], [String.raw`b'\xFF'`, 'byteChar', '255'],
  ];
  for (const [raw, kind, value] of cases) {
    const source = prefix + raw + ' + following';
    const result = tokenize(source, 'Rust');
    assert.deepEqual(result.tokens[0], { kind, value, raw, start: prefix.length, end: prefix.length + raw.length });
    assert.equal(typeof result.tokens[0].value, 'string');
    assert.equal(source.slice(result.tokens[0].start, result.tokens[0].end), raw);
    assert.deepEqual(result.comments, [{ text: prefix.slice(0, -1), start: 0, end: prefix.length - 1 }]);
    assert.deepEqual(result.tokens.slice(1).map(token => token.raw), ['+', 'following', '']);
  }
});

test('character braces quotes and comment markers remain opaque in complete original Rust sources', () => {
  const source = "// 😀\nfn main(){ let a='{'; let b=b'}'; let c='/'; let d='*'; let e='\"'; } // outside";
  const result = tokenize(source, 'Rust');
  assert.deepEqual(result.tokens.filter(token => ['char', 'byteChar'].includes(token.kind)).map(token => [token.kind, token.value, token.raw]), [
    ['char', '123', "'{'"], ['byteChar', '125', "b'}'"], ['char', '47', "'/'"],
    ['char', '42', "'*'"], ['char', '34', "'\"'"],
  ]);
  assert.deepEqual(result.tokens.filter(token => token.kind === 'punct' && ['{', '}'].includes(token.value)).map(token => token.value), ['{', '}']);
  assert.deepEqual(result.comments.map(comment => comment.text), ['// 😀', '// outside']);
  const original = "fn a(){m!('\"')} //\"\ntype A = _;";
  const actual = tokenize(original, 'Rust');
  assert.deepEqual(actual.tokens.filter(token => token.kind === 'char').map(token => token.raw), ["'\"'"]);
  assert.deepEqual(actual.comments.map(comment => comment.text), ['//\"']);
  assert.deepEqual(actual.tokens.slice(-6).map(token => token.raw), ['type', 'A', '=', '_', ';', '']);
});

test('complete owned Rust numeric formatting source retains all independently listed character uses', () => {
  const source = readFileSync(new URL('../../rust/src/translation/js_number.rs', import.meta.url), 'utf8');
  const tokens = tokenize(source, 'Rust').tokens;
  assert.deepEqual(tokens.filter(token => ['char', 'byteChar'].includes(token.kind)).map(token => [token.kind, token.value, token.raw]), [
    ['char', '53', "'5'"], ['byteChar', '48', "b'0'"], ['byteChar', '48', "b'0'"],
    ['char', '48', "'0'"], ['char', '45', "'-'"], ['char', '43', "'+'"],
    ['byteChar', '57', "b'9'"], ['byteChar', '48', "b'0'"],
    ['char', '101', "'e'"], ['char', '46', "'.'"], ['char', '48', "'0'"],
  ]);
  for (const token of tokens) assert.equal(source.slice(token.start, token.end), token.raw);
  assert.equal(tokens.at(-1).end, source.length);
});

test('cooked raw strings and comments do not elect embedded character spellings', () => {
  const cooked = "\"'{' b'x'\"";
  const raw = `r##"'}' " // /* b'y'"##`;
  const source = cooked + ' ' + raw + " // 'z'\n/* b'w' */ value";
  const result = tokenize(source, 'Rust');
  assert.deepEqual(tokenFacts(result), [
    ['string', "'{' b'x'", cooked], ['string', `'}' " // /* b'y'`, raw],
    ['identifier', 'value', 'value'], ['eof', '', ''],
  ]);
  assert.deepEqual(result.comments.map(comment => comment.text), ["// 'z'", "/* b'w' */"]);
});

test('nonliteral apostrophes lifetimes labels and foreign token paths remain unchanged', () => {
  const source = "'a 'static '_ 'label:";
  assert.deepEqual(tokenFacts(tokenize(source, 'Rust')), [
    ['punct', "'", "'"], ['identifier', 'a', 'a'],
    ['punct', "'", "'"], ['identifier', 'static', 'static'],
    ['punct', "'", "'"], ['identifier', '_', '_'],
    ['punct', "'", "'"], ['identifier', 'label', 'label'], ['punct', ':', ':'], ['eof', '', ''],
  ]);
  const lifetime = "fn keep<'a>(value: &'a str) -> &'a str { value }";
  const label = "fn run() { 'outer: loop { break 'outer; } }";
  for (const [original, entries] of [
    [lifetime, [
      ['identifier', 'fn'], ['identifier', 'keep'], ['punct', '<'], ['punct', "'"],
      ['identifier', 'a'], ['punct', '>'], ['punct', '('], ['identifier', 'value'],
      ['punct', ':'], ['punct', '&'], ['punct', "'"], ['identifier', 'a'],
      ['identifier', 'str'], ['punct', ')'], ['punct', '->'], ['punct', '&'],
      ['punct', "'"], ['identifier', 'a'], ['identifier', 'str'], ['punct', '{'],
      ['identifier', 'value'], ['punct', '}'],
    ]],
    [label, [
      ['identifier', 'fn'], ['identifier', 'run'], ['punct', '('], ['punct', ')'],
      ['punct', '{'], ['punct', "'"], ['identifier', 'outer'], ['punct', ':'],
      ['identifier', 'loop'], ['punct', '{'], ['identifier', 'break'], ['punct', "'"],
      ['identifier', 'outer'], ['punct', ';'], ['punct', '}'], ['punct', '}'],
    ]],
  ]) {
    let end = 0;
    const expected = entries.map(([kind, raw]) => {
      const start = original.indexOf(raw, end);
      assert.ok(start >= end, raw);
      end = start + raw.length;
      return { kind, value: raw, raw, start, end };
    });
    expected.push({ kind: 'eof', value: '', raw: '', start: original.length, end: original.length });
    assert.deepEqual(tokenize(original, 'Rust'), { tokens: expected, comments: [] });
  }
  for (const language of ['Lean', 'Rocq']) assert.deepEqual(tokenFacts(tokenize("'a' b'a' x'", language)), [
    ['punct', "'", "'"], ['identifier', "a'", "a'"], ['identifier', "b'a'", "b'a'"],
    ['identifier', "x'", "x'"], ['eof', '', ''],
  ]);
  assert.deepEqual(tokenFacts(tokenize("'a' b'a' x", 'JavaScript')), [
    ['string', 'a', "'a'"], ['identifier', 'b', 'b'], ['string', 'a', "'a'"],
    ['identifier', 'x', 'x'], ['eof', '', ''],
  ]);
});

test('unsupported suffixes and character semantics refuse without scalar or String coercion', () => {
  for (const literal of ["'a'word", "b'a'u8", "'a'_suffix", "'a'λ"]) {
    const source = '// 😀\n' + literal;
    const at = source.indexOf(literal) + literal.lastIndexOf("'") + 1;
    assert.throws(() => tokenize(source, 'Rust'), error => error.kind === 'unsupported'
      && error.reason === 'Rust character literal suffix is outside the supported lexical profile'
      && error.span.start === at && error.span.end === at + 1);
  }
  for (const literal of ["'{'", "b'}'", String.raw`'\u{1F600}'`, String.raw`b'\xFF'`]) {
    const source = 'fn value()->u32{' + literal + '}';
    if (literal.startsWith('b')) {
      // Preserve the original library-only input. Admitted byte expressions now
      // reach the existing required-main boundary; complete type checks follow
      // in the paired byte-expression controls.
      assert.throws(() => parseRust(source), error => error.kind === 'syntax'
        && error.reason === 'a Rust program needs fn main'
        && error.span.start === source.length && error.span.end === source.length);
    } else {
      assert.throws(() => parseRust(source), error => error.kind === 'unsupported'
        && error.reason === 'character literal: characters are outside the portable core'
        && error.span.start === source.indexOf(literal) && error.span.end === source.indexOf(literal) + literal.length);
    }
    const pattern = 'fn choose(x:u8)->u8{match x{' + literal + '=>1,_=>0}}';
    assert.throws(() => parseRust(pattern), error => error.kind === 'unsupported'
      && error.reason === 'character pattern: characters are outside the portable core');
  }
  for (const literal of ["''", "b'é'", String.raw`b'\u{41}'`, String.raw`'\x80'`]) {
    assert.throws(() => tokenize(literal, 'Rust'), error => error.kind === 'syntax'
      && error.span.start >= 0 && error.span.end <= literal.length);
  }
  assert.throws(() => tokenize(String.raw`'\u{D800}'`, 'Rust'), error => error.kind === 'unsupported'
    && error.reason === 'Rust character literal is not a Unicode scalar value');
});
