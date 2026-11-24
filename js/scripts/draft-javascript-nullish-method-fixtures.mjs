// Only input/expected declarations and preserved source history. No code runs.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,value=null,absence=null,member='toString',errorClass='TypeError',errorMessage=null){
 const abort=errorMessage??(absence===null?null:"Cannot read properties of "+absence+" (reading '"+member+"')");
 cases.push({name,source,entry:'nullishMethodCase'+cases.length,value,abort,...(abort===null?{}:{errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});
}
for(const absence of ['null','undefined']){
 add('literal '+absence+' method read','export function compute(){return '+absence+'.toString();}',null,absence);
 add('annotated String '+absence+' method read','/** @returns {string} */export function compute(){return '+absence+'.toString();}',null,absence);
 add('nullable String '+absence+' method read','export function compute(){const receiver=false?"abc":'+absence+';return receiver.charAt(1);}',null,absence,'charAt');
 add('nullable Array '+absence+' method read','export function compute(){const receiver=false?[1,2]:'+absence+';return receiver.push(3);}',null,absence,'push');
 add('nullable Map '+absence+' method read','export function compute(){const receiver=false?new Map([["key",1]]):'+absence+';return receiver.get("key");}',null,absence,'get');
 add('nullable Array '+absence+' callback method read','export function compute(){const receiver=false?[1,2]:'+absence+';return receiver.filter(value=>value>1);}',null,absence,'filter');
 add('nullable String '+absence+' concat read','export function compute(){const receiver=false?"abc":'+absence+';return receiver.concat("tail");}',null,absence,'concat');
 add('zero argument slice '+absence+' read','export function compute(){const receiver=false?[1,2]:'+absence+';return receiver.slice();}',null,absence,'slice');
}
const failure='/** @returns {number} */function failMethodArgument(){throw new RangeError("argument evaluated");}';
for(const [member,receiver,args]of[['charAt','"abc"','failMethodArgument()'],['push','[1]','failMethodArgument()'],['concat','"abc"','failMethodArgument()'],['toString','3','failMethodArgument()']]){
 add('absent '+member+' property load precedes abrupt argument',failure+'export function compute(){const receiver=false?'+receiver+':null;return receiver.'+member+'('+args+');}',null,'null',member);
}
add('present method evaluates abrupt argument',failure+'export function compute(){const receiver=true?"abc":null;return receiver.charAt(failMethodArgument());}',null,null,'charAt','RangeError','argument evaluated');
add('both absence alternatives choose null method diagnostic','export function compute(){const receiver=true?null:undefined;return receiver.toString();}',null,'null');
add('both absence alternatives choose undefined method diagnostic','export function compute(){const receiver=false?null:undefined;return receiver.toString();}',null,'undefined');
add('present zero remains a valid receiver','export function compute(){const receiver=true?0:null;return receiver.toString();}','0');
add('present false remains a valid receiver','export function compute(){const receiver=true?false:null;return receiver.toString();}','false');
add('present empty text remains a valid receiver','export function compute(){const receiver=true?"":null;return receiver.toString();}','');
add('String constructor still converts null','export function compute(){return String(null);}','null');
add('String constructor still converts undefined','export function compute(){return String(undefined);}','undefined');
add('present String call keeps its ordinary result','export function compute(){const receiver=true?"abc":null;return receiver.charAt(1);}','b');
add('present nullable substring keeps ordinary String result','export function compute(){const receiver=true?"abc":undefined;return receiver.substring(1);}','bc');
add('present nullable Map lookup preserves missing value','export function compute(){const receiver=true?new Map([["key",1]]):null;return receiver.get("missing")??7;}','7');
add('present nullable Array mutation retains the original handle','export function compute(){const values=[1];const receiver=true?values:null;return receiver.push(2)*10+values.length;}','22');
add('present nullable Set clear retains Unit effects','export function compute(){const values=new Set([1]);const receiver=true?values:null;return receiver.clear()===undefined&&values.size===0;}','true');
add('present own callable field retains invocation','export function compute(){const receiver=true?{invoke:value=>value+1}:null;return receiver.invoke(3);}','4');
add('absent own callable field fails at property load','export function compute(){const receiver=false?{invoke:value=>value+1}:null;return receiver.invoke(3);}',null,'null','invoke');
add('continuous optional chain skips an ordinary method suffix','export function compute(){const receiver=false?{nested:"abc"}:null;return receiver?.nested.charAt(1)??"missing";}','missing');
add('returned nullish in a present chain fails at method load','export function compute(){const receiver=true?{nested:false?"abc":null}:null;return receiver?.nested.charAt(1)??"missing";}',null,'null','charAt');
add('parentheses finish a chain before ordinary method load','export function compute(){const receiver=false?{nested:"abc"}:null;return (receiver?.nested).charAt(1);}',null,'undefined','charAt');
add('slice keeps a distinct live array handle','export function compute(){const values=[1,2];const copy=values.slice();copy.push(3);return values.length*10+copy.length;}','23');
add('nullable slice keeps a distinct live array handle','export function compute(){const values=[1,2];const receiver=true?values:null;const copy=receiver.slice();copy.push(3);return values.length*10+copy.length;}','23');
add('slice copies slots before later source mutation','export function compute(){const values=[1,2];const copy=values.slice();values.push(3);return values.length*10+copy.length;}','32');
add('slice retains shallow shared element identity','export function compute(){const item={value:1};const values=[item];const copy=values.slice();item.value=3;return copy[0].value;}','3');
add('own slice callable keeps its actual return type','export function compute(){const receiver={slice:()=>4};return receiver.slice();}','4');
add('nullable own slice callable keeps its actual return type','export function compute(){const receiver=true?{slice:()=>4}:null;return receiver.slice();}','4');
add('nullable String slice keeps actual type','export function compute(){const receiver=true?"😀x":null;return receiver.slice().length;}','3');
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('failMethodArgument','failMethodArgument'+row.entry)).join('\n');
const revision='371bf750';
const history=['js/src/translation/javascript-property-calls.js','js/src/translation/javascript-concatenation.js','js/src/translation/javascript-nullish-fields.js','rust/src/translation/check/javascript_property_calls.rs','rust/src/translation/check/primitive_methods.rs','rust/src/translation/javascript/infer/methods.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
 return {path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
});
writeProjectData(new URL('../../parity/fixtures/javascript-nullish-methods.lino',import.meta.url),{
 schemaVersion:1,model:'ordinary method property loads, source errors and shallow slice references 1',sourceLanguage:'JavaScript',cases,source,history,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:['https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-requireobjectcoercible','https://raw.githubusercontent.com/nodejs/node/v22.15.0/deps/v8/src/common/message-template.h','https://raw.githubusercontent.com/nodejs/node/v24.0.0/deps/v8/src/common/message-template.h'],
 assertions:['actualReceiverReadOnce','nullishPropertyErrorBeforeEveryArgument','presentArgumentsRemainOrdered','nullableMethodResultUsesItsPresentSignature','ordinaryMethodDistinctFromOptionalTextConversion','optionalChainAndParenthesesBoundaries','liveMutableReceiverRetained','sliceCreatesDistinctArrayReference','sliceRemainsShallow','ownMethodSpellingNeverProvesBuiltin','allUtf16ExceptionFields','nativeOriginalValuesAndSourceAst','bothCodecsFourEmissions'],
 boundary:'Future source expectations and full preserved preceding host definitions. No represented source or control executes. Arbitrary this/accessors/proxies, missing and inherited properties, general dynamic call/key protocols, full abrupt/unreachable inference, live traversal and every preceding requirement remain.',
});
