// Complete source declarations; no source execution or oracle construction.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['Number separator coercion','return "a12b12".split(12).join("|");','a|b|'],
 ['BigInt separator coercion','return "a12b".split(12n).join("|");','a|b'],
 ['Boolean separator coercion','return "xfalsey".split(false).join("|");','x|y'],
 ['null separator coercion','return "xnullx".split(null).join("|");','x|x'],
 ['explicit undefined separator stays missing','return "undefined".split(undefined).length;','1'],
 ['String limit numeric coercion','return "a,b,c".split(",","2").join("|");','a|b'],
 ['Boolean limit numeric coercion','return "a,b".split(",",true).join("|");','a'],
 ['null limit selects no fields','return "a,b".split(",",null).length;','0'],
 ['undefined limit selects full fields','return "a,b".split(",",undefined).length;','2'],
 ['void limit selects full fields','return "a,b".split(",",void 0).length;','2'],
 ['optional absent separator','const values=[","];return "a,b".split(values[1]).length;','1'],
 ['optional present separator','const values=[","];return "a,b".split(values[0]).length;','2'],
 ['optional absent limit','const values=[1];return "a,b".split(",",values[1]).length;','2'],
 ['optional present limit','const values=[1];return "a,b".split(",",values[0]).length;','1'],
 ['all split arguments captured before conversion','let separator=",";return "a,b".split(separator,2,separator="x").join("|");','a|b'],
 ['ignored split argument retains effects','let count=0;const parts="a,b".split(",",2,count=7);return parts.length*10+count;','27'],
 ['ignored split BigInt does not convert','return "a,b".split(",",2,1n).length;','2'],
 ['missing code unit position','return "abc".charCodeAt();','97'],
 ['coerced code unit position','return "abc".charCodeAt("1");','98'],
 ['Boolean character position','return "abc".charAt(true);','b'],
 ['null character position','return "abc".charAt(null);','a'],
 ['undefined character position','return "abc".charAt(undefined);','a'],
 ['missing point position','return "😀x".codePointAt()??-1;','128512'],
 ['coerced point position','return "😀x".codePointAt("1")??-1;','56832'],
 ['point out of bounds stays undefined','return "abc".codePointAt(5)===undefined;','true'],
 ['missing relative position','return "abc".at()??"missing";','a'],
 ['negative relative fraction truncates','return "abc".at(-1.9)??"missing";','c'],
 ['relative positive infinity stays undefined','return "abc".at(Infinity)===undefined;','true'],
 ['character ignored argument retains effects','let value=0;const text="abc".charAt(0,value=8);return text+value;','a8'],
 ['point ignored argument retains effects','let value=0;const point="abc".codePointAt(0,value=8);return (point??0)+value;','105'],
 ['position captured before later assignment','let position=0;return "abc".charCodeAt(position,position=2);','97'],
 ['own split keeps callable meaning','const item={split(value){return value+1;}};return item.split(8);','9'],
 ['own character method keeps callable meaning','const item={charAt(value){return value+2;}};return item.charAt(8);','10'],
];
for(const[operation,argument,value]of[
 ['charAt','true','B'],['charAt','false','A'],['charAt',"'1'",'B'],['charAt','0,1','A'],
 ['at','true','B'],['at','false','A'],['at',"'1'",'B'],['at','0,1','A'],
 ['charCodeAt','true','66'],['charCodeAt','false','65'],['charCodeAt',"'1'",'66'],['charCodeAt','0,1','65'],
])inputs.push(['original console refusal corrected '+operation+' '+argument,"console.log('ABC'."+operation+'('+argument+'));return 0;', '0']);
const cases=inputs.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'stringUnitCallCase'+index,value,abort:null}));
for(const[name,body]of[
 ['split limit BigInt conversion error','return "a,b".split(",",1n).length;'],
 ['character BigInt conversion error','return "abc".charAt(1n);'],
 ['point BigInt conversion error','return "abc".codePointAt(1n);'],
 ['relative BigInt conversion error','return "abc".at(1n);'],
 ['original relative BigInt refusal becomes runtime error',"console.log('ABC'.at(1n));return 0;"],
]){
 const abort='Cannot convert a BigInt value to a number';
 cases.push({name,source:'export function compute(){'+body+'}',entry:'stringUnitCallError'+cases.length,value:null,abort,errorClass:'TypeError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))});
}
for(const[name,expression]of[
 ['split extra throw precedes limit conversion','"a,b".split(",",1n,failStringUnitExtra())'],
 ['character extra throw precedes position conversion','"abc".charAt(1n,failStringUnitExtra())'],
 ['point extra throw precedes position conversion','"abc".codePointAt(1n,failStringUnitExtra())'],
]){
 const entry='stringUnitCallExtra'+cases.length,helper='failStringUnitExtra'+cases.length,abort='unit extra';
 cases.push({name,source:'/** @returns {number} */ function '+helper+'(){throw new SyntaxError("unit extra");} export function compute(){return '+expression.replace('failStringUnitExtra()',helper+'()')+';}',entry,value:null,abort,errorClass:'SyntaxError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))});
}
writeProjectData(new URL('../../parity/fixtures/string-unit-calls.lino',import.meta.url),{
 schemaVersion:1,model:'actual primitive string unit call normalization and effects 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.prototype.split ; https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.prototype.charcodeat ; https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.prototype.codepointat ; https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.prototype.at',
 assertionCorrections:{predecessorRevision:'186d640d',sourceBasis:'ECMAScript 2025 String unit positions apply ToIntegerOrInfinity; extra call arguments still evaluate and are ignored by these builtins. BigInt position conversion throws at runtime.',files:['js/tests/translation-string-character.test.js','js/tests/translation-string-relative-character.test.js','js/tests/translation-string-code-unit.test.js','rust/tests/unit/translation_primitive_methods.rs'].map(path=>{const source=execFileSync('git',['show','186d640d:'+path],{encoding:'utf8'});return {path,source,sha256:createHash('sha256').update(source).digest('hex')};})},
 boundary:'Future original Node values/errors/UTF16 message fields, native products, complete source decision definitions, both codecs and four emissions. Only declarations. Object/Symbol coercion, RegExp/Symbol.split, arbitrary primitive unions, prototypes and all earlier requirements remain.',
});
