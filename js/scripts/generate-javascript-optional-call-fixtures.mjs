// Whole source declarations only. No optional call, parser or test executes.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['direct declared function','function increment(value){return value+1;} export function compute(){return increment?.(2);}','3'],
 ['direct local callable','export function compute(){const callback=value=>value+2;return callback?.(3);}','5'],
 ['undefined local skips argument assignment','export function compute(){const callback=undefined;let visits=0;const value=callback?.(visits+=1);return value===undefined&&visits===0;}','true'],
 ['null local skips argument assignment','export function compute(){const callback=null;let visits=0;const value=callback?.(visits+=1);return value===undefined&&visits===0;}','true'],
 ['literal null produces undefined','export function compute(){return null?.(1)===undefined;}','true'],
 ['literal undefined produces undefined','export function compute(){return undefined?.(1)===undefined;}','true'],
 ['nullable callable present','export function compute(){const callback=true?(value=>value+2):undefined;return callback?.(3)??0;}','5'],
 ['nullable callable absent','export function compute(){const callback=false?(value=>value+2):undefined;let visits=0;const value=callback?.(visits+=1);return value===undefined&&visits===0;}','true'],
 ['null callable alternative stays absent','export function compute(){const callback=false?(value=>value+2):null;let visits=0;return (callback?.(visits+=1)??3)*10+visits;}','30'],
 ['call returns null without receiver absence','function produce(){return null;}export function compute(){return produce?.()===null;}','true'],
 ['nullable callable preserves returned null','export function compute(){const callback=true?(()=>null):undefined;return callback?.()===null;}','true'],
 ['nullable callable preserves returned undefined','export function compute(){const callback=true?(()=>undefined):null;return callback?.()===undefined;}','true'],
 ['false result stays present','export function compute(){const callback=true?(()=>false):null;return callback?.()===false;}','true'],
 ['zero result stays present','export function compute(){const callback=true?(()=>0):null;return callback?.()??7;}','0'],
 ['empty string stays present','export function compute(){const callback=true?(()=>""):null;return callback?.()??"fallback";}',''],
 ['present nullable callback result uses its own absence','export function compute(){const callback=true?(value=>value>0?value:undefined):null;return (callback?.(-1)??7)*10+(callback?.(2)??0);}','72'],
 ['present arguments evaluate once left to right','export function compute(){const callback=true?((first,second)=>first*10+second):undefined;let value=0;const result=callback?.(value+=1,value+=1);return (result??0)*10+value;}','122'],
 ['absent arguments both stay lazy','export function compute(){const callback=false?((first,second)=>first*10+second):undefined;let value=0;const result=callback?.(value+=1,value+=1);return (result??3)*10+value;}','30'],
 ['effectful factory receiver evaluates once','function create(values){values.push(1);return value=>value+2;}export function compute(){const values=[];const result=create(values)?.(3);return (result??0)*10+values.length;}','51'],
 ['zero argument factory remains a call rather than a function reference','function create(){return value=>value+2;}export function compute(){return create()?.(3)??0;}','5'],
 ['callee is captured before later argument assignment','export function compute(){let callback=value=>value+1;const replacement=value=>value+10;const result=callback?.((callback=replacement,2));return result*100+callback(2);}','312'],
 ['nullable callee is captured before becoming null','export function compute(){let callback=true?(value=>value+1):null;return callback?.((callback=null,2))??0;}','3'],
 ['nullable callback parameter inference','function invoke(callback,value){return callback?.(value)??7;}export function compute(){return invoke(undefined,3)*10+invoke(value=>value+1,3);}','74'],
 ['Unit result keeps callback effects and undefined absence','function report(values){values.push(7);return;}export function compute(){const values=[];const callback=true?report:null;const result=callback?.(values);return result===undefined&&values.length===1;}','true'],
 ['source capture name remains independent','export function compute(){const ml_optional_call_receiver1=7;const callback=true?(value=>value+1):undefined;return (callback?.(2)??0)+ml_optional_call_receiver1;}','10'],
];
const cases=inputs.map(([name,source,value],index)=>({name,source,entry:'optionalCallCase'+index,value,abort:null}));
const helper='/** @returns {number} */ function failOptionalArgument(){throw new SyntaxError("optional argument");}';
cases.push({name:'absent callable skips an abrupt argument',source:helper+'export function compute(){const callback=false?(value=>value):undefined;return callback?.(failOptionalArgument())??7;}',entry:'optionalCallSkippedAbort',value:'7',abort:null});
for(const[name,source,errorClass,abort]of[
 ['present callable evaluates abrupt argument',helper+'export function compute(){const callback=true?(value=>value):undefined;return callback?.(failOptionalArgument())??7;}','SyntaxError','optional argument'],
 ['abrupt receiver prevents argument evaluation','function create(flag){if(flag)throw new RangeError("optional receiver");return value=>value;}'+helper+'export function compute(){return create(true)?.(failOptionalArgument())??7;}','RangeError','optional receiver'],
])cases.push({name,source,entry:'optionalCallError'+cases.length,value:null,abort,errorClass,messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))});
// Identically named helper declarations in independent original sources keep
// their complete inputs while the aggregate receives distinct lexical names.
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('failOptionalArgument','failOptionalArgument'+row.entry).replaceAll('function create(','function create'+row.entry+'(').replaceAll('create(true)','create'+row.entry+'(true)').replaceAll('create(values)','create'+row.entry+'(values)').replaceAll('create()','create'+row.entry+'()')).join('\n');
writeProjectData(new URL('../../parity/fixtures/javascript-optional-calls.lino',import.meta.url),{
 schemaVersion:1,model:'lexical optional callable evaluation and absence source values 1',sourceLanguage:'JavaScript',cases,source,
 refusals:[{name:'property reference retains its this obligation',source:'export function compute(){const value={method(value){return value;}};return value.method?.(1);}',kind:'unsupported',reason:'optional method call: the property reference needs its complete original receiver and chain extent'}],
 assertionCorrection:{basis:'ECMAScript 2025 optional-chain evaluation keeps a continuous suffix inside the original nullish receiver branch. The two earlier chain refusals retain their exact inputs and full preceding writer/fixture/test source in javascript-optional-chains.lino; those exact inputs now have positive value, graph, native and codec controls.',range:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},
 originalRefusals:[
  {name:'property reference retains its this obligation',source:'export function compute(){const value={method(value){return value;}};return value.method?.(1);}',kind:'unsupported',reason:'optional method call: the property reference needs its complete original receiver and chain extent'},
  {name:'continuous field chain needs skipped-call discrimination',source:'export function compute(){const callback=value=>({value});return callback?.(1).value;}',kind:'unsupported',reason:'optional call chain: the chain needs its distinct skipped and returned value continuations'},
  {name:'continuous optional call chain retains its own extent',source:'export function compute(){const callback=value=>(second=>value+second);return callback?.(1)?.(2);}',kind:'unsupported',reason:'optional call chain: the chain needs its distinct skipped and returned value continuations'},
 ],
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-optional-chains',
 boundary:'Future original values/errors and all UTF16 exception fields, native products, full editable source decisions, both codecs and four emissions. No observations. Lexical/value/factory callees use the represented fixed callable signature. General property this references, uninterrupted optional chains, dynamic non-callables/callable objects, spread/missing/extra argument protocols, dead-code/TDZ inference and every preceding requirement remain.',
});
