// JS-first generation; the frozen kernel is paired natively in one later batch.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,checkProjectData,writeProjectData} from './project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
const digitPolicy=readProjectData(new URL('../src/translation/data/rust-cooked-string-escapes.lino',import.meta.url));
assert.equal(digitPolicy.model,'Rust cooked string escape 1');
assert.equal(digitPolicy.radix,16);
const policy={schemaVersion:1,model:'Rust Unicode scalar escape 1',reference:'https://doc.rust-lang.org/reference/tokens.html#character-escapes',grammar:'backslash u opening-brace (hexadecimal-digit underscore*){1..6} closing-brace',prefixUnits:[92,117,123],closingUnit:125,separatorUnit:95,minimumDigits:1,maximumDigits:6,radix:digitPolicy.radix,maximumScalar:1114111,surrogateFirst:55296,surrogateLast:57343,hexadecimalClassifier:{model:digitPolicy.model,policy:'js/src/translation/data/rust-cooked-string-escapes.lino',source:'js/src/translation/rust-cooked-string-rules.js',operation:'readRustAsciiEscapeDigit',meaning:'The existing shared operation returns one ASCII hexadecimal digit from zero through fifteen, or its negative invalid sentinel; this policy defines no second digit classifier.'},assumptions:['Inputs are original full-source UTF-16 code units with an actual Number start index; the adapter reuses its single lazy source buffer.','Only actual hexadecimal digits count toward the one-to-six limit. An underscore is permitted only after the first actual digit; any number may follow each actual digit, including before the closing brace.','The decoder scans forward once without a copied suffix or fixed escape-width cap. All reported end and escapeLength offsets are relative to the original backslash.','Well-formed surrogate and above-maximum values retain the unsupported result; malformed prefixes, leading underscores, too many actual digits, invalid units and missing braces retain the malformed result.','The existing generic decoder and JavaScript, Lean and Rocq source paths retain their prior value and refusal contracts.']};
const policyFile=new URL('../src/translation/data/rust-unicode-escapes.lino',import.meta.url);
if(process.argv.includes('--check'))checkProjectData(policyFile,policy);else writeProjectData(policyFile,policy);
const dependency="import {readRustAsciiEscapeDigit} from './rust-cooked-string-rules.js';\n";
const source='// Generated from the shared linked Rust Unicode scalar escape policy. Do not edit.\n'
 +dependency
 +'/** @typedef {{ $: "scalar", code: number, end: number } | { $: "malformed" } | { $: "unsupported", end: number, escapeLength: number }} RustUnicodeEscape */\n'
 +'/** Read one Rust Unicode escape in the original full UTF-16 source.\n * Result offsets are relative to start, the original backslash.\n * @param {number[]} units @param {number} start @returns {RustUnicodeEscape} */\n'
 +'export function decodeRustUnicodeEscape(units,start) {\n'
 +'  if (start < 0 || start >= units.length || start % 1 !== 0\n'
 +policy.prefixUnits.map((unit,index)=>`      || units[start${index?' + '+index:''}] !== ${unit}`).join('\n')+') return { $: "malformed" };\n'
 +`  let index = start + ${policy.prefixUnits.length};\n  let code = 0;\n  let digits = 0;\n`
 +'  while (index < units.length) {\n'
 +`    if (units[index] === ${policy.closingUnit}) {\n      if (digits < ${policy.minimumDigits}) return { $: "malformed" };\n`
 +'      const end = index + 1 - start;\n'
 +`      if (code > ${policy.maximumScalar} || (code >= ${policy.surrogateFirst} && code <= ${policy.surrogateLast})) return { $: "unsupported", end, escapeLength: end };\n`
 +'      return { $: "scalar", code, end };\n    }\n'
 +`    if (units[index] === ${policy.separatorUnit}) {\n      if (digits < ${policy.minimumDigits}) return { $: "malformed" };\n      index += 1;\n      continue;\n    }\n`
 +'    const digit = readRustAsciiEscapeDigit(units[index] ?? -1);\n'
 +`    if (digit < 0 || digits >= ${policy.maximumDigits}) return { $: "malformed" };\n    code = code * ${policy.radix} + digit;\n    digits += 1;\n    index += 1;\n  }\n  return { $: "malformed" };\n}\n`;
const file=new URL('../src/translation/rust-unicode-escape-rules.js',import.meta.url);
if(process.argv.includes('--check'))assert.equal(readFileSync(file,'utf8'),source,'Rust Unicode scalar escape decisions are fresh');else writeFileSync(file,source);
// Link the actual shared dependency definitions, rather than copying a second
// digit decision, for the portable checker. Native output is deferred until the
// complete JavaScript feature batch is frozen.
const shared=readFileSync(new URL('../src/translation/rust-cooked-string-rules.js',import.meta.url),'utf8');
assert.ok(source.includes(dependency));
checkProgram(parseJavaScript(shared+'\n'+source.replace(dependency,'')));
console.log('Rust Unicode scalar escapes: shared hexadecimal classifier and checked original-source decoder');
