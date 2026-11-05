// Write future products and complete retained source definitions. Execute none.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value,prefix=''){const ordinal=cases.length;cases.push({name,entry:'absentRecordPropertyCase'+ordinal,source:prefix+'export function compute(){'+body+'}',value,abort:null});}
add('an ordinary missing dot property is undefined','const object={};return object.missing;','undefined');
add('a quoted missing property is undefined','const object={};return object["missing"];','undefined');
add('an empty property name retains its lookup','const object={};return object[""];','undefined');
add('a nonempty record still has absent properties','const object={actual:3};return object.missing;','undefined');
add('a missing ordinary length is not an array length','const object={};return object.length;','undefined');
add('a missing ordinary size is not a collection size','const object={};return object.size;','undefined');
add('undefined keeps its observable type query','const object={};return typeof object.missing;','undefined');
add('undefined keeps its explicit String conversion','const object={};return String(object.missing);','undefined');
add('a missing field selects a numeric nullish fallback','const object={};return object.missing??3;','3');
add('a missing field selects a String nullish fallback','const object={};return object.missing??"fallback";','fallback');
add('an absent field permits a complete object fallback','const object={};return (object.missing??{value:3}).value;','3');
add('an absent field permits a complete array fallback','const object={};return (object.missing??[3,4]).join("|");','3|4');
add('a required lexical object binding may receive undefined','const {missing}={};return missing;','undefined');
add('a renamed lexical binding retains absent meaning','const {missing:result}={};return result;','undefined');
add('a nested closed owner retains its absent leaf','const {nested:{missing}}={nested:{}};return missing;','undefined');
add('a closed missing leaf selects its original initializer','const {missing=3}={};return missing;','3');
add('own undefined is retained independently from missing','const object={value:undefined};return String(Object.hasOwn(object,"value"))+":"+String(object.value);','true:undefined');
add('a missing property remains absent from own membership','const object={};return String(Object.hasOwn(object,"missing"))+":"+String(object.missing);','false:undefined');
add('reading an absent property adds no enumerable key','const object={actual:3};const ignored=object.missing;return Object.keys(object).join("|");','actual');
add('a provided inherited spelling keeps its actual own value','const object={constructor:3,toString:4};return object.constructor+object.toString;','7');
add('a provided zero remains present under coalescing','const object={value:0};return object.value??3;','0');
add('a provided false remains present under coalescing','const object={value:false};return object.value??true;','false');
add('a provided empty String remains present','const object={value:""};return object.value??"fallback";','');
add('null selects the same nullish fallback','return null??3;','3');
add('undefined selects its nullish fallback','return undefined??3;','3');
add('an absence-only return retains its fallback','return readAbsentRecordProperty()??3;','3','function readAbsentRecordProperty(){return {}.missing;}');
add('nullish fallback may itself remain absent','const object={};return object.missing??null;','null');
add('a field default callback sees a missing own property','return [{}].map(({value=3})=>value).join("|");','3');
const traces=[
 {name:'the missing-property receiver executes once',source:'function receiver(){console.log("receiver");return {};}console.log(String(receiver().missing));',lines:['receiver','undefined'],error:null},
 {name:'receiver effects precede the selected fallback',source:'function receiver(){console.log("receiver");return {};}function fallback(){console.log("fallback");return 3;}console.log(receiver().missing??fallback());',lines:['receiver','fallback','3'],error:null},
 {name:'a present field leaves the nullish fallback lazy',source:'function receiver(){console.log("receiver");return {value:7};}function fallback(){console.log("fallback");return 3;}console.log(receiver().value??fallback());',lines:['receiver','7'],error:null},
 {name:'own absence does not erase unrelated initializer effects',source:'function field(){console.log("field");return 3;}console.log(String(({actual:field()}).missing));',lines:['field','undefined'],error:null}
];
const revision='808a1bfb',history=['js/src/translation/check.js','js/src/translation/javascript-infer.js','js/src/translation/optional-inference.js','js/src/translation/javascript-indexed-properties.js','js/src/translation/javascript-record-parameter-rules.js','rust/src/translation/check/records.rs','rust/src/translation/check/arrays.rs','rust/src/translation/check/optional_values.rs','rust/src/translation/javascript/infer/records.rs','rust/src/translation/javascript/infer/optional_values.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
const refusals=['constructor','toString','valueOf','__proto__'].map(field=>({name:'an unrepresented inherited '+field+' is not replaced with undefined',source:'function f(){return {}.'+field+';}',kind:'type',reason:'record has no field '+field}));
writeProjectData(new URL('../../parity/fixtures/absent-record-properties.lino',import.meta.url),{schemaVersion:1,model:'closed ordinary record absence and nullish fallback 1',sourceLanguage:'JavaScript',cases,traces,history,refusals,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},references:['https://tc39.es/ecma262/2025/multipage/ordinary-and-exotic-objects-behaviours.html#sec-ordinaryget','https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-getv'],assertions:['actualClosedShapeAndInheritedNameSelection','sourceReceiverEvaluatedOnceBeforeUndefined','ordinaryFieldAndLiteralBracketAndBindingReads','absenceOnlyNullishFallbackUsesActualRightDomain','ownMembershipAndEnumerationRemainExact','unchangedCheckedProjectionAndOtherLanguageErrors','completeDefinitionsBothCodecsFourEmissionsNativeProducts','retainedFullPredecessorSourceDefinitionsAndTraceOrder'],boundary:'Unexecuted source declarations. Reuses the exact generated closed-record field policy; no own-field insertion or false prototype value. Open/inherited/getter/proxy/primitive protocols, arbitrary shape unions, parameter environments and all preceding requirements remain obligations.'});
