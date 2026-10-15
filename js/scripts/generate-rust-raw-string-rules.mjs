// Generate the frozen shared raw string decisions and their native library ABI.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { readProjectData, checkProjectData, writeProjectData } from './project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';

const scalarPolicy = readProjectData(new URL('../src/translation/data/source-text-byte-model.lino', import.meta.url));
assert.equal(scalarPolicy.model, 'Unicode scalar source text byte boundaries 1');
const policy = {
  schemaVersion: 1,
  model: 'Rust raw string literal 1',
  reference: 'https://doc.rust-lang.org/reference/tokens.html#raw-string-literals',
  inputReference: 'https://doc.rust-lang.org/reference/input-format.html',
  prefixUnit: 114,
  delimiterUnit: 35,
  quoteUnit: 34,
  minimumDelimiters: 0,
  maximumDelimiters: 255,
  carriageReturnUnit: 13,
  normalizedLineFeedUnit: 10,
  sourceLineBreak: {
    source: 'js/src/translation/rust-string-rules.js',
    operation: 'readRustSourceLineBreakEnd',
    meaning: 'Reuse the existing original-source CRLF decision; normalize each admitted pair once and refuse isolated carriage returns.',
  },
  sourceScalar: {
    model: scalarPolicy.model,
    policy: 'js/src/translation/data/source-text-byte-model.lino',
    source: 'js/src/translation/source-text-byte-rules.js',
    operation: 'readSourceTextScalar',
    supplementaryMinimum: scalarPolicy.constants.supplementaryStart,
    meaning: 'Reuse the existing exact UTF-16 scalar decision; retain supplementary pairs and refuse lone surrogates without replacement.',
  },
  result: ['notRaw', 'literal with absolute contentStart, contentEnd and end', 'tooManyDelimiters', 'carriageReturn with absolute at', 'nonScalar with absolute at', 'unterminated'],
  assumptions: [
    'Input is the original full-source UTF-16 code-unit array and the actual absolute token start; source text is never stripped or normalized before spans are assigned.',
    'Only lowercase r followed immediately by zero through 255 hashes and a quote starts this literal. Raw identifiers and byte or C string prefixes retain their existing paths.',
    'The first quote followed by the opening number of hashes closes the token; consume exactly that many hashes, leaving any extra hashes as subsequent source tokens.',
    'Every non-delimiter source unit is literal content: backslashes, apparent escapes, comments and shorter quote/hash sequences have no escape meaning.',
    'A source CRLF contributes one LF using the shared line-break decision; all other scalar content retains its original UTF-16 units, including supplementary pairs. Lone surrogates refuse without replacement.',
    'This decision adds a string value spelling, not a second string value type. The explicitly raw-excluding generated namespace model retains its refusal; models selecting decoded string values continue selecting those same values.',
  ],
};
const policyFile = new URL('../src/translation/data/rust-raw-string-literals.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(policyFile, policy);
else writeProjectData(policyFile, policy);
const fixture = {
  schemaVersion: 1,
  model: policy.model,
  literalCases: [
    { name: 'unescaped backslashes', delimiters: 0, content: String.raw`slash\n\t\x41\u{1F600}`, value: String.raw`slash\n\t\x41\u{1F600}` },
    { name: 'interior quotes and Unicode', delimiters: 1, content: 'α😀"quoted" // /* literal */\nβ', value: 'α😀"quoted" // /* literal */\nβ' },
    { name: 'shorter closing delimiter and literal continuation', delimiters: 2, content: 'a"#b"c\\\r\n  d', value: 'a"#b"c\\\n  d' },
    { name: 'control and Unicode whitespace values', delimiters: 3, content: '\0\t\u0085\u200e\u2028\ufeff', value: '\0\t\u0085\u200e\u2028\ufeff' },
    { name: 'maximum delimiter count', delimiters: policy.maximumDelimiters, content: 'max "' + '#'.repeat(policy.maximumDelimiters - 1) + ' α😀\\0', value: 'max "' + '#'.repeat(policy.maximumDelimiters - 1) + ' α😀\\0' },
  ],
  isolatedCarriageReturns: ['a\rb', '\r\r\n', '\n\r', '\\\r'],
  nonScalarUnits: [[55296], [57343], [97, 55296, 98], [55296, 55296], [57343, 55296]],
  malformed: ['r"', 'r#"', 'r##"value"#', 'r#"value"', 'r' + '#'.repeat(policy.maximumDelimiters + 1) + '"value"' + '#'.repeat(policy.maximumDelimiters + 1)],
  unchangedPrefixes: ['r#name', 'r##name', 'br"value"', 'br#"value"#', 'cr"value"', 'cr#"value"#'],
};
const fixtureFile = new URL('../../parity/fixtures/rust-raw-string-literals.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(fixtureFile, fixture);
else writeProjectData(fixtureFile, fixture);
const dependency = "import { readRustSourceLineBreakEnd } from './rust-string-rules.js';\n"
  + "import { readSourceTextScalar } from './source-text-byte-rules.js';\n";
const source = '// Generated from the shared linked Rust raw string literal policy. Do not edit.\n'
  + dependency
  + '/** @typedef {{ $: "notRaw" } | { $: "literal", contentStart: number, contentEnd: number, end: number } | { $: "tooManyDelimiters" } | { $: "carriageReturn", at: number } | { $: "nonScalar", at: number } | { $: "unterminated" }} RustRawString */\n'
  + '/** @param {number[]} units @param {number} start @returns {RustRawString} */\n'
  + 'export function decodeRustRawString(units, start) {\n'
  + `  if (start < 0 || start >= units.length || start % 1 !== 0 || units[start] !== ${policy.prefixUnit}) return { $: "notRaw" };\n`
  + '  let opening = start + 1;\n'
  + `  while (opening < units.length && units[opening] === ${policy.delimiterUnit}) opening += 1;\n`
  + `  if (units[opening] !== ${policy.quoteUnit}) return { $: "notRaw" };\n`
  + '  const delimiters = opening - start - 1;\n'
  + `  if (delimiters > ${policy.maximumDelimiters}) return { $: "tooManyDelimiters" };\n`
  + '  let cursor = opening + 1;\n'
  + '  while (cursor < units.length) {\n'
  + `    if (units[cursor] === ${policy.quoteUnit}) {\n`
  + '      let closing = cursor + 1;\n      let matched = 0;\n'
  + `      while (matched < delimiters && units[closing] === ${policy.delimiterUnit}) {\n        closing += 1;\n        matched += 1;\n      }\n`
  + '      if (matched === delimiters) return { $: "literal", contentStart: opening + 1, contentEnd: cursor, end: closing };\n    }\n'
  + `    if (units[cursor] === ${policy.carriageReturnUnit}) {\n`
  + '      const end = readRustSourceLineBreakEnd(units, cursor);\n'
  + '      if (end < 0) return { $: "carriageReturn", at: cursor };\n'
  + '      cursor = end;\n      continue;\n    }\n'
  + '    const scalar = readSourceTextScalar(units, cursor);\n'
  + '    if (scalar < 0) return { $: "nonScalar", at: cursor };\n'
  + `    cursor += scalar >= ${policy.sourceScalar.supplementaryMinimum} ? 2 : 1;\n  }\n`
  + '  return { $: "unterminated" };\n}\n';
const javascript = new URL('../src/translation/rust-raw-string-rules.js', import.meta.url);
if (process.argv.includes('--check')) assert.equal(readFileSync(javascript, 'utf8'), source, 'Rust raw-string JavaScript decisions are fresh');
else writeFileSync(javascript, source);
// Check the actual dependencies and kernel as one portable program.
const shared = readFileSync(new URL('../src/translation/rust-string-rules.js', import.meta.url), 'utf8');
const scalars = readFileSync(new URL('../src/translation/source-text-byte-rules.js', import.meta.url), 'utf8');
const checked = checkProgram(parseJavaScript(shared + '\n' + scalars + '\n' + source.replace(dependency, '')));
console.log('Rust raw strings: checked original-source decoder with shared CRLF decisions');

if (!process.argv.includes('--javascript-only')) {
  const { DecoratorSet } = await import('../src/decorators.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const { formatProductRuleSource } = await import('./product-rule-style-corrections.mjs');
  const { readSourceTextByteLibraryStyles, readSourceTextRustBlock } = await import('./source-text-byte-library-style.mjs');
  const imported = new Set(checkProgram(parseJavaScript(shared + '\n' + scalars)).items.map(item => item.fullName));
  const definitions = emitRust(checked).definitions;
  assert.equal(definitions.length, checked.items.length, 'each actual checked declaration owns its emitted definition');
  const raw = definitions.filter((_, index) => !imported.has(checked.items[index].fullName)).join('\n\n');
  assert.ok(raw.includes('pub enum RustRawString {'), 'the actual translated raw-string result is present');
  const provider = readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8');
  let adapted = raw.replaceAll('crate::read_rust_source_line_break_end', 'crate::translation::frontend_rules::read_rust_source_line_break_end')
    .replaceAll('crate::read_source_text_scalar', 'crate::translation::source_text_byte_rules::read_source_text_scalar');
  for (const match of [...adapted.matchAll(/crate::(?:RustRawString|MlDecodeRustRawString\w+|ml_decode_rust_raw_string_\w+)\b/gu)]) {
    adapted = adapted.replaceAll(match[0], match[0].replace('crate::', 'crate::translation::lexer::raw_strings::'));
  }
  // These six optional comparisons have positive nonzero finite constants;
  // bit equality has exactly the original Number equality for each constant.
  const compared = [...adapted.matchAll(/match \(crate::ml_array::optional_at\([^\n]+?, (\d+)f64\) \{ \(Some\(ml_left\), ml_right\) => ml_left == ml_right/gu)]
    .map(match => Number(match[1]));
  assert.deepEqual(compared, [policy.prefixUnit, policy.quoteUnit, policy.delimiterUnit, policy.delimiterUnit, policy.quoteUnit, policy.carriageReturnUnit],
    'the actual translated comparison constants retain their shared policy roles');
  let comparisons = 0;
  adapted = adapted.replace(/\(Some\(ml_left\), ml_right\) => ml_left == ml_right/gu, () => {
    comparisons += 1;
    return '(Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits()';
  });
  assert.equal(comparisons, 6, 'every actual optional raw delimiter comparison is represented');
  for (const value of [policy.prefixUnit, policy.delimiterUnit, policy.quoteUnit, policy.carriageReturnUnit]) {
    assert.ok(value > 0 && Number.isFinite(value), 'delimiter bit comparisons use positive finite nonzero constants');
  }
  // Both arithmetic tests preserve unordered NaN and signed-zero behavior.
  assert.ok(adapted.includes('(((start % 1f64)) != (0f64))'));
  adapted = adapted.replace('(((start % 1f64)) != (0f64))', '!matches!((start % 1f64).partial_cmp(&0f64), Some(std::cmp::Ordering::Equal))');
  assert.ok(adapted.includes('(matched_2 == delimiters)'));
  adapted = adapted.replace('(matched_2 == delimiters)', 'matches!(matched_2.partial_cmp(&delimiters), Some(std::cmp::Ordering::Equal))');
  adapted = adapted.replaceAll('mut delimiters: f64', 'delimiters: f64');
  assert.ok(adapted.includes('ml_decode_rust_raw_string_loop6(mut units: Vec<f64>, mut opening: f64,'));
  adapted = adapted.replace('ml_decode_rust_raw_string_loop6(mut units: Vec<f64>, mut opening: f64,', 'ml_decode_rust_raw_string_loop6(mut units: Vec<f64>, opening: f64,');
  // A single-constructor continuation is the same destructuring binding.
  const singleton = /match ml_s7\.clone\(\) \{\n([ \t]+)(crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop8Result::MlDecodeRustRawStringLoop8Done\(closing_2, matched_2\)) => \{([\s\S]*?)\n\1\}\n[ \t]+\}/u;
  assert.match(adapted, singleton, 'the actual inner delimiter loop has its one constructor');
  adapted = adapted.replace(singleton, 'let $2 = ml_s7;\n$3');
  assert.ok(adapted.includes('RustRawString::Literal((opening + 1f64), cursor, closing_2)'));
  adapted = adapted.replace('RustRawString::Literal((opening + 1f64), cursor, closing_2)', 'RustRawString::Literal(opening + 1f64, cursor, closing_2)');
  const array = readSourceTextRustBlock(provider, 'mod', 'ml_array');
  for (let pass = 0; pass < 2; pass += 1) {
    const library = array + '\n\n' + adapted.replace('pub use crate::translation::frontend_rules::ml_array;', '')
      .replaceAll('/// Evaluate the shared Unicode source-text byte model.\n#[must_use]\n', '');
    const correction = new DecoratorSet([{ id: 'rust-raw-string-expression-style', level: 'emitter', order: 0,
      when: [['format', 'Rust']], actions: readSourceTextByteLibraryStyles(library, provider) }]);
    adapted = correction.decorate('emitter', { format: 'Rust', source: library }).source;
  }
  adapted = adapted.replaceAll('/// Evaluate the shared Unicode source-text byte model.', '/// Read the shared original-source Rust raw string model.');
  for (const [from, to] of [
    ['MlDecodeRustRawStringLoop6Done', 'Done'], ['MlDecodeRustRawStringLoop6Return', 'Return'],
    ['MlDecodeRustRawStringJoin7Next', 'Next'], ['MlDecodeRustRawStringJoin7Return', 'Return'],
    ['MlDecodeRustRawStringLoop8Done', 'Done'],
  ]) adapted = adapted.replaceAll(from, to);
  adapted = adapted.replaceAll('pub enum MlDecodeRustRawString', 'enum MlDecodeRustRawString')
    .replaceAll('pub fn ml_decode_rust_raw_string_', 'fn ml_decode_rust_raw_string_');
  assert.ok(!/Vec<|\.clone\(|\.to_vec\(|#\[(?:allow|expect)\b/u.test(adapted), 'the full cached source remains borrowed without copies or lint suppression');
  assert.equal([...adapted.matchAll(/\bloop\s*\{/gu)].length, 3, 'all three actual scanning loops retain iterative lowering');
  const stylesFile = new URL('../../parity/self-translation/rust-raw-string-rules-decorators.lino', import.meta.url);
  const expected = new DecoratorSet([{ id: 'rust-raw-string-borrowed-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: [{ op: 'replace', field: 'source', from: raw, to: adapted }] }]).toLino();
  if (process.argv.includes('--update-styles')) writeFileSync(stylesFile, expected);
  else assert.equal(readFileSync(stylesFile, 'utf8'), expected, 'Rust raw string library adaptations are fresh');
  const decorated = DecoratorSet.fromLino(readFileSync(stylesFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const generated = '// Generated through the checked JavaScript Rust raw string decoder. Do not edit.\n' + formatProductRuleSource(decorated);
  assert.ok(generated.split('\n').length <= 1000, 'the native raw-string module remains bounded');
  const native = new URL('../../rust/src/translation/lexer/raw_strings.rs', import.meta.url);
  const copy = new URL('../../rust/src/data/rust-raw-string-literals.lino', import.meta.url);
  if (process.argv.includes('--check')) {
    assert.equal(readFileSync(native, 'utf8'), generated, 'the native Rust raw-string decoder is fresh');
    assert.ok(readFileSync(copy).equals(readFileSync(policyFile)), 'the native raw-string policy copy is identical');
  } else {
    writeFileSync(native, generated);
    writeFileSync(copy, readFileSync(policyFile));
  }
}
