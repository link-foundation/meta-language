import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { readProjectData, serializeProjectData, checkProjectData } from '../scripts/project-data.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { rustCharacterFixtures as original } from '../scripts/rust-character-fixture-source.mjs';
import {
  acceptRustCharacterFixtureArguments,
  assertRustCharacterFixtureExpectation,
  readRustCharacterFixtureCases,
  generateRustCharacterLiteralFixtures,
} from '../scripts/generate-rust-character-literal-fixtures.mjs';
import {
  decodeRustCharacterLiteral,
  readRustCharacterByteEscapeValue,
  readRustCharacterSimpleEscapeValue,
} from '../src/translation/rust-character-rules.js';

const file = new URL('../../parity/fixtures/rust-character-literals.lino', import.meta.url);
const sourceFile = new URL('../scripts/rust-character-fixture-source.mjs', import.meta.url);
const roots = { decodeRustCharacterLiteral, readRustCharacterByteEscapeValue, readRustCharacterSimpleEscapeValue };
const digest = source => createHash('sha256').update(source).digest('hex');

test('automatic Links character fixtures retain the whole original table and every expanded ordinal', () => {
  const value = readProjectData(file);
  const generated = generateRustCharacterLiteralFixtures({ check: true });
  assert.deepEqual(value, generated);
  assert.deepEqual(value.original, original);
  const controls = readFileSync(new URL('./translation-rust-character-rules.test.js', import.meta.url), 'utf8');
  assert.equal(digest(controls.slice(controls.indexOf('const units ='))),
    'd28465779c322394605245282af58055091a069988906a390f5e53bf0751b9c0', 'all seven original direct-control bodies and input expressions remain exact');
  const source = readFileSync(sourceFile, 'utf8');
  assert.equal(Buffer.byteLength(source), 5739);
  assert.equal(digest(source), 'e0f91ac8173484e64e78c3b501e5966cb2621c46d61d70c2639879ebc0245de8');
  assert.equal(value.fixtureSource.source, source);
  assert.equal(value.fixtureSource.sourceSha256, digest(source));
  assert.deepEqual(Object.keys(value.original), Object.keys(original));
  const all = readRustCharacterFixtureCases();
  assert.equal(all.length, 1985);
  assert.equal(value.cases.length, 1976);
  assert.equal(value.javascriptOnlyCases.length, 9);
  const retained = new Set([...value.cases, ...value.javascriptOnlyCases].map(row => row.id));
  assert.equal(retained.size, all.length);
  for (const row of all) assert.ok(retained.has(row.id), row.id);
  assert.deepEqual(value.cases.map(row => row.id), all.filter(row => acceptRustCharacterFixtureArguments(row.args)).map(row => row.id));
  assert.deepEqual(value.javascriptOnlyCases.map(row => row.id), all.filter(row => !acceptRustCharacterFixtureArguments(row.args)).map(row => row.id));
  assert.match(value.generation.baselineMeaning, /generation output, not an independent language oracle or native execution evidence/u);
  assert.throws(() => checkProjectData(file, { ...value, schemaVersion: 2 }), /stale/u);
});

test('shared character vectors preserve independent premises and generated full result boundaries separately', () => {
  const value = readProjectData(file);
  for (const row of value.cases) {
    assert.ok(acceptRustCharacterFixtureArguments(row.args), row.id);
    const before = structuredClone(row.args);
    const input = row.args.map(argument => Array.isArray(argument) ? Object.freeze([...argument]) : argument);
    const actual = roots[row.target](...input);
    assertRustCharacterFixtureExpectation(row, actual);
    assert.deepEqual(actual, row.baseline, row.id + ': exact generated baseline, not a new independent premise');
    assert.deepEqual(row.args, before, row.id + ': original numeric input is untouched');
    if (row.source !== null) {
      assert.deepEqual(row.args[0], Array.from({ length: row.source.length }, (_, at) => row.source.charCodeAt(at)), row.id);
    }
    if (row.expected.mode === 'tagAndBounds') {
      assert.equal(row.baseline.$, row.expected.tag);
      assert.ok(Number.isInteger(row.baseline.at) && row.baseline.at >= row.args[1]);
      assert.ok(Number.isInteger(row.baseline.end) && row.baseline.end >= row.baseline.at && row.baseline.end <= row.args[0].length);
      assert.equal(Object.hasOwn(row.expected, 'value'), false, 'generated exact bounds do not replace independent bounded premises');
    }
  }
  const zero = value.cases.find(row => row.id === 'negative-zero-unit');
  assert.ok(Object.is(zero.args[0][1], -0));
  assert.ok(Object.is(zero.baseline.value, -0));
  assert.ok(Object.is(value.cases.find(row => row.id === 'negative-zero-start').args[1], -0));
  const long = value.cases.find(row => row.id === 'long-unicode-separators');
  assert.equal(long.source, "'\\u{1" + '_'.repeat(32768) + "F600___}'");
  assert.equal(long.baseline.value, 128512);
  assert.equal(long.baseline.end, long.source.length);
  const wrong = { ...zero.baseline, value: 0 };
  assert.throws(() => assertRustCharacterFixtureExpectation(zero, wrong), /independent exact/u);
});

test('Links readers retain signed zero scalar boundaries and exact canonical fixture bytes', () => {
  const value = readProjectData(file);
  const text = readFileSync(file, 'utf8');
  assert.equal(serializeProjectData(value, file), text);
  assert.deepEqual(readStructuredDataLino(text), value);
  const probe = { zero: -0, positiveZero: 0, units: [39, 55296, 39], source: "'\ud800'", limit: 1114111 };
  const roundtrip = readStructuredDataLino(structuredDataToLino(probe));
  assert.deepEqual(roundtrip, probe);
  assert.ok(Object.is(roundtrip.zero, -0));
  assert.ok(Object.is(roundtrip.positiveZero, 0));
  assert.equal(roundtrip.source.charCodeAt(1), 55296);
  assert.equal(structuredDataToLino(roundtrip), structuredDataToLino(probe));
});

test('nonfinite foreign hole and accessor host controls never become paired Numbers by coercion', () => {
  const value = readProjectData(file);
  const all = readRustCharacterFixtureCases();
  for (const descriptor of value.javascriptOnlyCases) {
    const row = all.find(candidate => candidate.id === descriptor.id);
    assert.equal(acceptRustCharacterFixtureArguments(row.args), false);
    assertRustCharacterFixtureExpectation(row, roots[row.target](...row.args));
    assert.throws(() => structuredDataToLino(row.args), /finite binary64/u);
    assert.equal(Object.hasOwn(descriptor, 'args'), false, 'a nonfinite input is not stored as null or a replacement Number');
  }
  for (const cell of [undefined, null, false, true, '65', 65n, NaN, Infinity, -Infinity]) {
    assert.equal(acceptRustCharacterFixtureArguments([[39, cell, 39], 0]), false);
  }
  const hole = [39, 65, 39];
  delete hole[1];
  assert.equal(acceptRustCharacterFixtureArguments([hole, 0]), false);
  assert.throws(() => structuredDataToLino(hole), /sparse array/u);
  let reads = 0;
  const accessor = [39, 65, 39];
  Object.defineProperty(accessor, 1, { enumerable: true, get() { reads += 1; return 65; } });
  assert.equal(acceptRustCharacterFixtureArguments([accessor, 0]), false);
  assert.equal(reads, 0);
  assert.throws(() => structuredDataToLino(accessor), /non-data field/u);
  assert.equal(reads, 0);
  const extra = [39, 65, 39];
  extra.extra = 1;
  assert.equal(acceptRustCharacterFixtureArguments([extra, 0]), false);
  const foreignPrototype = [39, 65, 39];
  Object.setPrototypeOf(foreignPrototype, Object.create(Array.prototype));
  assert.equal(acceptRustCharacterFixtureArguments([foreignPrototype, 0]), false);
  assert.ok(acceptRustCharacterFixtureArguments([[39, -0, 39], 0]));
  assert.ok(acceptRustCharacterFixtureArguments([[39, 65.5, 39], -1]), 'finite malformed cells remain genuine Number rejection inputs');
});


test('automatic native character fixture copy retains the exact shared Links bytes and producer provenance', () => {
  const native = new URL('../../rust/src/data/rust-character-literals.lino', import.meta.url);
  const sharedText = readFileSync(file, 'utf8');
  const nativeText = readFileSync(native, 'utf8');
  assert.equal(nativeText, sharedText, 'the generator publishes one identical value to both actual packages');
  const shared = readProjectData(file);
  assert.deepEqual(readProjectData(native), shared);
  assert.equal(checkProjectData(native, shared), true);
  assert.throws(() => checkProjectData(native, { ...shared, generation: { ...shared.generation, sourceSha256: '0'.repeat(64) } }), /stale/u);
  for (const producer of [shared.fixtureSource, shared.generation]) {
    const actual = readFileSync(new URL('../../' + producer.path, import.meta.url));
    assert.equal(actual.length, producer.bytes, producer.path);
    assert.equal(digest(actual), producer.sourceSha256, producer.path);
  }
  assert.equal(shared.cases.length, 1976);
  assert.equal(shared.javascriptOnlyCases.length, 9);
});

// Separate additive transport; the five existing groups and direct-cell
// JavaScript-only descriptor assertions above remain unchanged.
import {
  encodeRustCharacterFixtureNumberWords,
  readRustCharacterFixtureNumberWordCases,
} from '../scripts/generate-rust-character-literal-fixtures.mjs';

function decodeRustCharacterFixtureNumberWords(value) {
  if (Array.isArray(value)) return value.map(decodeRustCharacterFixtureNumberWords);
  assert.deepEqual(Object.keys(value), ['highWord', 'lowWord']);
  assert.match(value.highWord, /^[0-9a-f]{8}$/u);
  assert.match(value.lowWord, /^[0-9a-f]{8}$/u);
  const view = new DataView(new ArrayBuffer(8));
  view.setUint32(0, Number.parseInt(value.highWord, 16), false);
  view.setUint32(4, Number.parseInt(value.lowWord, 16), false);
  return view.getFloat64(0, false);
}

test('additive IEEE754 words retain every original nonfinite Number without changing finite fixture cells', () => {
  const value = readProjectData(file);
  assert.equal(value.cases.length, 1976);
  assert.equal(value.javascriptOnlyCases.length, 9);
  assert.equal(value.numberWordCases.length, 9);
  assert.deepEqual(value.numberWordCases, readRustCharacterFixtureNumberWordCases());
  assert.deepEqual(value.numberWordCases.map(row => row.id), value.javascriptOnlyCases.map(row => row.id));
  assert.deepEqual(value.numberWordCases.map(row => row.id), [
    'invalid-start/2', 'invalid-start/3', 'invalid-start/4',
    'invalid-number-unit/1', 'invalid-number-hex/1',
    'invalid-number-unit/2', 'invalid-number-hex/2',
    'invalid-number-unit/3', 'invalid-number-hex/3',
  ]);
  const originals = readRustCharacterFixtureCases();
  const classes = { nan: 0, positiveInfinity: 0, negativeInfinity: 0 };
  for (const row of value.numberWordCases) {
    const original = originals.find(candidate => candidate.id === row.id);
    const args = decodeRustCharacterFixtureNumberWords(row.argumentWords);
    assert.deepEqual(args, original.args, row.id + ': exact original primitive Number arguments');
    assert.deepEqual(row.argumentWords, encodeRustCharacterFixtureNumberWords(original.args));
    const nonfinite = args.flat().filter(number => !Number.isFinite(number));
    assert.equal(nonfinite.length, 1);
    if (Number.isNaN(nonfinite[0])) classes.nan += 1;
    else if (nonfinite[0] === Infinity) classes.positiveInfinity += 1;
    else {
      assert.equal(nonfinite[0], -Infinity);
      classes.negativeInfinity += 1;
    }
    const actual = roots[row.target](...args);
    assertRustCharacterFixtureExpectation(original, actual);
    assert.deepEqual(actual, row.baseline, row.id + ': additional generated exact baseline only');
    assert.deepEqual(row.expected, original.expected);
    assert.equal(Object.hasOwn(value.javascriptOnlyCases.find(candidate => candidate.id === row.id), 'args'), false);
  }
  assert.deepEqual(classes, { nan: 3, positiveInfinity: 3, negativeInfinity: 3 });
  for (const [number, highWord, lowWord] of [
    [0, '00000000', '00000000'], [-0, '80000000', '00000000'],
    [1, '3ff00000', '00000000'], [Infinity, '7ff00000', '00000000'],
    [-Infinity, 'fff00000', '00000000'],
  ]) {
    const words = encodeRustCharacterFixtureNumberWords(number);
    assert.deepEqual(words, { highWord, lowWord });
    assert.ok(Object.is(decodeRustCharacterFixtureNumberWords(words), number));
  }
  assert.ok(Number.isNaN(decodeRustCharacterFixtureNumberWords(encodeRustCharacterFixtureNumberWords(NaN))));
  for (const host of [undefined, null, false, true, 'NaN', 1n, { highWord: '7ff00000', lowWord: '00000000' }]) {
    assert.throws(() => encodeRustCharacterFixtureNumberWords(host), /dense primitive Number arrays/u);
  }
  assert.throws(() => encodeRustCharacterFixtureNumberWords([39, , 39]), /dense primitive Number arrays|hole/u);
  let reads = 0;
  const accessor = [39, 65, 39];
  Object.defineProperty(accessor, 1, { enumerable: true, get() { reads += 1; return 65; } });
  assert.throws(() => encodeRustCharacterFixtureNumberWords(accessor), /accessor/u);
  assert.equal(reads, 0);
  assert.throws(() => decodeRustCharacterFixtureNumberWords({ highWord: '7ff0000', lowWord: '00000000' }));
  assert.throws(() => decodeRustCharacterFixtureNumberWords({ highWord: '7FF00000', lowWord: '00000000' }));
  assert.throws(() => decodeRustCharacterFixtureNumberWords({ highWord: '7ff00000', lowWord: '00000000', extra: 1 }));
  const text = structuredDataToLino(value.numberWordCases);
  assert.deepEqual(readStructuredDataLino(text), value.numberWordCases, 'all transport words are ordinary inspectable Links data');
});
