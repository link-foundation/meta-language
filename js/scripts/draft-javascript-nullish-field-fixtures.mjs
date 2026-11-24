// Declarations only. No field read, typecheck, emitter, oracle or test runs.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,value=null,absence=null,field='value'){
 const abort=absence===null?null:"Cannot read properties of "+absence+" (reading '"+field+"')";
 cases.push({name,source,entry:'nullishFieldCase'+cases.length,value,abort,...(abort===null?{}:{errorClass:'TypeError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});
}
add('literal null ordinary read','export function compute(){return null.value;}',null,'null');
add('literal undefined ordinary read','export function compute(){return undefined.value;}',null,'undefined');
add('Unit undefined ordinary read','export function compute(){return (void 3).value;}',null,'undefined');
add('annotated Boolean abrupt result','/** @returns {boolean} */export function compute(){return null.value;}',null,'null');
add('annotated String abrupt result','/** @returns {string} */export function compute(){return undefined.value;}',null,'undefined');
for(const [absence,expression]of[['null','null'],['undefined','undefined']]){
 add('absent nullable record '+absence,'export function compute(){const receiver=false?{value:4}:'+expression+';return receiver.value;}',null,absence);
 add('present nullable record '+absence,'export function compute(){const receiver=true?{value:4}:'+expression+';return receiver.value;}','4');
 add('nullable array length '+absence,'export function compute(){const receiver=false?[1,2]:'+expression+';return receiver.length;}',null,absence,'length');
 add('nullable String length '+absence,'export function compute(){const receiver=false?"a😀":'+expression+';return receiver.length;}',null,absence,'length');
 add('returned nullish stays abrupt inside present optional call '+absence,'export function compute(){const callback=true?(()=>'+expression+'):null;return callback?.().value??7;}',null,absence);
}
add('both absence alternatives select null message','export function compute(){const receiver=true?null:undefined;return receiver.value;}',null,'null');
add('both absence alternatives select undefined message','export function compute(){const receiver=false?null:undefined;return receiver.value;}',null,'undefined');
add('nullable nested field preserves its own absence','export function compute(){const receiver={nested:false?{value:3}:undefined};return receiver.nested.value;}',null,'undefined');
add('absent outer optional call skips ordinary field read','export function compute(){const callback=false?(()=>({value:3})):null;return callback?.().value??7;}','7');
add('parentheses finish chain before ordinary field read','export function compute(){const callback=false?(()=>({value:3})):null;return (callback?.()).value;}',null,'undefined');
add('parenthesized optional field has an ordinary outer read','export function compute(){const receiver=false?{nested:{value:3}}:null;return (receiver?.nested).value;}',null,'undefined');
add('present false property remains false','export function compute(){const receiver=true?{value:false}:null;return receiver.value===false;}','true');
add('present zero property remains zero','export function compute(){const receiver=true?{value:0}:undefined;return receiver.value;}','0');
add('present empty property remains empty','export function compute(){const receiver=true?{value:""}:null;return receiver.value;}','');
add('present null property is a value','export function compute(){const receiver=true?{value:null}:null;return receiver.value===null;}','true');
add('present undefined property is a value','export function compute(){const receiver=true?{value:undefined}:null;return receiver.value===undefined;}','true');
add('nullable String length keeps UTF16','export function compute(){const receiver=true?"😀":null;return receiver.length;}','2');
add('nullable empty Set size remains zero','export function compute(){const receiver=true?new Set([]):null;return receiver.size;}','0');
add('absent Map size throws','export function compute(){const receiver=false?new Map([["key",1]]):null;return receiver.size;}',null,'null','size');
add('private capture name remains independent','export function compute(){const ml_nullish_field_receiver1=7;const receiver=true?{value:3}:null;return receiver.value+ml_nullish_field_receiver1;}','10');
add('Unicode property name remains exact','export function compute(){const receiver=false?{𐐀:3}:null;return receiver.𐐀;}',null,'null','𐐀');
add('null coalescing does not swallow ordinary property errors','export function compute(){const receiver=false?{value:3}:null;return receiver.value??7;}',null,'null');
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n');
writeProjectData(new URL('../../parity/fixtures/javascript-nullish-fields.lino',import.meta.url),{
 schemaVersion:1,model:'ordinary nullish field source results and scoped errors 1',sourceLanguage:'JavaScript',cases,source,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:['https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-requireobjectcoercible','https://raw.githubusercontent.com/nodejs/node/v22.15.0/deps/v8/src/common/message-template.h','https://raw.githubusercontent.com/nodejs/node/v24.0.0/deps/v8/src/common/message-template.h'],
 assertions:['ordinaryReadNeverBecomesOptionalSkip','actualNullAndUndefinedDistinguished','presentValuesNeverFilteredByTruthiness','nullishFailurePolymorphicResult','exactUtf16MessageClassAndPresence','parenthesesEndOptionalChain','sourceAstAndNativeValueParity','completeRuleGraphsBothCodecsFourEmissions','allEarlierOptionalChainCasesRetained'],
 boundary:'Future inputs and independent source expectations only. V8 diagnostic wording is qualified to the declared Node profile; other engines are uncovered. Methods, computed-property keys/protocols, proxies/accessors, unknown fields/prototypes, complete abrupt/unreachable inference and all prior requirements remain work. No represented source, oracle, fixture control or code test ran.',
});
