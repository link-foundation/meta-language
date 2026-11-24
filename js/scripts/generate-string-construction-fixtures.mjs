// Source and expected-value declarations; never evaluate constructors here.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const rows=[
 ['empty character construction','return String.fromCharCode().length;','0'],
 ['empty point construction','return String.fromCodePoint().length;','0'],
 ['ordinary character','return String.fromCharCode(65);','A'],
 ['character modulo 65536','return String.fromCharCode(65601);','A'],
 ['negative character modulo','return String.fromCharCode(-1).charCodeAt(0);','65535'],
 ['fraction truncation','return String.fromCharCode(65.9);','A'],
 ['nonfinite character becomes zero','return String.fromCharCode(Infinity).charCodeAt(0);','0'],
 ['undefined character becomes zero','return String.fromCharCode(undefined).charCodeAt(0);','0'],
 ['primitive character coercion','return String.fromCharCode("65",true,null).charCodeAt(1);','1'],
 ['isolated constructed surrogate','return String.fromCharCode(55296).charCodeAt(0);','55296'],
 ['astral point','return String.fromCodePoint(128512);','😀'],
 ['isolated surrogate is valid code point','return String.fromCodePoint(55296).charCodeAt(0);','55296'],
 ['maximum valid point length','return String.fromCodePoint(1114111).length;','2'],
 ['coerced point','return String.fromCodePoint("65");','A'],
 ['null point becomes zero','return String.fromCodePoint(null).charCodeAt(0);','0'],
 ['all arguments captured first','let value=1;const text=String.fromCodePoint(value,value=65);return text.charCodeAt(0)*100+text.charCodeAt(1);','165'],
];
rows.push(
 ['String without arguments','return String();',''],
 ['String undefined argument','return String(undefined);','undefined'],
 ['String null argument','return String(null);','null'],
 ['String Boolean argument','return String(false);','false'],
 ['String BigInt argument','return String(9007199254740993n);','9007199254740993'],
 ['String ignores extra BigInt','return String(0,1n);','0'],
 ['String captures first before later assignments','let value=1;return String(value,value=7);','1'],
 ['String ignored extras evaluate','let value=0;const text=String("first",value=7);return text==="first"&&value===7;','true'],
 ['String preserves isolated units','return String("\\ud800").charCodeAt(0);','55296'],
);
const cases=rows.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'stringConstructionCase'+index,value,abort:null}));
for(const[name,body,errorClass,abort]of[
 ['point above maximum','return String.fromCodePoint(1114112);','RangeError','Invalid code point 1114112'],
 ['negative point','return String.fromCodePoint(-1);','RangeError','Invalid code point -1'],
 ['fractional point','return String.fromCodePoint(1.5);','RangeError','Invalid code point 1.5'],
 ['undefined point','return String.fromCodePoint(undefined);','RangeError','Invalid code point NaN'],
 ['range check precedes later conversion','return String.fromCodePoint(1114112,1n);','RangeError','Invalid code point 1114112'],
 ['NaN range check precedes later conversion','return String.fromCodePoint("invalid",1n);','RangeError','Invalid code point NaN'],
 ['first conversion error wins','return String.fromCodePoint(1n,1114112);','TypeError','Cannot convert a BigInt value to a number'],
 ['character BigInt conversion error','return String.fromCharCode(1n);','TypeError','Cannot convert a BigInt value to a number'],
])cases.push({name,source:'export function compute(){'+body+'}',entry:'stringConstructionError'+cases.length,value:null,abort,errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))});
const abort='later argument';
cases.push({name:'later argument exception precedes range validation',source:'/** @returns {number} */ function failStringConstructionArgument(){throw new SyntaxError("later argument");}\nexport function compute(){return String.fromCodePoint(1114112,failStringConstructionArgument());}',entry:'stringConstructionLaterArgument',value:null,abort,errorClass:'SyntaxError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))});
cases.push({name:'String converts void call to undefined text',entry:'stringConstructionVoid',source:'function stringVoidResult(){return;}\nexport function compute(){return String(stringVoidResult());}',value:'undefined',abort:null});
writeProjectData(new URL('../../parity/fixtures/string-construction.lino',import.meta.url),{
 schemaVersion:1,model:'JavaScript source string construction intrinsics 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.fromcharcode ; https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.fromcodepoint',
 boundary:'Future independent original-source, error class/units/presence, native execution and all program/definition codecs/emissions. No observations. Iterator spread, object/symbol conversion, formal UTF16 representation and all prior obligations remain required.',
});
