// Source declarations and exact prior history only, never behavioral execution.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,value=null,absence=null,field='invoke',errorClass='TypeError',errorMessage=null){
 const abort=errorMessage??(absence===null?null:'Cannot read properties of '+absence+" (reading '"+field+"')");
 cases.push({name,source,entry:'optionalPropertyCase'+cases.length,value,abort,...(abort===null?{}:{errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))})});
}
add('exact original property refusal is now admitted','export function compute(){const value={method(value){return value;}};return value.method?.(1);}','1');
for(const [key,call]of[['field','receiver.invoke'],['literal bracket','receiver["invoke"]']]){
 add(key+' present function','export function compute(){const receiver={invoke:value=>value+1};return '+call+'?.(2);}','3');
 add(key+' nullable present function','export function compute(){const receiver={invoke:true?(value=>value+1):null};return '+call+'?.(2)??7;}','3');
 add(key+' absent undefined function','export function compute(){const receiver={invoke:undefined};let count=0;return ('+call+'?.(count+=1)??7)*10+count;}','70');
 add(key+' absent null function','export function compute(){const receiver={invoke:null};let count=0;return ('+call+'?.(count+=1)??7)*10+count;}','70');
 add(key+' optional function returns actual null','export function compute(){const receiver={invoke:true?(()=>null):undefined};return '+call+'?.()===null;}','true');
 add(key+' optional function returns actual undefined','export function compute(){const receiver={invoke:true?(()=>undefined):null};return '+call+'?.()===undefined;}','true');
 add(key+' optional function retains false','export function compute(){const receiver={invoke:true?(()=>false):null};return '+call+'?.()===false;}','true');
 add(key+' optional function retains zero','export function compute(){const receiver={invoke:true?(()=>0):null};return '+call+'?.()??7;}','0');
 add(key+' nullish property base is not an absent function','export function compute(){const receiver=false?{invoke:value=>value+1}:null;return '+call+'?.(2);}',null,'null');
}
add('optional object and optional property both skip','export function compute(){const receiver=false?{invoke:value=>value+1}:null;let count=0;return (receiver?.invoke?.(count+=1)??7)*10+count;}','70');
add('optional object present and optional property absent skips','export function compute(){const receiver=true?{invoke:false?(value=>value+1):undefined}:null;let count=0;return (receiver?.invoke?.(count+=1)??7)*10+count;}','70');
add('optional object and optional property both present','export function compute(){const receiver=true?{invoke:true?(value=>value+1):null}:undefined;return receiver?.invoke?.(2)??7;}','3');
add('continuous property call suffix stays guarded','export function compute(){const receiver={invoke:false?(value=>({value})):null};return receiver.invoke?.(2).value??7;}','7');
add('present property result continues its field read','export function compute(){const receiver={invoke:true?(value=>({value})):null};return receiver.invoke?.(2).value??7;}','2');
add('parentheses end optional property call skip','export function compute(){const receiver={invoke:false?(value=>({value})):null};return (receiver.invoke?.(2)).value;}',null,'undefined','value');
add('indexed callable retains exact slot evaluation','export function compute(){const callbacks=[value=>value+1];const values=[];return (callbacks[(values.push(1),0)]?.((values.push(2),3))??7)*10+values.length;}','42');
add('missing array callable skips arguments','export function compute(){const callbacks=[value=>value+1];let count=0;return (callbacks[1]?.(count+=1)??7)*10+count;}','70');
add('loaded function captured before record reassignment','export function compute(){let receiver={invoke:value=>value+1};const replacement={invoke:value=>value+10};const result=receiver.invoke?.((receiver=replacement,2));return result*100+receiver.invoke(2);}','312');
add('nullable loaded function captured before field assignment','export function compute(){const receiver={invoke:true?(value=>value+1):null};return receiver.invoke?.((receiver.invoke=null,2))??7;}','3');
add('receiver factory evaluates once','function makeReceiver(values){values.push(1);return {invoke:value=>value+1};}export function compute(){const values=[];return makeReceiver(values).invoke?.(2)*10+values.length;}','31');
const fail='/** @returns {number} */function failPropertyArgument(){throw new RangeError("property argument");}';
add('absent property skips abrupt argument',fail+'export function compute(){const receiver={invoke:false?(value=>value+1):null};return receiver.invoke?.(failPropertyArgument())??7;}','7');
add('present property retains abrupt argument',fail+'export function compute(){const receiver={invoke:true?(value=>value+1):null};return receiver.invoke?.(failPropertyArgument())??7;}',null,null,'invoke','RangeError','property argument');
add('nullish base property error precedes abrupt argument',fail+'export function compute(){const receiver=false?{invoke:value=>value+1}:null;return receiver.invoke?.(failPropertyArgument());}',null,'null');
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('failPropertyArgument','failPropertyArgument'+row.entry).replaceAll('makeReceiver','makeReceiver'+row.entry)).join('\n');
const revision='ee39d016';
const history=['js/scripts/generate-javascript-optional-call-fixtures.mjs','parity/fixtures/javascript-optional-calls.lino','js/tests/translation-javascript-optional-calls.test.js','js/src/translation/javascript-optional-call-rules.js','js/src/translation/javascript-optional-calls.js','rust/src/translation/check/javascript_optional_calls.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
 return{path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
});
writeProjectData(new URL('../../parity/fixtures/javascript-optional-property-calls.lino',import.meta.url),{
 schemaVersion:1,model:'receiver-independent optional property call source values 1',sourceLanguage:'JavaScript',cases,source,history,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:['https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-optional-chains','https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-evaluatecall'],
 assertions:['originalRefusalInputNowPositiveWithFullHistory','actualBaseAndKeyPropertyLoadBeforeNullishCallableGuard','loadedFunctionCapturedBeforeArgumentMutation','absentArgumentsStayLazy','exactArgumentSourceOrder','returnedAbsenceDistinctFromSkippedCall','continuousSuffixAndParenthesisBoundary','receiverIndependentFunctionDomainExplicit','nativeOriginalValuesSourceAstAndGraphs','twoCodecsFourEmissions'],
 boundary:'Unexecuted declarations. The existing Function domain has explicit arguments and captures, no hidden this. Source this, arbitrary foreign callable objects/prototypes/accessors, missing record fields, dynamic noncallable messages and every prior requirement remain.',
});
