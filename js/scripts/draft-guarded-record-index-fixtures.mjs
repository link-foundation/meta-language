// Declare original source values and refusals without parsing or executing them.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,table,key,value){const owner='guardedRecordOwner'+cases.length,entry='guardedRecordCase'+cases.length;cases.push({name,source:'function '+owner+'(table,key){'+body+'}export function compute(){return '+owner+'('+table+','+key+');}',entry,value,abort:null});}
const read='return Object.hasOwn(table,key)?table[key]??99:99;';
add('first own field selected by a runtime String',read,'{z:3,a:4}','"z"','3');
add('canonical second field selected by runtime String',read,'{z:3,a:4}','"a"','4');
add('absent key retains the false branch',read,'{z:3,a:4}','"missing"','99');
add('inherited prototype property is not read',read,'{z:3,a:4}','"toString"','99');
add('prototype slot is not an invented own field',read,'{z:3,a:4}','"__proto__"','99');
add('actual own prototype spelling remains ordinary data',read,'{["__proto__"]:3,constructor:4}','"__proto__"','3');
add('actual own constructor name remains ordinary data',read,'{["__proto__"]:3,constructor:4}','"constructor"','4');
add('empty own domain preserves its false branch',read,'{}','"missing"','99');
add('Unicode name search uses exact UTF16 units',read,'{"😀":3,a:4}','"😀"','3');
add('numeric String keys remain property names',read,'{"2":3,a:4}','"2"','3');
add('negative-zero payload survives a named projection','return Object.hasOwn(table,key)?1/(table[key]??1):99;','{z:-0,a:1}','"z"','-Infinity');
add('own undefined payload remains undefined','return Object.hasOwn(table,key)?String(table[key]):"missing";','{z:undefined,a:undefined}','"z"','undefined');
add('own null payload remains null','return Object.hasOwn(table,key)?String(table[key]):"missing";','{z:null,a:null}','"z"','null');
add('homogeneous text payloads retain their exact type','return Object.hasOwn(table,key)?table[key]??"absent":"missing";','{z:"left",a:"right"}','"z"','left');
add('negated early return guards its complete continuation','if(!Object.hasOwn(table,key))return 99;return table[key]??99;','{z:3,a:4}','"z"','3');
add('negated guard false arm does not read inherited values','if(!Object.hasOwn(table,key))return 99;return table[key]??99;','{z:3,a:4}','"constructor"','99');
add('positive and guards its actual right operand','return Object.hasOwn(table,key)&&Boolean(table[key]);','{z:3,a:4}','"z"','true');
add('false and skips inherited-property lookup','return Object.hasOwn(table,key)&&Boolean(table[key]);','{z:3,a:4}','"toString"','false');
add('negative or guards its actual right operand','return !Object.hasOwn(table,key)||Boolean(table[key]);','{z:3,a:0}','"a"','false');
add('true or skips inherited-property lookup','return !Object.hasOwn(table,key)||Boolean(table[key]);','{z:3,a:0}','"constructor"','true');
add('own-key map reads original property order','return Object.keys(table).map(key=>key+":"+String(table[key]??99)).join("|");','{z:3,a:4}','"unused"','z:3|a:4');
add('own-key map retains numeric property order','return Object.keys(table).map(key=>key+":"+String(table[key]??99)).join("|");','{"2":3,"1":4,z:5}','"unused"','1:4|2:3|z:5');
add('own-key filter reads corresponding values','return Object.keys(table).filter(key=>(table[key]??0)>3).join("|");','{z:3,a:4}','"unused"','a');
add('own-key some reads corresponding values','return Object.keys(table).some(key=>(table[key]??0)===4);','{z:3,a:4}','"unused"','true');
add('own-key every reads corresponding values','return Object.keys(table).every(key=>(table[key]??0)>2);','{z:3,a:4}','"unused"','true');
add('own-key find retains optional absence','return Object.keys(table).find(key=>(table[key]??0)>9)??"missing";','{z:3,a:4}','"unused"','missing');
add('own-key find selects its actual key','return Object.keys(table).find(key=>(table[key]??0)>3)??"missing";','{z:3,a:4}','"unused"','a');
add('own-key findIndex retains receiver position','return Object.keys(table).findIndex(key=>(table[key]??0)>3);','{z:3,a:4}','"unused"','1');
add('own-key reduce uses its second callback binder','return Object.keys(table).reduce((total,key)=>total+(table[key]??0),0);','{z:3,a:4}','"unused"','7');
add('own-key reduce keeps accumulator and index separate','return Object.keys(table).reduce((total,key,index)=>total+String(index)+":"+String(table[key]??0)+"|","");','{z:3,a:4}','"unused"','0:3|1:4|');
add('own-key flatMap retains ordinary array results','return Object.keys(table).flatMap(key=>[table[key]??0]).join("|");','{z:3,a:4}','"unused"','3|4');
add('Array.from mapper retains the immediate key producer','return Array.from(Object.keys(table),key=>key+":"+String(table[key]??0)).join("|");','{z:3,a:4}','"unused"','z:3|a:4');
add('nested callback captures its original own key','return Object.keys(table).map(key=>[1,2].map(scale=>(table[key]??0)*scale).join(",")).join("|");','{z:3,a:4}','"unused"','3,6|4,8');
add('own-key nullable payloads retain their source values','return Object.keys(table).map(key=>String(table[key])).join("|");','{z:null,a:null}','"unused"','null|null');
add('own-key UTF16 names preserve exact search units','return Object.keys(table).map(key=>key+":"+String(table[key]??0)).join("|");','{"😀":3,a:4}','"unused"','😀:3|a:4');
add('a callback without parameters needs no membership fact','return Object.keys(table).map(()=>3).join("|");','{z:3,a:4}','"unused"','3|3');
const reason='dynamic record index: an actual positive Object.hasOwn branch must guard unchanged local record/String binders; inherited and exotic property domains need their complete protocol';
const refusals=[
 ['positive or does not guard its right operand','function f(table,key){return Object.hasOwn(table,key)||Boolean(table[key]);}console.log(f({z:3},"z"));'],
 ['negative and does not guard its right operand','function f(table,key){return !Object.hasOwn(table,key)&&Boolean(table[key]);}console.log(f({z:3},"z"));'],
 ['unguarded lookup has no own-property proof','function f(table,key){return table[key]??99;}console.log(f({z:3},"z"));'],
 ['the false branch has no own-property proof','function f(table,key){return Object.hasOwn(table,key)?99:table[key]??99;}console.log(f({z:3},"z"));'],
 ['a different local key has no own-property proof','function f(table,key){const other=key;return Object.hasOwn(table,key)?table[other]??99:99;}console.log(f({z:3},"z"));'],
 ['a nested key shadow invalidates the branch fact','function f(table,key){if(Object.hasOwn(table,key)){const key="toString";return table[key]??99;}return 99;}console.log(f({z:3},"z"));'],
 ['extra operands do not acquire a stronger guard profile','function f(table,key){return Object.hasOwn(table,key,0)?table[key]??99:99;}console.log(f({z:3},"z"));'],
 ['a saved key array needs its own origin and alias proof','function f(table){const keys=Object.keys(table);return keys.map(key=>table[key]??0).join("|");}console.log(f({z:3}));'],
 ['an extra key-producer operand supplies no callback fact','function f(table){return Object.keys(table,0).map(key=>table[key]??0).join("|");}console.log(f({z:3}));'],
 ['a callback key alias needs its own binder proof','function f(table){return Object.keys(table).map(key=>{const other=key;return table[other]??0;}).join("|");}console.log(f({z:3}));'],
 ['an exposed map receiver can invalidate later own keys','function f(table){return Object.keys(table).map((key,index,keys)=>table[key]??0).join("|");}console.log(f({z:3}));'],
 ['an exposed reduce receiver can invalidate later own keys','function f(table){return Object.keys(table).reduce((total,key,index,keys)=>total+(table[key]??0),0);}console.log(f({z:3}));'],
].map(([name,source])=>({name,source,kind:'unsupported',reason}));
writeProjectData(new URL('../../parity/fixtures/javascript-guarded-record-indices.lino',import.meta.url),{schemaVersion:1,model:'closed homogeneous record reads under actual own-property branch facts 1',sourceLanguage:'JavaScript',cases,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),refusals,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['actualUnshadowedHasOwnGuard','positiveAndNegatedContinuationScopes','exactImmutableBinderIdentityAndRebindingInvalidation','immediateOwnKeyProducerAndExactCallbackNode','sharedCallbackRolesAndUnexposedReceiver','canonicalNamedFieldsWithSourceDefinedUTF16Search','ownVersusInheritedPropertyDistinction','nullishPayloadPreservationAndOptionalResult','unchangedOperandEvaluationOrder','completeSourceDefinitionTwoCodecsFourEmissionsNativeProducts'],
 boundary:'Unexecuted future controls. Direct two-operand hasOwn conditions over unchanged local closed-record/String functional binders create branch facts. An immediate one-operand Object.keys producer may additionally supply membership to its actual inline callback element binder, using shared callback roles and withholding facts if any callback parameter shadows the owner or exposes the live key array. Saved/aliased key arrays, indirect callbacks, mutable key cells, extra/effectful or aliased guard operands, heterogeneous values, arbitrary inherited/exotic protocols and other full-language requirements retain obligations. No new Core node or source-provenance facet is introduced.'});
