import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData,writeProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/index.js';
import {ORIGINAL_LINK_SYMBOL_MODEL_COMMIT,verifyOriginalLinkSymbolNativeModel,originalLinkSymbolNativeMode,planOriginalLinkSymbolNative,runOriginalLinkSymbolNative} from './native-observations.mjs';

const nativeEnabled=originalLinkSymbolNativeMode(process.env),plan=planOriginalLinkSymbolNative(),entry=plan.entries[0];
const artifact=readProjectData(new URL('./data/native-definitions.lino',import.meta.url));
const digest=value=>createHash('sha256').update(value).digest('hex');
const fakeEnvironment={CI:'true',ORIGINAL_LINK_SYMBOL_NATIVE:'1',GITHUB_SHA:'injected-process-contract-only'};
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
const injected=(fake,options={})=>runOriginalLinkSymbolNative(plan,{environment:fakeEnvironment,spawn:fake.spawn,...options});
const fails=(operation,pattern)=>assert.throws(operation,error=>{assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,1);assert.match(error.message,pattern);return true;});
const byId=new Map(entry.observations.map(row=>[row.id,row]));
function outputParts(id){const result=byId.get(id).decision;assert.equal(result[0],1);const symbolEnd=4+result[1],nameEnd=symbolEnd+result[2]*2;assert.equal(result.length,nameEnd+result[3]);return{symbols:result.slice(4,symbolEnd),names:result.slice(symbolEnd,nameEnd),units:result.slice(nameEnd)};}

test('the oracle authenticates all twenty-two unchanged declarations and the actual value-vector ABI',()=>{
 assert.equal(plan.modelCommit,ORIGINAL_LINK_SYMBOL_MODEL_COMMIT);assert.equal(plan.modelCommit,'ad70aa52e2a0ad5755d2a90ed0bf04cb784f5e2d');assert.equal(plan.originalCount,1);assert.equal(plan.entries.length,1);
 assert.equal(entry.sourceBytes,28991);assert.equal(Buffer.byteLength(entry.source),entry.sourceBytes);assert.equal(digest(entry.source),'8e59fc8954cf258a7707984590f00ee7db1598365eb4a99702b5fb4d2b6a662b');assert.equal(entry.source,artifact.completeRust);assert.equal(entry.replaySha256,entry.sourceSha256);assert.equal(entry.textReplaySha256,entry.sourceSha256);
 assert.equal(entry.declarations.length,22);assert.equal(artifact.declarations.length,22);assert.deepEqual(entry.declarations.map(row=>row.name),artifact.declarationNames);
 for(const[index,definition]of artifact.declarations.entries()){assert.ok(entry.source.includes(definition));assert.equal(entry.declarations[index].sha256,digest(definition));}
 assert.equal(entry.declarationSha256,'45e2791c24fa70d9177eb5f216165244cac0600e0d26f47419b82df2a3c07d2c');assert.equal(entry.declarationSha256,digest(JSON.stringify(artifact.declarations)));
 assert.equal(entry.sourceModelSha256,'ece8ece9be52b5917944ebd53d604d956ad4e973880ee7f719e8b5c620db152f');assert.equal(entry.kernelSha256,artifact.sourceSha256);assert.equal(entry.interfaceSha256,'4538221a849d4132bd16015ef228144e336aa9c7766282691da99ddd2f81e343');
 assert.deepEqual(entry.parameterOrder,['fixed_starts','fixed_lengths','fixed_units','source_starts','source_lengths','source_units']);assert.deepEqual(entry.parameterModes,Array(6).fill('Vec<f64>'));assert.equal(entry.resultMode,'Vec<f64>');assert.equal(entry.referenceSemantics,false);
 assert.throws(()=>{entry.observations[0].resultBits.push('0000000000000000');},TypeError);
});

test('bounded complete outputs distinguish null, absent, empty, exact UTF16 and first-observed dictionary order',()=>{
 assert.equal(entry.invocationCount,84);assert.equal(entry.observations.length,84);assert.equal(entry.maximumObservedInputCells,1413);assert.equal(entry.maximumObservedOutputCells,1769);
 assert.deepEqual(byId.get('empty').decision,[1,0,1,0,0,0]);assert.deepEqual(outputParts('absent non-String values').symbols,[-1,-1,-1,-1]);
 assert.deepEqual(outputParts('empty Strings retain positive symbols').symbols,[1,1,2,3,2]);assert.deepEqual(outputParts('fixed dictionary precedes dynamic order').symbols,[3,4,2,4,-1,1]);
 const exact=outputParts('NUL pairs and isolated UTF16 units');assert.equal(exact.symbols[4],exact.symbols[8]);assert.notEqual(exact.symbols[4],exact.symbols[5]);assert.ok(exact.units.includes(55296));assert.ok(exact.units.includes(56320));assert.ok(exact.units.includes(0));
 assert.deepEqual(outputParts('normalization remains distinct').symbols,[1,2,1,2]);assert.deepEqual(outputParts('null is distinct from its spelling').symbols,[-1,1,2,-1,1]);
 const fixed=outputParts('actual fixed107 dictionary');assert.equal(fixed.symbols[0],107);assert.equal(fixed.symbols[1],108);assert.equal(fixed.symbols[2],1);assert.equal(fixed.symbols[3],108);assert.equal(fixed.symbols[4],-1);
 assert.match(entry.consumerSource,/oracle_source::project_original_link_symbol_values/u);assert.match(entry.consumerSource,/original input vector 5 changed/u);assert.match(entry.consumerSource,/actual complete symbol projection differs from JavaScript/u);assert.equal(digest(entry.consumerSource),entry.consumerSha256);
 assert.match(entry.observationScope,/prove no original LinkId identity/u);assert.match(entry.observationScope,/native100000-row/u);
});

test('all six numeric vector domains and exact refusal precedence retain complete negative vectors',()=>{
 for(const slot of[0,1,2,3,4,5])for(const label of['negative zero','negative','fraction','NaN','positive infinity','negative infinity','above uint16','above declared bound','subnormal','maximum Number'])assert.deepEqual(byId.get(label+' vector '+slot).decision,slot<3?[-1]:[-2]);
 assert.deepEqual(byId.get('duplicate fixed String').decision,[-3,2]);assert.deepEqual(byId.get('duplicate fixed empty String').decision,[-3,1]);assert.deepEqual(byId.get('source invalid before duplicate fixed').decision,[-2]);assert.deepEqual(byId.get('fixed invalid before source invalid').decision,[-1]);assert.deepEqual(byId.get('only minus one means absent').decision,[-2]);
 for(const row of entry.observations){assert.equal(row.inputBits.length,6);row.inputBits.flat().forEach(bits=>assert.match(bits,/^[0-9a-f]{16}$/u));row.resultBits.forEach(bits=>assert.match(bits,/^[0-9a-f]{16}$/u));}
});

test('source-model byte mutations fail before any native process can be planned',()=>{
 const receipts=verifyOriginalLinkSymbolNativeModel();assert.equal(receipts.length,5);
 for(const row of receipts)assert.throws(()=>verifyOriginalLinkSymbolNativeModel(file=>new URL(file).pathname.endsWith('/'+row.file)?Buffer.concat([readFileSync(file),Buffer.from('changed')]):readFileSync(file)),/frozen symbol projection source changed/u);
});

test('injected compiler and consumer contracts authenticate the original bytes without creating native evidence',()=>{
 const fake=fakeProcess(),receipt=injected(fake);assert.equal(receipt.processMode,'injected process contract');assert.equal(receipt.repositoryCommit,fakeEnvironment.GITHUB_SHA);
 assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.deepEqual(fake.calls.map(call=>call.phase),['library','consumer','execute']);
 for(const call of fake.calls){assert.equal(call.options.shell,false);assert.equal(call.options.env,fakeEnvironment);assert.ok(call.options.timeout>0);assert.equal(existsSync(call.options.cwd),false,'temporary process artifacts are removed');}
 assert.ok(fake.calls[0].args.includes('--edition=2024'));assert.ok(fake.calls[0].args.includes('--crate-type=rlib'));assert.ok(fake.calls[1].args.includes('--extern'));
 const observed=receipt.observations[0];assert.equal(observed.invocationCount,84);assert.equal(observed.observations.length,84);assert.equal(observed.sourceSha256,entry.sourceSha256);assert.equal(observed.interfaceSha256,entry.interfaceSha256);assert.equal(observed.declarationSha256,entry.declarationSha256);assert.equal(observed.sourceModelSha256,entry.sourceModelSha256);assert.equal(observed.referenceSemantics,false);
 assert.deepEqual(readStructuredDataLino(serializeProjectData(receipt,'symbol-native-receipt.lino')),receipt);
});

test('explicit CI authorization and the exact checked plan precede even injected processes',()=>{
 assert.equal(originalLinkSymbolNativeMode({}),false);assert.equal(originalLinkSymbolNativeMode({CI:'true'}),false);assert.equal(originalLinkSymbolNativeMode({CI:'true',ORIGINAL_LINK_SYMBOL_NATIVE:'1'}),true);
 for(const environment of[{ORIGINAL_LINK_SYMBOL_NATIVE:'1'},{CI:'false',ORIGINAL_LINK_SYMBOL_NATIVE:'1'}])assert.throws(()=>originalLinkSymbolNativeMode(environment),/requires CI=true/u);
 for(const environment of[{}, {CI:'true'}]){const fake=fakeProcess();assert.throws(()=>injected(fake,{environment}),/explicit CI-only switch/u);assert.equal(fake.calls.length,0);}
 const fake=fakeProcess();assert.throws(()=>runOriginalLinkSymbolNative({...plan},{environment:fakeEnvironment,spawn:fake.spawn}),/exact checked/u);assert.equal(fake.calls.length,0);
});

test('truncated, reordered, extra and incorrect variable projection streams cannot pass',()=>{
 const lines=stream.trimEnd().split('\n'),wrong=[stream.slice(0,-1),stream+'84\n',lines.slice(1).join('\n')+'\n',[lines[1],lines[0],...lines.slice(2)].join('\n')+'\n',lines.map((line,index)=>index===0?line+':0000000000000000':line).join('\n')+'\n',stream.replace('3ff0000000000000','4000000000000000')];
 for(const stdout of wrong){const fake=fakeProcess(call=>call.phase==='execute'?{status:0,signal:null,stdout,stderr:''}:null);fails(()=>injected(fake),/Original complete link symbol projection observations failed/u);assert.equal(fake.calls.length,3);}
});

test('compiler and execution failures, missing artifacts and process-side source edits fail closed',()=>{
 for(const phase of['library','consumer','execute'])for(const failure of[{status:1,stdout:'',stderr:'actual failure'},{status:null,signal:'SIGTERM',stdout:'',stderr:''},{error:{code:'ETIMEDOUT'},stdout:'',stderr:''}]){
  const fake=fakeProcess(call=>call.phase===phase?failure:null);fails(()=>injected(fake),/exit 1|terminated by SIGTERM|timed out/u);
 }
 for(const phase of['library','consumer']){const fake=fakeProcess(call=>call.phase===phase?{status:0,signal:null,stdout:'',stderr:''}:null);fails(()=>injected(fake),/compiler did not create/u);assert.equal(fake.calls.some(call=>call.phase==='execute'),false);}
 for(const phase of['library','consumer','execute']){const fake=fakeProcess(call=>{if(call.phase===phase){const file=join(call.options.cwd,phase==='consumer'?'consumer.rs':'original.rs');writeFileSync(file,readFileSync(file,'utf8')+'\n// changed by process\n');}});fails(()=>injected(fake),/bytes changed|consumer changed/u);}
});

// Only the explicit CI selector compiles, executes, then records actual observations.
if(nativeEnabled)test('CI executes the entire original symbol kernel over all eighty-four bounded vector sets',()=>{
 const receipt=runOriginalLinkSymbolNative(plan);assert.equal(receipt.processMode,'native subprocesses');assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.equal(receipt.observations[0].observations.length,84);
 const file=fileURLToPath(new URL('../../issue-195-artifacts/original-link-symbol-native-observations.lino',import.meta.url));mkdirSync(dirname(file),{recursive:true});writeProjectData(file,receipt);
 process.stdout.write('Original link symbol native oracle: 1 unchanged twenty-two-declaration module, 84 actual vector invocations; receipts '+file+'\n');
});
