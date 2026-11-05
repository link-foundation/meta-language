// Source/expected declarations and complete preceding inputs only. No oracles.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,value=null,errorClass=null,abort=null){cases.push({name,source,entry:'objectKeysCase'+cases.length,value,abort,...(abort===null?{}:{errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});}
for(const [name,source,value]of[
 ['non-numeric first insertion','{z:1,a:2}','z|a'],['numeric names first by value','{z:1,"10":2,a:3,"2":4}','2|10|z|a'],['overwrite keeps first insertion','{z:1,a:2,z:3}','z|a'],['computed own prototype name','{["__proto__"]:1,a:2}','__proto__|a'],['inherited names excluded','{}',''],['undefined and null own values retained','{z:undefined,a:null}','z|a'],['unicode field names','{"😀":1,"é":2,a:3}','😀|é|a'],['dense array own indices only','[1,2,3]','0|1|2'],['empty dense array','[]',''],['String virtual UTF16 keys','"😀x"','0|1|2'],['empty String','""',''],['Number wrapper empty','3',''],['false wrapper empty','false',''],['BigInt wrapper empty','3n',''],['Map ordinary own fields empty','new Map([[1,2]])',''],['Set ordinary own fields empty','new Set([1,2])',''],
])add(name,'export function compute(){return Object.keys('+source+').join("|");}',value);
add('actual order across a parameter','function passKeys(value){return Object.keys(value).join("|");}export function compute(){return passKeys({z:1,a:2});}','z|a');
add('same type carries two branch orders','function chooseKeys(flag){return flag?{z:1,a:2}:{a:2,z:1};}export function compute(){return Object.keys(chooseKeys(true)).join("|")+":"+Object.keys(chooseKeys(false)).join("|");}','z|a:a|z');
add('actual spread order and overwrites','export function compute(){const base={z:1,a:2};const value={b:0,...base,a:9,c:3,...base};return Object.keys(value).join("|");}','b|z|a|c');
add('numeric spread merge','export function compute(){const base={z:1,"10":10,a:2};return Object.keys({b:0,...base,"2":2,c:3}).join("|");}','2|10|b|z|a|c');
add('empty spreads preserve all fields','export function compute(){const base={};return Object.keys({...base,z:1,...base,a:2}).join("|");}','z|a');
add('spread copies fields before later writes','function changeSpread(base){base.z=9;return 0;}export function compute(){const base={z:1,a:2};const copied={...base,last:changeSpread(base)};return Object.keys(copied).join("|")+":"+copied.z+":"+base.z;}','z|a|last:1:9');
add('all extras precede enumeration','export function compute(){let count=0;const value={z:1,a:2};const keys=Object.keys(value,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1);return keys.join("|")+":"+count;}','z|a:10');
add('shadowed Object callable retained','export function compute(){const Object={keys:value=>["local"]};return Object.keys(3).join("|");}','local');
const long='a'.repeat(100);
add('long UTF16 name cannot be truncated by a formal fuel fallback','export function compute(){return Object.keys({['+JSON.stringify(long)+']:1}).join("|");}',long);
for(const receiver of ['null','undefined',''])add('nullish object coercion '+(receiver||'missing'),'export function compute(){return Object.keys('+receiver+').join("|");}',null,'TypeError','Cannot convert undefined or null to object');
add('argument error precedes ToObject','/** @returns {number} */function failEnumeration(){throw new RangeError("enumeration argument");}export function compute(){return Object.keys(null,failEnumeration()).join("|");}',null,'RangeError','enumeration argument');
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('passKeys','passKeys'+row.entry).replaceAll('chooseKeys','chooseKeys'+row.entry).replaceAll('changeSpread','changeSpread'+row.entry).replaceAll('failEnumeration','failEnumeration'+row.entry)).join('\n');
const revision='9afd36a8';
const history=['js/src/translation/check.js','js/src/translation/javascript.js','js/src/translation/javascript-record-order-rules.js','js/scripts/portable-source-library-style.mjs','rust/src/translation/check/records.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return{path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};});
writeProjectData(new URL('../../parity/fixtures/javascript-object-keys.lino',import.meta.url),{
 schemaVersion:1,model:'actual own-field key enumeration across source values and ordered spreads 1',sourceLanguage:'JavaScript',cases,source,history,refusals:[],
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:['https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-object.keys','https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-enumerableownproperties'],
 assertions:['actualUnshadowedOwner','everyArgumentBeforeObjectCoercion','numericKeysFirstByValue','StringKeysFirstInsertion','ownUndefinedValuesStillEnumerable','inheritedFieldsExcluded','UTF16StringVirtualIndices','perValueOrderAcrossParametersAndBranches','orderedSpreadRebaseAndFirstInsertionMerge','spreadCopiesBeforeLaterEffects','noFormalFuelTruncation','sourceErrorClassMessagePresenceAndUnits','completeNativeOriginalGraphsAndTwoCodecsFourEmissions'],
 boundary:'Unexecuted source drafts. Object.values/entries, full descriptors, Symbols, exotic/proxy/nominal objects, dynamic property addition/deletion and all preceding complete-language requirements remain obligations.',
});
