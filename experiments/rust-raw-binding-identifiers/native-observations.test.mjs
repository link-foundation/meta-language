import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {
 planRustRawBindingIdentifierNative,runRustRawBindingIdentifierNative,rustRawBindingIdentifierNativeMode,
 verifyRustRawBindingIdentifierModel,RUST_RAW_BINDING_IDENTIFIER_MODEL_COMMIT,
} from './native-observations.mjs';

// Fail before registration if a caller requests local native execution.
const nativeEnabled=rustRawBindingIdentifierNativeMode(process.env);
const plan=planRustRawBindingIdentifierNative();
const fixture=readProjectData(new URL('./data/fixtures.lino',import.meta.url));
const fakeEnvironment={CI:'true',RUST_RAW_BINDING_IDENTIFIER_NATIVE:'1',GITHUB_SHA:'injected-process-contract-only'};
const stream=entry=>entry.observations.map((row,index)=>`${index}:${row.type}:${row.canonical}\n`).join('');
function fakeProcess(override){
 const calls=[],directories=new Map();
 const spawn=(command,args,options)=>{
  if(!directories.has(options.cwd))directories.set(options.cwd,plan.entries[directories.size]);
  const entry=directories.get(options.cwd),phase=args.includes('--crate-type=rlib')?'library':args.includes('--extern')?'consumer':'execute';
  const call={command,args,options,entry,phase};calls.push(call);
  const replacement=override?.(call);if(replacement)return replacement;
  if(phase!=='execute')writeFileSync(args[args.indexOf('-o')+1],'injected compiler artifact; not executable Rust');
  return{status:0,signal:null,stdout:phase==='execute'?stream(entry):'',stderr:''};
 };
 return{spawn,calls};
}
function injected(fake,options={}){return runRustRawBindingIdentifierNative(plan,{environment:fakeEnvironment,spawn:fake.spawn,...options});}
function fails(action,count,pattern){
 assert.throws(action,error=>{assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,count);assert.match(error.message,pattern);return true;});
}

test('planning keeps all seven complete frozen sources and independent observations without a compiler',()=>{
 assert.equal(plan.modelCommit,RUST_RAW_BINDING_IDENTIFIER_MODEL_COMMIT);assert.equal(plan.modelCommit,'612faac96d60d01f0e31d19a4869add446d5da85');
 assert.equal(plan.originalCount,7);assert.equal(plan.entries.length,10);assert.equal(plan.modelSources.length,5);
 for(const[index,row]of fixture.cases.entries()){
  const entry=plan.entries[index];assert.equal(entry.id,row.id);assert.equal(entry.kind,'complete unmodified original');assert.equal(entry.source,row.source);
  assert.equal(entry.sourceBytes,Buffer.byteLength(row.source));assert.equal(entry.sourceSha256,createHash('sha256').update(row.source).digest('hex'));
  assert.equal(entry.sourceSha256,entry.originalReceipt.sourceSha256);assert.equal(entry.replaySha256,entry.sourceSha256);
  assert.equal(entry.functionName,row.name);assert.deepEqual(entry.archiveFormats,['Links Notation text','binary links']);
  assert.deepEqual(entry.observations.map(({args,value})=>({args,value})),[{args:row.arguments,value:row.value}]);
  assert.ok(entry.source.endsWith('fn main(){}'),'the real original main remains in the library source');
 }
 assert.throws(()=>{plan.entries[0].source='fn main(){}';},TypeError);
 assert.equal(planRustRawBindingIdentifierNative({includeEdits:false}).entries.length,7);
 const forged=structuredClone(fixture);forged.cases[0].source+=' ';assert.throws(()=>planRustRawBindingIdentifierNative({fixtures:forged}),/unchanged independently frozen fixture/u);
 assert.throws(()=>verifyRustRawBindingIdentifierModel(()=>Buffer.from('counterfeit source model')),/independently reviewed model changed/u);
});

test('external consumers use exact actual canonical raw names and Boolean/i32 parameter owners',()=>{
 for(const entry of plan.entries){
  const premise=entry.invocationPremises.find(row=>row.declarationName===entry.functionName);
  assert.ok(premise);assert.deepEqual(premise.parameterModes,entry.parameterModes);assert.equal(premise.resultMode,entry.resultMode);
  assert.match(entry.consumerSource,new RegExp('oracle_source::r#'+entry.functionName+'\\('));
  assert.match(entry.consumerSource,new RegExp('let observed:'+entry.resultMode+'='));
  assert.doesNotMatch(entry.consumerSource,/assert_eq!|include!|mod original|fn choose|fn r#/u);
  assert.equal((entry.consumerSource.match(/oracle_source::/gu)??[]).length,entry.observations.length);
  assert.ok(entry.invocationPremises.every(row=>row.parameterModes.every(mode=>['bool','i32'].includes(mode))));
 }
 assert.match(plan.entries[0].consumerSource,/let argument_0:i32=7i32;/u);
 assert.match(plan.entries[1].consumerSource,/let argument_0:bool=true;/u);
 assert.match(plan.entries[4].consumerSource,/let argument_0:bool=false;/u);
 assert.match(plan.entries[5].consumerSource,/r#choose\(argument_0,argument_1\)/u);
});

test('coherent parameter and actual callee keyword edits retain complete modules and change the independent result when appropriate',()=>{
 const original=plan.entries.find(entry=>entry.id==='two actual parameter owners');
 const retarget=plan.entries.find(entry=>entry.id==='parameter-retarget'),parameter=plan.entries.find(entry=>entry.id==='parameter-keyword-rename'),callee=plan.entries.find(entry=>entry.id==='callee-keyword-rename');
 assert.equal(original.observations[0].canonical,'13');assert.equal(retarget.observations[0].canonical,'17');
 assert.ok(retarget.source.includes('{r#type}'));assert.equal(retarget.originalReceipt.sourceSha256,original.sourceSha256);
 assert.ok(parameter.source.includes('r#loop:i32'));assert.ok(parameter.source.includes('{r#loop}'));assert.equal(parameter.observations[0].canonical,'17');
 assert.ok(callee.source.includes('{r#loop(value)}'));assert.ok(callee.source.includes('pub fn r#loop(r#arg:i32)'));assert.equal(callee.observations[0].canonical,'9');
 for(const entry of[retarget,parameter,callee]){
  assert.equal(entry.sourceSha256,entry.replaySha256);assert.notEqual(entry.sourceSha256,entry.originalReceipt.sourceSha256);assert.ok(entry.source.endsWith('fn main(){}'));
 }
});

test('both CI switches and the exact private checked plan are required before even an injected process',()=>{
 const fake=fakeProcess();assert.equal(rustRawBindingIdentifierNativeMode({}),false);assert.equal(rustRawBindingIdentifierNativeMode({CI:'true'}),false);
 for(const environment of[{RUST_RAW_BINDING_IDENTIFIER_NATIVE:'1'},{CI:'false',RUST_RAW_BINDING_IDENTIFIER_NATIVE:'1'}])assert.throws(()=>runRustRawBindingIdentifierNative(plan,{environment,spawn:fake.spawn}),/requires CI=true/u);
 assert.throws(()=>runRustRawBindingIdentifierNative(plan,{environment:{},spawn:fake.spawn}),/explicit CI-only switch/u);
 assert.throws(()=>runRustRawBindingIdentifierNative(plan,{environment:{CI:'true',RUST_TEXT_VALUE_PROJECTION_NATIVE:'1'},spawn:fake.spawn}),/explicit CI-only switch/u);
 assert.throws(()=>runRustRawBindingIdentifierNative(structuredClone(plan),{environment:fakeEnvironment,spawn:fake.spawn}),/exact checked in-memory/u);
 assert.equal(fake.calls.length,0);
});

test('injected contracts compile the complete untouched library then a separate consumer and read actual typed output',()=>{
 const fake=fakeProcess(({entry,phase,args,options})=>{
  assert.equal(options.shell,false);assert.equal(options.encoding,'utf8');assert.equal(options.maxBuffer,1048576);assert.equal(options.env,fakeEnvironment);
  if(phase==='library'){
   const file=args[args.indexOf('--crate-name')+2];assert.deepEqual(readFileSync(file),Buffer.from(entry.source));assert.ok(args.includes('--crate-type=rlib'));assert.equal(options.timeout,30000);
  }
  if(phase==='consumer'){assert.ok(args.includes('oracle_source='+join(options.cwd,'liboracle_source.rlib')));assert.equal(readFileSync(args[1],'utf8'),entry.consumerSource);}
  if(phase==='execute'){assert.deepEqual(args,[]);assert.equal(options.timeout,10000);}
 });
 const receipt=injected(fake);assert.equal(receipt.processMode,'injected process contract');assert.equal(receipt.originalCount,7);assert.equal(receipt.observations.length,10);
 assert.deepEqual(receipt.processCounts,{libraryCompilations:10,consumerCompilations:10,executions:10});assert.equal(fake.calls.length,30);
 for(const[index,entry]of receipt.observations.entries()){
  assert.equal(entry.sourceSha256,plan.entries[index].sourceSha256);assert.equal(entry.resultMode,plan.entries[index].resultMode);
  assert.deepEqual(entry.observations.map(row=>row.value),plan.entries[index].observations.map(row=>row.value));
 }
 for(const call of fake.calls)assert.equal(existsSync(call.options.cwd),false,'every private source/consumer/artifact directory is removed');
});

test('missing compilers and unsuccessful library or consumer status fail while every later fixture is attempted',()=>{
 const missing=fakeProcess(()=>({status:null,stdout:'',stderr:'',error:Object.assign(new Error('compiler unavailable'),{code:'ENOENT'})}));
 fails(()=>injected(missing,{compiler:'/missing/compiler'}),10,/compiler unavailable/u);assert.equal(missing.calls.length,10);
 for(const phase of['library','consumer']){
  const fake=fakeProcess(call=>call.phase===phase?{status:1,stdout:'',stderr:'actual compiler rejection'}:null);
  fails(()=>injected(fake),10,/actual compiler rejection/u);assert.ok(plan.entries.every(entry=>fake.calls.some(call=>call.entry===entry&&call.phase===phase)));
 }
});

test('compiler and execution timeout and signal errors cannot become native observations',()=>{
 for(const phase of['library','consumer','execute']){
  const fake=fakeProcess(call=>call.entry===plan.entries[0]&&call.phase===phase?{status:null,stdout:'',stderr:'',error:Object.assign(new Error('timeout'),{code:'ETIMEDOUT'})}:null);
  fails(()=>injected(fake),1,/timed out after/u);assert.ok(fake.calls.some(call=>call.entry===plan.entries.at(-1)&&call.phase==='execute'));
 }
 const signal=fakeProcess(call=>call.entry===plan.entries[0]&&call.phase==='execute'?{status:null,signal:'SIGTERM',stdout:'',stderr:''}:null);
 fails(()=>injected(signal),1,/terminated by SIGTERM/u);
});

test('wrong types, values, bounds, malformed decimals and omitted or duplicate actual native records fail closed',()=>{
 const outputs=['0:i32:8\n','0:bool:true\n','0:i32:true\n','0:i32:-0\n','0:i32:07\n','0:i32:+7\n','0:i32:7.0\n','0:i32:2147483648\n','0:i32:-2147483649\n','0:i32:7','0:i32:7\n0:i32:7\n','1:i32:7\n','unexpected\n','',{}];
 for(const stdout of outputs){
  const fake=fakeProcess(call=>call.entry===plan.entries[0]&&call.phase==='execute'?{status:0,stdout,stderr:''}:null);
  fails(()=>injected(fake),1,/Original Rust raw binding observations failed/u);assert.equal(fake.calls.filter(call=>call.phase==='execute').length,10);
 }
 for(const stdout of['0:bool:false\n','0:bool:1\n','0:i32:1\n']){
  const fake=fakeProcess(call=>call.entry===plan.entries[1]&&call.phase==='execute'?{status:0,stdout,stderr:''}:null);
  fails(()=>injected(fake),1,/Original Rust raw binding observations failed/u);
 }
});

test('successful status without actual compiler artifacts cannot execute a substitute binary',()=>{
 for(const phase of['library','consumer']){
  const fake=fakeProcess(call=>call.entry===plan.entries[0]&&call.phase===phase?{status:0,stdout:'',stderr:''}:null);
  fails(()=>injected(fake),1,/compiler did not create/u);assert.equal(fake.calls.some(call=>call.entry===plan.entries[0]&&call.phase==='execute'),false);
 }
});

test('process-side changes of complete original source or external consumer invalidate the exact byte receipt',()=>{
 for(const phase of['library','consumer','execute']){
  const fake=fakeProcess(call=>{
   if(call.entry===plan.entries[0]&&call.phase===phase){const file=phase==='consumer'?call.args[1]:join(call.options.cwd,'original.rs');writeFileSync(file,readFileSync(file,'utf8')+'\n// process-side modification\n');}
  });
  fails(()=>injected(fake),1,/bytes changed|external consumer changed/u);
  if(phase!=='execute')assert.equal(fake.calls.some(call=>call.entry===plan.entries[0]&&call.phase==='execute'),false);
 }
});

// Real compilers are reachable only through this separately selected CI group.
// Planner/fake-process tests never create a persisted observation artifact.
if(nativeEnabled)test('CI native source oracle executes all seven complete raw-name modules and three coherent edited modules',()=>{
 const receipts=runRustRawBindingIdentifierNative(plan);
 assert.equal(receipts.processMode,'native subprocesses');assert.equal(receipts.observations.filter(row=>row.kind==='complete unmodified original').length,7);
 assert.equal(receipts.observations.length,10);assert.deepEqual(receipts.processCounts,{libraryCompilations:10,consumerCompilations:10,executions:10});
 const file=fileURLToPath(new URL('../../issue-195-artifacts/rust-raw-binding-identifiers-native-observations.lino',import.meta.url));
 mkdirSync(dirname(file),{recursive:true});writeProjectData(file,receipts);
 process.stdout.write('Original Rust raw binding observations: '+receipts.observations.length+' complete source modules; Links Notation receipts: '+file+'\n');
});
