// JavaScript-first raw string decisions; native generation follows the batch freeze.
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
// Check the actual dependency and kernel as one portable program. This phase
// intentionally writes no native module, native policy copy or Rust styles.
const shared = readFileSync(new URL('../src/translation/rust-string-rules.js', import.meta.url), 'utf8');
const scalars = readFileSync(new URL('../src/translation/source-text-byte-rules.js', import.meta.url), 'utf8');
checkProgram(parseJavaScript(shared + '\n' + scalars + '\n' + source.replace(dependency, '')));
console.log('Rust raw strings: checked original-source decoder with shared CRLF decisions');
