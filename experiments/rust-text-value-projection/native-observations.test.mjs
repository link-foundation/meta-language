import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {planRustTextValueProjectionNative,runRustTextValueProjectionNative,rustTextNativeMode,verifyRustTextValueProjectionModel,RUST_TEXT_VALUE_PROJECTION_MODEL_COMMIT} from './native-observations.mjs';

// This check precedes registration; an explicit local native request is an error.
const nativeEnabled=rustTextNativeMode(process.env);
const plan=planRustTextValueProjectionNative();
const fixture=readProjectData(new URL('./data/fixtures.lino',import.meta.url));
const fakeEnvironment={CI:'true',RUST_TEXT_VALUE_PROJECTION_NATIVE:'1',GITHUB_SHA:'injected-process-contract-only'};
const stream=entry=>entry.observations.map((row,index)=>index+':'+row.utf8Hex+'\n').join('');
function fakeProcess(override){
 const calls=[],directories=new Map();
 const spawn=(command,args,options)=>{
  if(!directories.has(options.cwd))directories.set(options.cwd,plan.entries[directories.size]);
  const entry=directories.get(options.cwd),phase=args.includes('--crate-type=rlib')?'library':args.includes('--extern')?'consumer':'execute';
  const call={command,args,options,entry,phase};calls.push(call);
  const replacement=override?.(call);if(replacement)return replacement;
  if(phase!=='execute')writeFileSync(args[args.indexOf('-o')+1],'injected compiler artifact; never executable Rust');
  return{status:0,signal:null,stdout:phase==='execute'?stream(entry):'',stderr:''};
 };
 return{spawn,calls};
}
function injected(fake,options={}){return runRustTextValueProjectionNative(plan,{environment:fakeEnvironment,spawn:fake.spawn,...options});}
function fails(action,count,pattern){
 assert.throws(action,error=>{assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,count);assert.match(error.message,pattern);return true;});
}

test('planning binds thirteen complete original modules and their exact independent model receipts without a compiler',()=>{
 assert.equal(plan.modelCommit,RUST_TEXT_VALUE_PROJECTION_MODEL_COMMIT);assert.equal(plan.modelCommit,'0e78a59417dd5d7861abee9dd3b46c4ee7539ae4');
 assert.equal(plan.originalCount,13);assert.equal(plan.entries.length,16);assert.equal(plan.modelSources.length,5);
 for(const[index,row]of fixture.cases.entries()){
  const entry=plan.entries[index];assert.equal(entry.kind,'complete unmodified original');assert.equal(entry.id,row.id);assert.equal(entry.source,row.source);
  assert.equal(Buffer.byteLength(entry.source),row.sourceBytes);assert.equal(entry.sourceSha256,row.sourceSha256);assert.equal(createHash('sha256').update(entry.source).digest('hex'),row.sourceSha256);
  assert.equal(entry.replaySha256,row.sourceSha256);assert.deepEqual(entry.archiveFormats,['Links Notation text','binary links']);assert.ok(entry.source.endsWith(' fn main(){}'));
  assert.deepEqual(entry.observations.map(({args,value})=>({args,value})),row.observations);assert.equal(entry.functionName,'choose');
 }
 assert.throws(()=>{plan.entries[0].source='fn main(){}';},TypeError);
 const forged=structuredClone(fixture);forged.cases[0].source+=' ';assert.throws(()=>planRustTextValueProjectionNative({fixtures:forged}),/unchanged independently frozen fixture/u);
 assert.throws(()=>verifyRustTextValueProjectionModel(()=>Buffer.from('counterfeit model')),/independently reviewed model changed/u);
});

test('external consumers derive String, shared String and str arguments from actual owned source signatures',()=>{
 const entry=id=>plan.entries.find(row=>row.id===id);
 assert.deepEqual(entry('owned-move').parameterModes,['owned String']);assert.match(entry('owned-move').consumerSource,/oracle_source::choose\(argument_0\)/u);
 assert.deepEqual(entry('shared-string-clone').parameterModes,['shared String']);assert.match(entry('shared-string-clone').consumerSource,/oracle_source::choose\(&argument_0\)/u);
 assert.deepEqual(entry('borrowed-str-owned').parameterModes,['borrowed str']);assert.match(entry('borrowed-str-owned').consumerSource,/oracle_source::choose\(argument_0\.as_str\(\)\)/u);
 assert.deepEqual(entry('valid-clone-retarget').parameterModes,['owned String','shared String']);assert.match(entry('valid-clone-retarget').consumerSource,/oracle_source::choose\(argument_0,&argument_1\)/u);
 assert.deepEqual(entry('raw-literal-construction').parameterModes,[]);assert.match(entry('raw-literal-construction').consumerSource,/oracle_source::choose\(\)/u);
 for(const current of plan.entries){assert.match(current.consumerSource,/extern crate oracle_source;/u);assert.match(current.consumerSource,/let observed:String=oracle_source::choose/u);assert.doesNotMatch(current.consumerSource,/assert_eq!|fn choose|include!|mod original/u);}
});

test('coherent clone and raw/C0 literal edits generate complete checked source modules with independent expected values',()=>{
 const retarget=plan.entries.find(row=>row.id==='valid-clone-retarget-edited');assert.ok(retarget.source.includes('{right.clone()}'));assert.ok(retarget.source.startsWith('pub fn choose(left:String,right:&String)'));assert.ok(retarget.source.endsWith(' fn main(){}'));
 assert.equal(retarget.observations[0].value,'second');assert.equal(retarget.originalReceipt.sourceSha256,fixture.cases.find(row=>row.id==='valid-clone-retarget').sourceSha256);assert.notEqual(retarget.sourceSha256,retarget.originalReceipt.sourceSha256);
 const raw=plan.entries.find(row=>row.id==='raw-literal-construction-edited');assert.equal(raw.observations[0].value,'raw edited "\n🌊');assert.ok(raw.source.startsWith('pub fn choose()->String{String::from('));
 const c0=plan.entries.find(row=>row.id==='literal-owned-edited');assert.equal(c0.observations[0].value,String.fromCharCode(...Array.from({length:32},(_,index)=>index))+'\u007fΩ😀');assert.equal(c0.observations[0].utf8Hex,Buffer.from(c0.observations[0].value).toString('hex'));
 for(const current of[retarget,raw,c0])assert.equal(current.sourceSha256,current.replaySha256);
});

test('default and incomplete CI switches fail before any process, including a forged serialized plan',()=>{
 const fake=fakeProcess();assert.equal(rustTextNativeMode({}),false);assert.equal(rustTextNativeMode({CI:'true'}),false);
 for(const environment of[{RUST_TEXT_VALUE_PROJECTION_NATIVE:'1'},{RUST_TEXT_VALUE_PROJECTION_NATIVE:'1',CI:'false'}])assert.throws(()=>runRustTextValueProjectionNative(plan,{environment,spawn:fake.spawn}),/requires CI=true/u);
 assert.throws(()=>runRustTextValueProjectionNative(plan,{environment:{},spawn:fake.spawn}),/explicit CI-only switch/u);
 assert.throws(()=>runRustTextValueProjectionNative(structuredClone(plan),{environment:fakeEnvironment,spawn:fake.spawn}),/exact checked in-memory/u);assert.equal(fake.calls.length,0);
});

test('injected process contracts compile whole unchanged libraries then external consumers and compare only actual hex output',()=>{
 const fake=fakeProcess(({entry,phase,args,options})=>{
  assert.equal(options.shell,false);assert.equal(options.encoding,'utf8');assert.equal(options.maxBuffer,1048576);assert.equal(options.env,fakeEnvironment);
  if(phase==='library'){const file=args[args.indexOf('--crate-name')+2];assert.equal(readFileSync(file,'utf8'),entry.source);assert.equal(options.timeout,30000);assert.ok(args.includes('--crate-type=rlib'));}
  if(phase==='consumer'){assert.ok(args.includes('oracle_source='+join(options.cwd,'liboracle_source.rlib')));assert.equal(readFileSync(args[1],'utf8'),entry.consumerSource);}
  if(phase==='execute'){assert.deepEqual(args,[]);assert.equal(options.timeout,10000);}
 });
 const receipt=injected(fake);assert.equal(fake.calls.length,plan.entries.length*3);assert.equal(receipt.observations.length,16);assert.equal(receipt.originalCount,13);assert.deepEqual(receipt.processCounts,{libraryCompilations:16,consumerCompilations:16,executions:16});
 for(const[index,row]of receipt.observations.entries()){assert.equal(row.sourceSha256,plan.entries[index].sourceSha256);assert.deepEqual(row.observations.map(value=>value.value),plan.entries[index].observations.map(value=>value.value));}
 for(const call of fake.calls)assert.equal(existsSync(call.options.cwd),false,'private source and executable files are removed');
});

test('missing compilers and unsuccessful compilation aggregate every original and edited case',()=>{
 const missing=fakeProcess(()=>({status:null,stdout:'',stderr:'',error:Object.assign(new Error('compiler unavailable'),{code:'ENOENT'})}));
 fails(()=>injected(missing,{compiler:'/missing/compiler'}),16,/compiler unavailable/u);assert.equal(missing.calls.length,16);
 const invalid=fakeProcess(()=>({status:1,stdout:'',stderr:'error: original library rejected'}));fails(()=>injected(invalid),16,/original library rejected/u);
 assert.ok(plan.entries.every(row=>invalid.calls.some(call=>call.entry.id===row.id)));
});

test('compiler and execution timeouts and signals are errors and later fixtures still execute',()=>{
 for(const phase of['library','execute']){
  const fake=fakeProcess(call=>call.entry===plan.entries[0]&&call.phase===phase?{status:null,stdout:'',stderr:'',error:Object.assign(new Error('timeout'),{code:'ETIMEDOUT'})}:null);
  fails(()=>injected(fake),1,/timed out after/u);assert.ok(fake.calls.some(call=>call.entry===plan.entries.at(-1)&&call.phase==='execute'));
 }
 const signal=fakeProcess(call=>call.entry===plan.entries[0]&&call.phase==='execute'?{status:null,signal:'SIGTERM',stdout:'',stderr:''}:null);fails(()=>injected(signal),1,/terminated by SIGTERM/u);
});

test('wrong, invalid UTF-8, truncated, reordered, duplicate and nontext native output cannot count as observations',()=>{
 const first=stream(plan.entries[0]),lines=first.trimEnd().split('\n');
 const outputs=[first.replace('0:','0:61'),first.replace('0:','0:ff'),first.slice(0,-1),first+'0:\n',[lines[1],lines[0],lines[2]].join('\n')+'\n','unexpected output\n',{},first.replace('0:','0:f')];
 for(const stdout of outputs){const fake=fakeProcess(call=>call.entry===plan.entries[0]&&call.phase==='execute'?{status:0,stdout,stderr:''}:null);fails(()=>injected(fake),1,/Original Rust text observations failed/u);assert.equal(fake.calls.filter(call=>call.phase==='execute').length,16);}
});

test('a successful compiler status without its actual output artifact cannot execute a substitute program',()=>{
 for(const phase of['library','consumer']){
  const fake=fakeProcess(call=>call.entry===plan.entries[0]&&call.phase===phase?{status:0,stdout:'',stderr:''}:null);
  fails(()=>injected(fake),1,/compiler did not create/u);assert.equal(fake.calls.some(call=>call.entry===plan.entries[0]&&call.phase==='execute'),false);
 }
});

test('process-side edits of either the original complete module or external consumer invalidate its byte receipt',()=>{
 for(const phase of['library','consumer']){
  const fake=fakeProcess(call=>{if(call.entry===plan.entries[0]&&call.phase===phase){const file=phase==='library'?call.args[call.args.indexOf('--crate-name')+2]:call.args[1];writeFileSync(file,readFileSync(file,'utf8')+'\n// process-side source modification\n');}});
  fails(()=>injected(fake),1,/bytes changed|external consumer changed/u);assert.equal(fake.calls.some(call=>call.entry===plan.entries[0]&&call.phase==='execute'),false);
 }
});

// The only real subprocess path. Never registered by ordinary/local test runs.
if(nativeEnabled)test('CI original Rust oracle executes all thirteen full public modules and three coherent edited originals',()=>{
 const receipts=runRustTextValueProjectionNative(plan);
 assert.equal(receipts.observations.filter(row=>row.kind==='complete unmodified original').length,13);assert.equal(receipts.observations.length,16);
 assert.deepEqual(receipts.processCounts,{libraryCompilations:16,consumerCompilations:16,executions:16});
 const file=fileURLToPath(new URL('../../issue-195-artifacts/rust-text-value-projection-native-observations.lino',import.meta.url));
 mkdirSync(dirname(file),{recursive:true});writeProjectData(file,receipts);process.stdout.write('Original Rust text observations: '+receipts.observations.length+' complete source modules; Links Notation receipts: '+file+'\n');
});
