// Whole source declarations only. No splitting, oracle or assertion execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['missing separator','return "abc".split().join("|");','abc'],
 ['missing separator with zero limit','return "abc".split(undefined,0).length;','0'],
 ['empty input retains field','return "".split(",").length;','1'],
 ['empty input empty separator','return "".split("").length;','0'],
 ['missing separator empty input','return "".split().length;','1'],
 ['ordinary separator','return "one::two::three".split("::").join("|");','one|two|three'],
 ['leading and trailing fields','return ",a,,".split(",").join("|");','|a||'],
 ['nonoverlapping matches','return "aaaaa".split("aa").join("|");','||a'],
 ['absent separator text','return "abc".split("z").length;','1'],
 ['partial terminal separator','return "ababa".split("aba").join("|");','|ba'],
 ['empty separator splits code units','return "A😀B".split("").length;','4'],
 ['high half remains exact','return "😀".split("")[0].charCodeAt(0);','55357'],
 ['low half remains exact','return "😀".split("")[1].charCodeAt(0);','56832'],
 ['isolated separator remains a unit','return "x\\ud800y\\ud800".split("\\ud800").join("|");','x|y|'],
 ['zero limit','return "a,b".split(",",0).length;','0'],
 ['one field limit','return "a,b".split(",",1).join("|");','a'],
 ['limit counts empty fields','return ",a,,".split(",",3).join("|");','|a|'],
 ['fractional limit truncation','return "a,b,c".split(",",2.9).join("|");','a|b'],
 ['NaN limit','return "a,b".split(",",NaN).length;','0'],
 ['infinite limit','return "a,b".split(",",Infinity).length;','0'],
 ['negative limit wraps','return "a,b".split(",",-1).length;','2'],
 ['exact unsigned wrap','return "a,b".split(",",4294967296).length;','0'],
 ['unsigned wrap plus one','return "a,b".split(",",4294967297).join("|");','a'],
 ['negative exact unsigned wrap','return "a,b".split(",",-4294967296).length;','0'],
 ['negative fraction truncates before wrap','return "a,b".split(",",-0.9).length;','0'],
 ['empty separator obeys limit','return "abc".split("",2).join("|");','a|b'],
 ['reserved helper name remains source binding','const ml_utf16_text=7;return "a,b".split(",").length+ml_utf16_text;','9'],
 ['separator parameter remains borrowed','const separator=",";const parts="a,b".split(separator);return parts.join(separator)+separator;','a,b,'],
];
const cases=inputs.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'stringSplitCase'+index,value,abort:null}));
writeProjectData(new URL('../../parity/fixtures/string-split.lino',import.meta.url),{
 schemaVersion:1,model:'shared typed literal UTF16 split and unsigned limit 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.prototype.split ; https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-touint32',
 boundary:'Only declarations; no observations. Future original-source/native values, complete UTF16 program definitions, both codecs and four emissions. Raw primitive argument coercion/effects, RegExp/Symbol.split protocols, generic receivers and every preceding requirement remain.',
});
