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
