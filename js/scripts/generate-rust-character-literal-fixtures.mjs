// Automatically convert every independent source fixture and expanded ordinal to
// shared Links data. No receipt, observation archive or native result is input.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { rustCharacterFixtures as fixture } from './rust-character-fixture-source.mjs';
import {
  decodeRustCharacterLiteral,
  readRustCharacterByteEscapeValue,
  readRustCharacterSimpleEscapeValue,
} from '../src/translation/rust-character-rules.js';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/scripts/rust-character-fixture-source.mjs';
const kernelPath = 'js/src/translation/rust-character-rules.js';
const destination = new URL('parity/fixtures/rust-character-literals.lino', root);
const nativeDestination = new URL('rust/src/data/rust-character-literals.lino', root);
const digest = source => createHash('sha256').update(source).digest('hex');
const sourceUnits = source => Array.from({ length: source.length }, (_, at) => source.charCodeAt(at));
const roots = ['decodeRustCharacterLiteral', 'readRustCharacterByteEscapeValue', 'readRustCharacterSimpleEscapeValue'];
const actualRoots = { decodeRustCharacterLiteral, readRustCharacterByteEscapeValue, readRustCharacterSimpleEscapeValue };

// This builder is transplanted mechanically from the original private matrix.
// Its expected values are independent literal/formula premises, never scanner
// outputs. Nonfinite Number cases are retained, then classified as JS-only.
export function readRustCharacterFixtureCases() {
  const cases = [];
  const add = (id, target, args, expected, source = null) => {
    assert.ok(roots.includes(target));
    for (const value of args) {
      if (Array.isArray(value)) {
        for (let at = 0; at < value.length; at += 1) assert.ok(Object.hasOwn(value, at) && typeof value[at] === 'number');
      } else assert.equal(typeof value, 'number');
    }
    cases.push({ id, target, args, expected, source });
  };
  const scan = (id, source, start, expected) => add(id, roots[0], [sourceUnits(source), start], expected, source);
  const positive = (id, source, start, token, kind, value) => scan(id, source, start, {
    mode: 'exact', value: { $: kind, value, contentStart: start + (kind === 'byteChar' ? 2 : 1),
      contentEnd: start + token.length - 1, end: start + token.length },
  });
  for (const [ordinal, row] of fixture.positives.entries()) {
    for (const [prefixOrdinal, prefix] of ['', '// 😀 α\n'].entries()) {
      positive('positive/' + ordinal + '/' + prefixOrdinal, prefix + row.source + ' + following', prefix.length,
        row.source, row.kind, row.value);
    }
  }
  for (let code = 0; code < 256; code += 1) {
    for (const [spelling, digits] of [code.toString(16).padStart(2, '0'), code.toString(16).padStart(2, '0').toUpperCase()].entries()) {
      add('hex-helper/' + code + '/' + spelling, roots[1], [digits.charCodeAt(0), digits.charCodeAt(1)], { mode: 'exact', value: code });
      const byte = "b'\\x" + digits + "'";
      positive('byte-hex/' + code + '/' + spelling, byte, 0, byte, 'byteChar', code);
      const char = "'\\x" + digits + "'";
      if (code < 128) positive('char-hex/' + code + '/' + spelling, char, 0, char, 'char', code);
      else scan('char-hex/' + code + '/' + spelling, char, 0, { mode: 'tagAndBounds', tag: 'malformed' });
    }
  }
  for (const code of [0, 8, 11, 12, 32, 34, 123, 125, 127, 128, 55295, 57344, 65535, 65536, 128512, 1114111]) {
    const token = "'" + String.fromCodePoint(code) + "'";
    positive('raw-char/' + code, token, 0, token, 'char', code);
    if (code < 128) positive('raw-byte/' + code, 'b' + token, 0, 'b' + token, 'byteChar', code);
    else scan('raw-byte/' + code, 'b' + token, 0, { mode: 'tagAndBounds', tag: 'nonAscii' });
  }
  for (const [ordinal, input] of fixture.nonScalarPayloads.entries()) {
    add('non-scalar/' + ordinal, roots[0], [[...input], 0], { mode: 'tagAndBounds', tag: 'nonScalar' });
  }
  for (const [ordinal, row] of fixture.negatives.entries()) {
    const prefix = '// 😀\n';
    scan('negative/' + ordinal, prefix + row.source, prefix.length, { mode: 'tagAndBounds', tag: row.kind });
  }
  for (const [ordinal, source] of [...fixture.unchangedApostrophePaths, ...fixture.nonPrefixes].entries()) {
    scan('unchanged-apostrophe/' + ordinal, source, 0, { mode: 'exact', value: { $: 'notCharacter' } });
  }
  for (const [ordinal, start] of [-1, 0.5, NaN, Infinity, -Infinity, 5].entries()) {
    scan('invalid-start/' + ordinal, "'a'", start, { mode: 'exact', value: { $: 'notCharacter' } });
  }
  positive('negative-zero-start', "'a'", -0, "'a'", 'char', 97);
  const simple = new Map([[110, 10], [114, 13], [116, 9], [48, 0], [92, 92], [39, 39], [34, 34]]);
  for (let code = 0; code < 256; code += 1) {
    add('simple/' + code, roots[2], [code], { mode: 'exact', value: simple.get(code) ?? -1 });
  }
  for (const [ordinal, value] of [65.5, NaN, Infinity, -Infinity, -1, 65536].entries()) {
    add('invalid-number-unit/' + ordinal, roots[0], [[39, value, 39], 0], { mode: 'tagAndBounds', tag: 'nonScalar' });
    add('invalid-number-hex/' + ordinal, roots[1], [value, 48], { mode: 'exact', value: -1 });
  }
  // The scanner's primitive Number model retains signed zero as a raw code
  // unit; this checks the actual source behavior without a JSON round trip.
  add('negative-zero-unit', roots[0], [[39, -0, 39], 0], { mode: 'exact',
    value: { $: 'char', value: -0, contentStart: 1, contentEnd: 2, end: 3 } });
  const long = "'\\u{1" + '_'.repeat(32768) + "F600___}'";
  positive('long-unicode-separators', long, 0, long, 'char', 128512);
  for (const [ordinal, separator] of ['\u0085', '\u200e', '\u200f', '\u2028', '\u2029'].entries()) {
    positive('following-unicode-whitespace/' + ordinal, "'a'" + separator, 0, "'a'", 'char', 97);
  }
  assert.equal(new Set(cases.map(row => row.id)).size, cases.length, 'each original vector keeps a distinct observation identifier');
  return cases;
}

export function assertRustCharacterFixtureExpectation(row, actual) {
  if (row.expected.mode === 'exact') assert.deepEqual(actual, row.expected.value, row.id + ': independent exact result');
  else {
    assert.equal(row.expected.mode, 'tagAndBounds');
    assert.equal(actual.$, row.expected.tag, row.id + ': independent refusal tag');
    const [units, start] = row.args;
    assert.ok(Number.isInteger(actual.at) && actual.at >= start && actual.at <= units.length, row.id + ': absolute refusal start');
    assert.ok(Number.isInteger(actual.end) && actual.end >= actual.at && actual.end <= units.length, row.id + ': bounded absolute refusal end');
  }
  if (row.source !== null && actual && (actual.$ === 'char' || actual.$ === 'byteChar')) {
    assert.equal(row.source.slice(row.args[1], actual.end), row.source.slice(row.args[1], row.expected.value.end),
      row.id + ': the original raw token slice is preserved');
  }
}

// A paired fixture contains finite binary64 Number cells only. Reject foreign
// values, holes and accessors before inspecting or serializing their contents.
export function acceptRustCharacterFixtureArguments(args) {
  if (!Array.isArray(args) || Object.getPrototypeOf(args) !== Array.prototype
    || Reflect.ownKeys(args).length !== args.length + 1) return false;
  for (let at = 0; at < args.length; at += 1) {
    const slot = Object.getOwnPropertyDescriptor(args, at);
    if (!slot || !Object.hasOwn(slot, 'value')) return false;
    const value = slot.value;
    if (typeof value === 'number') {
      if (!Number.isFinite(value)) return false;
      continue;
    }
    if (!Array.isArray(value) || Object.getPrototypeOf(value) !== Array.prototype
      || Reflect.ownKeys(value).length !== value.length + 1) return false;
    for (let index = 0; index < value.length; index += 1) {
      const unit = Object.getOwnPropertyDescriptor(value, index);
      if (!unit || !Object.hasOwn(unit, 'value')
        || typeof unit.value !== 'number' || !Number.isFinite(unit.value)) return false;
    }
  }
  return true;
}


// The existing finite-cell fixture remains unchanged. These additional words
// transport only explicit primitive Numbers, never a JSON null/string coercion.
export function encodeRustCharacterFixtureNumberWords(value) {
  if (typeof value === 'number') {
    const view = new DataView(new ArrayBuffer(8));
    view.setFloat64(0, value, false);
    return { highWord: view.getUint32(0, false).toString(16).padStart(8, '0'),
      lowWord: view.getUint32(4, false).toString(16).padStart(8, '0') };
  }
  assert.ok(Array.isArray(value) && Object.getPrototypeOf(value) === Array.prototype
    && Reflect.ownKeys(value).length === value.length + 1,
  'Number-word transport admits only dense primitive Number arrays');
  return Array.from({ length: value.length }, (_, at) => {
    const slot = Object.getOwnPropertyDescriptor(value, at);
    assert.ok(slot && Object.hasOwn(slot, 'value'), 'Number-word input has no hole or accessor');
    return encodeRustCharacterFixtureNumberWords(slot.value);
  });
}

export function readRustCharacterFixtureNumberWordCases() {
  const cases = [];
  for (const row of readRustCharacterFixtureCases()) {
    if (acceptRustCharacterFixtureArguments(row.args)) continue;
    const argumentWords = encodeRustCharacterFixtureNumberWords(row.args);
    const input = row.args.map(value => Array.isArray(value) ? Object.freeze([...value]) : value);
    const baseline = actualRoots[row.target](...input);
    assertRustCharacterFixtureExpectation(row, baseline);
    cases.push({ id: row.id, target: row.target, argumentWords,
      expected: row.expected, source: row.source, baseline });
  }
  assert.equal(cases.length, 9, 'every historical nonfinite Number descriptor gains a distinct word transport');
  return cases;
}

export function rustCharacterLiteralFixture() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  assert.equal(Buffer.byteLength(source), 5739, 'the complete original fixture authoring table is retained');
  assert.equal(digest(source), 'e0f91ac8173484e64e78c3b501e5966cb2621c46d61d70c2639879ebc0245de8');
  const kernel = readFileSync(new URL(kernelPath, root), 'utf8');
  const cases = [], javascriptOnlyCases = [];
  for (const row of readRustCharacterFixtureCases()) {
    if (!acceptRustCharacterFixtureArguments(row.args)) {
      javascriptOnlyCases.push({ id: row.id, target: row.target,
        boundary: 'Nonfinite Number input remains an explicit JavaScript-only control; no replacement Number is stored.' });
      continue;
    }
    const input = row.args.map(value => Array.isArray(value) ? Object.freeze([...value]) : value);
    const baseline = actualRoots[row.target](...input);
    assertRustCharacterFixtureExpectation(row, baseline);
    cases.push({ ...row, baseline });
  }
  assert.equal(cases.length + javascriptOnlyCases.length, 1985, 'every original expanded ordinal remains classified');
  assert.equal(cases.length, 1976);
  assert.equal(javascriptOnlyCases.length, 9);
  return {
    schemaVersion: 1, model: fixture.model, id: 'rust-character-literals',
    fixtureSource: { path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source) },
    original: fixture,
    generation: { path: kernelPath, bytes: Buffer.byteLength(kernel), sourceSha256: digest(kernel),
      baselineMeaning: 'Exact full result generated from this JavaScript kernel after the independent premise passes. This baseline is generation output, not an independent language oracle or native execution evidence.' },
    inputBoundary: 'Shared vectors are dense arrays of primitive finite Number cells. Source text uses exact charCodeAt UTF16 units; invalid finite Number cells and offsets remain explicit rejection cases. Signed zero is retained by the standard Links Number encoding. Nonfinite, foreign and hole host inputs are JavaScript-only controls.',
    expectationBoundary: 'Independent positive values and offsets, finite helper mappings, negative tags and absolute bound predicates are retained unchanged. Each additional exact baseline is derived generation output; it does not strengthen the independent negative premises.',
    cases, javascriptOnlyCases,
    numberWordTransport: { schemaVersion: 1, encoding: 'IEEE754 binary64 big-endian two unsigned hexadecimal words',
      meaning: 'DataView captures each original primitive Number as two exact eight-digit lowercase words. Historical direct-cell JavaScript-only descriptors remain unchanged; this separate transport supplies native f64::from_bits inputs. Baselines remain current-kernel generation output, not independent or prior native evidence. The historical JavaScript-only classification concerns unencoded direct-cell storage only. Number-word fixtures are generated inputs; native runtime observations require CI execution.' },
    numberWordCases: readRustCharacterFixtureNumberWordCases(),
  };
}

export function generateRustCharacterLiteralFixtures({ check = false } = {}) {
  const value = rustCharacterLiteralFixture();
  for (const file of [destination, nativeDestination]) {
    if (check) checkProjectData(file, value);
    else writeProjectData(file, value);
  }
  return value;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.ok(process.argv.slice(2).every(argument => argument === '--check'), 'only automatic generation or freshness is admitted');
  const value = generateRustCharacterLiteralFixtures({ check: process.argv.includes('--check') });
  console.log('Rust character fixtures: ' + value.cases.length + ' finite vectors; '
    + value.javascriptOnlyCases.length + ' retained nonfinite descriptors and '
    + value.numberWordCases.length + ' binary64-word vectors');
}
