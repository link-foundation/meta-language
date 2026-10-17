// Original independent character controls retain every source input and assertion.
import assert from 'node:assert/strict';
import test from 'node:test';
import {
  decodeRustCharacterLiteral,
  readRustCharacterByteEscapeValue,
  readRustCharacterSimpleEscapeValue,
} from '../src/translation/rust-character-rules.js';
import { readProjectData } from '../scripts/project-data.mjs';
const fixture = readProjectData(new URL('../../parity/fixtures/rust-character-literals.lino', import.meta.url)).original;
import { decodeUnicodeEscape } from '../src/translation/frontend-rules.js';
import { tokenize } from '../src/translation/lexer.js';

const units = source => Array.from({ length: source.length }, (_, at) => source.charCodeAt(at));

test('independent character and byte values retain exact original UTF16 slices', () => {
  for (const row of fixture.positives) {
    for (const prefix of ['', '// 😀 α\n']) {
      const source = prefix + row.source + ' + following';
      const before = units(source);
      const input = Object.freeze([...before]);
      const result = decodeRustCharacterLiteral(input, prefix.length);
      assert.deepEqual(result, {
        $: row.kind, value: row.value,
        contentStart: prefix.length + (row.kind === 'byteChar' ? 2 : 1),
        contentEnd: prefix.length + row.source.length - 1,
        end: prefix.length + row.source.length,
      }, row.source);
      assert.equal(source.slice(prefix.length, result.end), row.source);
      assert.deepEqual(input, before, 'the scanner never rewrites the original source units');
    }
  }
});

test('all 256 byte hex values and both spellings have independent exact numeric expectations', () => {
  for (let code = 0; code < 256; code += 1) {
    for (const digits of [code.toString(16).padStart(2, '0'), code.toString(16).padStart(2, '0').toUpperCase()]) {
      assert.equal(readRustCharacterByteEscapeValue(digits.charCodeAt(0), digits.charCodeAt(1)), code);
      const byteSource = "b'\\x" + digits + "'";
      const result = decodeRustCharacterLiteral(units(byteSource), 0);
      assert.equal(result.$, 'byteChar');
      assert.equal(result.value, code);
      assert.equal(result.end, byteSource.length);
      const charSource = "'\\x" + digits + "'";
      const character = decodeRustCharacterLiteral(units(charSource), 0);
      if (code < 128) {
        assert.equal(character.$, 'char');
        assert.equal(character.value, code);
      } else assert.equal(character.$, 'malformed');
    }
  }
});

test('raw control and scalar boundaries are independent of the escape decoder', () => {
  for (const code of [0, 8, 11, 12, 32, 34, 123, 125, 127, 128, 55295, 57344, 65535, 65536, 128512, 1114111]) {
    const source = "'" + String.fromCodePoint(code) + "'";
    const result = decodeRustCharacterLiteral(units(source), 0);
    assert.equal(result.$, 'char', String(code));
    assert.equal(result.value, code);
    assert.equal(result.end, source.length);
    const byte = decodeRustCharacterLiteral(units('b' + source), 0);
    assert.equal(byte.$, code < 128 ? 'byteChar' : 'nonAscii');
  }
  for (const input of fixture.nonScalarPayloads) assert.equal(decodeRustCharacterLiteral(input, 0).$, 'nonScalar');
});

test('malformed and unsupported spellings refuse with bounded absolute source positions', () => {
  for (const row of fixture.negatives) {
    const prefix = '// 😀\n';
    const source = prefix + row.source;
    const result = decodeRustCharacterLiteral(units(source), prefix.length);
    assert.equal(result.$, row.kind, row.source);
    assert.ok(result.at >= prefix.length && result.at <= source.length, row.source);
    assert.ok(result.end >= result.at && result.end <= source.length, row.source);
  }
  for (const at of [-1, 0.5, NaN, Infinity, -Infinity, 5]) {
    assert.deepEqual(decodeRustCharacterLiteral(units("'a'"), at), { $: 'notCharacter' });
  }
  assert.equal(decodeRustCharacterLiteral(units("'a'"), -0).$, 'char');
});

test('nonliteral apostrophes do not consume or certify lifetime and label syntax', () => {
  for (const source of [...fixture.unchangedApostrophePaths, ...fixture.nonPrefixes]) {
    const input = units(source);
    assert.deepEqual(decodeRustCharacterLiteral(input, 0), { $: 'notCharacter' }, source);
    assert.deepEqual(input, units(source));
  }
  // Record today's actual tokenizer output as the conservation oracle only for
  // paths that this new scanner explicitly leaves unselected. Integration will
  // compare these whole token/comment vectors before and after the lexer hook.
  const lifetime = tokenize(fixture.dependencyControls.validLifetimeProgram, 'Rust');
  const label = tokenize(fixture.dependencyControls.labelSource, 'Rust');
  assert.ok(lifetime.tokens.some(token => token.value === "'"));
  assert.ok(label.tokens.some(token => token.value === "'"));
});

test('simple escapes and consumed numeric units reject foreign host cells without coercion', () => {
  const values = new Map([[110, 10], [114, 13], [116, 9], [48, 0], [92, 92], [39, 39], [34, 34]]);
  for (let code = 0; code < 256; code += 1) assert.equal(readRustCharacterSimpleEscapeValue(code), values.get(code) ?? -1);
  for (const value of [undefined, null, false, true, '65', 65n, 65.5, NaN, Infinity, -1, 65536]) {
    assert.equal(decodeRustCharacterLiteral([39, value, 39], 0).$, 'nonScalar');
    assert.equal(readRustCharacterByteEscapeValue(value, 48), -1);
  }
  const hole = [39, 65, 39];
  delete hole[1];
  assert.equal(decodeRustCharacterLiteral(hole, 0).$, 'nonScalar');
  assert.deepEqual(decodeUnicodeEscape(units(fixture.dependencyControls.genericProtectedEscape), false), { $: 'malformed' });
  assert.throws(() => tokenize(fixture.dependencyControls.cookedProtectedEscape, 'Rust'), /malformed Rust ASCII escape/u);
});

test('long Unicode separators retain exact values and original offsets without a fixed width cap', () => {
  const source = "'\\u{1" + '_'.repeat(32768) + "F600___}'";
  const input = units(source);
  const before = [...input];
  const result = decodeRustCharacterLiteral(input, 0);
  assert.equal(result.$, 'char');
  assert.equal(result.value, 128512);
  assert.equal(result.end, source.length);
  assert.deepEqual(input, before);
  for (const separator of ['\u0085', '\u200e', '\u200f', '\u2028', '\u2029']) {
    assert.equal(decodeRustCharacterLiteral(units("'a'" + separator), 0).$, 'char');
  }
});
