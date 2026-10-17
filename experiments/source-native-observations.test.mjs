// Mechanical bundle contracts only. Every process is injected; no compiler can run here.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,symlinkSync,unlinkSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {createSourceNativeObservationRunner} from './source-native-observations.mjs';
import {readProjectData} from '../js/scripts/project-data.mjs';
const sha256=value=>createHash('sha256').update(value).digest('hex');
const textBytes=value=>{if(typeof value!=='string'||!value.isWellFormed())throw new TypeError('primitive scalar source text required');return Buffer.from(value);};
const descriptor=(relativePath,source)=>({relativePath,source,sourceBytes:textBytes(source).length,sourceSha256:sha256(textBytes(source))});
const environment={CI:'true',SOURCE_BUNDLE_CONTRACT:'1'};
function makePlan(change=()=>{},bundle=true){
 const entry={id:'mechanical source fixture',kind:'contract-only',fixtureId:'mechanical source fixture',source:'#[path="library/mod.rs"] pub mod sample;\n',consumerSource:'fn main(){}\n',replaySha256:'contract-only',originalReceipt:{source:'contract-only'},javaScriptSha256:'contract-only',observations:[true]};
 entry.sourceBytes=textBytes(entry.source).length;entry.sourceSha256=sha256(entry.source);entry.consumerSha256=sha256(entry.consumerSource);
 if(bundle)entry.additionalSources=[descriptor('library/mod.rs','include!("part_0.rs");\n'),...Array.from({length:4},(_,index)=>descriptor('library/part_'+index+'.rs','// Ω😀 '+index+'\r\npub fn part_'+index+'()->bool{true}\r\n'))];
 change(entry);return{model:'source bundle transport contracts only',modelCommit:'contract-only',modelSources:[],originalCount:1,entries:[entry]};
}
function harness(plan,compilerProfile='original'){
 const checked=new WeakSet([plan]),calls=[];
 const run=createSourceNativeObservationRunner({
  nativeMode:env=>{if(env.SOURCE_BUNDLE_CONTRACT!=='1')return false;if(env.CI!=='true')throw new TypeError('contract requires CI=true');return true;},
  checkedPlan:value=>checked.has(value),textBytes,parseObservations:(output,expected)=>{assert.equal(output,'actual fake output\n');assert.deepEqual(expected,[true]);return[{value:true}];},
  entryReceipt:()=>({contract:'only fake subprocesses'}),temporaryPrefix:'source-bundle-contract-',failureLabel:'Source bundle contract failed',
  compilerProfile,
 });
 const fake=override=>(command,args,options)=>{
  const phase=args.includes('--crate-type=rlib')?'library':args.includes('--extern')?'consumer':'execute';const call={command,args,options,phase};calls.push(call);
  const result=override?.(call);if(result)return result;
  if(phase!=='execute')writeFileSync(args[args.indexOf('-o')+1],'fake compiler artifact, not Rust');
  return{status:0,stdout:phase==='execute'?'actual fake output\n':'',stderr:'',signal:null};
 };
 return{run,calls,fake};
}
function failure(action,pattern){assert.throws(action,error=>{assert.ok(error instanceof AggregateError);assert.match(error.message,pattern);return true;});}

test('optional source bundles retain every exact sidecar and unchanged compiler arguments through one fake execution',()=>{
 const plan=makePlan(),h=harness(plan),entry=plan.entries[0];
 const receipt=h.run(plan,{environment,spawn:h.fake(({args,options,phase})=>{
  assert.equal(options.shell,false);assert.equal(options.encoding,'utf8');assert.equal(options.env,environment);
  assert.equal(readFileSync(join(options.cwd,'original.rs'),'utf8'),entry.source);assert.equal(readFileSync(join(options.cwd,'consumer.rs'),'utf8'),entry.consumerSource);
  for(const source of entry.additionalSources)assert.deepEqual(readFileSync(join(options.cwd,source.relativePath)),textBytes(source.source));
  if(phase==='library')assert.deepEqual(args,['--edition=2024','--crate-type=rlib','--crate-name','oracle_source',join(options.cwd,'original.rs'),'-o',join(options.cwd,'liboracle_source.rlib')]);
  if(phase==='consumer')assert.deepEqual(args,['--edition=2024',join(options.cwd,'consumer.rs'),'--extern','oracle_source='+join(options.cwd,'liboracle_source.rlib'),'-o',join(options.cwd,'consumer')]);
  if(phase==='execute')assert.deepEqual(args,[]);
 })});
 assert.equal(receipt.processMode,'injected process contract');assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.equal(h.calls.length,3);
 assert.deepEqual(receipt.observations[0].additionalSources,entry.additionalSources.map(({relativePath,sourceBytes,sourceSha256})=>({relativePath,sourceBytes,sourceSha256})));
 assert.ok(receipt.observations[0].additionalSources.every(source=>!Object.hasOwn(source,'source')));
 assert.equal(existsSync(h.calls[0].options.cwd),false);
});

test('absent additionalSources retains the exact existing single-file receipt shape and no new files',()=>{
 const plan=makePlan(()=>{},false),entry=plan.entries[0],h=harness(plan);
 const receipt=h.run(plan,{environment,spawn:h.fake(({options,phase})=>{
  const expected=phase==='library'?['consumer.rs','original.rs']:phase==='consumer'?['consumer.rs','liboracle_source.rlib','original.rs']:['consumer','consumer.rs','liboracle_source.rlib','original.rs'];
  assert.deepEqual(readdirSync(options.cwd).sort(),expected);
 })});
 const actual=receipt.observations[0];assert.deepEqual(Object.keys(actual),['id','kind','fixtureId','sourceBytes','sourceSha256','replaySha256','originalReceipt','contract','consumerSha256','javaScriptSha256','compiler','compilerArguments','observations']);
 assert.equal(Object.hasOwn(actual,'additionalSources'),false);assert.equal(entry.sourceSha256,actual.sourceSha256);assert.equal(h.calls.length,3);
});

test('explicit CI and checked-plan authorization run before any bundle parsing, file creation or process',()=>{
 const plan=makePlan(),h=harness(plan);let reads=0;Object.defineProperty(plan.entries[0],'additionalSources',{get(){reads++;throw new Error('must not read');},enumerable:true});
 for(const env of[{}, {SOURCE_BUNDLE_CONTRACT:'1'}])assert.throws(()=>h.run(plan,{environment:env,spawn:h.fake()}),/explicit CI-only switch|requires CI=true/u);
 assert.throws(()=>h.run({...plan},{environment,spawn:h.fake()}),/exact checked in-memory/u);assert.equal(reads,0);assert.equal(h.calls.length,0);
});

test('all unsafe, absolute, traversal, reserved and file-directory/case-colliding paths refuse before any write or process',()=>{
 const paths=['','/absolute.rs','../outside.rs','library/../outside.rs','./part.rs','library//part.rs','library/','library\\part.rs','C:/part.rs','part.rs\0','library/.hidden.rs','original.rs','ORIGINAL.RS','consumer.rs/part.rs','consumer','consumer.exe','liboracle_source.rlib','CON.rs','library/nul','library/LPT1.rs'];
 const mutations=paths.map(path=>entry=>{entry.additionalSources[4].relativePath=path;});
 mutations.push(entry=>{entry.additionalSources[4].relativePath='LIBRARY/PART_0.RS';},entry=>{entry.additionalSources[4].relativePath='library';},entry=>{entry.additionalSources[4].relativePath='library/part_0.rs/child.rs';});
 for(const mutate of mutations){
  const root=mkdtempSync(join(tmpdir(),'source-bundle-preflight-'));
  try{const plan=makePlan(mutate),h=harness(plan);failure(()=>h.run(plan,{environment,spawn:h.fake(),temporaryRoot:root}),/safe relative path|collides/u);assert.deepEqual(readdirSync(root),[]);assert.equal(h.calls.length,0);}finally{rmSync(root,{recursive:true,force:true});}
 }
});

test('every manifest descriptor and all source/hash receipts validate before the first write, including later plan entries',()=>{
 const mutations=[
  entry=>{entry.additionalSources=null;},entry=>{entry.additionalSources={};},entry=>{entry.additionalSources.push(null);},entry=>{entry.additionalSources[0].source+='changed';},
  entry=>{entry.additionalSources[0].sourceBytes++;},entry=>{entry.additionalSources[0].sourceSha256='wrong';},entry=>{entry.additionalSources[0].extra=true;},
  entry=>{delete entry.additionalSources[0];},entry=>{entry.additionalSources.extra=true;},entry=>{entry.additionalSources[0].source=String.fromCharCode(0xd800);},
  entry=>{entry.consumerSource+='changed';},entry=>{entry.source+='changed';},
 ];
 for(const mutate of mutations){
  const root=mkdtempSync(join(tmpdir(),'source-bundle-all-before-write-'));
  try{const plan=makePlan(mutate);plan.entries.unshift(makePlan(()=>{},false).entries[0]);const h=harness(plan);failure(()=>h.run(plan,{environment,spawn:h.fake(),temporaryRoot:root}),/Source bundle contract failed/u);assert.equal(h.calls.length,0);assert.deepEqual(readdirSync(root),[]);}finally{rmSync(root,{recursive:true,force:true});}
 }
});

test('source manifest accessors are rejected without observing getter values',()=>{
 for(const role of['manifest','entry','array']){
  let reads=0;const plan=makePlan(entry=>{
   const getter={get(){reads++;throw new Error('getter observed');},enumerable:true};
   if(role==='manifest')Object.defineProperty(entry,'additionalSources',getter);
   else if(role==='entry')Object.defineProperty(entry.additionalSources[0],'source',getter);
   else Object.defineProperty(entry.additionalSources,'0',getter);
  }),h=harness(plan);
  failure(()=>h.run(plan,{environment,spawn:h.fake()}),/owned data field|accessors/u);assert.equal(reads,0);assert.equal(h.calls.length,0);
 }
});

test('sidecar byte edits during each process stop subsequent phases and cleanup all private files',()=>{
 for(const phase of['library','consumer','execute']){
  const plan=makePlan(),h=harness(plan);failure(()=>h.run(plan,{environment,spawn:h.fake(call=>{
   if(call.phase===phase)writeFileSync(join(call.options.cwd,'library/part_3.rs'),'process-side edit');
  })}),/additional source bytes changed/u);
  assert.equal(h.calls.at(-1).phase,phase);assert.equal(existsSync(h.calls[0].options.cwd),false);
 }
});

test('primary and consumer mutations remain authenticated alongside the bundle after every process',()=>{
 for(const name of['original.rs','consumer.rs'])for(const phase of['library','consumer','execute']){
  const plan=makePlan(),h=harness(plan);failure(()=>h.run(plan,{environment,spawn:h.fake(call=>{if(call.phase===phase)writeFileSync(join(call.options.cwd,name),'process-side edit');})}),/bytes changed|external consumer changed/u);
  assert.equal(h.calls.at(-1).phase,phase);assert.equal(existsSync(h.calls[0].options.cwd),false);
 }
});

test('sidecar files, parent directories and generated artifacts cannot become symlinks even to matching bytes',()=>{
 const outside=mkdtempSync(join(tmpdir(),'source-bundle-external-'));
 try{
  const plan=makePlan();for(const source of plan.entries[0].additionalSources){const file=join(outside,source.relativePath);mkdirSync(join(file,'..'),{recursive:true});writeFileSync(file,source.source);}
  writeFileSync(join(outside,'artifact'),'matching fake compiler bytes');
  for(const role of['file','directory','library','executable']){
   const current=makePlan(),h=harness(current);failure(()=>h.run(current,{environment,spawn:h.fake(call=>{
    if(call.phase==='library'){
     if(role==='file'){const file=join(call.options.cwd,'library/part_0.rs');unlinkSync(file);symlinkSync(join(outside,'library/part_0.rs'),file);}
     if(role==='directory'){const directory=join(call.options.cwd,'library');rmSync(directory,{recursive:true});symlinkSync(join(outside,'library'),directory,'dir');}
     if(role==='library')symlinkSync(join(outside,'artifact'),call.args[call.args.indexOf('-o')+1]);
    }
    if(call.phase==='consumer'&&role==='executable')symlinkSync(join(outside,'artifact'),call.args[call.args.indexOf('-o')+1]);
    if((call.phase==='library'&&role==='library')||(call.phase==='consumer'&&role==='executable'))return{status:0,stdout:'',stderr:'',signal:null};
   })}),/symbolic links/u);
   assert.equal(h.calls.some(call=>call.phase==='execute'),false);assert.equal(existsSync(h.calls[0].options.cwd),false);
  }
  assert.equal(readFileSync(join(outside,'artifact'),'utf8'),'matching fake compiler bytes');
 }finally{rmSync(outside,{recursive:true,force:true});}
});

test('dangling library and executable symlinks refuse after every phase before any later process can follow them',()=>{
 for(const artifact of['liboracle_source.rlib','consumer'])for(const phase of['library','consumer','execute']){
  const plan=makePlan(),h=harness(plan);
  failure(()=>h.run(plan,{environment,spawn:h.fake(call=>{
   if(call.phase!==phase)return;
   if(phase!=='execute')writeFileSync(call.args[call.args.indexOf('-o')+1],'valid fake compiler artifact');
   const path=join(call.options.cwd,artifact),target=join(call.options.cwd,'missing-target');
   rmSync(path,{force:true});symlinkSync(target,path);
   assert.equal(existsSync(path),false,'following presence checks cannot detect this dangling link');
   return{status:0,stdout:phase==='execute'?'actual fake output\n':'',stderr:'',signal:null};
  })}),/symbolic links/u);
  assert.equal(h.calls.at(-1).phase,phase,'no subsequent compiler or execution can follow a dangling artifact');
  assert.equal(h.calls.length,['library','consumer','execute'].indexOf(phase)+1);
  assert.equal(existsSync(h.calls[0].options.cwd),false,'all private paths and dangling links are removed');
 }
});

test('failed processes still authenticate the entire bundle and cleanup before reporting failure',()=>{
 for(const phase of['library','consumer','execute']){
  const plan=makePlan(),h=harness(plan);failure(()=>h.run(plan,{environment,spawn:h.fake(call=>{
   if(call.phase===phase){writeFileSync(join(call.options.cwd,'library/part_1.rs'),'changed during failed process');return{status:1,stdout:'',stderr:'actual process failure'};}
  })}),/additional source bytes changed/u);assert.equal(h.calls.at(-1).phase,phase);assert.equal(existsSync(h.calls[0].options.cwd),false);
 }
});

test('selected compiler profiles retain exact commands, checked optimization and every original process bound',()=>{
 const profiles={original:[],shortDiagnostics:['--error-format=short'],checkedOptimized:['--error-format=short','-Copt-level=2','-Cdebug-assertions=on','-Coverflow-checks=on']};
 assert.deepEqual(readProjectData(new URL('./source-native-compiler-profiles.lino',import.meta.url)),profiles);
 for(const[profile,args]of Object.entries(profiles)){
  const plan=makePlan(),h=harness(plan,profile),receipt=h.run(plan,{environment,spawn:h.fake()});
  assert.deepEqual(h.calls.map(call=>call.phase),['library','consumer','execute']);
  const directory=h.calls[0].options.cwd;
  assert.deepEqual(receipt.observations[0].compilerArguments,[
   ['--edition=2024',...args,'--crate-type=rlib','--crate-name','oracle_source',join(directory,'original.rs'),'-o',join(directory,'liboracle_source.rlib')],
   ['--edition=2024',...args,join(directory,'consumer.rs'),'--extern','oracle_source='+join(directory,'liboracle_source.rlib'),'-o',join(directory,'consumer')],
  ]);
  for(const call of h.calls){assert.equal(call.options.maxBuffer,1048576);assert.equal(call.options.timeout,call.phase==='execute'?10000:30000);assert.equal(call.options.shell,false);}
  assert.deepEqual(h.calls[2].args,[]);assert.deepEqual(receipt.observations[0].observations,[{value:true}]);
 }
 assert.throws(()=>harness(makePlan(),'uncheckedFast'),/unknown source native compiler profile/u);
});

test('buffer errors retain actual captured stdout and stderr without emitting an observation or hiding later fixtures',()=>{
 const root=mkdtempSync(join(tmpdir(),'source-process-diagnostic-contract-'));
 try{
  const plan=makePlan();plan.entries.push({...makePlan().entries[0],id:'later mechanical fixture'});const h=harness(plan);
  const stdout='compiler output Ω\r\n',stderr='warning: actual compiler diagnostic 😀\n';let failed=false,caught;
  try{h.run(plan,{environment,diagnosticsDirectory:root,spawn:h.fake(call=>{
   if(!failed&&call.phase==='library'){failed=true;return{status:null,signal:'SIGTERM',stdout,stderr,error:Object.assign(new Error('spawnSync rustc ENOBUFS'),{code:'ENOBUFS'})};}
  })});}catch(error){caught=error;}
  assert.ok(caught instanceof AggregateError);assert.match(caught.message,/ENOBUFS/u);assert.ok(caught.message.length<8192);
  assert.deepEqual(h.calls.map(call=>call.phase),['library','library','consumer','execute']);
  const directories=readdirSync(root);assert.equal(directories.length,1);const directory=join(root,directories[0]);
  assert.equal(readFileSync(join(directory,'stdout.log'),'utf8'),stdout);assert.equal(readFileSync(join(directory,'stderr.log'),'utf8'),stderr);
  const receipt=readProjectData(join(directory,'receipt.lino'));
  assert.equal(receipt.processMode,'injected process contract');assert.equal(receipt.error.code,'ENOBUFS');assert.equal(receipt.maximumBufferBytes,1048576);
  assert.equal(receipt.stdout.sha256,sha256(stdout));assert.equal(receipt.stderr.sha256,sha256(stderr));assert.equal(receipt.stderr.truncated,false);
  assert.ok(h.calls.every(call=>!existsSync(call.options.cwd)),'compiler scratch is removed while its diagnostics survive');
 }finally{rmSync(root,{recursive:true,force:true});}
});

test('successful compiler warnings remain visible as bounded logs without changing returned observations',()=>{
 const root=mkdtempSync(join(tmpdir(),'source-process-warning-contract-'));
 try{
  const plan=makePlan(),h=harness(plan,'shortDiagnostics');
  const receipt=h.run(plan,{environment,diagnosticsDirectory:root,spawn:h.fake(call=>{
   if(call.phase==='library'){
    writeFileSync(call.args[call.args.indexOf('-o')+1],'fake compiler artifact, not Rust');
    return{status:0,signal:null,stdout:'',stderr:'original.rs:1:1: warning: exact original warning\n'};
   }
  })});
  assert.deepEqual(receipt.observations[0].observations,[{value:true}]);
  const directories=readdirSync(root);assert.equal(directories.length,1);const directory=join(root,directories[0]);
  assert.equal(readFileSync(join(directory,'stderr.log'),'utf8'),'original.rs:1:1: warning: exact original warning\n');
  const diagnostic=readProjectData(join(directory,'receipt.lino'));assert.equal(diagnostic.status,0);assert.equal(diagnostic.error,null);
  assert.ok(diagnostic.arguments.includes('--error-format=short'));assert.equal(diagnostic.stderr.truncated,false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test('timeout, signal, nonzero and over-limit captured bytes remain bounded process failures',()=>{
 for(const actual of[
  {status:null,signal:null,error:Object.assign(new Error('actual timeout'),{code:'ETIMEDOUT'}),stdout:'partial output',stderr:'actual timeout diagnostic'},
  {status:null,signal:'SIGTERM',stdout:'partial output',stderr:'actual signal diagnostic'},
  {status:2,signal:null,stdout:'partial output',stderr:'actual rejection diagnostic'},
  {status:null,signal:null,error:Object.assign(new Error('actual buffer failure'),{code:'ENOBUFS'}),stdout:'partial output',stderr:Buffer.alloc(1048592,65)},
 ]){
  const root=mkdtempSync(join(tmpdir(),'source-process-failure-contract-'));
  try{
   const plan=makePlan(),h=harness(plan);failure(()=>h.run(plan,{environment,diagnosticsDirectory:root,spawn:h.fake(call=>call.phase==='library'?actual:null)}),/timed out after 30000ms|SIGTERM|exit 2|buffer failure/u);
   assert.equal(h.calls.length,1);const directory=join(root,readdirSync(root)[0]),receipt=readProjectData(join(directory,'receipt.lino'));
   const captured=Buffer.from(actual.stderr),kept=readFileSync(join(directory,'stderr.log'));
   assert.deepEqual(kept,captured.subarray(0,1048576));assert.equal(receipt.stderr.capturedBytes,captured.length);
   assert.equal(receipt.stderr.retainedBytes,kept.length);assert.equal(receipt.stderr.truncated,captured.length>1048576);assert.equal(receipt.stderr.sha256,sha256(kept));
   assert.equal(h.calls[0].options.maxBuffer,1048576);assert.equal(h.calls[0].options.timeout,30000);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});
