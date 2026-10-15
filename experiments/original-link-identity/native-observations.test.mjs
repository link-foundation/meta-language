import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData,writeProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/index.js';
import {ORIGINAL_LINK_IDENTITY_MODEL_COMMIT,verifyOriginalLinkIdentityNativeModel,originalLinkIdentityNativeMode,planOriginalLinkIdentityNative,runOriginalLinkIdentityNative} from './native-observations.mjs';

const nativeEnabled=originalLinkIdentityNativeMode(process.env),plan=planOriginalLinkIdentityNative(),entry=plan.entries[0];
const artifact=readProjectData(new URL('./data/native-definitions.lino',import.meta.url));
const digest=value=>createHash('sha256').update(value).digest('hex');
const fakeEnvironment={CI:'true',ORIGINAL_LINK_IDENTITY_NATIVE:'1',GITHUB_SHA:'injected-process-contract-only'};
const stream=entry.observations.map((row,index)=>String(index)+row.resultBits.map(bits=>':'+bits).join('')+'\n').join('');
function fakeProcess(override){
 const calls=[];
 const spawn=(command,args,options)=>{
  const phase=args.includes('--crate-type=rlib')?'library':args.includes('--extern')?'consumer':'execute',call={command,args,options,phase};calls.push(call);
  const replacement=override?.(call);if(replacement)return replacement;
  if(phase!=='execute')writeFileSync(args[args.indexOf('-o')+1],'injected process artifact; not native evidence');
  return{status:0,signal:null,stdout:phase==='execute'?stream:'',stderr:''};
 };
 return{spawn,calls};
}
const injected=(fake,options={})=>runOriginalLinkIdentityNative(plan,{environment:fakeEnvironment,spawn:fake.spawn,...options});
const fails=(operation,pattern)=>assert.throws(operation,error=>{assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,1);assert.match(error.message,pattern);return true;});

test('the oracle authenticates all eleven unchanged declarations and their actual complete native ABI',()=>{
 assert.equal(plan.modelCommit,ORIGINAL_LINK_IDENTITY_MODEL_COMMIT);assert.equal(plan.modelCommit,'9d578197fa32541d146d78c86c0a0d721ff540e4');assert.equal(plan.originalCount,1);assert.equal(plan.entries.length,1);
 assert.equal(entry.sourceBytes,17378);assert.equal(Buffer.byteLength(entry.source),entry.sourceBytes);assert.equal(digest(entry.source),'fe8afbe10f64da5198fcb1fba4354e13958e4201587047dedd4899e2ce784b2a');assert.equal(entry.replaySha256,entry.sourceSha256);
 assert.equal(entry.declarations.length,11);assert.equal(artifact.declarations.length,11);for(const[index,definition]of artifact.declarations.entries()){assert.ok(entry.source.includes(definition));assert.equal(entry.declarations[index].sha256,digest(definition));}
 assert.equal(entry.declarationSha256,'1598d5319f0749c5c104e79c5796095142cd0acff2cc58b436cde73c0f1bc54d');assert.equal(entry.declarationSha256,digest(JSON.stringify(artifact.declarations)));
 assert.equal(entry.interfaceSha256,'82a68ea599fbc46f3b3150a60db713dd911609c45646b497841ba35a70e616bd');assert.equal(entry.sourceModelSha256,'c287a0ef8c8eba5a8ee077924fc0f05b306c8e03e44ad0033d4c6f437eb4ed89');assert.equal(entry.kernelSha256,artifact.sourceSha256);
 assert.deepEqual(entry.parameterOrder,['high','low','query_high','query_low']);assert.equal(entry.parameterModes.length,4);assert.ok(entry.parameterModes.every(mode=>mode==='std::rc::Rc<std::cell::RefCell<Vec<f64>>>'));assert.equal(entry.resultMode,entry.parameterModes[0]);
 assert.throws(()=>{entry.observations[0].resultBits.push('0000000000000000');},TypeError);
});

test('all bounded consumer invocations retain exact edge, refusal, alias and ordering vectors',()=>{
 assert.equal(entry.invocationCount,47);assert.equal(entry.observations.length,47);assert.equal(entry.maximumObservedIndexRows,6);assert.equal(entry.maximumObservedQueryRows,5);
 const byId=new Map(entry.observations.map(row=>[row.id,row]));
 assert.deepEqual(byId.get('empty').decision,[]);assert.deepEqual(byId.get('adjacent above safe Number').decision,[2,1,2]);assert.deepEqual(byId.get('maximum unsigned64').decision,[1]);assert.deepEqual(byId.get('ordered repeated references').decision,[6,4,2,4,1]);
 assert.deepEqual(byId.get('duplicate index').decision,[-1]);assert.deepEqual(byId.get('query vector length').decision,[-2]);assert.deepEqual(byId.get('missing second identity').decision,[-4,1]);assert.deepEqual(byId.get('NaN vector 2').decision,[-3,0]);
 for(const row of entry.observations){assert.equal(row.inputBits.length,4);assert.ok(row.inputBits[0].length<=6);assert.ok(row.inputBits[2].length<=5);row.inputBits.flat().forEach(bits=>assert.match(bits,/^[0-9a-f]{16}$/u));}
 assert.match(entry.consumerSource,/oracle_source::read_original_link_identity_ordinals/u);assert.match(entry.consumerSource,/original input vector 0 changed/u);assert.match(entry.consumerSource,/actual ordinal vector differs from JavaScript/u);
 assert.equal(digest(entry.consumerSource),entry.consumerSha256);assert.match(entry.observationScope,/prove no source ownership/u);assert.match(entry.observationScope,/native100000-row/u);
});

test('source-model byte mutations fail before any native process can be planned',()=>{
 const receipts=verifyOriginalLinkIdentityNativeModel();assert.equal(receipts.length,5);
 for(const row of receipts)assert.throws(()=>verifyOriginalLinkIdentityNativeModel(file=>new URL(file).pathname.endsWith('/'+row.file)?Buffer.concat([readFileSync(file),Buffer.from('changed')]):readFileSync(file)),/frozen identity source changed/u);
});

test('injected contracts authenticate the original library and separate consumer without becoming native evidence',()=>{
 const fake=fakeProcess(),receipt=injected(fake);assert.equal(receipt.processMode,'injected process contract');assert.equal(receipt.repositoryCommit,fakeEnvironment.GITHUB_SHA);
 assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.deepEqual(fake.calls.map(call=>call.phase),['library','consumer','execute']);
 for(const call of fake.calls){assert.equal(call.options.shell,false);assert.equal(call.options.env,fakeEnvironment);assert.ok(call.options.timeout>0);assert.equal(existsSync(call.options.cwd),false,'temporary process artifacts are removed');}
 assert.ok(fake.calls[0].args.includes('--edition=2024'));assert.ok(fake.calls[0].args.includes('--crate-type=rlib'));assert.ok(fake.calls[1].args.includes('--extern'));
 const observed=receipt.observations[0];assert.equal(observed.invocationCount,47);assert.equal(observed.observations.length,47);assert.equal(observed.sourceSha256,entry.sourceSha256);assert.equal(observed.interfaceSha256,entry.interfaceSha256);assert.equal(observed.declarationSha256,entry.declarationSha256);assert.equal(observed.sourceModelSha256,entry.sourceModelSha256);
 assert.deepEqual(readStructuredDataLino(serializeProjectData(receipt,'identity-native-receipt.lino')),receipt);
});

test('explicit CI authorization and the exact checked plan precede even injected processes',()=>{
 assert.equal(originalLinkIdentityNativeMode({}),false);assert.equal(originalLinkIdentityNativeMode({CI:'true'}),false);assert.equal(originalLinkIdentityNativeMode({CI:'true',ORIGINAL_LINK_IDENTITY_NATIVE:'1'}),true);
 for(const environment of[{ORIGINAL_LINK_IDENTITY_NATIVE:'1'},{CI:'false',ORIGINAL_LINK_IDENTITY_NATIVE:'1'}])assert.throws(()=>originalLinkIdentityNativeMode(environment),/requires CI=true/u);
 for(const environment of[{}, {CI:'true'}]){const fake=fakeProcess();assert.throws(()=>injected(fake,{environment}),/explicit CI-only switch/u);assert.equal(fake.calls.length,0);}
 const fake=fakeProcess();assert.throws(()=>runOriginalLinkIdentityNative({...plan},{environment:fakeEnvironment,spawn:fake.spawn}),/exact checked/u);assert.equal(fake.calls.length,0);
});

test('truncated, reordered, extra and incorrect native result streams cannot pass',()=>{
 const lines=stream.trimEnd().split('\n'),wrong=[stream.slice(0,-1),stream+'47\n',lines.slice(1).join('\n')+'\n',[lines[1],lines[0],...lines.slice(2)].join('\n')+'\n',stream.replace('1\n','1:0000000000000000\n'),stream.replace('3ff0000000000000','4000000000000000')];
 for(const stdout of wrong){const fake=fakeProcess(call=>call.phase==='execute'?{status:0,signal:null,stdout,stderr:''}:null);fails(()=>injected(fake),/Original complete link identity kernel observations failed/u);assert.equal(fake.calls.length,3);}
});

test('compiler and execution failures, missing artifacts and process-side source edits fail closed',()=>{
 for(const phase of['library','consumer','execute'])for(const failure of[{status:1,stdout:'',stderr:'actual failure'},{status:null,signal:'SIGTERM',stdout:'',stderr:''},{error:{code:'ETIMEDOUT'},stdout:'',stderr:''}]){
  const fake=fakeProcess(call=>call.phase===phase?failure:null);fails(()=>injected(fake),/exit 1|terminated by SIGTERM|timed out/u);
 }
 for(const phase of['library','consumer']){const fake=fakeProcess(call=>call.phase===phase?{status:0,signal:null,stdout:'',stderr:''}:null);fails(()=>injected(fake),/compiler did not create/u);assert.equal(fake.calls.some(call=>call.phase==='execute'),false);}
 for(const phase of['library','consumer','execute']){const fake=fakeProcess(call=>{if(call.phase===phase){const file=join(call.options.cwd,phase==='consumer'?'consumer.rs':'original.rs');writeFileSync(file,readFileSync(file,'utf8')+'\n// changed by process\n');}});fails(()=>injected(fake),/bytes changed|consumer changed/u);}
});

// Only CI's explicit selector may compile and execute, then persist actual receipts.
if(nativeEnabled)test('CI executes the entire original identity kernel over all forty-seven bounded vectors',()=>{
 const receipt=runOriginalLinkIdentityNative(plan);assert.equal(receipt.processMode,'native subprocesses');assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.equal(receipt.observations[0].observations.length,47);
 const file=fileURLToPath(new URL('../../issue-195-artifacts/original-link-identity-native-observations.lino',import.meta.url));mkdirSync(dirname(file),{recursive:true});writeProjectData(file,receipt);
 process.stdout.write('Original link identity native oracle: 1 unchanged eleven-declaration module, 47 actual vector invocations; receipts '+file+'\n');
});
