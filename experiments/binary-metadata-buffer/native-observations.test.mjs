import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync,unlinkSync,symlinkSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData,writeProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/index.js';
import {BINARY_METADATA_BUFFER_MODEL_COMMIT,verifyBinaryMetadataBufferNativeModel,binaryMetadataBufferNativeMode,planBinaryMetadataBufferNative,runBinaryMetadataBufferNative} from './native-observations.mjs';

const nativeEnabled=binaryMetadataBufferNativeMode(process.env),plan=planBinaryMetadataBufferNative(),entry=plan.entries[0],generation=readProjectData(new URL('./native-generation.lino',import.meta.url));
const digest=value=>createHash('sha256').update(value).digest('hex'),byId=new Map(entry.observations.map(row=>[row.id,row]));
const fakeEnvironment={CI:'true',BINARY_METADATA_BUFFER_NATIVE:'1',GITHUB_SHA:'injected-process-contract-only'},stream=entry.observations.map((row,index)=>index+':'+row.resultHex+'\n').join('');
function fakeProcess(override){const calls=[];const spawn=(command,args,options)=>{const phase=args.includes('--crate-type=rlib')?'library':args.includes('--extern')?'consumer':'execute',call={command,args,options,phase};calls.push(call);const replacement=override?.(call);if(replacement)return replacement;if(phase!=='execute')writeFileSync(args[args.indexOf('-o')+1],'injected process artifact; not native evidence');return{status:0,signal:null,stdout:phase==='execute'?stream:'',stderr:''};};return{spawn,calls};}
const injected=(fake,options={})=>runBinaryMetadataBufferNative(plan,{environment:fakeEnvironment,spawn:fake.spawn,...options});
const fails=(operation,pattern)=>assert.throws(operation,error=>{assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,1);assert.match(error.message,pattern);return true;});

test('the entire candidate bundle retains all fifty-three definitions, four preludes and the original provider',()=>{
 assert.equal(plan.modelCommit,BINARY_METADATA_BUFFER_MODEL_COMMIT);assert.equal(plan.modelCommit,'1bb41fae1cbe4c88592e3241d01563346c97fac0');assert.equal(plan.originalCount,1);assert.equal(plan.entries.length,1);assert.equal(entry.candidateDeclarations,53);assert.equal(entry.candidatePreludeCount,4);assert.equal(entry.baselineDeclarations,54);assert.equal(entry.declarations.length,57);
 assert.deepEqual(entry.declarations.map(row=>row.name),generation.declarations.map(row=>row.name));assert.equal(entry.declarationSha256,digest(JSON.stringify(entry.declarations)));assert.equal(entry.declarationSha256,'8cdbaa6ad0122f874dfcada3205cfff69023f6a490522b1407d21e827e251843');assert.equal(entry.sourceBundleSha256,'664d1f5180e9e2f7dc171efd79e07f9dff5c19576b4fccd81bb55037fb7a331b');
 assert.equal(entry.additionalSources.length,6);assert.deepEqual(entry.additionalSources.map(row=>row.relativePath),['candidate/part_0.rs','candidate/part_1.rs','candidate/part_2.rs','candidate/part_3.rs','candidate/mod.rs','provider_array.rs']);
 for(const source of entry.additionalSources){assert.equal(Buffer.byteLength(source.source),source.sourceBytes);assert.equal(digest(source.source),source.sourceSha256);const replay=entry.archives.find(row=>row.path===source.relativePath);assert.equal(replay.textReplaySha256,source.sourceSha256);assert.equal(replay.binaryReplaySha256,source.sourceSha256);}
 for(const original of generation.nativeArtifacts){const source=entry.additionalSources.find(row=>row.relativePath==='candidate/'+original.path);assert.equal(source.source,readFileSync(new URL('./native/'+original.path,import.meta.url),'utf8'));assert.equal(source.sourceSha256,original.sha256);}
 const provider=entry.additionalSources.at(-1);assert.equal(provider.sourceSha256,'c2a535a8fbc2ab09deca3f7c92cad278b0a08a526f5f87a54e2aebb822c8b76a');assert.equal(provider.sourceBytes,3648);
 assert.equal(entry.sourceSha256,digest(entry.source));assert.equal(entry.replaySha256,entry.sourceSha256);assert.match(entry.source,/include!\("provider_array.rs"\)/u);assert.match(entry.source,/#\[path = "candidate\/mod.rs"\]/u);assert.doesNotMatch(entry.source,/\bfn\b/u);
 assert.deepEqual(entry.parameterOrder,['input','binary','record_limit','byte_limit']);assert.deepEqual(entry.parameterModes,['&[f64]','bool','f64','f64']);assert.equal(entry.resultMode,'candidate::ml_string::Text');assert.equal(entry.interfaceSha256,'0ab3a063f6b81f5e7f82e3c4d06633ab80c96e695d1c1aadbf392d78c4db108c');assert.equal(entry.generationPolicy.parser.referenceSemantics,false);assert.equal(entry.generationPolicy.checkedProgram.utf16Strings,true);
 assert.throws(()=>{entry.additionalSources[0].source+='changed';},TypeError);
});

test('bounded complete results cover both profiles, all twenty-two exact errors and size or refusal precedence',()=>{
 assert.equal(entry.observations.length,entry.invocationCount);assert.equal(entry.invocationCount,74);assert.equal(entry.maximumObservedInputCells,1320);assert.equal(entry.maximumObservedResultBytes,2640);assert.deepEqual(entry.errorCodes,Array.from({length:22},(_,index)=>-index-1));
 const fixture=readProjectData(new URL('../../parity/fixtures/link-network-binary.lino',import.meta.url));
 assert.equal(byId.get('golden v1 encode').result,Buffer.from(fixture.legacyVersion1Bytes).toString('hex'));assert.equal(byId.get('golden v2 encode').result,Buffer.from(fixture.version2Bytes).toString('hex'));
 assert.equal(byId.get('exact refusal 9').result,'-8,14,');assert.equal(byId.get('exact refusal 20').result,'-19,50,');assert.equal(byId.get('exact refusal 22').result,'-21,17,');
 for(const version of[1,2]){const encoded=byId.get('distinct metadata v'+version+' encode').result;assert.equal(byId.get('metadata v'+version+' encode byte bound 0').result,encoded);assert.equal(byId.get('metadata v'+version+' encode byte bound 1').result,encoded);assert.ok(byId.get('metadata v'+version+' encode byte bound -1').result.startsWith('-2,'));assert.equal(byId.get('metadata v'+version+' decode byte bound -1').result,'-2,0,');assert.ok(byId.get('malformed v'+version+' precedes output bound').result.startsWith('-13,'));assert.ok(byId.get('duplicate v'+version+' precedes output bound').result.startsWith('-8,'));}
 assert.equal(byId.get('malformed metadata before final output limit').result,'-13,4,');assert.equal(byId.get('invalid limits before malformed header').result,'-1,0,');assert.equal(byId.get('input bound before malformed header').result,'-2,0,');
 for(const row of entry.observations){assert.equal(Buffer.from(row.resultHex,'hex').toString('ascii'),row.result);row.inputBits.forEach(bits=>assert.match(bits,/^[0-9a-f]{16}$/u));assert.match(row.recordLimitBits,/^[0-9a-f]{16}$/u);assert.match(row.byteLimitBits,/^[0-9a-f]{16}$/u);}
 assert.match(entry.consumerSource,/oracle_source::candidate::transcode_link_network_binary\(&input,/u);assert.match(entry.consumerSource,/\.to_string\(\)/u);assert.match(entry.consumerSource,/complete byte or error framing differs from JavaScript/u);assert.equal(digest(entry.consumerSource),entry.consumerSha256);assert.match(entry.observationScope,/no production selection/u);
});

test('every pinned source, model, policy, style, builder, provider and fixture refuses byte changes',()=>{
 const receipts=verifyBinaryMetadataBufferNativeModel();assert.equal(receipts.length,13);
 for(const row of receipts)assert.throws(()=>verifyBinaryMetadataBufferNativeModel(file=>new URL(file).pathname.endsWith('/'+row.file)?Buffer.concat([readFileSync(file),Buffer.from('changed')]):readFileSync(file)),/frozen binary candidate model changed/u);
 const revisionAddress=new URL('./frontend-owner-revision.lino',import.meta.url),revision=readProjectData(revisionAddress);
 assert.equal(digest(readFileSync(revisionAddress)),'95f86b39d65d4b2faa57ceb942bc77feab2086184b239ed3f4210c8be6a4720c');
 const owner=receipts.find(row=>row.file==='rust/src/translation/frontend_rules.rs');
 assert.equal(owner.sha256,'017c049132b271f5c219736086ac04b2cfb349d88de767478edcad46f05d597f');
 assert.equal(owner.ownerRevision.actualSourceSha256,'37033e27b97bc145b0d51b89016606e141d5962c0f423ee131202432a301dad3');
 assert.equal(owner.ownerRevision.originalCommit,BINARY_METADATA_BUFFER_MODEL_COMMIT);
 const original=verifyBinaryMetadataBufferNativeModel(file=>new URL(file).pathname.endsWith('/'+owner.file)?Buffer.from(revision.original.source):readFileSync(file));
 assert.deepEqual(original.map(({file,sha256})=>({file,sha256})),receipts.map(({file,sha256})=>({file,sha256})),'every original model binding remains exact');
 assert.equal(revision.provider.source,entry.additionalSources.at(-1).source);
 assert.equal(revision.provider.sourceBytes,3648);assert.equal(revision.provider.sha256,'c2a535a8fbc2ab09deca3f7c92cad278b0a08a526f5f87a54e2aebb822c8b76a');
 for(const changed of [revision.current.source+'\n',revision.current.source.replace('kind == "record"','kind == "counterfeit"'),revision.current.source.replace('(present_kind == "tuple")','(present_kind == "array")'),revision.current.source.replace('array index out of range','changed provider refusal')]){
  assert.notEqual(changed,revision.current.source);
  assert.throws(()=>verifyBinaryMetadataBufferNativeModel(file=>new URL(file).pathname.endsWith('/'+owner.file)?Buffer.from(changed):readFileSync(file)),/frozen binary candidate model changed/u);
 }
 for(const change of ['append','coherent-current-pin','changed-original','third-context']){
  const changed=structuredClone(revision);
  if(change==='coherent-current-pin'){changed.current.source+='\n';changed.current.sourceBytes++;changed.current.sha256=digest(changed.current.source);}
  if(change==='changed-original')changed.original.source+='\n';
  if(change==='third-context')changed.changes.push({kind:'addition',owner:'unreviewed',original:'',current:'\n'});
  const bytes=change==='append'?Buffer.concat([readFileSync(revisionAddress),Buffer.from('changed')]):Buffer.from(serializeProjectData(changed,'owner-revision.lino'));
  assert.throws(()=>verifyBinaryMetadataBufferNativeModel(file=>new URL(file).href===revisionAddress.href?bytes:readFileSync(file)),/frozen binary candidate model changed/u);
 }
});

test('injected source-bundle process contracts stay distinct from actual native evidence',()=>{
 const fake=fakeProcess(),receipt=injected(fake);assert.equal(receipt.processMode,'injected process contract');assert.equal(receipt.repositoryCommit,fakeEnvironment.GITHUB_SHA);assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.deepEqual(fake.calls.map(call=>call.phase),['library','consumer','execute']);
 for(const call of fake.calls){assert.equal(call.options.shell,false);assert.equal(call.options.env,fakeEnvironment);assert.ok(call.options.timeout>0);assert.equal(existsSync(call.options.cwd),false,'temporary process artifacts are removed');}
 assert.ok(fake.calls[0].args.includes('--crate-type=rlib'));assert.ok(fake.calls[1].args.includes('--extern'));const observed=receipt.observations[0];assert.equal(observed.observations.length,entry.invocationCount);assert.equal(observed.sourceBundleSha256,entry.sourceBundleSha256);assert.equal(observed.interfaceSha256,entry.interfaceSha256);assert.equal(observed.declarationSha256,entry.declarationSha256);assert.equal(observed.generationReceiptSha256,entry.generationReceiptSha256);assert.equal(observed.additionalSources.length,6);assert.deepEqual(observed.errorCodes,entry.errorCodes);
 assert.deepEqual(readStructuredDataLino(serializeProjectData(receipt,'binary-native-receipt.lino')),receipt);
});

test('explicit CI authorization and the exact checked plan precede even injected processes',()=>{
 assert.equal(binaryMetadataBufferNativeMode({}),false);assert.equal(binaryMetadataBufferNativeMode({CI:'true'}),false);assert.equal(binaryMetadataBufferNativeMode({CI:'true',BINARY_METADATA_BUFFER_NATIVE:'1'}),true);
 for(const environment of[{BINARY_METADATA_BUFFER_NATIVE:'1'},{CI:'false',BINARY_METADATA_BUFFER_NATIVE:'1'}])assert.throws(()=>binaryMetadataBufferNativeMode(environment),/requires CI=true/u);
 for(const environment of[{}, {CI:'true'}]){const fake=fakeProcess();assert.throws(()=>injected(fake,{environment}),/explicit CI-only switch/u);assert.equal(fake.calls.length,0);}
 const fake=fakeProcess();assert.throws(()=>runBinaryMetadataBufferNative({...plan},{environment:fakeEnvironment,spawn:fake.spawn}),/exact checked/u);assert.equal(fake.calls.length,0);
});

test('incomplete, reordered, extra, malformed and incorrect byte streams cannot pass',()=>{
 const lines=stream.trimEnd().split('\n'),wrong=[stream.slice(0,-1),stream+entry.invocationCount+':\n',lines.slice(1).join('\n')+'\n',[lines[1],lines[0],...lines.slice(2)].join('\n')+'\n',lines.map((line,index)=>index===0?line+'00':line).join('\n')+'\n',stream.replace(':',':z'),stream.replace('2d312c302c','2d322c302c')];
 for(const stdout of wrong){const fake=fakeProcess(call=>call.phase==='execute'?{status:0,signal:null,stdout,stderr:''}:null);fails(()=>injected(fake),/Complete binary metadata buffer candidate observations failed/u);assert.equal(fake.calls.length,3);}
});

test('compiler and execution failures or missing artifacts cannot create observations',()=>{
 for(const phase of['library','consumer','execute'])for(const failure of[{status:1,stdout:'',stderr:'actual failure'},{status:null,signal:'SIGTERM',stdout:'',stderr:''},{error:{code:'ETIMEDOUT'},stdout:'',stderr:''}]){const fake=fakeProcess(call=>call.phase===phase?failure:null);fails(()=>injected(fake),/exit 1|terminated by SIGTERM|timed out/u);}
 for(const phase of['library','consumer']){const fake=fakeProcess(call=>call.phase===phase?{status:0,signal:null,stdout:'',stderr:''}:null);fails(()=>injected(fake),/compiler did not create/u);assert.equal(fake.calls.some(call=>call.phase==='execute'),false);}
});

test('every complete candidate or provider source is authenticated around all three process phases',()=>{
 for(const source of entry.additionalSources)for(const phase of['library','consumer','execute']){const fake=fakeProcess(call=>{if(call.phase===phase){const file=join(call.options.cwd,source.relativePath);writeFileSync(file,readFileSync(file,'utf8')+'\n// process changed original source\n');}});fails(()=>injected(fake),/additional source bytes changed/u);}
 for(const source of entry.additionalSources){const fake=fakeProcess(call=>{if(call.phase==='library'){const file=join(call.options.cwd,source.relativePath),other=join(call.options.cwd,'identical.rs');writeFileSync(other,readFileSync(file));unlinkSync(file);symlinkSync(other,file);}});fails(()=>injected(fake),/cannot be symbolic links/u);}
});

test('process changes to the wrapper or actual invocation consumer fail closed',()=>{
 for(const name of['original.rs','consumer.rs'])for(const phase of['library','consumer','execute']){const fake=fakeProcess(call=>{if(call.phase===phase){const file=join(call.options.cwd,name);writeFileSync(file,readFileSync(file,'utf8')+'\n// process changed bytes\n');}});fails(()=>injected(fake),/bytes changed|consumer changed/u);}
});

if(nativeEnabled)test('CI compiles the complete candidate bundle and executes every bounded framing invocation',()=>{
 const receipt=runBinaryMetadataBufferNative(plan);assert.equal(receipt.processMode,'native subprocesses');assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.equal(receipt.observations[0].observations.length,entry.invocationCount);
 const file=fileURLToPath(new URL('../../issue-195-artifacts/binary-metadata-buffer-native-observations.lino',import.meta.url));mkdirSync(dirname(file),{recursive:true});writeProjectData(file,receipt);process.stdout.write('Complete binary metadata candidate oracle: 53 definitions, 4 preludes, '+entry.invocationCount+' actual invocations; receipts '+file+'\n');
});
