// Complete source inputs/expectations only; no source or product code runs.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,value=null,errorClass=null,abort=null){cases.push({name,source,entry:'objectValuesCase'+cases.length,value,abort,...(abort===null?{}:{errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});}
for(const [name,object,value]of[
 ['value order differs from canonical shape','{z:26,a:1}','26|1'],['numeric own values first','{z:26,"10":10,a:1,"2":2}','2|10|26|1'],['overwrite retains its first place','{z:1,a:2,z:3}','3|2'],['empty own value set','{}',''],['false payload preserved','{z:false,a:true}','false|true'],['empty String payload preserved','{z:"",a:"x"}','|x'],['BigInt payload preserved','{z:0n,a:2n}','0|2'],['dense array own values','[3,4,5]','3|4|5'],['empty dense array values','[]',''],['String ASCII code units','"abc"','a|b|c'],['empty String values','""',''],['Number empty own fields','3',''],['Boolean empty own fields','false',''],['BigInt empty own fields','3n',''],['Map empty own fields','new Map([[1,2]])',''],['Set empty own fields','new Set([1,2])',''],
])add(name,'export function compute(){return Object.values('+object+').join("|");}',value);
add('equal shapes carry different value orders','function chooseValues(flag){return flag?{z:26,a:1}:{a:1,z:26};}export function compute(){return Object.values(chooseValues(true)).join("|")+";"+Object.values(chooseValues(false)).join("|");}','26|1;1|26');
add('ordered spread values and overwrites','export function compute(){const base={z:1,a:2};return Object.values({b:0,...base,a:9,c:3,...base}).join("|");}','0|1|2|3');
add('argument writes precede the field snapshot','function changeValues(base){base.z=9;return 0;}export function compute(){const base={z:1,a:2};return Object.values(base,changeValues(base)).join("|");}','9|2');
add('Array values produce a separate array','export function compute(){const original=[1,2];const copied=Object.values(original);copied.push(3);return original.length*10+copied.length;}','23');
add('Array read follows all extra arguments','export function compute(){const original=[3];return Object.values(original,original.push(4)).join("|");}','3|4');
add('negative zero payload sign survives wrapping','export function compute(){const values=Object.values({z:0,a:-0});return 1/(values[1]??1);}','-Infinity');
add('undefined payload is not a missing wrapper','export function compute(){const values=Object.values({z:undefined,a:undefined});return values.length===2&&values[0]===undefined&&values[1]===undefined;}','true');
add('null payload is not a missing wrapper','export function compute(){const values=Object.values({z:null,a:null});return values.length===2&&values[0]===null&&values[1]===null;}','true');
add('nullable payloads preserve actual presence','function nullableValues(first,second){return Object.values({z:first?3:undefined,a:second?4:undefined});}export function compute(){const values=nullableValues(true,false);return values[0]===3&&values[1]===undefined;}','true');
add('record payload retains its shared identity','export function compute(){const shared={text:"start"};const values=Object.values({z:shared,a:shared});const first=values[0]??shared;return first===shared;}','true');
add('String virtual values are UTF16 units','export function compute(){return Object.values("😀x").map(value=>value.charCodeAt(0)).join("|");}','55357|56832|120');
const long='a'.repeat(100);
add('long String value enumeration cannot use a finite fuel fallback','export function compute(){return Object.values('+JSON.stringify(long)+').join("|");}',Array.from({length:100},()=> 'a').join('|'));
add('isolated unit record payload remains exact','export function compute(){return Object.values({z:"\\ud800",a:"x"}).map(value=>value.charCodeAt(0)).join("|");}','55296|120');
for(const receiver of ['null','undefined',''])add('nullish value enumeration '+(receiver||'missing'),'export function compute(){return Object.values('+receiver+').join("|");}',null,'TypeError','Cannot convert undefined or null to object');
add('argument error precedes value enumeration','/** @returns {number} */function failValues(){throw new RangeError("value argument");}export function compute(){return Object.values(null,failValues()).join("|");}',null,'RangeError','value argument');
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('chooseValues','chooseValues'+row.entry).replaceAll('changeValues','changeValues'+row.entry).replaceAll('nullableValues','nullableValues'+row.entry).replaceAll('failValues','failValues'+row.entry)).join('\n');
const revision='f5a1b081';
const history=['js/src/translation/javascript-object-enumeration.js','js/src/translation/ordered-record-preparation.js','js/src/translation/javascript-record-order-rules.js','rust/src/translation/check/javascript_object_enumeration.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return{path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};});
writeProjectData(new URL('../../parity/fixtures/javascript-object-values.lino',import.meta.url),{
 schemaVersion:1,model:'homogeneous own-value enumeration by complete opaque payload source specialization 1',sourceLanguage:'JavaScript',cases,source,history,refusals:[],
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:['https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-object.values','https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-enumerableownproperties'],
 assertions:['actualValueOrderAcrossShapesAndBranches','allArgumentsBeforeValueSnapshot','shallowArrayCopiesAndPayloadAliases','exactPrimitiveAndNullablePayloads','nullishPayloadDistinctFromMissingWrapper','UTF16StringUnits','wholeSourceTemplateAndOneOwnedTypeSlot','sourceAlgorithmSpecializedRatherThanNativeReplacement','sourceErrorsRetainClassMessagePresenceAndUnits','completeNativeOriginalGraphsAndTwoCodecsFourEmissions'],
 boundary:'Unexecuted source declarations. Heterogeneous dynamic value unions, mutable Object.entries pairs, descriptors/getters/proxies/Symbols and all preceding full-language requirements remain obligations.',
});
