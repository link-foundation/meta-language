// Rust-specific cooked escapes; the generic Unicode decoder retains its scope.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';

const policy = {
  schemaVersion: 1,
  model: 'Rust cooked string escape 1',
  reference: 'https://doc.rust-lang.org/reference/tokens.html#character-escapes',
  prefixUnits: [34, 39, 48, 92, 110, 114, 116, 117, 120],
  hexadecimalRanges: [[48, 57, 0], [65, 70, 10], [97, 102, 10]],
  radix: 16,
  maximumHighDigit: 7,
  invalid: -1,
  consumedAsciiUnits: 4,
  nativeComparison: 'Every compared code unit is a positive finite nonzero binary64 constant. Numeric equality therefore equals bit equality for every Number input, including both zero signs and all NaNs.',
  assumptions: [
    'The adapter projects original UTF-16 source units; an unavailable unit is represented by the negative sentinel.',
    'After backslash and lowercase x, consume exactly two hexadecimal digits and admit only seven-bit ASCII values.',
    'Continuation recognition precedes the prefix decision; Unicode decoding retains the existing separately checked scalar domain.',
    'Source CRLF normalizes through the existing shared line-break decision; an isolated source carriage return refuses while escaped carriage return remains a value.',
    'Raw, byte and C literals and Unicode escape separators are outside this change. Other language lexers retain their existing rules.',
  ],
};
const policyFile = new URL('../src/translation/data/rust-cooked-string-escapes.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(policyFile, policy);
else writeProjectData(policyFile, policy);
const digits = new Map();
for (const [first, last, offset] of policy.hexadecimalRanges) {
  for (let unit = first; unit <= last; unit += 1) {
    const digit = unit - first + offset;
    if (!digits.has(digit)) digits.set(digit, []);
    digits.get(digit).push(unit);
  }
}
const source = '// Generated from the shared linked Rust cooked-string escape policy. Do not edit.\n'
  + '/** @param {number} unit @returns {boolean} */\n'
  + 'export function acceptRustCookedStringEscapePrefix(unit) {\n  return '
  + policy.prefixUnits.map(unit => `unit === ${unit}`).join(' || ') + ';\n}\n\n'
  + '/** @param {number} unit @returns {number} */\n'
  + 'export function readRustAsciiEscapeDigit(unit) {\n'
  + [...digits].map(([digit, units]) => '  if (' + units.map(unit => `unit === ${unit}`).join(' || ') + `) return ${digit};\n`).join('')
  + `  return ${policy.invalid};\n}\n\n`
  + '/** @param {number} highUnit @param {number} lowUnit @returns {number} */\n'
  + 'export function readRustAsciiEscapeValue(highUnit, lowUnit) {\n'
  + '  const high = readRustAsciiEscapeDigit(highUnit);\n  const low = readRustAsciiEscapeDigit(lowUnit);\n'
  + `  if (high < 0 || high > ${policy.maximumHighDigit} || low < 0) return ${policy.invalid};\n`
  + `  return high * ${policy.radix} + low;\n}\n`;
const javascript = new URL('../src/translation/rust-cooked-string-rules.js', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(javascript, 'utf8') !== source) throw new Error('Rust cooked-string JavaScript decisions are stale');
} else writeFileSync(javascript, source);
// Always check the whole portable kernel, including during the JS-first draft.
const checked = checkProgram(parseJavaScript(source));
if (!process.argv.includes('--javascript-only')) {
  const raw = emitRust(checked).definitions.join('\n\n');
  let adapted = raw.replaceAll('crate::read_rust_ascii_escape_digit', 'crate::translation::lexer::cooked_strings::read_rust_ascii_escape_digit');
  let comparisons = 0;
  const comparedUnits = [...policy.prefixUnits, ...[...digits.values()].flat()];
  adapted = adapted.replace(/unit == \((\d+f64)\)/gu, (comparison, literal) => {
    const unit = Number(literal.slice(0, -3));
    if (!(unit > 0) || !comparedUnits.includes(unit)) throw new Error('Unexpected Rust cooked-string numeric equality');
    comparisons += 1;
    return `unit.to_bits() == ${literal}.to_bits()`;
  });
  if (comparisons !== comparedUnits.length) throw new Error('Rust cooked-string equality inventory changed');
  adapted = formatProductRuleSource(adapted).replace(/^pub fn /gmu, '/// Read the corresponding shared Rust cooked-string escape decision.\n#[must_use]\npub const fn ');
  const stylesFile = new URL('../../parity/self-translation/rust-cooked-string-rules-decorators.lino', import.meta.url);
  const styles = new DecoratorSet([{ id: 'rust-cooked-string-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: [{ op: 'replace', field: 'source', from: raw, to: adapted }] }]).toLino();
  if (process.argv.includes('--update-styles')) writeFileSync(stylesFile, styles);
  else if (readFileSync(stylesFile, 'utf8') !== styles) throw new Error('Rust cooked-string library styles are stale');
  const decorated = DecoratorSet.fromLino(readFileSync(stylesFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const generated = '// Generated by the checked JavaScript cooked-string decisions. Do not edit.\n' + formatted.stdout;
  const native = new URL('../../rust/src/translation/lexer/cooked_strings.rs', import.meta.url);
  const copy = new URL('../../rust/src/data/rust-cooked-string-escapes.lino', import.meta.url);
  if (process.argv.includes('--check')) {
    if (readFileSync(native, 'utf8') !== generated || !readFileSync(copy).equals(readFileSync(policyFile))) throw new Error('Rust cooked-string native decisions or linked policy are stale');
  } else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(policyFile)); }
}
console.log('Rust cooked strings: checked prefix and seven-bit ASCII decisions');
