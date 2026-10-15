import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData,writeProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/index.js';
import {ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT,verifyOriginalLinkCompleteTableNativeModel,originalLinkCompleteTableNativeMode,planOriginalLinkCompleteTableNative,runOriginalLinkCompleteTableNative} from './native-observations.mjs';

const nativeEnabled=originalLinkCompleteTableNativeMode(process.env);
const digest=value=>createHash('sha256').update(value).digest('hex');
const fakeEnvironment={CI:'true',ORIGINAL_LINK_COMPLETE_TABLE_NATIVE:'1',GITHUB_SHA:'injected-process-contract-only'};
function bitStream(observations){const dictionary=[],indices=new Map(),lines=observations.map((row,index)=>{const fields=[String(index),String(row.resultRows.length)];for(const result of row.resultRows){fields.push(String(result.bits.length));for(const bits of result.bits){if(!indices.has(bits)){indices.set(bits,dictionary.length);dictionary.push(bits);}fields.push(indices.get(bits).toString(16));}}return fields.join(':');});return['dictionary:'+dictionary.length+':'+dictionary.join(':'),...lines].join('\n')+'\n';}
function fakeProcess(stream,override){const calls=[];const spawn=(command,args,options)=>{const phase=args.includes('--crate-type=rlib')?'library':args.includes('--extern')?'consumer':'execute',call={command,args,options,phase};calls.push(call);const replacement=override?.(call);if(replacement)return replacement;if(phase!=='execute')writeFileSync(args[args.indexOf('-o')+1],'injected process artifact; not native evidence');return{status:0,signal:null,stdout:phase==='execute'?stream:'',stderr:''};};return{spawn,calls};}
const fails=(operation,pattern)=>assert.throws(operation,error=>{assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,1);assert.match(error.message,pattern);return true;});

// No heavy planner, CST, source-free graph or subprocess work occurs locally.
test('heavy planning and native processes require their separate explicit CI authorization',()=>{
 assert.equal(originalLinkCompleteTableNativeMode({}),false);assert.equal(originalLinkCompleteTableNativeMode({CI:'true'}),false);assert.equal(originalLinkCompleteTableNativeMode({CI:'true',ORIGINAL_LINK_COMPLETE_TABLE_NATIVE:'1'}),true);
 for(const environment of[{ORIGINAL_LINK_COMPLETE_TABLE_NATIVE:'1'},{CI:'false',ORIGINAL_LINK_COMPLETE_TABLE_NATIVE:'1'}])assert.throws(()=>originalLinkCompleteTableNativeMode(environment),/requires CI=true/u);
 for(const environment of[{}, {CI:'false'}])assert.throws(()=>planOriginalLinkCompleteTableNative(environment),/planning requires CI=true/u);
 const fake=fakeProcess('');for(const environment of[{}, {CI:'true'}])assert.throws(()=>runOriginalLinkCompleteTableNative({}, {environment,spawn:fake.spawn}),/explicit CI-only switch/u);assert.equal(fake.calls.length,0);
});

// CI flag0 performs the checked planner and injected contracts. Flag1 adds the
// real native group below. Neither default local import nor a fake writes proof.
if(process.env.CI==='true'){
 const plan=planOriginalLinkCompleteTableNative(),entry=plan.entries[0],stream=bitStream(entry.observations),byId=new Map(entry.observations.map(row=>[row.id,row]));
 const artifact=readProjectData(new URL('./data/native-definitions.lino',import.meta.url));
 const injected=(fake,options={})=>runOriginalLinkCompleteTableNative(plan,{environment:fakeEnvironment,spawn:fake.spawn,...options});
 const finite=bits=>Buffer.from(bits,'hex').readDoubleBE();
 process.stdout.write(`Complete-table CI plan: ${entry.invocationCount} calls; ${entry.protocolBytes}/${entry.maximumProtocolBytes} stdout bytes; ${entry.consumerBytes}/${entry.maximumConsumerBytes} consumer bytes; native=${nativeEnabled}\n`);

 test('the actual whole unchanged sixty-six-definition source and mixed/nested ABI are authenticated',()=>{
  assert.equal(plan.modelCommit,ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT);assert.equal(plan.modelCommit,'9a835e469572341e6a8e5c6e9fe6f40b8655dfbe');assert.equal(plan.originalCount,1);assert.equal(plan.entries.length,1);
  assert.equal(entry.sourceBytes,193995);assert.equal(Buffer.byteLength(entry.source),193995);assert.equal(entry.sourceSha256,'761b3611c682e4bbcc85c29a1b91ed58a565d89e47a295703488c69a7b64fe6b');assert.equal(digest(entry.source),entry.sourceSha256);assert.equal(entry.source,artifact.completeRust);assert.equal(entry.replaySha256,entry.sourceSha256);assert.equal(entry.textReplaySha256,entry.sourceSha256);
  assert.equal(entry.declarations.length,66);assert.deepEqual(entry.declarations.map(row=>row.name),artifact.declarationNames);entry.declarations.forEach((row,index)=>assert.equal(row.sha256,digest(artifact.declarations[index])));assert.equal(entry.declarationSha256,digest(JSON.stringify(artifact.declarations)));
  assert.equal(entry.functionName,'project_original_link_complete_certificate_tables');assert.equal(entry.parameterOrder.length,17);assert.deepEqual(entry.parameterModes,['f64','f64',...Array(15).fill('Vec<f64>')]);assert.equal(entry.resultMode,'Vec<Vec<f64>>');assert.equal(entry.referenceSemantics,false);assert.equal(entry.tableOrder.length,17);
  assert.match(entry.consumerSource,/oracle_source::project_original_link_complete_certificate_tables\(input_0,input_1,input_2\.clone\(\)/u);assert.match(entry.consumerSource,/original vector 16 changed/u);assert.match(entry.consumerSource,/whole result row count differs/u);assert.match(entry.consumerSource,/whole result row differs/u);assert.equal(digest(entry.consumerSource),entry.consumerSha256);
  assert.equal(Buffer.byteLength(stream),entry.protocolBytes);assert.ok(entry.protocolBytes<=1024*1024);assert.equal(Buffer.byteLength(entry.consumerSource),entry.consumerBytes);assert.ok(entry.consumerBytes<=4*1024*1024);assert.match(entry.observationScope,/No native pre-Serde/u);assert.match(entry.observationScope,/not a universal NaN/u);
 });

 test('all thirteen originals and four coherent edits retain both complete archive premises before exact-bit deduplication',()=>{
  assert.equal(entry.invocationCount,45);assert.equal(entry.originalFixtureCount,13);assert.equal(entry.coherentEditCount,4);assert.equal(entry.archiveTransportCount,34);assert.equal(entry.observations.length,45);
  const graphRows=entry.observations.filter(row=>row.kind==='actual checked original-ID source graph');assert.equal(graphRows.length,17);assert.equal(graphRows.filter(row=>row.id.startsWith('fixture ')).length,13);
  for(const row of graphRows){assert.equal(row.provenance.length,2);assert.deepEqual(row.provenance.map(item=>item.format),['text','binary']);assert.equal(row.provenance[0].inputSha256,row.provenance[1].inputSha256);assert.equal(row.resultRows.length,17);assert.equal(row.certificate.referenceRuntime,'JavaScript');assert.equal(row.certificate.kind,'complete certificate decision');assert.equal(finite(row.certificate.bits[0]),1);for(const receipt of row.provenance){assert.match(receipt.root,/^[0-9]+$/u);assert.match(receipt.body,/^[0-9]+$/u);assert.match(receipt.proof,/^[0-9]+$/u);assert.match(receipt.networkSha256,/^[0-9a-f]{64}$/u);}}
  for(const id of['coherent clone receiver retarget','coherent declaration and parameter rename','coherent control and Unicode literal','coherent empty String literal'])assert.ok(byId.has(id));
  assert.equal(entry.rawRefusalCount,28);assert.equal(entry.rawRefusals.length,28);for(const row of entry.rawRefusals){assert.notEqual(row.before,row.after);assert.equal(row.refusal.name,'TypeError');assert.match(row.scope,/no native graph-decoder claim/u);}assert.equal(new Set(entry.rawRefusals.map(row=>row.role+':'+row.kind+':'+row.format)).size,28);
 });

 test('primitive owners, exact marker/prefix refusals and chosen nonfinite bits remain separately observable',()=>{
  for(const role of['declaration','signature','resultPosition','operationNode','coreValue','input','position']){const row=byId.get('primitive foreign owner '+role);assert.equal(row.resultRows.length,17);assert.equal(row.certificate.kind,'complete certificate decision');assert.notEqual(finite(row.certificate.bits[0]),1);}
  for(const id of['root marker payload/code contradiction','child marker payload/code contradiction','dynamic String forged fixed code','fixed prefix contradiction 14','fixed prefix contradiction 15','fixed prefix contradiction 16'])assert.deepEqual(byId.get(id).resultRows.map(row=>row.bits.map(finite)),[[-32]]);
  assert.deepEqual(byId.get('root marker wrong scalar tag precedence').resultRows.map(row=>row.bits.map(finite)),[[-22]]);assert.deepEqual(byId.get('shape failure before alien root').resultRows.map(row=>row.bits.map(finite)),[[-30]]);
  for(const[id,bits]of[['negative zero','8000000000000000'],['chosen NaN','7ff8000000000000'],['positive infinity','7ff0000000000000'],['negative infinity','fff0000000000000']]){const row=byId.get('Number payload '+id);assert.ok(row.inputArguments[8].bits.includes(bits));assert.equal(row.resultRows[0].bits[11],bits);assert.ok(row.certificate.kind==='complete certificate scalar refusal'||finite(row.certificate.bits[0])!==1);}
  for(const row of entry.observations){assert.equal(row.inputArguments.length,17);for(const[index,arg]of row.inputArguments.entries()){assert.equal(arg.kind,index<2?'scalar':'vector');for(const bits of arg.kind==='scalar'?[arg.bits]:arg.bits)assert.match(bits,/^[0-9a-f]{16}$/u);}for(const result of row.resultRows)result.bits.forEach(bits=>assert.match(bits,/^[0-9a-f]{16}$/u));}
 });

 test('all fixed experiment and dependency byte mutations refuse before checked planning',()=>{
  const receipts=verifyOriginalLinkCompleteTableNativeModel();assert.equal(receipts.length,35);
  for(const row of receipts){const url=new URL(row.file,import.meta.url).href;assert.throws(()=>verifyOriginalLinkCompleteTableNativeModel(file=>new URL(file).href===url?Buffer.concat([readFileSync(file),Buffer.from('changed')]):readFileSync(file)),/frozen complete-table source changed/u);}
 });

 test('injected processes reconstruct every complete result through the reversible bit dictionary without native evidence',()=>{
  const fake=fakeProcess(stream),receipt=injected(fake);assert.equal(receipt.processMode,'injected process contract');assert.equal(receipt.repositoryCommit,fakeEnvironment.GITHUB_SHA);assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.deepEqual(fake.calls.map(call=>call.phase),['library','consumer','execute']);
  for(const call of fake.calls){assert.equal(call.options.shell,false);assert.equal(call.options.env,fakeEnvironment);assert.equal(call.options.maxBuffer,1024*1024);assert.equal(existsSync(call.options.cwd),false,'all temporary sources/artifacts are removed');}assert.ok(fake.calls[0].args.includes('--crate-type=rlib'));assert.ok(fake.calls[1].args.includes('--extern'));
  const observed=receipt.observations[0];assert.equal(observed.observations.length,45);assert.equal(observed.sourceSha256,entry.sourceSha256);assert.equal(observed.interfaceSha256,entry.interfaceSha256);assert.equal(observed.declarationSha256,entry.declarationSha256);assert.equal(observed.protocolBytes,Buffer.byteLength(stream));assert.equal(observed.consumerBytes,entry.consumerBytes);assert.equal(observed.rawRefusals.length,28);
  for(const[index,row]of observed.observations.entries()){assert.deepEqual(row.resultRows,entry.observations[index].resultRows);assert.deepEqual(row.inputArguments,entry.observations[index].inputArguments);assert.deepEqual(row.provenance,entry.observations[index].provenance);}
  const selected={processMode:receipt.processMode,sourceSha256:observed.sourceSha256,observations:observed.observations.filter(row=>row.id.startsWith('Number payload '))};assert.deepEqual(readStructuredDataLino(serializeProjectData(selected,'complete-table-bit-receipt.lino')),selected);
 });

 test('the exact checked plan and experiment-specific native switch precede even injected process calls',()=>{
  for(const environment of[{}, {CI:'true'},{CI:'true',ORIGINAL_LINK_COMPLETE_TABLE_NATIVE:'0'}]){const fake=fakeProcess(stream);assert.throws(()=>injected(fake,{environment}),/explicit CI-only switch/u);assert.equal(fake.calls.length,0);}
  const fake=fakeProcess(stream);assert.throws(()=>runOriginalLinkCompleteTableNative({...plan},{environment:fakeEnvironment,spawn:fake.spawn}),/exact checked/u);assert.equal(fake.calls.length,0);
 });

 test('strict dictionary/index/row/cell/stream mutations cannot be accepted as full observations',()=>{
  const lines=stream.trimEnd().split('\n'),header=lines[0].split(':'),dictionary=header.slice(2),first=lines[1].split(':'),streams=[];
  const altered=(line,value)=>{const copy=[...lines];copy[line]=value;return copy.join('\n')+'\n';};
  streams.push(stream.slice(0,-1),stream+'\n',lines.slice(0,-1).join('\n')+'\n',[lines[0],lines[2],lines[1],...lines.slice(3)].join('\n')+'\n');
  const duplicate=[...header];duplicate[3]=duplicate[2];streams.push(altered(0,duplicate.join(':')));const count=[...header];count[1]=String(Number(count[1])+1);streams.push(altered(0,count.join(':')));
  const reordered=[...header];[reordered[2],reordered[3]]=[reordered[3],reordered[2]];streams.push(altered(0,reordered.join(':')));
  const coherent=[reordered.join(':')];for(const line of lines.slice(1)){const parts=line.split(':'),rows=Number(parts[1]);let at=2;for(let row=0;row<rows;row++){const length=Number(parts[at++]);for(let cell=0;cell<length;cell++,at++){if(parts[at]==='0')parts[at]='1';else if(parts[at]==='1')parts[at]='0';}}coherent.push(parts.join(':'));}streams.push(coherent.join('\n')+'\n');
  let unused=0n;while(dictionary.includes(unused.toString(16).padStart(16,'0')))unused++;const extra=[...header,unused.toString(16).padStart(16,'0')];extra[1]=String(dictionary.length+1);streams.push(altered(0,extra.join(':')));
  for(const value of[dictionary.length.toString(16),'00','g']){const parts=[...first];parts[3]=value;streams.push(altered(1,parts.join(':')));}
  for(const index of[1,2]){const parts=[...first];parts[index]=String(Number(parts[index])+1);streams.push(altered(1,parts.join(':')));}
  streams.push(altered(1,[...first,'0'].join(':')),altered(1,first.slice(0,-1).join(':')));const badBits=[...header];badBits[2]=unused.toString(16).padStart(16,'0');streams.push(altered(0,badBits.join(':')));
  for(const stdout of streams){const fake=fakeProcess(stream,call=>call.phase==='execute'?{status:0,signal:null,stdout,stderr:''}:null);fails(()=>injected(fake),/Original complete link table observations failed/u);assert.equal(fake.calls.length,3);}
 });

 test('compiler/execution failures, missing artifacts and source/consumer mutation fail closed and clean up',()=>{
  for(const phase of['library','consumer','execute'])for(const failure of[{status:1,stdout:'',stderr:'actual failure'},{status:null,signal:'SIGTERM',stdout:'',stderr:''},{error:{code:'ETIMEDOUT'},stdout:'',stderr:''}]){const fake=fakeProcess(stream,call=>call.phase===phase?failure:null);fails(()=>injected(fake),/exit 1|terminated by SIGTERM|timed out/u);for(const call of fake.calls)assert.equal(existsSync(call.options.cwd),false);}
  for(const phase of['library','consumer']){const fake=fakeProcess(stream,call=>call.phase===phase?{status:0,signal:null,stdout:'',stderr:''}:null);fails(()=>injected(fake),/compiler did not create/u);assert.equal(fake.calls.some(call=>call.phase==='execute'),false);}
  for(const phase of['library','consumer','execute']){const fake=fakeProcess(stream,call=>{if(call.phase===phase){const file=join(call.options.cwd,phase==='consumer'?'consumer.rs':'original.rs');writeFileSync(file,readFileSync(file,'utf8')+'\n// process mutation\n');}});fails(()=>injected(fake),/bytes changed|consumer changed/u);for(const call of fake.calls)assert.equal(existsSync(call.options.cwd),false);}
 });

 if(nativeEnabled)test('CI executes the complete original projector for all forty-five whole-table/refusal invocations',()=>{
  const receipt=runOriginalLinkCompleteTableNative(plan);assert.equal(receipt.processMode,'native subprocesses');assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.equal(receipt.observations[0].observations.length,45);assert.equal(receipt.observations[0].rawRefusals.length,28);
  const file=fileURLToPath(new URL('../../issue-195-artifacts/original-link-complete-table-native-observations.lino',import.meta.url));mkdirSync(dirname(file),{recursive:true});writeProjectData(file,receipt);process.stdout.write('Original complete table native oracle: 1 unchanged sixty-six-definition artifact, 45 actual whole-table/refusal invocations; receipts '+file+'\n');
 });
}
