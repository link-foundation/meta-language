import assert from 'node:assert/strict';
import test from 'node:test';
import { readRustCharacterSourceClosure, readRustCharacterLibraryInterface } from '../scripts/generate-rust-character-rules.mjs';

test('character generation closes every complete actual dependency without choosing a source subset', () => {
  const closed = readRustCharacterSourceClosure();
  assert.deepEqual(closed.captures.map(capture => capture.path), [
    'js/src/translation/source-text-byte-rules.js',
    'js/src/translation/rust-cooked-string-rules.js',
    'js/src/translation/rust-unicode-escape-rules.js',
    'js/src/translation/rust-source-header-emission-rules.js',
    'js/src/translation/rust-number-rules.js',
    'js/src/translation/rust-source-whitespace-rules.js',
    'js/src/translation/rust-character-rules.js',
  ]);
  assert.equal(closed.source, closed.captures.at(-1).source);
  assert.ok(closed.closure.includes('export function renderRustSourceHeaderFunction('),
    'the complete imported module retains even its unused real rendering declarations');
  assert.ok(closed.closure.includes('export function readSourceTextByte('));
  assert.doesNotMatch(closed.closure, /^\s*import\b/mu);
  for (const capture of closed.captures) {
    for (const removed of capture.removedImports) {
      assert.equal(capture.source.slice(removed.start, removed.end), removed.statement);
      assert.equal(Buffer.from(capture.source).subarray(removed.byteStart, removed.byteEnd).toString(), removed.statement);
    }
  }
});

test('character library admission checks actual immutable interfaces retained owners and source copies', () => {
  const declarations = [
    ['scalar', 'read_source_text_scalar'], ['unicode', 'decode_rust_unicode_escape'],
    ['finish', 'finish_rust_character_literal'], ['scanner', 'decode_rust_character_literal'],
  ];
  const inventory = declarations.map(([fullName]) => ({ kind: 'fn', fullName }));
  inventory.push({ kind: 'data', fullName: 'result' });
  const emitted = { mappings: declarations.map(([source, target]) => ({ kind: 'function', source, target })) };
  emitted.mappings.push({ kind: 'data', source: 'result', target: 'RustCharacterLiteral' });
  // Own synthetic source tests only the packaging boundary; it is never used
  // as a decoder or substituted for complete source generation.
  const source = 'pub enum RustCharacterLiteral { NotCharacter }\n' + declarations.map(([, name]) =>
    'pub fn ' + name + '(units: &[f64], start: f64) -> f64 { start }').join('\n');
  emitted.text = source.replaceAll('units: &[f64]', 'units: Vec<f64>');
  const files = [{ path: 'rust_character_rules/part_1.rs', source }];
  assert.equal(readRustCharacterLibraryInterface(files, emitted, inventory).interfaces.length, 4);
  const formatted = source.replaceAll('(units: &[f64], start: f64)', '(\n    units: &[f64],\n    start: f64,\n)');
  assert.equal(readRustCharacterLibraryInterface([{ ...files[0], source: formatted }], emitted, inventory).interfaces.length, 4);
  for (const malformed of [
    formatted.replace('start: f64,', 'start: f64,,'),
    formatted.replace('units: &[f64],', 'units: &[f64], ,'),
  ]) assert.throws(() => readRustCharacterLibraryInterface([{ ...files[0], source: malformed }], emitted, inventory), /no empty parameter slots/u);
  assert.throws(() => readRustCharacterLibraryInterface([{ ...files[0], source: source.replace('units: &[f64]', 'units: Vec<f64>') }], emitted, inventory), /actual argument interface|source vector is borrowed/u);
  assert.throws(() => readRustCharacterLibraryInterface([{ ...files[0], source: source.replace('start: f64', 'start: bool') }], emitted, inventory), /actual argument interface/u);
  assert.throws(() => readRustCharacterLibraryInterface([{ ...files[0], source: source.replace('RustCharacterLiteral', 'OtherResult') }], emitted, inventory), /retains the actual source owner/u);
  assert.throws(() => readRustCharacterLibraryInterface([{ ...files[0], source: source.replace('{ start }', '{ let copied = units.to_vec(); start }') }], emitted, inventory), /must not be copied/u);
  assert.throws(() => readRustCharacterLibraryInterface(files, { mappings: emitted.mappings.slice(1) }, inventory), /one actual emitted mapping/u);
});
