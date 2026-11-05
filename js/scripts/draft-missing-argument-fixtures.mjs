// Complete future controls and predecessor source bytes; execute none.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value,prefix=''){const ordinal=cases.length;cases.push({name,entry:'missingArgumentCase'+ordinal,source:(prefix+'export function compute(){'+body+'}').replaceAll('OWNER','missingArgumentOwner'+ordinal),value,abort:null});}
add('an unused inferred required argument receives undefined','return OWNER();','7','function OWNER(value){return 7;}');
add('String sees an omitted inferred argument','return OWNER();','undefined','function OWNER(value){return String(value);}');
add('Number sees an omitted inferred argument','return OWNER();','NaN','function OWNER(value){return Number(value);}');
add('Boolean sees an omitted inferred argument','return OWNER();','false','function OWNER(value){return Boolean(value);}');
add('typeof sees an omitted inferred argument','return OWNER();','undefined','function OWNER(value){return typeof value;}');
add('an omitted required input reaches a nullish fallback','return OWNER();','7','function OWNER(value){return value??7;}');
add('an omitted required input is strictly undefined','return OWNER();','true','function OWNER(value){return value===undefined;}');
add('an omitted required input is distinct from null','return OWNER();','false','function OWNER(value){return value===null;}');
add('a required suffix remains undefined after a supplied prefix','return OWNER(3);','3|undefined','function OWNER(first,last){return String(first)+"|"+String(last);}');
add('an earlier omitted input reaches a dependent written default','return OWNER();','7','function OWNER(first,last=first??7){return last;}');
add('a written default retains its own initializer beside an omitted input','return OWNER();','undefined|7','function OWNER(first,last=7){return String(first)+"|"+String(last);}');
add('several omitted inputs each retain undefined','return OWNER();','undefined|undefined','function OWNER(first,last){return String(first)+"|"+String(last);}');
add('a present Number and an omitted argument share the actual optional domain','return OWNER(3)+"|"+OWNER();','3|undefined','function OWNER(value){return String(value);}');
add('reversed call order retains present and omitted inputs','return OWNER()+"|"+OWNER(3);','undefined|3','function OWNER(value){return String(value);}');
add('a present false is not replaced by implicit undefined','return OWNER(false)+"|"+OWNER();','false|undefined','function OWNER(value){return String(value);}');
add('present empty text is not replaced by implicit undefined','return OWNER("")+"|"+OWNER();','|undefined','function OWNER(value){return String(value);}');
add('a stored named reference receives a missing input','const read=OWNER;return read();','undefined','function OWNER(value){return String(value);}');
add('a local arrow receives a missing input','const read=value=>String(value);return read();','undefined');
add('a function expression receives a missing input','const read=function(value){return String(value);};return read();','undefined');
add('an ordinary record method receives a missing input','const object={read(value){return String(value);}};return object.read();','undefined');
add('a record property function receives a missing input','const object={read:value=>String(value)};return object.read();','undefined');
add('a returned function receives a missing input','return OWNER()();','undefined','function OWNER(){return value=>String(value);}');
add('a higher-order named callback receives a missing input','return OWNER(read=>read());','undefined','function OWNER(use){return use(value=>String(value));}');
add('an optional declared input retains its existing omission route','return OWNER();','undefined','/** @param {number|undefined} value @returns {string} */function OWNER(value){return String(value);}');
add('ordinary Number multiplication receives undefined as NaN','return OWNER();','NaN','function OWNER(value){return value*2;}');
add('a BigInt value remains distinct from missing input during String conversion','return OWNER(3n)+"|"+OWNER();','3|undefined','function OWNER(value){return String(value);}');
const traces=[
 {name:'supplied arguments still precede omitted input body execution',source:'function argument(){console.log("argument");return 3;}function read(first,last){console.log("body");return String(first)+"|"+String(last);}console.log(read(argument()));',lines:['argument','body','3|undefined'],error:null},
 {name:'written initialization remains ordered beside implicit undefined',source:'function initialize(){console.log("default");return 7;}function read(first,last=initialize()){console.log(String(first));return last;}console.log(read());',lines:['default','undefined','7'],error:null},
 {name:'computed receiver evaluation precedes its omitted input body',source:'function factory(){console.log("callee");return value=>{console.log("body");return String(value);};}console.log(factory()());',lines:['callee','body','undefined'],error:null}
];
const revision='fd3d747f',history=['js/src/translation/javascript-infer.js','js/src/translation/optional-callables.js','js/src/translation/optional-callable-rules.js','rust/src/translation/javascript/infer/products.rs','rust/src/translation/javascript/infer/callables.rs','rust/src/translation/javascript/infer/callables/optional_arguments.rs','rust/src/translation/javascript/infer/optional_values.rs','rust/src/translation/javascript/infer/methods.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/missing-arguments.lino',import.meta.url),{schemaVersion:1,model:'actual implicit JavaScript undefined inputs for inferred callable parameters 1',sourceLanguage:'JavaScript',cases,traces,history,refusals:[],source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},assertions:['inferredMissingInputsAreActualUndefinedTerms','optionalDomainBindingReusesOriginalVariableOwners','writtenDefaultsAndDeclaredDomainsRetainTheirRoutes','namedComputedReturnedAndRecordCallableInputs','suppliedArgumentEffectsPrecedeBodyAndInitializers','completePredecessorsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Infers actual omission only for original variable parameter owners without written defaults. Existing supplied-argument coercions and default initialization remain authoritative. Declared nonoptional parameters, general variance, full callable environments, arbitrary unions and unsupported coercions remain explicit obligations rather than fabricated successful translations.'});
