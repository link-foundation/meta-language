// Only fixture declarations/source history; never parse, emit or execute them.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['original field continuation input','export function compute(){const callback=value=>({value});return callback?.(1).value;}','1'],
 ['original nested call input','export function compute(){const callback=value=>(second=>value+second);return callback?.(1)?.(2);}','3'],
 ['skipped call skips its entire field tail','export function compute(){const callback=false?(value=>({value})):undefined;return callback?.(1).value??7;}','7'],
 ['present nullable call retains its field tail','export function compute(){const callback=true?(value=>({value})):null;return callback?.(2).value??7;}','2'],
 ['continuous field tail keeps zero','export function compute(){const callback=true?(()=>({value:0})):null;return callback?.().value??7;}','0'],
 ['continuous field tail keeps false','export function compute(){const callback=true?(()=>({value:false})):null;return callback?.().value===false;}','true'],
 ['continuous field tail keeps empty text','export function compute(){const callback=true?(()=>({value:""})):null;return callback?.().value??"fallback";}',''],
 ['optional returned callback skips only its own call','export function compute(){const callback=true?(()=>undefined):null;let visits=0;return (callback?.()?.(visits+=1)??7)*10+visits;}','70'],
 ['present returned callback keeps argument order','export function compute(){const callback=true?(()=>((first,second)=>first*10+second)):null;let visits=0;return (callback?.()?.(visits+=1,visits+=1)??0)*10+visits;}','122'],
 ['record field chain is inside receiver guard','export function compute(){const value=true?{nested:{value:3}}:null;return value?.nested.value??7;}','3'],
 ['absent record field chain skips all following fields','export function compute(){const value=false?{nested:{value:3}}:null;return value?.nested.value??7;}','7'],
 ['nested field guards each retain their own boundary','export function compute(){const value=true?{nested:false?{leaf:{value:3}}:null}:null;return value?.nested?.leaf.value??7;}','7'],
 ['nested present field guards preserve final value','export function compute(){const value=true?{nested:true?{leaf:{value:3}}:null}:null;return value?.nested?.leaf.value??7;}','3'],
 ['array computed index evaluates once','export function compute(){const values=true?[4,5]:null;let index=0;return (values?.[index++]??0)*10+index;}','41'],
 ['absent array skips computed index','export function compute(){const values=false?[4,5]:null;let index=0;return (values?.[index++]??7)*10+index;}','70'],
 ['literal null skips computed index','export function compute(){let index=0;return (null?.[index++]??7)*10+index;}','70'],
 ['nested arrays keep both computed indices lazy','export function compute(){const values=false?[[4]]:null;let index=0;return (values?.[index++][index++]??7)*10+index;}','70'],
 ['record field followed by array read','export function compute(){const value=true?{items:[4,5]}:null;return value?.items[1]??7;}','5'],
 ['record field followed by string method','export function compute(){const value=true?{text:"a😀"}:null;return value?.text.charAt(1).length??7;}','1'],
 ['record field followed by method skips its arguments','export function compute(){const value=false?{text:"abc"}:null;let visits=0;return (value?.text.charAt(visits+=1).length??7)*10+visits;}','70'],
 ['callee captured before argument assignment in field chain','export function compute(){let callback=value=>({value:value+1});const replacement=value=>({value:value+10});return callback?.((callback=replacement,2)).value*100+callback(2).value;}','312'],
 ['returned null is preserved at chain boundary','export function compute(){const callback=true?(()=>({value:null})):undefined;return callback?.().value===null;}','true'],
 ['returned undefined is preserved at chain boundary','export function compute(){const callback=true?(()=>({value:undefined})):null;return callback?.().value===undefined;}','true'],
 ['nullish array read keeps out of bounds absence','export function compute(){const values=true?[4]:null;return values?.[9]??7;}','7'],
 ['parentheses finish the chain before coalescing','export function compute(){const value=false?{nested:{value:4}}:null;return (value?.nested)?.value??7;}','7'],
 ['private chain spelling remains independent','export function compute(){const ml_temporary_optional_chain1=5;const value=true?{nested:{value:4}}:null;return (value?.nested.value??0)+ml_temporary_optional_chain1;}','9'],
 ['live array handle is retained in continued method','export function compute(){const values=[1];const value=true?{items:values}:null;value?.items.push(2);return values.length;}','2'],
];
inputs.push(
 ['present optional string method','export function compute(){const value=true?"abc":null;return value?.charAt(1)??"missing";}','b'],
 ['absent optional string method skips arguments','export function compute(){const value=false?"abc":null;let visits=0;return (value?.charAt(visits+=1)??"missing")+visits;}','missing0'],
 ['empty present string has zero optional length','export function compute(){const value=true?"":undefined;return value?.length??7;}','0'],
 ['absent optional string length','export function compute(){const value=false?"":undefined;return value?.length??7;}','7'],
 ['false receiver is present for optional method','export function compute(){const value=true?false:null;return value?.toString()??"missing";}','false'],
 ['zero receiver is present for optional method','export function compute(){const value=true?0:null;return value?.toString()??"missing";}','0'],
 ['NaN receiver is present for optional method','export function compute(){const value=true?NaN:null;return value?.toString()??"missing";}','NaN'],
 ['zero BigInt receiver is present for optional method','export function compute(){const value=true?0n:null;return value?.toString()??"missing";}','0'],
 ['optional array mutation keeps original handle','export function compute(){const values=[1];const receiver=true?values:null;const length=receiver?.push(2);return (length??0)*10+values.length;}','22'],
 ['absent optional array mutation skips arguments','export function compute(){const values=[1];const receiver=false?values:null;let visits=0;return (receiver?.push(visits+=1)??7)*100+visits*10+values.length;}','701'],
 ['optional Map lookup keeps absence','export function compute(){const receiver=true?new Map([["key",3]]):null;return receiver?.get("missing")??7;}','7'],
 ['optional Map lookup keeps its value','export function compute(){const receiver=true?new Map([["key",3]]):null;return receiver?.get("key")??7;}','3'],
 ['optional Set membership keeps its value','export function compute(){const receiver=true?new Set([3]):null;return receiver?.has(3)??false;}','true'],
 ['optional Set size retains zero','export function compute(){const receiver=true?new Set([]):null;return receiver?.size??7;}','0'],
 ['optional clear keeps Unit result and original effects','export function compute(){const values=new Set([3]);const receiver=true?values:null;return receiver?.clear()===undefined&&values.size===0;}','true'],
 ['absent optional clear preserves original collection','export function compute(){const values=new Set([3]);const receiver=false?values:null;return receiver?.clear()===undefined&&values.size===1;}','true'],
 ['optional own record method retains source call form','export function compute(){const receiver=true?{invoke:value=>value+1}:null;return receiver?.invoke(2)??7;}','3'],
);
const cases=inputs.map(([name,source,value],index)=>({name,source,entry:'optionalChainCase'+index,value,abort:null}));
const message='chain argument';
for(const[present,value,abort]of[[false,'7',null],[true,null,message]])cases.push({name:'guarded chain '+(present?'executes':'skips')+' abrupt argument',source:'/** @returns {number} */function failChainArgument(){throw new RangeError("'+message+'");} export function compute(){const callback='+present+'?(value=>({value})):null;return callback?.(failChainArgument()).value??7;}',entry:'optionalChainError'+cases.length,value,abort,errorClass:'RangeError',messagePresent:true,messageUnits:Array.from(message,letter=>letter.charCodeAt(0))});
const source=cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(').replaceAll('failChainArgument','failChainArgument'+row.entry)).join('\n');
const revision='b3ca8297';
const history=['js/scripts/generate-javascript-optional-call-fixtures.mjs','js/tests/translation-javascript-optional-calls.test.js','parity/fixtures/javascript-optional-calls.lino','js/src/translation/javascript.js','js/src/translation/javascript-infer.js','rust/src/translation/javascript/expressions.rs','rust/src/translation/javascript/infer.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
 return {path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
});
writeProjectData(new URL('../../parity/fixtures/javascript-optional-chains.lino',import.meta.url),{
 schemaVersion:1,model:'continuous optional chain lexical continuation source values 1',sourceLanguage:'JavaScript',cases,source,history,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-optional-chains',
 assertions:['allOriginalInputsRetained','wholeChainSkippedOnlyForNullishReceiver','presentFalsyResultsRetained','actualArgumentAndIndexOrder','receiverCapturedOnce','nestedOptionalBoundaryRetained','parenthesesEndChain','liveReferenceHandleRetained','allSourceExceptionFields','nativeOriginalValues','sourceAstAndTypedGraphParity','bothCodecsFourEmissions'],
 boundary:'Future declarations only. Ordinary property access on nullish returned values still needs runtime source errors, separate from skipped-chain semantics; property calls retaining dynamic this/proxy/accessors, general object keys, noncallables, spreads/signature protocols and every earlier obligation remain. No represented source, control, oracle or test has executed.',
});
