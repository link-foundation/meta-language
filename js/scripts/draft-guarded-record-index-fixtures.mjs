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
const reason='dynamic record index: an actual positive Object.hasOwn branch must guard unchanged local record/String binders; inherited and exotic property domains need their complete protocol';
const refusals=[
 ['positive or does not guard its right operand','function f(table,key){return Object.hasOwn(table,key)||Boolean(table[key]);}console.log(f({z:3},"z"));'],
 ['negative and does not guard its right operand','function f(table,key){return !Object.hasOwn(table,key)&&Boolean(table[key]);}console.log(f({z:3},"z"));'],
 ['unguarded lookup has no own-property proof','function f(table,key){return table[key]??99;}console.log(f({z:3},"z"));'],
 ['the false branch has no own-property proof','function f(table,key){return Object.hasOwn(table,key)?99:table[key]??99;}console.log(f({z:3},"z"));'],
 ['a different local key has no own-property proof','function f(table,key){const other=key;return Object.hasOwn(table,key)?table[other]??99:99;}console.log(f({z:3},"z"));'],
 ['a nested key shadow invalidates the branch fact','function f(table,key){if(Object.hasOwn(table,key)){const key="toString";return table[key]??99;}return 99;}console.log(f({z:3},"z"));'],
 ['extra operands do not acquire a stronger guard profile','function f(table,key){return Object.hasOwn(table,key,0)?table[key]??99:99;}console.log(f({z:3},"z"));'],
].map(([name,source])=>({name,source,kind:'unsupported',reason}));
writeProjectData(new URL('../../parity/fixtures/javascript-guarded-record-indices.lino',import.meta.url),{schemaVersion:1,model:'closed homogeneous record reads under actual own-property branch facts 1',sourceLanguage:'JavaScript',cases,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),refusals,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['actualUnshadowedHasOwnGuard','positiveAndNegatedContinuationScopes','exactImmutableBinderIdentityAndRebindingInvalidation','canonicalNamedFieldsWithSourceDefinedUTF16Search','ownVersusInheritedPropertyDistinction','nullishPayloadPreservationAndOptionalResult','unchangedOperandEvaluationOrder','completeSourceDefinitionTwoCodecsFourEmissionsNativeProducts'],
 boundary:'Unexecuted future controls. Only direct two-operand hasOwn conditions over unchanged local closed-record/String functional binders create branch facts. Mutable key cells, extra/effectful or aliased guard operands, heterogeneous values, arbitrary inherited/exotic protocols and other full-language requirements retain obligations. No new Core node or source-provenance facet is introduced.'});
