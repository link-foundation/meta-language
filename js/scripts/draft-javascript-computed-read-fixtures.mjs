// Input declarations and exact preceding definitions only. No represented code runs.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,value=null,absence=null,key='',errorClass='TypeError',errorMessage=null){
 const abort=errorMessage??(absence===null?null:'Cannot read properties of '+absence+(key===null?'':" (reading '"+key+"')"));
 cases.push({name,source,entry:'computedReadCase'+cases.length,value,abort,...(abort===null?{}:{errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))})});
}
for(const absence of ['null','undefined'])for(const [expression,key]of [['0','0'],['-0','0'],['1.5','1.5'],['NaN','NaN'],['Infinity','Infinity'],['-Infinity','-Infinity'],['true','true'],['false','false'],['2000n','2000'],['-2000n','-2000'],['"text"','text'],['"😀"','😀'],['null','null'],['undefined','undefined'],['(void 3)','undefined'],['[]',null],['[1,2]',null],['{}','#<Object>'],['({toString:()=>"never invoked"})',null],['new Map([])','#<Map>'],['new Set([])','#<Set>'],['(1n<<6400n)','<a very large BigInt>'],['(-(1n<<6400n))','<a very large BigInt>']]){
 add(absence+' computed key '+expression,'export function compute(){return '+absence+'['+expression+'];}',null,absence,key);
}
const failure='/** @returns {number} */function failComputedKey(){throw new RangeError("key evaluated");}';
add('computed key abrupt completion precedes nullish read',failure+'export function compute(){return null[failComputedKey()];}',null,null,'','RangeError','key evaluated');
add('computed key abrupt completion precedes undefined read',failure+'export function compute(){return undefined[failComputedKey()];}',null,null,'','RangeError','key evaluated');
add('nullable present Array read','export function compute(){const receiver=true?[3,4]:null;return receiver[1]??7;}','4');
add('nullable absent Array read','export function compute(){const receiver=false?[3,4]:null;return receiver[1]??7;}',null,'null','1');
add('nullable Array out of bounds is undefined','export function compute(){const receiver=true?[3,4]:undefined;return receiver[2]??7;}','7');
add('nullable Array key captures actual shared mutation','export function compute(){const values=[3];const receiver=true?values:null;return receiver[(values.push(4),1)]??7;}','4');
add('nullable receiver evaluated once before key','export function compute(){const values=[3];return (values.push(4),true?values:null)[(values.push(5),2)]??7;}','5');
add('nullable key preserves BigInt read','export function compute(){const receiver=true?[3,4]:null;return receiver[1n]??7;}','4');
add('both alternatives choose null computed diagnostic','export function compute(){const receiver=true?null:undefined;return receiver[3];}',null,'null','3');
add('both alternatives choose undefined computed diagnostic','export function compute(){const receiver=false?null:undefined;return receiver[3];}',null,'undefined','3');
add('optional chain skips the computed key',failure+'export function compute(){const receiver=false?[3,4]:null;return receiver?.[failComputedKey()]??7;}','7');
add('parentheses finish chain before computed key',failure+'export function compute(){const receiver=false?{values:[3,4]}:null;return (receiver?.values)[failComputedKey()];}',null,null,'','RangeError','key evaluated');
add('plain object key coercion is not called on null','/** @returns {number} */function failKeyCoercion(){throw new RangeError("must not coerce");}export function compute(){return null[{toString:()=>failKeyCoercion()}];}',null,'null',null);
add('nullable BigInt key diagnostic uses its present value','export function compute(){return null[true?2000n:undefined];}',null,'null','2000');
add('nullable BigInt key diagnostic retains null','export function compute(){return null[false?2000n:null];}',null,'null','null');
add('nullable BigInt key diagnostic retains undefined','export function compute(){return null[false?2000n:undefined];}',null,'null','undefined');
add('nullable very large BigInt key retains side effect free abbreviation','export function compute(){return null[true?(1n<<6400n):undefined];}',null,'null','<a very large BigInt>');
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('failComputedKey','failComputedKey'+row.entry).replaceAll('failKeyCoercion','failKeyCoercion'+row.entry)).join('\n');
const revision='7a9a0488';
const history=['js/src/translation/javascript-nullish-fields.js','js/src/translation/javascript-infer.js','rust/src/translation/check/javascript_nullish_fields.rs','rust/src/translation/javascript/infer/products.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
 return{path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
});
writeProjectData(new URL('../../parity/fixtures/javascript-computed-reads.lino',import.meta.url),{
 schemaVersion:1,model:'computed property evaluation and side-effect-free nullish diagnostics 1',sourceLanguage:'JavaScript',cases,source,history,
 oracleArchitectures:['x64','arm64'],
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; source architecture x64/arm64',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:['https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-runtime-semantics-evaluatepropertyaccesswithexpressionkey','https://tc39.es/ecma262/2025/multipage/ecmascript-data-types-and-values.html#sec-getvalue','https://raw.githubusercontent.com/nodejs/node/v24.0.0/deps/v8/src/execution/messages.cc','https://raw.githubusercontent.com/nodejs/node/v24.0.0/deps/v8/src/objects/objects.cc','https://raw.githubusercontent.com/nodejs/node/v22.15.0/deps/v8/src/objects/bigint.cc','https://raw.githubusercontent.com/nodejs/node/v24.0.0/deps/v8/src/objects/bigint.cc'],
 pinnedBigIntegerSources:[{revision:'v22.15.0',bytes:72962,sha256:'4f869b7df4a4627e2b14db9d8fcace0416454fc62136704d4e58c421ea02a414'},{revision:'v24.0.0',bytes:67849,sha256:'7ad0d18b2277f9dc8d31042722e08ce8367a3376451101e6b92fb42826c22d59'}],
 assertions:['baseThenKeyThenGetValue','computedKeyAbruptCompletionPrecedesNullishRead','receiverAndKeyEvaluatedOnce','actualMutableArrayReadAfterKey','optionalChainSkipsKeyParenthesesDoNot','preserveBothAbsenceAlternatives','diagnosticDoesNotInvokeKeyCoercion','actualConstructorProvenanceRequired','arraysAndOwnToStringUseUnnamedError','bigIntegerWordLimitIsSourceArchitectureProfile','allExceptionFieldsAndUtf16Units','nativeOriginalValuesAndSourceAst','bothCodecsFourEmissions'],
 boundary:'Unexecuted declarations. Prototype-sensitive structural parameter keys, arbitrary functions/symbols/key coercion, heterogeneous dynamic properties, proxies/accessors, full abrupt inference and all preceding requirements remain. Source inspection is not execution evidence.',
});
