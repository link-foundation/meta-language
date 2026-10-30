// Author independent source/normalization declarations; never call a lexer or ICU.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
const inputs = [
  ['Chinese function and parameters', 'fn 加算(左:i64,右:i64)->i64{左+右} pub fn result()->i64{加算(20,22)}', '42'],
  ['canonical acute binding', 'pub fn result()->i64{let e\u0301=43;é}', '43'],
  ['canonical angstrom binding', 'pub fn result()->i64{let Å=44;Å}', '44'],
  ['canonical Kelvin function', 'fn K(値:i64)->i64{値} pub fn result()->i64{K(45)}', '45'],
  ['Hangul algorithmic composition', 'pub fn result()->i64{let 가=46;가}', '46'],
  ['canonical combining order', 'pub fn result()->i64{let a\u0301\u0323=47;ạ\u0301}', '47'],
  ['astral source binder', 'pub fn result()->i64{let 𐐀=48;𐐀}', '48'],
  ['non-Latin raw identifier', 'pub fn result()->i64{let r#東京=49;東京}', '49'],
  ['raw canonical identifier', 'pub fn result()->i64{let r#e\u0301=50;é}', '50'],
  ['Unicode struct and field', 'struct 座標{値:i64} pub fn result()->i64{座標{値:51}.値}', '51'],
  ['Unicode inherent method', 'struct 値{i:i64} impl 値{fn 読む(&self)->i64{self.i}} pub fn result()->i64{値{i:52}.読む()}', '52'],
  ['normalized shadowing', 'pub fn result()->i64{let é=1;let e\u0301=53;é}', '53'],
];
const cases = inputs.map(([name, source, value], index) => ({ name, module: 'unicode_case_' + index, source, value, abort: null }));
const lexical = [
  ['e\u0301 é', [['e\u0301', 'identifier', 'é'], ['é', 'identifier', 'é']]],
  ['K Å', [['K', 'identifier', 'K'], ['Å', 'identifier', 'Å']]],
  ['가 각', [['가', 'identifier', '가'], ['각', 'identifier', '각']]],
  ['a\u0301\u0323', [['a\u0301\u0323', 'identifier', 'ạ\u0301']]],
  ['𐐀x r#𐐀', [['𐐀x', 'identifier', '𐐀x'], ['r#𐐀', 'identifier', '𐐀']]],
  ['r#東京 r#e\u0301', [['r#東京', 'identifier', '東京'], ['r#e\u0301', 'identifier', 'é']]],
  ['東京!', [['東京!', 'macro', '東京']]],
  ['क\u093c क़', [['क\u093c', 'identifier', 'क\u093c'], ['क़', 'identifier', 'क\u093c']]],
  ['a\u200cb', [['a', 'identifier', 'a'], ['\u200c', 'punct', '\u200c'], ['b', 'identifier', 'b']]],
  ['\u0301x', [['\u0301', 'punct', '\u0301'], ['x', 'identifier', 'x']]],
].map(([source, tokens]) => {
  let offset = 0;
  return { source, tokens: tokens.map(([raw, kind, value]) => {
    const start = source.indexOf(raw, offset), end = start + raw.length; offset = end;
    return { raw, kind, value, start, end, matchesValue: kind !== 'macro' };
  }) };
});
const versions = { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; common Unicode 16/17 characters',
  Rust: '>=1.90.0 <1.100.0; edition 2024; common Unicode 16/17 characters' };
writeProjectData(new URL('../../parity/fixtures/unicode-rust-identifiers.lino', import.meta.url), {
  schemaVersion: 1, model: 'Unicode 17 scalar source identifier and canonical normalization 1', unicode: '17.0.0',
  cases, refusals: [], lexicalCases: lexical, lexicalErrors: [], versions,
  source: cases.map(row => 'pub mod ' + row.module + '{\n' + row.source + '\n}').join('\n') + '\nfn main(){}\n',
  sourceBasis: 'https://doc.rust-lang.org/reference/identifiers.html ; https://www.unicode.org/reports/tr15/tr15-57.html ; https://doc.rust-lang.org/releases.html#version-1940-2026-03-05',
  boundary: 'Future complete native/original Rust values, exact Unicode token values/raw/UTF-16 spans, program and definition graphs, codecs and four emissions. Pinned source language uses Unicode 17; the original compiler controls use only characters already shared with Unicode 16. No observations.' });
writeProjectData(new URL('../../parity/fixtures/unicode-source-scalars.lino', import.meta.url), {
  schemaVersion: 1, unicode: '17.0.0', versions,
  normalizations: [
    ['e\u0301', 'é'], ['K', 'K'], ['Å', 'Å'], ['가', '가'], ['각', '각'],
    ['a\u0301\u0323', 'ạ\u0301'], ['क़', 'क\u093c'], ['\u0301\u0323', '\u0323\u0301'],
    ['a\u0305\u0301', 'a\u0305\u0301'], ['①', '①'], ['ﬀ', 'ﬀ'], ['𐐀', '𐐀'], ['', ''],
  ].map(([source, expected]) => ({ source, expected })),
  identifierSources: [
    ['東京', true, true], ['𐐀', true, true], ['_', true, true], ['$', true, false],
    ['a\u200c', true, false], ['a\u200d', true, false], ['a\u0301', true, true],
    ['\u0301a', false, false], ['🙂', false, false], ['a0', true, true], ['0a', false, false],
  ].map(([source, JavaScript, Rust]) => ({ source, JavaScript, Rust })),
  boundary: 'Future independent ICU normalization/property checks on common Unicode 16/17 characters, surrogate decoding, table invariants, source preservation and complete definition graph/codec/four-target controls. No executions.' });
const javascriptInputs = [
  ['Chinese source', 'const 左=20,右=22;return 左+右;', '42'],
  ['canonically distinct names', 'const é=30,e\u0301=13;return é+e\u0301;', '43'],
  ['Kelvin remains distinct', 'const K=40,K=4;return K+K;', '44'],
  ['astral source name', 'const 𐐀=45;return 𐐀;', '45'],
  ['joiner continuation', 'const a\u200db=46;return a\u200db;', '46'],
  ['nonjoiner continuation', 'const a\u200cb=47;return a\u200cb;', '47'],
  ['Unicode property', 'const item={東京:48};return item.東京;', '48'],
  ['Hangul names remain distinct', 'const 가=40,가=9;return 가+가;', '49'],
];
const javascriptCases = javascriptInputs.map(([name, body, value], index) => ({ name, source: 'export function compute(){' + body + '}',
  entry: 'unicodeIdentifierCase' + index, value, abort: null }));
writeProjectData(new URL('../../parity/fixtures/unicode-javascript-identifiers.lino', import.meta.url), {
  schemaVersion: 1, model: 'Unicode 17 scalar source identifier and canonical normalization 1', sourceLanguage: 'JavaScript',
  cases: javascriptCases, refusals: [], versions,
  source: javascriptCases.map(row => row.source.replace('function compute(', 'function ' + row.entry + '(')).join('\n'),
  sourceBasis: 'https://tc39.es/ecma262/2025/multipage/ecmascript-language-lexical-grammar.html#sec-names-and-keywords',
  boundary: 'Future strict original Node/emitted JavaScript/native Rust values, exact definition/program graphs, both codecs and all four emissions. JavaScript identifiers keep their distinct scalar spellings. No observations.' });
