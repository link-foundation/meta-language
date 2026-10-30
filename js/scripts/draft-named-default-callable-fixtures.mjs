// Future products and complete predecessor sources; execute none.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,prefix,body,value,abort=null){const ordinal=cases.length;cases.push({name,entry:'namedDefaultCallableCase'+ordinal,source:(prefix+'export function compute(){'+body+'}').replaceAll('OWNER','namedDefaultCallableOwner'+ordinal),value,abort,...(abort===null?{}:{errorClass:'RangeError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});}
add('a named default survives a stored function value','function OWNER(value=3){return value;}','const read=OWNER;return read();','3');
add('a stored named default retains a supplied value','function OWNER(value=3){return value;}','const read=OWNER;return read(7);','7');
add('a stored named default selects explicit undefined','function OWNER(value=3){return value;}','const read=OWNER;return read(undefined);','3');
add('a stored default keeps an explicit zero present','function OWNER(value=3){return value;}','const read=OWNER;return read(0);','0');
add('a stored Boolean default retains false','function OWNER(value=true){return value;}','const read=OWNER;return read(false);','false');
add('a stored String default retains empty text','function OWNER(value="fallback"){return value;}','const read=OWNER;return read("");','');
add('dependent named defaults survive a function value','function OWNER(first=3,second=first+1){return first+second;}','const read=OWNER;return read();','7');
add('a required prefix precedes its dependent default','function OWNER(first,second=first+1){return first+second;}','const read=OWNER;return read(3);','7');
add('a required later slot retains the original supplied value','function OWNER(first=3,second){return first+second;}','const read=OWNER;return read(undefined,4);','7');
add('a returned named callable retains its initialization','function OWNER(value=3){return value;}function OWNERFactory(){return OWNER;}','return OWNERFactory()();','3');
add('a named default in a record field remains callable','function OWNER(value=3){return value;}','const object={read:OWNER};return object.read();','3');
add('a higher-order call retains the actual optional interface','function OWNER(value=3){return value;}function OWNERUse(read){return read();}','return OWNERUse(OWNER);','3');
add('a higher-order explicit undefined selects the original default','function OWNER(value=3){return value;}function OWNERUse(read){return read(undefined);}','return OWNERUse(OWNER);','3');
add('a constant arrow default retains its value interface','const OWNER=(value=3)=>value;','const read=OWNER;return read();','3');
add('a constant function expression retains its value interface','const OWNER=function(value=3){return value;};','const read=OWNER;return read();','3');
add('an existing whole-default reference keeps its body entry','function OWNER({answer}={answer:3}){return answer;}','const read=OWNER;return read();','3');
add('a named default map callback sees actual undefined','function OWNER(value=3){return value+1;}','return [undefined,4].map(OWNER).join("|");','4|5');
add('a named default map callback wraps a present value','function OWNER(value=3){return value+1;}','return [3,4].map(OWNER).join("|");','4|5');
add('Array.from uses the same named default initialization','function OWNER(value=3){return value+1;}','return Array.from([undefined,4],OWNER).join("|");','4|5');
add('a named default callback keeps the supplied ordinal','function OWNER(value=3,index=4){return value+index;}','return [7].map(OWNER).join("|");','7');
add('a stored named default array is fresh per invocation','function OWNER(values=[3]){values.push(4);return values.join("|");}','const read=OWNER;return read()+":"+read();','3|4:3|4');
add('a documented optional default retains its actual source initializer','/** @param {number} [value=99] @returns {number} */function OWNER(value=2){return value;}','const read=OWNER;return read();','2');
add('an actual undefined initializer retains absence','function OWNER(value=undefined){return String(value);}','const read=OWNER;return read();','undefined');
add('an actual null initializer retains its tag','function OWNER(value=null){return String(value);}','const read=OWNER;return read();','null');
const nullable='/** @param {number|null|undefined} value @returns {string} */function OWNER(value=3){return String(value);}';
add('a broad optional parameter removes undefined before the call',nullable,'const read=OWNER;return read();','3');
add('a broad optional parameter preserves supplied null',nullable,'const read=OWNER;return read(null);','null');
add('a broad optional parameter preserves supplied zero',nullable,'const read=OWNER;return read(0);','0');
add('a guarded default exposes its original BigInt input domain','function OWNER(value=2n){if(value<0n)throw new RangeError("negative");return value;}','const read=OWNER;return read();','2');
add('a guarded function value still checks a negative supplied input','function OWNER(value=2n){if(value<0n)throw new RangeError("negative");return value;}','const read=OWNER;return read(-1n);',null,'negative');
const numberTarget='/** @param {number|null|undefined} value @returns {string} */function OWNERTarget(value){return String(value);}';
add('undefined-only optional policy widens without losing zero',numberTarget+'function OWNERSource(flag){return flag?0:undefined;}','return OWNERTarget(OWNERSource(true));','0');
add('undefined-only optional policy retains its absence',numberTarget+'function OWNERSource(flag){return flag?0:undefined;}','return OWNERTarget(OWNERSource(false));','undefined');
add('null-only optional policy widens without losing zero',numberTarget+'function OWNERSource(flag){return flag?0:null;}','return OWNERTarget(OWNERSource(true));','0');
add('null-only optional policy retains its original tag',numberTarget+'function OWNERSource(flag){return flag?0:null;}','return OWNERTarget(OWNERSource(false));','null');
add('optional Boolean widening preserves a present false','/** @param {boolean|null|undefined} value @returns {string} */function OWNERTarget(value){return String(value);}function OWNERSource(flag){return flag?false:undefined;}','return OWNERTarget(OWNERSource(true));','false');
add('optional String widening preserves present empty text','/** @param {string|null|undefined} value @returns {string} */function OWNERTarget(value){return String(value);}function OWNERSource(flag){return flag?"":null;}','return OWNERTarget(OWNERSource(true));','');
const traces=[
 {name:'named reference creation does not initialize its defaults',source:'function initialize(){console.log("default");return 3;}function read(value=initialize()){return value;}const saved=read;console.log("created");console.log(saved());console.log(saved());',lines:['created','default','3','default','3'],error:null},
 {name:'all supplied arguments precede defaults through a named value',source:'function initialize(){console.log("default");return 3;}function extra(){console.log("extra");return 4;}function read(value=initialize()){console.log("body");return value;}const saved=read;console.log(saved(undefined,extra()));',lines:['extra','default','body','3'],error:null},
 {name:'named callback defaults execute once for each missing value',source:'function initialize(){console.log("default");return 3;}function read(value=initialize()){return value;}console.log([undefined,undefined].map(read).join("|"));',lines:['default','default','3|3'],error:null},
 {name:'guard checks follow every default through a function value',source:'function initialize(){console.log("default");return 2n;}function input(){console.log("argument");return -1n;}function read(value=initialize(),other=0n){if(other<0n)throw new RangeError("negative");return value+other;}const saved=read;console.log(saved(undefined,input()));',lines:['argument','default'],error:{name:'RangeError',message:'negative'}}
];
const revision='07c66cdb',history=['js/src/translation/check.js','js/src/translation/javascript-infer.js','js/src/translation/sequential-parameter-defaults.js','js/src/typed-program-checked-projection.js','rust/src/translation/check/callables.rs','rust/src/translation/check/expressions/calls.rs','rust/src/translation/check/expressions/calls/sequential_defaults.rs','rust/src/translation/javascript/infer/solver.rs','rust/src/translation/javascript/infer/callables.rs','js/tests/translation-defaults.test.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/named-default-callables.lino',import.meta.url),{schemaVersion:1,model:'named callable value defaults through existing initialization 1',sourceLanguage:'JavaScript',cases,traces,history,refusals:[],source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},assertions:['actualNamedFunctionValueInitializationInterface','defaultMetadataAndAlreadyBodyInitializedMarkersSeparated','allArgumentsThenOrderedDefaultsThenOriginalGuards','higherOrderReturnedRecordMapAndArrayFromInvocations','exactPresentDomainAndNullishTagPreservingPolicyWidening','projectedCoreCallsDoNotInitializeSourceDefaultsAgain','completePredecessorsBothDefinitionsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Real default references reuse actual declaration initialization; existing body-entry markers and references without real defaults retain their prior path. Optional policy conversion requires identical present domains and a wider absence policy. General present-type unions and variance, reflection/identity/this/arguments, asynchronous behavior, complete parameter environments and all previous requirements remain obligations.'});
