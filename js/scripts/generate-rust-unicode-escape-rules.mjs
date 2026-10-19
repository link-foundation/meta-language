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

if(!process.argv.includes('--javascript-only')){
 const {DecoratorSet}=await import('../src/decorators.js');
 const {emitRust}=await import('../src/translation/emit-rust.js');
 const {formatProductRuleSource}=await import('./product-rule-style-corrections.mjs');
 const {readSourceTextByteLibraryStyles,readSourceTextRustBlock}=await import('./source-text-byte-library-style.mjs');
 const genericSource=readFileSync(new URL('../src/translation/frontend-rules.js',import.meta.url),'utf8');
 const alias=parseJavaScript(source.replace(dependency,'')).items.find(item=>item.k==='data'&&item.name==='RustUnicodeEscape');
 const canonical=parseJavaScript(genericSource).items.find(item=>item.k==='data'&&item.name==='UnicodeEscape');
 assert.ok(alias&&canonical,'both actual source result definitions are present');
 assert.deepEqual(alias.ctors,canonical.ctors,'the shared result constructors, semantic fields and ordered Number roles are identical');
 const imported=new Set(checkProgram(parseJavaScript(shared)).items.map(item=>item.fullName));
 const checked=checkProgram(parseJavaScript(shared+'\n'+source.replace(dependency,'')));
 const definitions=emitRust(checked).definitions;
 assert.equal(definitions.length,checked.items.length,'each actual checked declaration owns its emitted definition');
 const own=definitions.filter((_,index)=>!imported.has(checked.items[index].fullName));
 const raw=own.join('\n\n');
 const aliasDefinition=own.find(definition=>definition.includes('pub enum RustUnicodeEscape {'));
 assert.ok(aliasDefinition,'the translated result alias is present before library adaptation');
 const provider=readFileSync(new URL('../../rust/src/translation/frontend_rules.rs',import.meta.url),'utf8');
 const original=readSourceTextRustBlock(provider,'enum','UnicodeEscape');
 assert.equal(readSourceTextRustBlock(aliasDefinition,'enum','RustUnicodeEscape').replace('RustUnicodeEscape','UnicodeEscape'),original,'the existing native result has the same actual variant field layout');
 const resultActions=[{op:'replace',field:'source',from:aliasDefinition,to:''},{op:'replace',field:'source',from:'crate::RustUnicodeEscape',to:'crate::translation::frontend_rules::UnicodeEscape'}];
 let adapted=raw.replace(aliasDefinition,'').replaceAll('crate::RustUnicodeEscape','crate::translation::frontend_rules::UnicodeEscape');
 const sharedResult=adapted;
 adapted=adapted.replaceAll('crate::read_rust_ascii_escape_digit','crate::translation::lexer::cooked_strings::read_rust_ascii_escape_digit');
 for(const name of ['MlDecodeRustUnicodeEscapeLoop1Result','ml_decode_rust_unicode_escape_loop1'])adapted=adapted.replaceAll('crate::'+name,'crate::translation::lexer::unicode_escapes::'+name);
 // Optional array reads preserve None and every Number input, including NaN.
 // Only the five positive nonzero source delimiters use bit equality.
 let comparisons=0;
 const compared=[...policy.prefixUnits,policy.closingUnit,policy.separatorUnit];
 adapted=adapted.replace(/\(Some\(ml_left\), ml_right\) => ml_left == ml_right/gu,()=>{comparisons++;return '(Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits()';});
 assert.equal(comparisons,compared.length,'all five actual optional delimiter comparisons are represented');
 for(const value of compared)assert.ok(value>0&&Number.isFinite(value),'bit comparisons have positive finite nonzero constants');
 adapted=adapted.replaceAll('mut start: f64','start: f64');
 // Partial comparison retains NaN != 0 and both signed-zero equalities.
 assert.ok(adapted.includes('(((start % 1f64)) != (0f64))'));
 adapted=adapted.replace('(((start % 1f64)) != (0f64))','!matches!((start % 1f64).partial_cmp(&0f64), Some(std::cmp::Ordering::Equal))');
 // Keep multiplication and addition separately rounded, as in JavaScript.
 assert.ok(adapted.includes('((code * 16f64) + digit)'));
 adapted=adapted.replace('((code * 16f64) + digit)','{ let scaled = code * 16f64; scaled + digit }');
 const array=readSourceTextRustBlock(provider,'mod','ml_array');
 for(let pass=0;pass<2;pass++){
  const library=array+'\n\n'+adapted.replace('pub use crate::translation::frontend_rules::ml_array;','').replaceAll('/// Evaluate the shared Unicode source-text byte model.\n#[must_use]\n','');
  const correction=new DecoratorSet([{id:'rust-unicode-expression-style',level:'emitter',order:0,when:[['format','Rust']],actions:readSourceTextByteLibraryStyles(library,provider)}]);
  adapted=correction.decorate('emitter',{format:'Rust',source:library}).source;
 }
 adapted=adapted.replaceAll('/// Evaluate the shared Unicode source-text byte model.','/// Decode the shared Rust Unicode scalar escape model.')
  .replaceAll('MlDecodeRustUnicodeEscapeLoop1Done','Done').replaceAll('MlDecodeRustUnicodeEscapeLoop1Return','Return')
  .replace('pub enum MlDecodeRustUnicodeEscapeLoop1Result','enum MlDecodeRustUnicodeEscapeLoop1Result')
  .replace('pub fn ml_decode_rust_unicode_escape_loop1','fn ml_decode_rust_unicode_escape_loop1');
 adapted=adapted.replace('(code >= 55_296f64) && (code <= 57_343f64)','(55_296f64..=57_343f64).contains(&code)');
 assert.ok(!/Vec<|\.clone\(|\.to_vec\(|crate::RustUnicodeEscape|pub enum RustUnicodeEscape/u.test(adapted),'cached source data and the canonical result are reused without copies');
 assert.match(adapted,/loop\s*\{/u,'the translated scanner retains its actual iterative lowering');
 const stylesFile=new URL('../../parity/self-translation/rust-unicode-escape-rules-decorators.lino',import.meta.url);
 const expected=new DecoratorSet([{id:'rust-unicode-shared-result-type',level:'emitter',order:0,when:[['format','Rust']],actions:resultActions},{id:'rust-unicode-borrowed-library-style',level:'emitter',order:1,when:[['format','Rust']],actions:[{op:'replace',field:'source',from:sharedResult,to:adapted}]}]).toLino();
 if(process.argv.includes('--update-styles'))writeFileSync(stylesFile,expected);else assert.equal(readFileSync(stylesFile,'utf8'),expected,'Rust Unicode library adaptations are fresh');
 const decorated=DecoratorSet.fromLino(readFileSync(stylesFile,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source;
 const generated='// Generated through the checked JavaScript Rust Unicode decoder. Do not edit.\n'+formatProductRuleSource(decorated);
 const native=new URL('../../rust/src/translation/lexer/unicode_escapes.rs',import.meta.url),copy=new URL('../../rust/src/data/rust-unicode-escapes.lino',import.meta.url);
 if(process.argv.includes('--check')){assert.equal(readFileSync(native,'utf8'),generated,'the native Rust Unicode decoder is fresh');assert.ok(readFileSync(copy).equals(readFileSync(policyFile)),'the native Unicode policy copy is identical');}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
}
