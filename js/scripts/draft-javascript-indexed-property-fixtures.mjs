// Declarations and prior-source capture only; no represented program executes.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,value=null,absence=null,key='',errorClass='TypeError',errorMessage=null){
 const abort=errorMessage??(absence===null?null:'Cannot read properties of '+absence+" (reading '"+key+"')");
 cases.push({name,source,entry:'indexedPropertyCase'+cases.length,value,abort,...(abort===null?{}:{errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))})});
}
for(const [name,text,index,value]of [
 ['ASCII','"abc"','1','b'],['negative zero','"abc"','-0','a'],['fractional','"abc"','1.5','missing'],['negative','"abc"','-1','missing'],['past end','"abc"','3','missing'],['empty','""','0','missing'],['NaN','"abc"','NaN','missing'],['infinite','"abc"','Infinity','missing'],['negative infinite','"abc"','-Infinity','missing'],['BigInt','"abc"','1n','b'],['negative BigInt','"abc"','-1n','missing'],['huge BigInt','"abc"','(1n<<6400n)','missing'],['optional Number present','"abc"','(true?1:undefined)','b'],['optional Number null','"abc"','(false?1:null)','missing'],['optional Number undefined','"abc"','(false?1:undefined)','missing'],['optional BigInt present','"abc"','(true?1n:undefined)','b'],['optional BigInt null','"abc"','(false?1n:null)','missing'],['optional BigInt undefined','"abc"','(false?1n:undefined)','missing'],['undefined','"abc"','undefined','missing'],['Unit','"abc"','(void 1)','missing']]){
 add('indexed String '+name,'export function compute(){return '+text+'['+index+']??"missing";}',value);
}
add('astral first UTF16 unit','export function compute(){return "😀x"[0].charCodeAt(0);}','55357');
add('astral second UTF16 unit','export function compute(){return "😀x"[1].charCodeAt(0);}','56832');
add('astral tail index is a code unit index','export function compute(){return "😀x"[2];}','x');
add('indexed unit length remains one','export function compute(){return "😀x"[0].length;}','1');
add('literal numeric String key','export function compute(){return "😀x"["2"]??"missing";}','x');
add('literal Array index key','export function compute(){return [3,4]["1"]??7;}','4');
add('literal Array out of bounds key','export function compute(){return [3,4]["2"]??7;}','7');
add('Array length bracket alias','export function compute(){return [3,4]["length"];}','2');
add('String length bracket alias is UTF16','export function compute(){return "😀x"["length"];}','3');
add('Map size bracket alias','export function compute(){return new Map([["key",3]])["size"];}','1');
add('Set size bracket alias','export function compute(){return new Set([3,4])["size"];}','2');
add('record String key','export function compute(){return {value:3}["value"];}','3');
add('record decimal-looking key remains an own name','export function compute(){return {"0":3}["0"];}','3');
add('record leading-zero key remains an own name','export function compute(){return {"01":3}["01"];}','3');
add('record length is its actual field','export function compute(){return {length:7}["length"];}','7');
add('record Unicode key stays exact','export function compute(){return {"😀":3}["😀"];}','3');
add('bracket method keeps primitive owner','export function compute(){return "😀x"["charAt"](2);}','x');
add('bracket method keeps record callable','export function compute(){return {invoke:value=>value+1}["invoke"](3);}','4');
add('bracket method keeps mutable array handle','export function compute(){const values=[3];return values["push"](4)*10+values.length;}','22');
add('bracket slice creates a distinct handle','export function compute(){const values=[3];const copy=values["slice"]();copy.push(4);return values.length*10+copy.length;}','12');
add('bracket own slice retains callable result','export function compute(){return {slice:()=>7}["slice"]();}','7');
add('nullable present record literal key','export function compute(){const receiver=true?{value:3}:null;return receiver["value"];}','3');
add('nullable absent record literal key','export function compute(){const receiver=false?{value:3}:null;return receiver["value"];}',null,'null','value');
add('nullable present array literal key','export function compute(){const receiver=true?[3,4]:null;return receiver["1"]??7;}','4');
add('nullable String literal length','export function compute(){const receiver=true?"😀x":null;return receiver["length"];}','3');
add('nullable String indexed unit','export function compute(){const receiver=true?"abc":null;return receiver[1]??"missing";}','b');
add('nullable absent String numeric read','export function compute(){const receiver=false?"abc":null;return receiver[1]??"missing";}',null,'null','1');
add('nullable bracket method fails before arguments','/** @returns {number} */function failIndexArgument(){throw new RangeError("argument evaluated");}export function compute(){const receiver=false?"abc":null;return receiver["charAt"](failIndexArgument());}',null,'null','charAt');
add('optional bracket name skips read','export function compute(){const receiver=false?{value:3}:null;return receiver?.["value"]??7;}','7');
add('optional indexed String skips key','/** @returns {number} */function failIndexArgument(){throw new RangeError("key evaluated");}export function compute(){const receiver=false?"abc":null;return receiver?.[failIndexArgument()]??"missing";}','missing');
add('computed String receiver and key are ordered once','export function compute(){const values=[];return (values.push(1),"abc")[(values.push(2),1)].length*100+values[0]*10+values[1];}','112');
add('out of bounds indexed String optional suffix','export function compute(){return "abc"[3]?.length??7;}','7');
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('failIndexArgument','failIndexArgument'+row.entry)).join('\n');
const revision='f40a26d2';
const history=['js/src/translation/javascript.js','js/src/translation/javascript-infer.js','js/src/translation/check.js','rust/src/translation/javascript/expressions.rs','rust/src/translation/javascript/infer/values.rs','rust/src/translation/check/arrays.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
 return{path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
});
writeProjectData(new URL('../../parity/fixtures/javascript-indexed-properties.lino',import.meta.url),{
 schemaVersion:1,model:'literal bracket properties, method owners and UTF16 indexed String 1',sourceLanguage:'JavaScript',cases,source,history,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:['https://tc39.es/ecma262/2025/multipage/ordinary-and-exotic-objects-behaviours.html#sec-string-exotic-objects-getownproperty-p','https://tc39.es/ecma262/2025/multipage/ordinary-and-exotic-objects-behaviours.html#sec-stringgetownproperty','https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-function-calls'],
 assertions:['literalKeyClassificationRetainsExactUtf16Units','canonicalArrayIndexRangeAndLeadingZeroBoundary','recordNumericNamesAreNotArraySlots','bracketMethodRetainsActualOwner','stringIndexReturnsOneCodeUnitOrUndefined','numericIndexNeverTruncatesFractionOrNegative','bigIntegerConversionOnlyAfterKeyCapture','optionalNumericAbsenceNeverCoercedToZero','actualReceiverThenKeyOnce','nullishAndOptionalChainBoundaries','sourceInferencePreservesPendingConstraints','nativeOriginalValuesSourceAstAndGraphs','twoCodecsFourEmissions'],
 boundary:'Unexecuted future declarations. Dynamic String-key unions/coercion, inherited/unknown properties, accessors/proxies, assignment/deletion descriptors, this and every earlier requirement remain.',
});
