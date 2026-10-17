import assert from 'node:assert/strict';
import test from 'node:test';
import { readRustCharacterSourceClosure, readRustCharacterLibraryInterface, readRustCharacterLibraryStyles } from '../scripts/generate-rust-character-rules.mjs';
import { DecoratorSet } from '../src/decorators.js';

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

function reviewCharacterLibraryStyle(source) {
  const reviewed = readRustCharacterLibraryStyles(source);
  const styles = new DecoratorSet(reviewed.actions.length ? [{id: 'character-library-style-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: reviewed.actions}] : []);
  assert.equal(styles.decorate('emitter', {format: 'Rust', source}).source, reviewed.source);
  const inverse = reviewed.actions.slice().reverse().map(action => ({...action, from: action.to, to: action.from}));
  const restored = new DecoratorSet(inverse.length ? [{id: 'character-library-inverse-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: inverse}] : []);
  assert.equal(restored.decorate('emitter', {format: 'Rust', source: reviewed.source}).source, source);
  assert.deepEqual(readRustCharacterLibraryStyles(reviewed.source), {source: reviewed.source, actions: []});
  return reviewed.source;
}

test('character library boxed returns require the actual owned primitive enum and complete callee signature', () => {
  // Own syntax probes exercise packaging only, never replace a decoder.
  const declarations = '#[derive(Clone, Debug, PartialEq)] pub enum RustUnicodeEscape { Scalar(f64, f64), Malformed, Unsupported(f64, f64), }\n'
    + '#[derive(Clone, Debug, PartialEq)] pub enum MlDecodeRustUnicodeEscapeLoop4Result { MlDecodeRustUnicodeEscapeLoop4Done, MlDecodeRustUnicodeEscapeLoop4Return(Box<self::RustUnicodeEscape>), }\n'
    + 'pub fn selected(units: &[f64]) -> self::MlDecodeRustUnicodeEscapeLoop4Result { self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Malformed)) }\n';
  const body = 'pub fn decode_rust_unicode_escape(units: &[f64]) -> self::RustUnicodeEscape { let state = self::selected(units); '
    + 'match state { self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Done => { self::RustUnicodeEscape::Malformed } '
    + 'self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Return(payload) => { let payload = *payload; payload.clone() } } }';
  const source = declarations + body;
  assert.equal(reviewCharacterLibraryStyle(source), source.replace('let payload = *payload; payload.clone()', '*payload'));
  for (const refused of [
    source.replace(') -> self::MlDecodeRustUnicodeEscapeLoop4Result', ') -> &self::MlDecodeRustUnicodeEscapeLoop4Result'),
    source.replace('Box<self::RustUnicodeEscape>', '&self::RustUnicodeEscape'),
    source.replace('Scalar(f64, f64)', 'Scalar(String, f64)'),
    source.replace('match state', 'match &state'),
    source.replace('payload.clone()', 'payload./* retain */clone()'),
    source.replace('let payload = *payload; payload.clone()', 'let payload = *payload; let copied = payload.clone(); self::observe(payload); copied'),
    source.replace('match state', 'self::observe(state); match state'),
    'pub struct Box;\n' + source,
  ]) assert.equal(reviewCharacterLibraryStyle(refused), refused);
});

test('character library formats borrow only actual str parameters and reuse the shared header flattening proof', () => {
  const source = 'pub fn render_rust_source_header_parameter(name: &str, type_: &str) -> String { format!("{}: {}", name.to_owned(), type_.to_owned()) }';
  assert.equal(reviewCharacterLibraryStyle(source), source.replace('format!("{}: {}", name.to_owned(), type_.to_owned())', 'format!("{name}: {type_}")'));
  const braces = 'pub fn render_rust_source_header_copy(name: &str, inner: &str) -> String { format!("let {} = {{ {} }};", name.to_owned(), inner.to_owned()) }';
  assert.equal(reviewCharacterLibraryStyle(braces), braces.replace('format!("let {} = {{ {} }};", name.to_owned(), inner.to_owned())', 'format!("let {name} = {{ {inner} }};")'));
  for (const refused of [
    source.replaceAll('&str', 'String'),
    source.replace('{ format!', '{ let name = custom(); format!'),
    source.replace('name.to_owned()', 'name./* retain */to_owned()'),
    source.replace('format!("{}: {}", name.to_owned(), type_.to_owned())', 'format!("{:?}", name.to_owned())'),
    source.replace('render_rust_source_header_parameter', 'unrelated'),
    'pub type str = Custom;\n' + source,
  ]) assert.equal(reviewCharacterLibraryStyle(refused), refused);
});

test('character library const qualification proves the finite primitive dependency closure without changing bodies', () => {
  const definitions = [
    'pub fn read_rust_character_byte_escape_value(high_unit: f64, low_unit: f64) -> f64 { let high = self::read_rust_ascii_escape_digit(high_unit); let low = self::read_rust_ascii_escape_digit(low_unit); if (high < 0f64) || (low < 0f64) { -1f64 } else { (high * 16f64) + low } }',
    'pub fn read_rust_ascii_escape_value(high_unit: f64, low_unit: f64) -> f64 { let high = self::read_rust_ascii_escape_digit(high_unit); let low = self::read_rust_ascii_escape_digit(low_unit); if ((high < 0f64) || (high > 7f64)) || (low < 0f64) { -1f64 } else { (high * 16f64) + low } }',
    'pub fn accept_rust_cooked_string_escape_prefix(unit: f64) -> bool { (unit == 34f64) || (unit == 39f64) }',
    'pub fn read_rust_ascii_escape_digit(unit: f64) -> f64 { if unit == 48f64 { 0f64 } else if unit == 49f64 { 1f64 } else { -1f64 } }',
    'pub fn accept_rust_source_whitespace(unit: f64) -> bool { (unit == 9f64) || (unit == 10f64) }',
    'pub fn accept_rust_source_byte_order_mark(unit: f64, first_position: bool) -> bool { first_position && (unit == 65279f64) }',
    'pub fn read_rust_character_simple_escape_value(unit: f64) -> f64 { if unit == 110f64 { 10f64 } else if unit == 39f64 { unit } else { -1f64 } }',
  ];
  const source = definitions.join('\n');
  assert.equal(reviewCharacterLibraryStyle(source), definitions.map(definition => definition.replace('pub fn', 'pub const fn')).join('\n'));
  for (const refused of [
    definitions[2].replace('unit: f64', 'unit: Custom'),
    definitions[2].replace('(unit == 34f64) || (unit == 39f64)', '(0f64..=127f64).contains(&unit)'),
    definitions[2].replace('(unit == 34f64) || (unit == 39f64)', 'self::unknown(unit)'),
    definitions[2].replace('unit: f64', 'unit: &str').replace('(unit == 34f64) || (unit == 39f64)', 'unit == "x"'),
    definitions[0],
    'pub type f64 = Custom;\n' + source,
  ]) assert.equal(reviewCharacterLibraryStyle(refused), refused);
  const unavailable = source.replace('if unit == 48f64', 'if self::unknown(unit) == 48f64');
  const result = reviewCharacterLibraryStyle(unavailable);
  for (const definition of definitions.slice(0, 2)) assert.ok(result.includes(definition), 'a nonconstant actual digit owner keeps both callers nonconstant');
});
