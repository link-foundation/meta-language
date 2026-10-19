import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData,writeProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/index.js';
import {
 RUST_TEXT_CERTIFICATE_MODEL_COMMIT,verifyRustTextCertificateModel,rustTextCertificateNativeMode,
 planRustTextCertificateNative,runRustTextCertificateNative,
} from './native-observations.mjs';

// An explicit local native request fails before any test is registered.
const nativeEnabled=rustTextCertificateNativeMode(process.env);
const plan=planRustTextCertificateNative(),entry=plan.entries[0];
const artifact=readProjectData(new URL('./data/translation.lino',import.meta.url));
const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
const fakeEnvironment={CI:'true',RUST_TEXT_CERTIFICATE_NATIVE:'1',GITHUB_SHA:'injected-process-contract-only'};
const stream=entry.observations.map((row,index)=>index+':'+row.resultBits.join(':')+'\n').join('');
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
function fakeProcess(override){
 const calls=[];
 const spawn=(command,args,options)=>{
  const phase=args.includes('--crate-type=rlib')?'library':args.includes('--extern')?'consumer':'execute';
  const call={command,args,options,phase};calls.push(call);const replacement=override?.(call);if(replacement)return replacement;
  if(phase!=='execute')writeFileSync(args[args.indexOf('-o')+1],'injected compiler artifact; not native evidence');
  return{status:0,signal:null,stdout:phase==='execute'?stream:'',stderr:''};
 };
 return{spawn,calls};
}
function injected(fake,options={}){return runRustTextCertificateNative(plan,{environment:fakeEnvironment,spawn:fake.spawn,...options});}
function fails(action,pattern){assert.throws(action,error=>{assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,1);assert.match(error.message,pattern);return true;});}

test('one plan owns the entire unchanged generated artifact, actual 92-declaration inventory and preserved main',()=>{
 assert.equal(plan.modelCommit,RUST_TEXT_CERTIFICATE_MODEL_COMMIT);assert.equal(plan.modelCommit,'f4ac8664ece42a0299beae645c7fc22f14c4d793');assert.equal(plan.originalCount,1);assert.equal(plan.entries.length,1);
 assert.equal(entry.source,artifact.completeRust);assert.equal(entry.sourceBytes,273314);assert.equal(Buffer.byteLength(entry.source),entry.sourceBytes);
 assert.equal(entry.sourceSha256,'94b71be66e72ac887afce970c03d620bf6460ea862b8c72ceba8524899cced35');assert.equal(digest(entry.source),entry.sourceSha256);assert.equal(entry.replaySha256,entry.sourceSha256);
 assert.deepEqual(entry.declarations,artifact.declarations);assert.equal(entry.declarations.length,92);assert.equal(artifact.definitions.length,92);
 for(const definition of artifact.definitions)assert.ok(entry.source.includes(definition));
 assert.ok(entry.source.includes('\nfn ml_main() {'));assert.ok(entry.source.includes('\nfn main() {'));assert.ok(entry.source.includes('.stack_size(1 << 28)'));assert.ok(entry.source.includes('worker.join()'));
 assert.equal(entry.tokenCount,27792);assert.throws(()=>{entry.source='fn main(){}';},TypeError);
});

test('all thirteen original source hashes and genuine graph archives remain distinct from the 27 corruption invocations',()=>{
 assert.equal(fixtures.cases.length,13);assert.equal(entry.originalGraphCount,13);assert.equal(entry.corruptionCount,27);assert.equal(entry.invocationCount,40);assert.equal(entry.observations.length,40);
 const originals=entry.observations.filter(row=>row.kind==='original graph transport');assert.equal(originals.length,13);
 for(const[index,fixture]of fixtures.cases.entries()){
  const row=originals[index];assert.equal(row.id,fixture.id);assert.equal(row.fixtureId,fixture.id);
  assert.equal(Buffer.byteLength(fixture.source),fixture.sourceBytes);assert.equal(digest(fixture.source),fixture.sourceSha256);
  assert.deepEqual(row.sourceReceipt,{sourceBytes:fixture.sourceBytes,sourceSha256:fixture.sourceSha256});assert.deepEqual(row.archiveFormats,['Links Notation text','binary links']);assert.equal(row.decision[0],1);
 }
 const corruptions=entry.observations.filter(row=>row.kind==='corrupted graph transport');assert.equal(corruptions.length,27);assert.ok(corruptions.every(row=>row.decision[0]!==1));assert.equal(new Set(entry.observations.map(row=>row.id)).size,40);
 assert.equal(plan.modelSources.length,10);assert.throws(()=>verifyRustTextCertificateModel(()=>Buffer.from('counterfeit frozen model')),/frozen portable certificate changed/u);
 const read=file=>String(file).endsWith('/rust-text-value-projection/source.mjs')?Buffer.from('counterfeit old source'):readFileSync(file);
 assert.throws(()=>verifyRustTextCertificateModel(read),/original text model\/source\/fixture changed/u);
});

test('the actual 17-vector ABI and all five result roles drive one external consumer with no source replacement',()=>{
 assert.equal(entry.functionName,'check_rust_text_certificate');assert.equal(entry.tableOrder.length,17);assert.equal(new Set(entry.tableOrder).size,17);assert.deepEqual(entry.parameterModes,Array(17).fill('Vec<f64>'));assert.equal(entry.resultMode,'Vec<f64>');
 assert.deepEqual(entry.outputRoles,['decision','consumedTokens','functions','inputConditions','allocationConditions']);
 assert.equal((entry.consumerSource.match(/oracle_source::check_rust_text_certificate\(/gu)??[]).length,40);
 assert.equal((entry.consumerSource.match(/let observed:Vec<f64>=/gu)??[]).length,40);assert.ok(entry.consumerSource.startsWith('extern crate oracle_source;'));
 assert.doesNotMatch(entry.consumerSource,/include!|mod original|fn check_rust_text_certificate|fn ml_main/u);
 const count=entry.observations.reduce((sum,row)=>sum+row.inputBits.reduce((sum,table)=>sum+table.length,0),0);
 assert.equal((entry.consumerSource.match(/f64::from_bits\(0x[0-9a-f]{16}u64\)/gu)??[]).length,count,'every actual argument cell is constructed from its complete binary64 bits');
 assert.match(entry.consumerSource,/value\.to_bits\(\)/u);assert.match(entry.consumerSource,/observed\.len\(\),5/u);
});

test('input transports preserve signed zero, nonfinite values and subnormal bits before the kernel refuses them',()=>{
 const context=entry.tableOrder.indexOf('context'),at=entry.observations[0].inputBits[context].findIndex(bits=>bits==='0000000000000000');assert.ok(at>=0);
 const bits=id=>entry.observations.find(row=>row.id===id).inputBits[context];
 assert.ok(bits('numeric negative-zero').includes('8000000000000000'));
 assert.ok(bits('numeric nan').some(bits=>/^7ff[8-9a-f][0-9a-f]{12}$/u.test(bits)));
 assert.ok(bits('numeric positive-infinity').includes('7ff0000000000000'));assert.ok(bits('numeric negative-infinity').includes('fff0000000000000'));
 assert.ok(bits('numeric subnormal').includes('0000000000000001'));assert.ok(entry.observations.every(row=>row.resultBits.length===5&&row.resultBits.every(bits=>/^[0-9a-f]{16}$/u.test(bits))));
});

test('both explicit CI flags and the exact in-memory checked plan precede every process',()=>{
 const fake=fakeProcess();assert.equal(rustTextCertificateNativeMode({}),false);assert.equal(rustTextCertificateNativeMode({CI:'true'}),false);
 for(const environment of[{RUST_TEXT_CERTIFICATE_NATIVE:'1'},{CI:'false',RUST_TEXT_CERTIFICATE_NATIVE:'1'}])assert.throws(()=>runRustTextCertificateNative(plan,{environment,spawn:fake.spawn}),/requires CI=true/u);
 for(const environment of[{}, {CI:'true',RUST_RAW_BINDING_IDENTIFIER_NATIVE:'1'}])assert.throws(()=>runRustTextCertificateNative(plan,{environment,spawn:fake.spawn}),/explicit CI-only switch/u);
 assert.throws(()=>runRustTextCertificateNative(structuredClone(plan),{environment:fakeEnvironment,spawn:fake.spawn}),/exact checked in-memory/u);assert.equal(fake.calls.length,0);
});

test('an injected contract builds one unchanged full library and one consumer then observes exactly 40 native-shaped results',()=>{
 const fake=fakeProcess(({phase,args,options})=>{
  assert.equal(options.shell,false);assert.equal(options.encoding,'utf8');assert.equal(options.maxBuffer,1048576);assert.equal(options.env,fakeEnvironment);
  if(phase==='library'){assert.equal(options.timeout,30000);assert.deepEqual(readFileSync(args[args.indexOf('--crate-name')+2]),Buffer.from(artifact.completeRust));assert.ok(args.includes('--crate-type=rlib'));}
  if(phase==='consumer'){assert.equal(options.timeout,30000);assert.equal(readFileSync(args[1],'utf8'),entry.consumerSource);assert.ok(args.includes('oracle_source='+join(options.cwd,'liboracle_source.rlib')));}
  if(phase==='execute'){assert.equal(options.timeout,10000);assert.deepEqual(args,[]);}
 });
 const receipt=injected(fake);assert.equal(receipt.processMode,'injected process contract');assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.equal(fake.calls.length,3);assert.equal(receipt.observations.length,1);
 const observations=receipt.observations[0].observations;assert.equal(observations.length,40);assert.deepEqual(observations.map(row=>row.resultBits),entry.observations.map(row=>row.resultBits));
 assert.deepEqual(receipt.observations[0].declarations,artifact.declarations);assert.equal(receipt.observations[0].invocationCount,40);
 for(const call of fake.calls)assert.equal(existsSync(call.options.cwd),false,'whole source, consumer and temporary artifacts are cleaned');
 assert.deepEqual(readStructuredDataLino(serializeProjectData(receipt,'observations.lino')),receipt,'typed bits receipts survive LiNo without JSON files or nonfinite scalar coercion');
});

test('missing compiler and library, consumer or executable failures never count as observations',()=>{
 const missing=fakeProcess(()=>({status:null,stdout:'',stderr:'',error:Object.assign(new Error('compiler unavailable'),{code:'ENOENT'})}));fails(()=>injected(missing,{compiler:'/missing/compiler'}),/compiler unavailable/u);assert.equal(missing.calls.length,1);
 for(const phase of['library','consumer','execute']){
  const fake=fakeProcess(call=>call.phase===phase?{status:1,stdout:'',stderr:'actual subprocess rejected'}:null);fails(()=>injected(fake),/actual subprocess rejected/u);
  assert.equal(fake.calls.at(-1).phase,phase);assert.equal(existsSync(fake.calls[0].options.cwd),false);
 }
});

test('actual process timeout or signal is a failure rather than a cached expected decision',()=>{
 for(const phase of['library','consumer','execute']){
  const fake=fakeProcess(call=>call.phase===phase?{status:null,stdout:'',stderr:'',error:Object.assign(new Error('timeout'),{code:'ETIMEDOUT'})}:null);fails(()=>injected(fake),/timed out after/u);
 }
 const signal=fakeProcess(call=>call.phase==='execute'?{status:null,signal:'SIGTERM',stdout:'',stderr:''}:null);fails(()=>injected(signal),/terminated by SIGTERM/u);
});

test('native result bits reject signed-zero erasure, wrong values, bad width, order, count and nontext output',()=>{
 const lines=stream.trimEnd().split('\n'),first=entry.observations[0].resultBits;
 const changed=[...first];changed[changed.length-1]='8000000000000000';assert.equal(first.at(-1),'0000000000000000');
 const outputs=[
  ['0:'+changed.join(':'),...lines.slice(1)].join('\n')+'\n',
  stream.replace('3ff0000000000000','0000000000000000'),
  stream.replace('3ff0000000000000','3FF0000000000000'),
  stream.replace('3ff0000000000000','3ff000000000000'),
  stream.replace('3ff0000000000000','true'),
  [lines[1],lines[0],...lines.slice(2)].join('\n')+'\n',
  stream.slice(0,-1),lines.slice(1).join('\n')+'\n',stream+lines[0]+'\n',stream.replace('0:','00:'),'unexpected output\n',{},
 ];
 for(const stdout of outputs){const fake=fakeProcess(call=>call.phase==='execute'?{status:0,stdout,stderr:''}:null);fails(()=>injected(fake),/Original complete Rust text certificate observations failed/u);assert.equal(fake.calls.length,3);}
});

test('successful compiler status needs actual artifacts and original or consumer byte edits cannot pass',()=>{
 for(const phase of['library','consumer']){
  const fake=fakeProcess(call=>call.phase===phase?{status:0,stdout:'',stderr:''}:null);fails(()=>injected(fake),/compiler did not create/u);assert.equal(fake.calls.some(call=>call.phase==='execute'),false);
 }
 for(const phase of['library','consumer','execute']){
  const fake=fakeProcess(call=>{if(call.phase===phase){const path=join(call.options.cwd,phase==='consumer'?'consumer.rs':'original.rs');writeFileSync(path,readFileSync(path,'utf8')+'\n// process-side byte change\n');}});
  fails(()=>injected(fake),/bytes changed|external consumer changed/u);if(phase!=='execute')assert.equal(fake.calls.some(call=>call.phase==='execute'),false);
 }
});

// Only this explicitly selected CI group can run a real compiler and persist receipts.
if(nativeEnabled)test('CI executes the complete unchanged Rust certificate artifact over all 40 declared transports',()=>{
 const receipt=runRustTextCertificateNative(plan);assert.equal(receipt.processMode,'native subprocesses');assert.equal(receipt.originalCount,1);assert.equal(receipt.observations.length,1);
 assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});const row=receipt.observations[0];assert.equal(row.invocationCount,40);assert.equal(row.observations.length,40);assert.equal(row.originalGraphCount,13);assert.equal(row.corruptionCount,27);
 assert.equal(row.sourceSha256,entry.sourceSha256);assert.deepEqual(row.declarations,artifact.declarations);
 const file=fileURLToPath(new URL('../../issue-195-artifacts/rust-text-certificate-native-observations.lino',import.meta.url));mkdirSync(dirname(file),{recursive:true});writeProjectData(file,receipt);
 process.stdout.write('Complete original Rust text certificate observations: 1 unchanged artifact, 40 actual invocations; Links Notation receipts: '+file+'\n');
});
