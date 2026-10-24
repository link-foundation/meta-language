// Declarations and complete prior source records only. No represented code runs.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,value=null,errorClass=null,abort=null){
 cases.push({name,source,entry:'ownPropertyCase'+cases.length,value,abort,...(abort===null?{}:{errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))})});
}
for(const [name,receiver,key,result]of[
 ['record own present','{value:3}','"value"',true],['record own undefined','{value:undefined}','"value"',true],['record own null','{value:null}','"value"',true],['record missing','{value:3}','"missing"',false],['record inherited','{}','"toString"',false],['record own builtin name','{toString:()=>3}','"toString"',true],['record own constructor','{constructor:3}','"constructor"',true],['record computed prototype name','{["__proto__"]:3}','"__proto__"',true],['record numeric property','{"0":3}','0',true],['record negative zero property','{"0":3}','-0',true],['record BigInt property','{"1":3}','1n',true],['record leading zero property','{"01":3}','"01"',true],['record Unicode property','{"😀":3}','"😀"',true],['record null key','{"null":3}','null',true],['record undefined key','{"undefined":3}','undefined',true],['record Boolean key','{"true":3}','true',true],['record NaN key','{"NaN":3}','NaN',true],['record infinity key','{"Infinity":3}','Infinity',true],
 ['array first index','[3,4]','0',true],['array last index','[3,4]','"1"',true],['array missing index','[3,4]','2',false],['array fractional key','[3,4]','1.5',false],['array negative key','[3,4]','-1',false],['array length','[]','"length"',true],['array inherited method','[3]','"map"',false],['array noncanonical numeric key','[3]','"00"',false],['array String negative zero','[3]','"-0"',false],['array Number negative zero','[3]','-0',true],['array BigInt key','[3,4]','1n',true],['array huge BigInt key','[3]','(1n<<6400n)',false],['array radix String key','[3]','"0x0"',false],['array whitespace String key','[3]','" 0 "',false],['array scientific String key','[3]','"0e0"',false],['array maximum non-index','[]','"4294967295"',false],
 ['String first UTF16 index','"😀x"','0',true],['String second UTF16 index','"😀x"','1',true],['String tail UTF16 index','"😀x"','2',true],['String past end','"😀x"','3',false],['String length','""','"length"',true],['String inherited method','"abc"','"charAt"',false],['String noncanonical zero','"abc"','"00"',false],['String negative zero','"abc"','"-0"',false],['String numeric negative zero','"abc"','-0',true],
 ['Number wrapper has no own toString','3','"toString"',false],['Boolean wrapper has no own valueOf','false','"valueOf"',false],['BigInt wrapper has no own toString','3n','"toString"',false],['Map size is inherited','new Map([["key",3]])','"size"',false],['Set size is inherited','new Set([3])','"size"',false],
])add(name,'export function compute(){return Object.hasOwn('+receiver+','+key+');}',String(result));
for(const absence of ['null','undefined']){
 add(absence+' receiver','export function compute(){return Object.hasOwn('+absence+',"value");}',null,'TypeError','Cannot convert undefined or null to object');
 add(absence+' receiver never coerces object key','/** @returns {string} */function failOwnKeyCoercion(){throw new RangeError("must not coerce");}export function compute(){return Object.hasOwn('+absence+',{toString:()=>failOwnKeyCoercion()});}',null,'TypeError','Cannot convert undefined or null to object');
}
add('missing receiver','export function compute(){return Object.hasOwn();}',null,'TypeError','Cannot convert undefined or null to object');
add('missing key converts undefined','export function compute(){return Object.hasOwn({undefined:3});}','true');
add('optional record present','export function compute(){const receiver=true?{value:3}:null;return Object.hasOwn(receiver,"value");}','true');
add('optional record absent','export function compute(){const receiver=false?{value:3}:null;return Object.hasOwn(receiver,"value");}',null,'TypeError','Cannot convert undefined or null to object');
add('optional String falsey present','export function compute(){const receiver=true?"":null;return Object.hasOwn(receiver,"length");}','true');
add('optional Number zero present','export function compute(){const receiver=true?0:null;return Object.hasOwn(receiver,"missing");}','false');
add('optional Boolean false present','export function compute(){const receiver=true?false:null;return Object.hasOwn(receiver,"missing");}','false');
add('optional key retains null','export function compute(){const key=false?3:null;return Object.hasOwn({null:3},key);}','true');
add('optional key retains undefined','export function compute(){const key=false?3:undefined;return Object.hasOwn({undefined:3},key);}','true');
add('receiver reference read follows argument effects','export function compute(){const values=[3];return Object.hasOwn(values,(values.push(4),1));}','true');
add('extra arguments remain evaluated before membership','export function compute(){const values=[3];return Object.hasOwn(values,1,values.push(4));}','true');
add('actual receiver and all arguments evaluated once in order','export function compute(){const values=[];const found=Object.hasOwn((values.push(1),{value:3}),(values.push(2),"value"),values.push(3));return (found?1000:0)+values[0]*100+values[1]*10+values[2];}','1123');
add('ten extra arguments retain canonical capture names','export function compute(){let count=0;const found=Object.hasOwn({value:3},"value",count+=1,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1,count+=1);return (found?100:0)+count;}','110');
const fail='/** @returns {number} */function failOwnArgument(){throw new RangeError("own argument");}';
add('key evaluation error precedes nullish base coercion',fail+'export function compute(){return Object.hasOwn(null,failOwnArgument());}',null,'RangeError','own argument');
add('extra argument error precedes nullish base coercion',fail+'export function compute(){return Object.hasOwn(null,"value",failOwnArgument());}',null,'RangeError','own argument');
add('shadowed Object retains actual own callable','export function compute(){const Object={hasOwn:(value,key)=>key==="value"};return Object.hasOwn(null,"value");}','true');
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('failOwnArgument','failOwnArgument'+row.entry).replaceAll('failOwnKeyCoercion','failOwnKeyCoercion'+row.entry)).join('\n');
const revision='899630a5';
const history=['js/src/translation/javascript.js','js/src/translation/check.js','js/src/translation/javascript-nullish-fields.js','js/src/translation/javascript-indexed-properties.js','js/src/typed-program-expression-projection.js','rust/src/translation/check/javascript_indexed_properties.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
 return{path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
});
writeProjectData(new URL('../../parity/fixtures/javascript-own-properties.lino',import.meta.url),{
 schemaVersion:1,model:'closed own property membership with full argument capture and coercion order 1',sourceLanguage:'JavaScript',cases,source,history,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:['https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-object.hasown','https://raw.githubusercontent.com/nodejs/node/v22.15.0/deps/v8/src/common/message-template.h','https://raw.githubusercontent.com/nodejs/node/v24.0.0/deps/v8/src/common/message-template.h'],
 assertions:['actualUnshadowedBuiltinOwner','evaluateEveryArgumentBeforeObjectCoercion','objectCoercionBeforeKeyConversion','nullishBaseNeverInvokesKeyCoercion','ownMembershipDistinctFromValueAndPrototype','canonicalArrayIndexAndUtf16StringDomain','primitiveKeyConversionAndActualAbsences','readLiveLengthAfterArguments','privateCapturesCanonicalForEveryArity','allSourceExceptionsRetained','nativeOriginalValuesSourceAstAndGraphs','twoCodecsFourEmissions'],
 boundary:'Unexecuted source declarations. General object/symbol ToPropertyKey, open/nominal/callable/proxy/descriptor objects, dynamic deletion/addition, enumeration insertion order and all preceding requirements remain.',
});
