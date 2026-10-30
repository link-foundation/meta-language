// Author declarations from retained source spelling and explicit expected units.
// No tokenizer, decoder, constructor or represented source executes here.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const rows=[
 ['isolated high surrogate',String.raw`return "\ud800".charCodeAt(0);`,'55296'],
 ['isolated low surrogate',String.raw`return "\udfff".charCodeAt(0);`,'57343'],
 ['braced isolated surrogate',String.raw`return "\u{d800}".charCodeAt(0);`,'55296'],
 ['long braced zero prefix',String.raw`return "\u{000000000000000000000000000041}";`,'A'],
 ['long braced astral prefix',String.raw`return "\u{0000000000000000000001f600}".length;`,'2'],
 ['mixed isolated source units',String.raw`return "\ud800x\udfff".length;`,'3'],
 ['mixed isolated source positions',String.raw`return "\ud800x\udfff".charCodeAt(2);`,'57343'],
 ['isolated template chunk',String.raw`return `+'`'+String.raw`\ud800${"x"}\udfff`+'`'+'.charCodeAt(2);','57343'],
 ['isolated string numeric conversion',String.raw`return String(Number("\ud800"));`,'NaN'],
 ['concatenated source units pair',String.raw`return ("\ud83d"+"\ude00").codePointAt(0)?? -1;`,'128512'],
 ['source unit method slice',String.raw`return "\ud800x\udfff".slice(1,2);`,'x'],
 ['maximum braced code point',String.raw`return "\u{10ffff}".codePointAt(0)?? -1;`,'1114111'],
 ['hexadecimal escape',String.raw`return "\xff".charCodeAt(0);`,'255'],
];
const cases=rows.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'javaScriptStringUnitCase'+index,value,abort:null}));
const errors=[
 ['BigInt invalid isolated high',String.raw`return BigInt("\ud800");`,'SyntaxError',[67,97,110,110,111,116,32,99,111,110,118,101,114,116,32,55296,32,116,111,32,97,32,66,105,103,73,110,116]],
 ['error retains source units',String.raw`throw new TypeError("\ud800");`,'TypeError',[55296]],
];
for(const[name,body,errorClass,messageUnits]of errors)cases.push({name,source:'export function compute(){'+body+'}',entry:'javaScriptStringUnitCase'+cases.length,value:null,abort:messageUnits.map(unit=>String.fromCharCode(unit)).join('').toWellFormed(),errorClass,messagePresent:true,messageUnits});
const lexicalCases=[
 {name:'isolated cooked string token',source:String.raw`"\ud800"`,kind:'string',units:[55296],value:'\ufffd'},
 {name:'isolated cooked template chunk',source:'`'+String.raw`\udfff`+'`',kind:'template',units:[57343],value:'\ufffd'},
 {name:'valid surrogate pair',source:String.raw`"\ud83d\ude00"`,kind:'string',units:[55357,56832],value:'😀'},
];
writeProjectData(new URL('../../parity/fixtures/javascript-string-units.lino',import.meta.url),{
 schemaVersion:1,model:'JavaScript full UTF16 literal escape and unit construction 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 unitLexicalCases:lexicalCases,malformedEscapes:[String.raw`\u{110000}`,String.raw`\u{}`,String.raw`\u{0000000000000000110000}`,String.raw`\u123`,String.raw`\xG0`,String.raw`\u{1_0}`],
 previousExpectations:[{scope:'isolated JavaScript source escapes',kind:'unsupported',message:'not a scalar value; Rust strings hold scalar values only'},{scope:'JavaScript escape above 0x10ffff',kind:'unsupported',message:'scalar values only'}],
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-lexical-grammar.html#sec-literals-string-literals',
 boundary:'Declarations only; no observations. Exact source token units, original and emitted values/error fields, full program/definition text/binary codecs and all emissions remain required. Formal string profiles and non-value string keys remain required.',
});
