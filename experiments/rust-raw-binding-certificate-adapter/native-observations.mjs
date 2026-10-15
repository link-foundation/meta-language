// CI-only execution oracle for the original complete six-rule artifact, not adapter/package support.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {createSourceNativeObservationRunner} from '../source-native-observations.mjs';
import * as selectors from '../rust-raw-binding-identifiers/rules.mjs';

export const RUST_RAW_BINDING_RULE_MODEL_COMMIT='258d3baa11fab25b8701c3fe1243a9176d214bee';
const sourceRoot=new URL('../rust-raw-binding-identifiers/',import.meta.url);
const digests={
 'model.mjs':'86c80174c000183a4c807a077f307452ee355ecf7b326f953a5e17c7c071278e',
 'syntax.mjs':'158f2ac2a50ef9e9113391a340f59a1fc17c5a5cab889a35aefa045be9668784',
 'rules.mjs':'9e54e5bd9e1cf0a3366f55ef31cb4c4f86c6b3aabc0ef9939b07a3f1926932d3',
 'generate-rules.mjs':'a52f49eef3ba414ac99d8bc7d10030f4ea5f651a9b3e58adc533994de642e519',
 'data/policy.lino':'1c6ed55cde0a20886d25ba6761aa54885f659726f1ec6ca6094aedc33aa0c33d',
 'data/fixtures.lino':'575ad1992ab8359c95e2e15d8f043890942a03b6de4c738c71433d137705b25a',
 'data/library-styles.lino':'a5da7792248b0aeda2c94f0fc7c03d02de92c8010401672342f6c089bab44f43',
 'data/rules.rs':'5ab7da7314104264d8f46a0b678eb8f967f19f581fabdcf33e7594b3f28d42f1',
};
const sha256=value=>createHash('sha256').update(value).digest('hex'),checkedPlans=new WeakSet();
const textBytes=value=>{if(typeof value!=='string'||!value.isWellFormed())throw new TypeError('oracle strings must be primitive Unicode scalar text');return Buffer.from(value,'utf8');};
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
function numberBits(value){assert.equal(typeof value,'number');const bytes=Buffer.alloc(8);bytes.writeDoubleBE(value);return bytes.toString('hex');}
export function verifyRustRawBindingRuleModel(read=readFileSync){return Object.entries(digests).map(([file,expected])=>{const actual=sha256(read(new URL(file,sourceRoot)));assert.equal(actual,expected,'the frozen raw interpretation or selector artifact changed: '+file);return{file,sha256:actual};});}
export function rustRawBindingRuleNativeMode(environment=process.env){
 if(environment.RUST_RAW_BINDING_RULE_NATIVE!=='1')return false;
 if(environment.CI!=='true')throw new TypeError('RUST_RAW_BINDING_RULE_NATIVE=1 requires CI=true; native compilation is forbidden locally');return true;
}
function selectorInventory(source){
 const javaScript=readFileSync(new URL('rules.mjs',sourceRoot),'utf8'),program=checkProgram(parseJavaScript(javaScript));
 const emitted=emitJavaScript(program),rust=emitRust(program),context={};vm.runInNewContext(emitted.text,context,{timeout:1000});
 assert.equal(program.items.length,6,'the complete original selector artifact has exactly six declarations');
 const inventory=program.items.map(entry=>{
  assert.equal(entry.k,'fn');assert.deepEqual(entry.ret,{kind:'bool'});
  const native=rust.mappings.find(row=>row.kind==='function'&&row.source===entry.fullName)?.target;
  const target=emitted.mappings.find(row=>row.kind==='function'&&row.source===entry.fullName)?.target;assert.equal(typeof context[target],'function');assert.equal(typeof selectors[entry.name],'function');assert.match(native,/^[a-z_]+$/u);
  for(const parameter of entry.params){assert.ok(['string','bool','float'].includes(parameter.type.kind));assert.deepEqual(parameter.type,{kind:parameter.type.kind});assert.equal(parameter.guard,null);assert.equal(parameter.default,null);}
  return{name:entry.name,native,parameters:entry.params.map(parameter=>({name:parameter.name,kind:parameter.type.kind})),result:'bool',target};
 });
 const tokens=tokenize(source,'Rust').tokens,actual=[];let depth=0;
 for(let index=0;index<tokens.length;index++){
  if(depth===0&&tokens[index].value==='pub'){
   let cursor=index+1;if(tokens[cursor].value==='const')cursor++;assert.equal(tokens[cursor++].value,'fn');actual.push(tokens[cursor].value);
   const row=inventory[actual.length-1];assert.equal(tokens[cursor++].value,row.native);assert.equal(tokens[cursor++].value,'(');
   row.parameters.forEach((parameter,slot)=>{
    if(slot)assert.equal(tokens[cursor++].value,',');assert.equal(tokens[cursor++].kind,'identifier');assert.equal(tokens[cursor++].value,':');
    const type=parameter.kind==='string'?['&','str']:parameter.kind==='float'?['f64']:['bool'];for(const value of type)assert.equal(tokens[cursor++].value,value);
   });
   if(tokens[cursor].value===',')cursor++;for(const value of[')','->','bool','{'])assert.equal(tokens[cursor++].value,value);
  }
  if(tokens[index].kind==='punct'&&tokens[index].value==='{')depth++;if(tokens[index].kind==='punct'&&tokens[index].value==='}')depth--;
 }
 assert.equal(depth,0);assert.deepEqual(actual,inventory.map(row=>row.native),'every native export is the corresponding actual complete JS selector');
 return{inventory,javascript:emitted.text,context};
}
function boundaryCalls(policy){
 const calls=[],add=(name,args,label)=>calls.push({name,args,label});
 for(const value of[policy.model,'','unknown',policy.invocation.model,policy.model+' ','Ω model'])add('acceptRustRawBindingIdentifierModel',[value],'model '+value);
 const names=[...new Set([...policy.forbiddenNames,...policy.ordinaryKeywords,'','foo','sourceFunction0','sourceParameter0_0','_value','ml_value','crate_value','main','é','a\u0301','😀','a#b','a b','0name','\0','r#foo'])];
 assert.equal(names.length,69,'the fixed name boundary inventory must remain complete');
 // The pure selector receives an explicit character-premise Boolean. Both values
 // are exercised as arguments; a true premise is not a Unicode/name-validity claim.
 for(const name of names)for(const raw of[false,true])for(const charactersAccepted of[false,true])add('acceptRustRawBindingIdentifierName',[name,raw,charactersAccepted],'conditional character premise '+name);
 const bits=[32,31,33,8,16,64,0,-0,-1,NaN,Infinity,-Infinity,32.5,32-2**-48,32+2**-47,Number.MIN_VALUE,Number.MAX_VALUE,Number.MIN_SAFE_INTEGER,Number.MAX_SAFE_INTEGER,2**53];assert.equal(bits.length,20);
 const kinds=['bool','fixed','float','int','nat','string','','fixed signed 32'];
 for(const kind of kinds)for(const signed of[false,true])for(const value of bits)add('acceptRustRawBindingIdentifierType',[kind,signed,value],'type '+kind);
 for(const recursive of[false,true])for(const mutual of[false,true])add('acceptRustRawBindingIdentifierClosure',[recursive,mutual],'conditional closure flags');
 for(const value of[policy.invocation.model,'','unknown',policy.model,policy.invocation.model+' ','Ω model'])add('acceptRustRawBindingIdentifierInvocationModel',[value],'invocation '+value);
 for(const kind of kinds)for(const booleanInput of[false,true])for(const integerInput of[false,true])add('acceptRustRawBindingIdentifierInput',[kind,booleanInput,integerInput],'conditional input premises '+kind);
 assert.equal(calls.length,644,'all six selectors have an explicit bounded total');return calls;
}
function typedArgument(parameter,value){
 if(parameter.kind==='float')return{kind:'float',bits:numberBits(value)};
 if(parameter.kind==='bool'){assert.equal(typeof value,'boolean');return{kind:'bool',value};}
 textBytes(value);return{kind:'string',value};
}
function consumerSource(observations){
 const blocks=observations.map((row,index)=>{
  const arguments_=row.arguments.map((argument,slot)=>{
   if(argument.kind==='string')return`let argument_${slot}=String::from_utf8(vec![${[...textBytes(argument.value)].map(byte=>byte+'u8').join(',')}]).expect("scalar selector argument");`;
   return`let argument_${slot}:${argument.kind==='float'?'f64':'bool'}=${argument.kind==='float'?'f64::from_bits(0x'+argument.bits+'u64)':String(argument.value)};`;
  });
  const call=row.arguments.map((argument,slot)=>(argument.kind==='string'?'&':'')+'argument_'+slot).join(',');
  return`{\n${arguments_.join('\n')}\nlet observed:bool=oracle_source::${row.native}(${call});\nwrite!(&mut line,"${index}:bool:{observed}\\n").expect("format actual selector result");\n}`;
 });
 return`extern crate oracle_source;\nuse std::fmt::Write as _;\nfn main(){\nlet mut line=String::new();\n${blocks.join('\n')}\nprint!("{line}");\n}\n`;
}
/** Local planning executes actual JavaScript selector calls, never a compiler. */
export function planRustRawBindingRuleNative(){
 const modelSources=verifyRustRawBindingRuleModel(),source=readFileSync(new URL('data/rules.rs',sourceRoot),'utf8'),policy=readProjectData(new URL('data/policy.lino',sourceRoot));assert.equal(textBytes(source).length,3588);
 const kernel=selectorInventory(source),calls=boundaryCalls(policy),byName=new Map(kernel.inventory.map(row=>[row.name,row]));
 const observations=calls.map((call,index)=>{
  const row=byName.get(call.name);assert.ok(row);assert.equal(call.args.length,row.parameters.length);
  const arguments_=call.args.map((value,slot)=>typedArgument(row.parameters[slot],value)),direct=selectors[row.name](...call.args),actual=kernel.context[row.target](...call.args);assert.equal(typeof actual,'boolean');assert.equal(actual,direct,'the complete emitted JS selector must match its actual frozen JS source');
  return{index,selector:row.name,native:row.native,label:call.label,arguments:arguments_,value:actual};
 });
 const cst=LinkNetwork.parse(source,'Rust'),restored=decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(cst.toLino()))).reconstructText();assert.deepEqual(textBytes(restored),textBytes(source));
 const consumer=consumerSource(observations),sourceSha256=sha256(textBytes(source)),declarations=kernel.inventory.map(({target,...row})=>row);
 const entry={id:'complete six raw binding rules',kind:'complete unmodified generated selector artifact',fixtureId:'experiments/rust-raw-binding-identifiers/data/rules.rs',source,sourceBytes:textBytes(source).length,sourceSha256,
  replaySha256:sha256(textBytes(restored)),originalReceipt:{sourceBytes:textBytes(source).length,sourceSha256},declarations,invocationCount:644,observationScope:'Conditional primitive selector decisions under explicit Boolean premises; these calls do not establish source grammar, character validation or actual invocation ownership.',
  archiveFormats:['complete source CST Links Notation text','binary links'],consumerSource:consumer,consumerSha256:sha256(consumer),javaScriptSha256:sha256(kernel.javascript),observations};
 const plan=freeze({model:policy.model,modelCommit:RUST_RAW_BINDING_RULE_MODEL_COMMIT,modelSources,originalCount:1,entries:[entry]});checkedPlans.add(plan);return plan;
}
function parseObservations(output,expected){
 const lines=output.split('\n');assert.equal(lines.pop(),'','native selector output ends with a newline');assert.equal(lines.length,644);assert.equal(lines.length,expected.length,'every actual selector call must be observed once');
 return lines.map((line,index)=>{
  const match=/^(0|[1-9][0-9]*):bool:(true|false)$/u.exec(line);assert.ok(match,'native selectors report strictly typed canonical Boolean text');assert.equal(match[1],String(index));const value=match[2]==='true';assert.equal(value,expected[index].value,'actual native selector differs from actual JavaScript');
  return{index,selector:expected[index].selector,native:expected[index].native,label:expected[index].label,arguments:expected[index].arguments,value};
 });
}
export const runRustRawBindingRuleNative=createSourceNativeObservationRunner({
 nativeMode:rustRawBindingRuleNativeMode,checkedPlan:plan=>checkedPlans.has(plan),textBytes,parseObservations,
 entryReceipt:entry=>({declarations:entry.declarations,invocationCount:entry.invocationCount,observationScope:entry.observationScope,archiveFormats:entry.archiveFormats}),
 temporaryPrefix:'rust-raw-binding-rules-original-',failureLabel:'Original complete Rust raw binding rule observations failed',
});
