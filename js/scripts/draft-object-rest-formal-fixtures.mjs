// Deferred actual callable sources and retained predecessor controls only.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,declaration,body,value){const ordinal=cases.length,entry='objectRestFormalCase'+ordinal,owner='objectRestFormalOwner'+ordinal;cases.push({name,source:(declaration+'export function compute(){'+body+'}').replaceAll('OWNER',owner),entry,value,abort:null});}
add('named object rest copies its actual fields','function OWNER({...rest}){return rest.value;}','return OWNER({value:3});','3');
add('named selected property is excluded','function OWNER({left,...rest}){return String(left)+":"+String(rest.right);}','return OWNER({left:3,right:4});','3:4');
add('aliased selected property uses its original key','function OWNER({left:renamed,...rest}){return String(renamed)+":"+String(rest.renamed);}','return OWNER({left:3,renamed:4});','3:4');
add('missing field defaults precede the copy','function OWNER({missing=3,...rest}){return String(missing)+":"+Object.keys(rest).join(",");}','return OWNER({value:4});','3:value');
add('ordinary argument slots keep their actual positions','function OWNER(before,{value,...rest},after){return before+value+rest.other+after;}','return OWNER(1,{value:2,other:3},4);','10');
add('separate object formals keep independent exclusions','function OWNER({left,...first},{right,...second}){return first.right+second.left;}','return OWNER({left:1,right:2},{left:3,right:4});','5');
add('repeated property reads retain distinct local names','function OWNER({value:first,value:second,...rest}){return first+second+rest.other;}','return OWNER({value:3,other:4});','10');
add('constant arrow formals retain object rest','const OWNER=({value,...rest})=>value+rest.other;','return OWNER({value:3,other:4});','7');
add('constant function expressions retain object rest','const OWNER=function({value,...rest}){return value+rest.other;};','return OWNER({value:3,other:4});','7');
add('namespace methods retain object rest','const OWNER={read({value,...rest}){return value+rest.other;}};','return OWNER.read({value:3,other:4});','7');
add('nested namespace methods retain object rest','const OWNER={Nested:{read({...rest}){return rest.value;}}};','return OWNER.Nested.read({value:3});','3');
add('callback arrow formals retain object rest','','return [{value:3},{value:4}].map(({...rest})=>rest.value+1).join("|");','4|5');
add('callback function expressions retain object rest','','return [{value:3},{value:4}].map(function({...rest}){return rest.value+1;}).join("|");','4|5');
add('retained exact former callback rest refusal','','return [{value:3}].map(({...rest})=>rest.value).join("|");','3');
add('nested closures retain actual outer captures','','const offset=7;const read=({value,...rest})=>offset+value+rest.other;return read({value:3,other:4});','14');
add('mutable rest parameter bindings do not modify their source','function OWNER({value,...rest}){rest={other:value+rest.other};return rest.other;}','const source={value:3,other:4};return String(OWNER(source))+":"+String(source.other);','7:4');
add('a rest parameter has fresh outer identity','function OWNER(source){const read=({...rest})=>rest===source;return read(source);}','return String(OWNER({value:3}));','false');
add('formal rest preserves actual runtime field order','function OWNER({selected,...rest}){return Object.keys(rest).join(",");}','return OWNER({zeta:1,selected:2,alpha:3});','zeta,alpha');
add('empty formal copies keep an empty own-field layout','function OWNER({...rest}){return Object.keys(rest).join(",");}','return OWNER({});','');
add('rest keeps shallow shared child identity','function OWNER(source){const read=({...rest})=>rest.child===source.child;return read(source);}','return String(OWNER({child:{value:3}}));','true');
const reason=code=>'record parameter binding: a nonempty flat required object pattern needs distinct lexical names; defaults, arrays, nested/rest/computed patterns are separate obligations (reason '+code+')';
const refusals=[
 {name:'formal rest is final in its actual object pattern',source:'function f({...rest,value}){return value;}',kind:'unsupported',reason:reason(6)},
 {name:'formal rest has no initializer',source:'function f({...rest={}}){return rest;}',kind:'unsupported',reason:reason(6)},
 {name:'formal rest has no trailing comma inside its pattern',source:'function f({...rest,}){return rest;}',kind:'unsupported',reason:reason(6)},
 {name:'a default cannot read the later rest binding',source:'function f({value=rest,...rest}){return value;}',kind:'unsupported',reason:'record formal default reading parameter rest: the current default profile requires a closed parameter initialization environment'},
 {name:'rest names remain distinct across actual argument slots',source:'function f({...rest},rest){return rest;}',kind:'unsupported',reason:reason(8)},
 {name:'a formal object rest list remains non-simple for strict directives',source:'function f({...rest}){"use strict";return rest;}',kind:'unsupported',reason:'record parameter strict directive: a non-simple formal parameter list cannot have a Use Strict Directive in its function body'},
];
const revision='2c5f652e',history=['parity/fixtures/record-closure-parameters.lino','js/scripts/draft-record-closure-parameter-fixtures.mjs','js/tests/translation-record-closure-parameters.test.js','rust/tests/unit/translation_javascript_record_parameters.rs','js/src/translation/javascript-record-parameter-rules.js','js/src/translation/javascript-record-parameters.js','rust/src/translation/javascript/record_parameters.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/object-rest-formals.lino',import.meta.url),{schemaVersion:1,model:'actual closed own-field object rest formal parameters 1',sourceLanguage:'JavaScript',cases,refusals,history,versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),assertions:['actualNamedConstantNamespaceAndCallbackContexts','separateLegacyRequiredAndDefaultPlanContracts','originalExclusionsAndArgumentSlots','orderedDefaultsBeforeFreshShallowCopy','mutableActualRestBindingWithoutSourceWrites','runtimeOwnFieldOrderAndEmptyLayouts','retainedFormerCallbackRestInputAndValue','uninitializedRestBindingAndNonSimpleDirectiveRules','completeDefinitionBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Final object rest in existing synchronous formal contexts reuses the lexical complete own-field copy operation. Whole-parameter defaults, mixed recursive binding patterns, async/generator environments, symbol/getter/proxy/inherited/primitive ToObject protocols and every earlier requirement remain implementation obligations.'});
