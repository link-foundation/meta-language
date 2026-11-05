// Declare future controls and retained assertion sources; execute no programs.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,definition,call,value,abort=null){const owner='recordFormalOwner'+cases.length,entry='recordFormalCase'+cases.length;cases.push({name,source:definition.replaceAll('OWNER',owner)+'export function compute(){'+call.replaceAll('OWNER',owner)+'}',entry,value,abort,...(abort===null?{}:{errorClass:'RangeError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});}
const choose='function OWNER({value=3}){return value;}';
add('present field bypasses its initializer',choose,'return OWNER({value:7});','7');
add('own undefined selects the initializer',choose,'return OWNER({value:undefined});','3');
add('null does not select the initializer',choose,'return String(OWNER({value:null}));','null');
add('zero does not select the initializer',choose,'return OWNER({value:0});','0');
add('false does not select the initializer','function OWNER({value=true}){return value;}','return OWNER({value:false});','false');
add('empty text does not select the initializer','function OWNER({value="default"}){return value;}','return OWNER({value:""});','');
add('property alias retains its original local name','function OWNER({value:amount=3}){return amount;}','return OWNER({value:undefined});','3');
add('initializer reads an earlier ordinary argument','function OWNER(first,{value=first}){return value;}','return OWNER(7,{value:undefined});','7');
add('initializer reads an earlier required record binding','function OWNER({first,second=first+1}){return second;}','return OWNER({first:7,second:undefined});','8');
add('initializer reads an earlier initialized record binding','function OWNER({first=3,second=first+1}){return second;}','return OWNER({first:undefined,second:undefined});','4');
add('separate actual pattern arguments retain initialization order','function OWNER({first=3},{second=first+1}){return second;}','return OWNER({first:undefined},{second:undefined});','4');
add('repeated property reads retain distinct actual BoundNames','function OWNER({value:first=3,value:second=first+1}){return first+second;}','return OWNER({value:undefined});','7');
add('nested call delimiters remain inside the initializer','function OWNER({value=Math.max(3,4)}){return value;}','return OWNER({value:undefined});','4');
add('parenthesized sequence retains its source interval','function OWNER({value=(3,4)}){return value;}','return OWNER({value:undefined});','4');
add('opaque punctuation text does not terminate a record initializer','function OWNER({value="} , )"}){return value;}','return OWNER({value:undefined});','} , )');
add('opaque template retains its embedded expression','function OWNER({value=`record${3}`}){return value;}','return OWNER({value:undefined});','record3');
add('constant arrow retains its source role','const OWNER=({value=3})=>value;','return OWNER({value:undefined});','3');
add('constant function expression retains its source role','const OWNER=function({value=3}){return value;};','return OWNER({value:undefined});','3');
add('namespace object method retains its source role','const OWNER={read({value=3}){return value;}};','return OWNER.read({value:undefined});','3');
add('nested arrow callback retains its record initializer','','return [{value:undefined}].map(({value=3})=>value).join("|");','3');
add('nested function callback retains its record initializer','','return [{value:undefined}].map(function({value=3}){return value;}).join("|");','3');
add('nested callback reads an earlier initialized binding','','return [{first:undefined,second:undefined}].map(({first=3,second=first+1})=>second).join("|");','4');
add('local writes leave the original own field unchanged','function OWNER({value=3}){value+=1;return value;}','const row={value:7};return String(OWNER(row))+":"+String(row.value);','8:7');
add('present value preserves a throwing initializer lazily','/** @returns {number} */function failRecordFormalPresent(){throw new RangeError("record initializer");}function OWNER({value=failRecordFormalPresent()}){return value;}','return OWNER({value:7});','7');
add('undefined value observes its actual throwing initializer','/** @returns {number} */function failRecordFormalMissing(){throw new RangeError("record initializer");}function OWNER({value=failRecordFormalMissing()}){return value;}','return OWNER({value:undefined});',null,'record initializer');
add('empty ordinary record selects its missing field initializer',choose,'return OWNER({});','3');
add('other own fields do not manufacture a missing property',choose,'return OWNER({other:7});','3');
add('missing aliased field selects its original initializer','function OWNER({value:amount=3}){return amount;}','return OWNER({});','3');
add('two missing fields retain ordered earlier initialization','function OWNER({first=3,second=first+1}){return first+second;}','return OWNER({});','7');
add('missing field initializer reads an earlier ordinary argument','function OWNER(first,{value=first+1}){return value;}','return OWNER(7,{});','8');
add('missing nullable initializer retains its null value','function OWNER({value=null}){return String(value);} ','return OWNER({});','null');
add('missing callback record field retains its initializer','','return [{}].map(({value=3})=>value).join("|");','3');
add('own prototype method spelling overrides its inherited meaning','function OWNER({toString:value=3}){return value;}','return OWNER({toString:undefined});','3');
add('missing field observes a throwing initializer','/** @returns {number} */function failRecordFormalAbsent(){throw new RangeError("record absent initializer");}function OWNER({value=failRecordFormalAbsent()}){return value;}','return OWNER({});',null,'record absent initializer');
add('exact former whole parameter input','function f({value=3}={}){return value;}','return f();','3');
const refusals=[
 {name:'self reference retains its uninitialized binding obligation',source:'function f({value=value}){return value;}',kind:'unsupported',reason:'record formal default reading parameter value: the current default profile requires a closed parameter initialization environment'},
 {name:'later binding retains its uninitialized environment',source:'function f({first=second,second=3}){return first;}',kind:'unsupported',reason:'record formal default reading parameter second: the current default profile requires a closed parameter initialization environment'},
 {name:'initializers do not silently write earlier bindings',source:'function f({first=3,second=(first=4)}){return second;}',kind:'unsupported',reason:'record formal default assigning parameter first: writes during parameter initialization need their shared environment interpretation'},
 ...['toString','constructor','__proto__','hasOwnProperty','__defineGetter__'].map(property=>({name:'inherited '+property+' is not silently treated as an absent value',source:'function f({'+property+':value=3}){return value;}console.log(f({}));',kind:'unsupported',reason:'record formal inherited property: an absent own field cannot replace the actual prototype property '+property})),
];
const revision='0abcfe2c',history=['parity/fixtures/record-closure-parameters.lino','js/tests/translation-record-closure-parameters.test.js','js/tests/translation-javascript-record-parameters.test.js','rust/tests/unit/translation_javascript_record_parameters.rs','js/src/translation/javascript-record-parameter-rules.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/record-formal-defaults.lino',import.meta.url),{schemaVersion:1,model:'actual represented record formal initializers 1',sourceLanguage:'JavaScript',cases,refusals,history,correctedInputs:[{originalInput:'function f({value=3}={}){return value;}',source:'function f({value=3}={}){return value;}export function compute(){return f();}',expected:'3',predecessorReason:'record parameter binding: a nonempty flat required object pattern needs distinct lexical names; defaults, arrays, nested/rest/computed patterns are separate obligations (reason 2)'},{source:'export function compute(){return [{value:3}].map(({value=1})=>value).join("|");}',expected:'3',predecessorReason:'record parameter binding: a nonempty flat required object pattern needs distinct lexical names; defaults, arrays, nested/rest/computed patterns are separate obligations (reason 2)'}],source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['completeOriginalInitializerIntervals','closedMissingOwnFieldsAndInheritedPropertyRefusals','undefinedOnlyLazyDefaultsAndNullishPayloads','earlierActualBindingsAndUninitializedNameRestrictions','separateRequiredOnlyPlanContract','actualNamedConstantNamespaceAndNestedCallableRoles','localWritesPreserveOriginalRecordFields','retainedFormerDefaultRefusalWithCorrectedOriginalAndEmittedValues','completeDefinitionTwoCodecsFourEmissionsNativeProducts'],
 boundary:'Unexecuted future controls. Represented named fields use the existing field/type protocol and undefined-only default IR. Missing normal names on closed ordinary records select undefined; inherited Object prototype spellings refuse unless an actual own field shadows them. Open shapes and arbitrary prototype/getter/proxy domains remain implementation obligations, as do whole-parameter defaults, initialization writes, self/later and nested/rest/computed patterns, full iterators and all inherited requirements.'});
