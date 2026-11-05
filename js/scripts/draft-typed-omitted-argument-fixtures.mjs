// Future source products and immutable predecessor transport. Execute none.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,prefix,body,value){const ordinal=cases.length;cases.push({name,entry:'typedOmittedArgumentCase'+ordinal,source:(prefix+'export function compute(){'+body+'}').replaceAll('OWNER','typedOmittedArgumentOwner'+ordinal),value,abort:null});}
add('an inferred optional Number admits an omitted input','function OWNER(value){return value??3;}','return OWNER();','3');
add('a provided Number remains present','function OWNER(value){return value??3;}','return OWNER(7);','7');
add('an explicit undefined selects the same fallback','function OWNER(value){return value??3;}','return OWNER(undefined);','3');
add('null follows the original nullish fallback','function OWNER(value){return value??3;}','return OWNER(null);','3');
add('an omitted optional String keeps its real result type','function OWNER(value){return value??"fallback";}','return OWNER();','fallback');
add('an omitted optional Boolean keeps its real result type','function OWNER(value){return value??true;}','return OWNER();','true');
add('a later optional parameter may be omitted','function OWNER(first,second){return first+(second??4);}','return OWNER(3);','7');
add('a nontrailing ordinary default precedes a typed optional input','function OWNER(first=3,second){return first+(second??4);}','return OWNER();','7');
add('an earlier typed optional input precedes an ordinary default','function OWNER(first,second=4){return (first??3)+second;}','return OWNER();','7');
add('several actual optional domains can be absent together','function OWNER(first,second){return (first??3)+(second??4);}','return OWNER();','7');
add('a dependent default may read the actual initialized optional value','function OWNER(first,second=(first??3)+1){return second;}','return OWNER();','4');
add('a supplied prefix constrains only its actual parameter','function OWNER(first,second){return first+(second??4);}','return OWNER(3);','7');
add('a complete record fallback retains its field domain','function OWNER(value){return (value??{answer:3}).answer;}','return OWNER();','3');
add('a complete array fallback retains its element domain','function OWNER(value){return (value??[3,4]).join("|");}','return OWNER();','3|4');
add('a missing optional input survives a forward declaration','function OWNER(){return OWNERRead();}function OWNERRead(value){return value??"fallback";}','return OWNER();','fallback');
add('optional defaults preserve explicit zero','function OWNER(value){return value??3;}','return OWNER(0);','0');
add('optional defaults preserve explicit false','function OWNER(value){return value??true;}','return OWNER(false);','false');
add('optional defaults preserve explicit empty String','function OWNER(value){return value??"fallback";}','return OWNER("");','');
const traces=[
 {name:'a missing typed input does not hide the fallback effect',source:'function fallback(){console.log("fallback");return 3;}function read(value){console.log("body");return value??fallback();}console.log(read());',lines:['body','fallback','3'],error:null},
 {name:'a provided typed input keeps its fallback lazy',source:'function fallback(){console.log("fallback");return 3;}function read(value){console.log("body");return value??fallback();}console.log(read(7));',lines:['body','7'],error:null},
 {name:'ordinary defaults keep their original order around typed missing slots',source:'function initialize(){console.log("default");return 3;}function read(first=initialize(),second){console.log("body");return first+(second??4);}console.log(read());',lines:['default','body','7'],error:null}
];
const revision='0eb4b5a0',history=['js/src/translation/check.js','js/src/translation/javascript-infer.js','js/src/translation/optional-callables.js','js/src/translation/sequential-parameter-defaults.js','rust/src/translation/check/optional_callables.rs','rust/src/translation/check/expressions/calls.rs','rust/src/translation/check/expressions/calls/sequential_defaults.rs','rust/src/translation/javascript/infer/products.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/typed-omitted-arguments.lino',import.meta.url),{schemaVersion:1,model:'actual resolved optional domains on named JavaScript calls 1',sourceLanguage:'JavaScript',cases,traces,history,refusals:[],source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},assertions:['sharedActualUndefinedAdmissionForNamedAndComputedCalls','actualResultTypesRetainedBeforeOmissionResolution','suppliedPrefixConstraintsRetained','ordinaryDefaultAndOmittedOptionalSlotInterleaving','fallbackEffectsAndFalsyPresence','completeSourcesBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted declarations. Only actual resolved optional types admitting undefined gain omission, alongside original written defaults. No inferred required nonoptional type becomes optional by declaration. Missing nonoptional inputs, arbitrary signature widening, spreads, complete environments and all preceding requirements remain obligations.'});
