// Future source products and full immutable predecessors; execute none.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value,prefix=''){const ordinal=cases.length;cases.push({name,entry:'absenceConversionCase'+ordinal,source:(prefix+'export function compute(){'+body+'}').replaceAll('OWNER','absenceConversionOwner'+ordinal),value,abort:null});}
const describe='/** @param {number|null|undefined} value @returns {string} */function OWNER(value){return value===null?"null":value===undefined?"undefined":"present";}';
add('a stored missing field reaches its actual optional numeric parameter','const value={}.missing;return OWNER(value);','3','function OWNER(value){return value??3;}');
add('a stored missing field reaches an optional String parameter','const value={}.missing;return OWNER(value);','fallback','function OWNER(value){return value??"fallback";}');
add('a stored missing field reaches an optional Boolean parameter','const value={}.missing;return OWNER(value);','true','function OWNER(value){return value??true;}');
add('a stored missing field reaches a whole record default','const value={}.missing;const read=({answer}={answer:3})=>answer;return read(value);','3');
add('an array joins an absent field with its actual Number domain','const value={}.missing;return [value,7].map(item=>item??0).join("|");','0|7');
add('an array joins an absent field with its actual String domain','const value={}.missing;return [value,"present"].map(item=>item??"fallback").join("|");','fallback|present');
add('an array joins an absent field with its actual Boolean domain','const value={}.missing;return [value,false].map(item=>String(item??true)).join("|");','true|false');
add('two absent array slots retain their later Number context','const first={}.missing;const second={}.missing;return [first,second,7].map(item=>item??0).join("|");','0|0|7');
add('a contextual optional parameter retains actual undefined','const value=undefined;return OWNER(value);','undefined',describe);
add('a contextual optional parameter retains actual null','const value=null;return OWNER(value);','null',describe);
add('a contextual optional parameter retains a missing property','const value={}.missing;return OWNER(value);','undefined',describe);
add('a stored nullish union retains its selected null','const value=true?null:undefined;return OWNER(value);','null',describe);
add('a stored nullish union retains its selected undefined','const value=false?null:undefined;return OWNER(value);','undefined',describe);
add('a real Number does not become absence','return OWNER(0);','present',describe);
add('String conversion retains stored null in the target domain','const value=null;return OWNER(value);','null','/** @param {number|null|undefined} value @returns {string} */function OWNER(value){return String(value);}');
add('String conversion retains stored undefined in the target domain','const value=undefined;return OWNER(value);','undefined','/** @param {number|null|undefined} value @returns {string} */function OWNER(value){return String(value);}');
add('an optional array target retains stored undefined','const value={}.missing;return OWNER(value);','3','function OWNER(value){return (value??[3]).join("|");}');
add('an optional record target retains stored undefined','const value={}.missing;return OWNER(value);','3','function OWNER(value){return (value??{answer:3}).answer;}');
const traces=[
 {name:'a stored absent receiver still runs once before its consumer',source:'function receiver(){console.log("receiver");return {};}function read(value){console.log("body");return value??3;}const value=receiver().missing;console.log(read(value));',lines:['receiver','body','3'],error:null},
 {name:'tag-preserving conversion retains stored null effects',source:'function input(){console.log("input");return null;}/** @param {number|null|undefined} value @returns {string} */function read(value){return value===null?"null":"undefined";}const value=input();console.log(read(value));',lines:['input','null'],error:null}
];
const revision='43fa637b',history=['js/src/translation/check.js','js/src/translation/javascript-infer.js','js/src/translation/absent-record-properties.js','js/src/translation/absence-only-rules.js','rust/src/translation/check.rs','rust/src/translation/javascript/infer/records.rs','parity/fixtures/absent-record-properties.lino','js/tests/translation-absent-record-properties.test.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/absence-conversions.lino',import.meta.url),{schemaVersion:1,model:'tag-preserving JavaScript absence-only optional conversion 1',sourceLanguage:'JavaScript',cases,traces,history,refusals:[],source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},assertions:['actualAbsentValuesRetainEffectsAndNullishTags','contextualPresentDomainsAreInferenceVariablesNotUnitConstraints','storedMissingValuesEnterActualOptionalCallsAndArrays','presentUnitConvertsToUndefinedWithoutFabricatingPayloads','narrowerAbsencePoliciesAndOtherLanguagesRetainErrors','fullPredecessorSourcesBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted controls. Only the source JavaScript absence-only domain converts to an optional target admitting every represented nullish value; null-only targets need stronger actual-variant evidence when a present Unit is possible. No present payload is invented. General unions, full environments, concept/grammar/knowledge/inference and all preceding requirements remain obligations.'});
