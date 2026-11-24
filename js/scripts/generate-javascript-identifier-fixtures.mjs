// Source declarations only. Never lex, parse, execute or normalize these inputs.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
const bodies = [
  ['fixed escaped binding', String.raw`const \u0061=42;return a;`, '42'],
  ['escaped continuation', String.raw`const v\u0061lue=43;return value;`, '43'],
  ['astral escaped binding', String.raw`const \u{10400}=44;return 𐐀;`, '44'],
  ['long leading zero escape', String.raw`const \u{000000000000000061}=45;return a;`, '45'],
  ['canonical spellings remain distinct', String.raw`const e\u0301=3,é=43;return é+é;`, '46'],
  ['escaped joiner continuation', String.raw`const a\u200Db=47;return a‍b;`, '47'],
  ['escaped dollar', String.raw`const \u0024value=48;return $value;`, '48'],
  ['escaped underscore', String.raw`const \u005fvalue=49;return _value;`, '49'],
  ['reserved property name', String.raw`const item={\u0069f:50};return item.\u0069f;`, '50'],
  ['reserved function property name', String.raw`const item={\u0066unction:51};return item.\u0066unction;`, '51'],
  ['reserved method name', String.raw`const item={\u0069f(){return 52;}};return item.\u0069f();`, '52'],
  ['contextual async binding', String.raw`const \u0061sync=53;return async;`, '53'],
  ['escaped of remains division operand', String.raw`const \u006ff=108;return \u006ff/2;`, '54'],
  ['escaped as remains division operand', String.raw`const \u0061s=110;return \u0061s/2;`, '55'],
  ['contextual get binding', String.raw`const \u0067et=56;return get;`, '56'],
  ['escaped arrow parameter', String.raw`const read=\u0076alue=>value+1;return read(58);`, '59'],
  ['escaped array binding', String.raw`const [\u0061]=[60];return a;`, '60'],
  ['escaped object binding', String.raw`const {value:\u0061}={value:61};return a;`, '61'],
  ['escaped builtin owner', String.raw`return \u004dath.floor(62.9);`, '62'],
  ['escaped property with shorthand value', String.raw`const \u0061=64;const item={a};return item.\u0061;`, '64'],
];
const cases = bodies.map(([name, body, value], index) => ({ name, entry: 'escapedIdentifierCase' + index,
  source: 'export function compute(){' + body + '}', value, abort: null }));
cases.push({ name: 'escaped source function', entry: 'escapedIdentifierFunction', value: '57', abort: null,
  source: String.raw`function \u0065scapedFunction(value){return value;} export function compute(){return escapedFunction(57);}`.replace(' export ', '\nexport ') });
cases.push({ name: 'escaped ordinary parameter', entry: 'escapedIdentifierParameter', value: '58', abort: null,
  source: String.raw`function readEscapedParameter(\u0076alue){return value;} export function compute(){return readEscapedParameter(58);}`.replace(' export ', '\nexport ') });
const reserved = [
  ['constant', String.raw`export function compute(){const \u0069f=1;return 1;}`, 'if'],
  ['function', String.raw`function \u0069f(){return 1;} export function compute(){return 1;}`, 'if'],
  ['parameter', String.raw`function read(\u0069f){return 1;} export function compute(){return 1;}`, 'if'],
  ['arrow parameter', String.raw`export function compute(){const read=\u0069f=>1;return read(1);}`, 'if'],
  ['array binding', String.raw`export function compute(){const [\u0069f]=[1];return 1;}`, 'if'],
  ['object binding', String.raw`export function compute(){const {value:\u0069f}={value:1};return 1;}`, 'if'],
  ['record parameter', String.raw`function read({value:\u0069f}){return 1;} export function compute(){return 1;}`, 'if'],
  ['Boolean literal escape', String.raw`export function compute(){return \u0074rue;}`, 'true'],
  ['module await binding', String.raw`export function compute(){const \u0061wait=1;return 1;}`, 'await'],
  ['strict yield binding', String.raw`export function compute(){const \u0079ield=1;return 1;}`, 'yield'],
  ['property shorthand', String.raw`export function compute(){return {\u0069f};}`, 'if'],
].map(([name, source, word]) => ({ name, source, kind: 'syntax', reason: 'escaped JavaScript identifier is reserved: ' + word }));
const strict = ['eval', 'arguments'].map(name => ({ name: 'escaped strict ' + name,
  source: 'export function compute(){const \\u' + name.charCodeAt(0).toString(16).padStart(4, '0') + name.slice(1) + '=1;return 1;}',
  kind: 'syntax', reason: 'invalid strict JavaScript binding identifier: ' + name }));
const lexicalErrors = [
  [String.raw`\u0030`, 0, 6], [String.raw`a\u002D`, 1, 7], [String.raw`\uD801\uDC00`, 0, 6],
  [String.raw`\u{110000}`, 0, 9], [String.raw`\u{}`, 0, 4], [String.raw`\u00zz`, 0, 5],
  [String.raw`\x61`, 0, 2], [String.raw`\u{D800}`, 0, 8], [String.raw`\u{61`, 0, 5],
].map(([source, start, end]) => ({ source, kind: 'syntax', reason: 'invalid Unicode escape in a JavaScript identifier', span: { start, end } }));
const numericErrors = [['1é', 2], ['1𐐀', 3], [String.raw`1\u0061`, 2]].map(([source, end]) => ({ source,
  kind: 'syntax', reason: 'malformed number ' + source.slice(0, end), span: { start: 0, end } }));
const lexicalCases = [
  [String.raw`\u0061`, 'a', true], [String.raw`v\u0061lue`, 'value', true], [String.raw`\u{10400}`, '𐐀', true],
  [String.raw`\u{000000000061}`, 'a', true], [String.raw`\u0061sync`, 'async', false], [String.raw`\u006ff`, 'of', false],
  [String.raw`\u0069f`, 'if', false], [String.raw`a\u200D`, 'a‍', true], [String.raw`e\u0301`, 'é', true],
].map(([source, value, matchesValue]) => ({ source, tokens: [{ kind: 'identifier', value, raw: source, start: 0, end: source.length, matchesValue }] }));
const versions = { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript module/strict goal',
  Rust: '>=1.90.0 <1.100.0; edition 2024; source characters shared by Unicode 16/17' };
writeProjectData(new URL('../../parity/fixtures/javascript-escaped-identifiers.lino', import.meta.url), {
  schemaVersion: 1, model: 'JavaScript Unicode identifier escapes and strict module roles 1', sourceLanguage: 'JavaScript', sourceSyntaxOracle: true,
  cases, refusals: [...reserved, ...strict], lexicalCases, lexicalErrors: [...lexicalErrors, ...numericErrors], versions,
  source: cases.map(row => row.source.replace('function compute(', 'function ' + row.entry + '(')).join('\n'),
  sourceBasis: 'https://tc39.es/ecma262/2025/multipage/ecmascript-language-lexical-grammar.html#sec-names-and-keywords ; https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-identifiers-static-semantics-early-errors',
  boundary: 'Future strict original Node/emitted JavaScript/native Rust values, independent original module syntax errors, exact token spelling/value/span/cursor and errors, definition/program graphs, codecs and all four emissions. No observations. Script/sloppy/generator contexts and all inherited full-language requirements remain separate obligations.' });
const lifetimeInputs = [
  ['Greek lifetime', "fn read<'α>(value:&'α i64)->i64{*value} pub fn result()->i64{read(&65)}", '65'],
  ['Cyrillic lifetime', "fn read<'имя>(value:&'имя i64)->i64{*value} pub fn result()->i64{read(&66)}", '66'],
  ['astral lifetime', "fn read<'𐐀>(value:&'𐐀 i64)->i64{*value} pub fn result()->i64{read(&67)}", '67'],
  ['canonical lifetime spelling', "fn read<'é>(value:&'é i64)->i64{*value} pub fn result()->i64{read(&68)}", '68'],
  ['Unicode inherent receiver lifetime', "struct Point{x:i64} impl Point{fn read<'α>(&'α self)->i64{self.x}} pub fn result()->i64{Point{x:69}.read()}", '69'],
];
const lifetimeCases = lifetimeInputs.map(([name, source, value], index) => ({ name, module: 'unicode_lifetime_case_' + index, source, value, abort: null }));
const lifetimeTokens = ["'α", "'é", "'𐐀", "'aα"].map(source => ({ source,
  tokens: [{ raw: "'", kind: 'punct', value: "'", start: 0, end: 1, matchesValue: true },
    { raw: source.slice(1), kind: 'identifier', value: source === "'é" ? 'é' : source.slice(1), start: 1, end: source.length, matchesValue: true }] }));
lifetimeTokens.push(...[["'é'", '233'], ["'𐐀'", '66560']].map(([source, value]) => ({ source,
  tokens: [{ raw: source, kind: 'char', value, start: 0, end: source.length, matchesValue: false }] })));
writeProjectData(new URL('../../parity/fixtures/rust-unicode-lifetimes.lino', import.meta.url), {
  schemaVersion: 1, model: 'Unicode 17 scalar source identifier and canonical normalization 1', cases: lifetimeCases,
  refusals: [], lexicalCases: lifetimeTokens, lexicalErrors: [], versions,
  source: lifetimeCases.map(row => 'pub mod ' + row.module + '{\n' + row.source + '\n}').join('\n') + '\nfn main(){}\n',
  sourceBasis: 'https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels ; https://doc.rust-lang.org/reference/identifiers.html#normalization',
  boundary: 'Future original/emitted Rust values, normalized lifetime spelling with exact source intervals, complete graphs/codecs/four emissions and independent character-token controls. No observations. The separately selected ASCII-only source-header profile retains its original admission rule; a Unicode source-header profile remains to draft.' });
