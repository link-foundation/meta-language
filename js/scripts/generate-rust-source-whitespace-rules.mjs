// Generate the JavaScript-first Rust source whitespace decisions from links data.
import { readFileSync, writeFileSync } from 'node:fs';
import { checkProjectData, writeProjectData } from './project-data.mjs';

const policy = {
  schemaVersion: 1,
  model: 'Rust source pattern whitespace 1',
  sources: [
    'https://doc.rust-lang.org/reference/whitespace.html',
    'https://doc.rust-lang.org/reference/input-format.html#byte-order-mark-removal',
  ],
  sourceRules: ['whitespace.syntax', 'input.byte-order-mark'],
  codeUnits: [9, 10, 11, 12, 13, 32, 133, 8206, 8207, 8232, 8233],
  byteOrderMark: { codeUnit: 65279, position: 'first source code unit only', count: 1 },
  numberBoundary: {
    originalOperation: 'acceptRustNumberDigitBoundary',
    whitespaceOperation: 'acceptRustSourceWhitespace',
    canonicalCodeUnit: 32,
    interpretation: 'Preserve every original numeric boundary result. A proven Rust whitespace boundary may additionally reuse that same original decision with the canonical ASCII space; no other following character gains admission.',
  },
  sourceSpans: 'Original UTF-16 offsets remain unchanged; neither whitespace nor the initial byte order mark is removed from the source string.',
  scope: 'Rust token separation only. Other languages keep their existing whitespace rules. Literal content, comment boundaries, attributes, shebangs and other input transformations retain their separate interpretations.',
};
const policyFile = new URL('../src/translation/data/rust-source-whitespace.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(policyFile, policy);
else writeProjectData(policyFile, policy);

const source = '// Generated from the linked Rust source whitespace policy. Do not edit.\n'
  + "import { acceptRustNumberDigitBoundary } from './rust-number-rules.js';\n"
  + '/** @param {number} unit @returns {boolean} */\n'
  + 'export function acceptRustSourceWhitespace(unit) {\n'
  + '  return ' + policy.codeUnits.map(unit => `unit === ${unit}`).join(' || ') + ';\n}\n'
  + '/** @param {number} unit @param {boolean} firstPosition @returns {boolean} */\n'
  + 'export function acceptRustSourceByteOrderMark(unit, firstPosition) {\n'
  + `  return firstPosition && unit === ${policy.byteOrderMark.codeUnit};\n}\n`
  + '/** @param {boolean} floating @param {boolean} trailingSeparator @param {string} suffix @param {boolean} continuedSuffix @param {number} followingCodeUnit @returns {boolean} */\n'
  + 'export function acceptRustSourceNumberDigitBoundary(floating, trailingSeparator, suffix, continuedSuffix, followingCodeUnit) {\n'
  + '  return acceptRustNumberDigitBoundary(floating, trailingSeparator, suffix, continuedSuffix, followingCodeUnit)\n'
  + '    || (acceptRustSourceWhitespace(followingCodeUnit)\n'
  + `      && acceptRustNumberDigitBoundary(floating, trailingSeparator, suffix, continuedSuffix, ${policy.numberBoundary.canonicalCodeUnit}));\n}\n`;
const javascript = new URL('../src/translation/rust-source-whitespace-rules.js', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(javascript, 'utf8') !== source) throw new Error('Rust source whitespace rules are stale');
} else writeFileSync(javascript, source);
console.log('Rust source whitespace: eleven exact code units and one initial byte order mark');

if (!process.argv.includes('--javascript-only')) {
  const { default: assert } = await import('node:assert/strict');
  const { DecoratorSet } = await import('../src/decorators.js');
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const { formatProductRuleSource } = await import('./product-rule-style-corrections.mjs');
  const { readSourceTextByteLibraryStyles, readSourceTextRustBlock } = await import('./source-text-byte-library-style.mjs');
  const dependency = "import { acceptRustNumberDigitBoundary } from './rust-number-rules.js';\n";
  assert.ok(source.includes(dependency));
  const shared = readFileSync(new URL('../src/translation/rust-number-rules.js', import.meta.url), 'utf8');
  const imported = new Set(checkProgram(parseJavaScript(shared)).items.map(item => item.fullName));
  const checked = checkProgram(parseJavaScript(shared + '\n' + source.replace(dependency, '')));
  const definitions = emitRust(checked).definitions;
  assert.equal(definitions.length, checked.items.length, 'each checked declaration owns its emitted definition');
  const raw = definitions.filter((_, index) => !imported.has(checked.items[index].fullName)).join('\n\n');
  let adapted = raw.replaceAll('crate::accept_rust_number_digit_boundary', 'crate::translation::lexer::number_rules::accept_rust_number_digit_boundary')
    .replaceAll('crate::accept_rust_source_whitespace', 'crate::translation::lexer::source_whitespace::accept_rust_source_whitespace')
    .replaceAll('suffix: String', 'suffix: &str').replaceAll('suffix.clone()', 'suffix');
  let comparisons = 0;
  const compared = [...policy.codeUnits, policy.byteOrderMark.codeUnit];
  adapted = adapted.replace(/unit == \((\d+f64)\)/gu, (_, literal) => {
    const unit = Number(literal.slice(0, -3));
    assert.ok(unit > 0 && compared.includes(unit), 'every bit comparison preserves its positive finite Number equality');
    comparisons += 1;
    return `unit.to_bits() == ${literal}.to_bits()`;
  });
  assert.equal(comparisons, compared.length, 'all eleven whitespace and one byte-order-mark comparisons are represented');
  const provider = readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8');
  const library = readSourceTextRustBlock(provider, 'mod', 'ml_array') + '\n\n' + adapted;
  const corrections = new DecoratorSet([{ id: 'rust-source-whitespace-expression-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: readSourceTextByteLibraryStyles(library, provider) }]);
  adapted = corrections.decorate('emitter', { format: 'Rust', source: library }).source
    .replace('pub use crate::translation::frontend_rules::ml_array;', '')
    .replaceAll('/// Evaluate the shared Unicode source-text byte model.', '/// Read the shared Rust source whitespace decision.')
    .replace('pub fn accept_rust_source_whitespace(', 'pub const fn accept_rust_source_whitespace(')
    .replace('pub fn accept_rust_source_byte_order_mark(', 'pub const fn accept_rust_source_byte_order_mark(');
  const stylesFile = new URL('../../parity/self-translation/rust-source-whitespace-rules-decorators.lino', import.meta.url);
  const expected = new DecoratorSet([{ id: 'rust-source-whitespace-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: [{ op: 'replace', field: 'source', from: raw, to: adapted }] }]).toLino();
  if (process.argv.includes('--update-styles')) writeFileSync(stylesFile, expected);
  else assert.equal(readFileSync(stylesFile, 'utf8'), expected, 'Rust source whitespace library styles are fresh');
  const decorated = DecoratorSet.fromLino(readFileSync(stylesFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const generated = '// Generated through the checked JavaScript source whitespace decisions. Do not edit.\n' + formatProductRuleSource(decorated);
  const native = new URL('../../rust/src/translation/lexer/source_whitespace.rs', import.meta.url);
  const copy = new URL('../../rust/src/data/rust-source-whitespace.lino', import.meta.url);
  if (process.argv.includes('--check')) {
    assert.equal(readFileSync(native, 'utf8'), generated, 'Rust source whitespace native decisions are fresh');
    assert.ok(readFileSync(copy).equals(readFileSync(policyFile)), 'the copied source whitespace policy is identical');
  } else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(policyFile)); }
}
