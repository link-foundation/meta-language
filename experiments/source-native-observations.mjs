// Shared process mechanics only; each experiment supplies its checked-plan and CI authorization.
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {dirname,join} from 'node:path';
import {readSourceNativeCompilerArguments,retainSourceNativeProcessDiagnostics} from './source-native-process-diagnostics.mjs';
const sha256=value=>createHash('sha256').update(value).digest('hex');
/** Configure the existing whole-library byte-receipt/process protocol, with no process at import. */
export function createSourceNativeObservationRunner({nativeMode,checkedPlan,textBytes,parseObservations,entryReceipt,temporaryPrefix,failureLabel,compilerProfile='original'}){
const compilerArguments=readSourceNativeCompilerArguments(compilerProfile);
function prepareBundle(entry){
 const descriptor=Object.getOwnPropertyDescriptor(entry,'additionalSources');if(!descriptor)return null;
 assert.ok(Object.hasOwn(descriptor,'value')&&descriptor.enumerable,'additionalSources must be an owned data field');
 const sources=descriptor.value;assert.ok(Array.isArray(sources)&&Object.getPrototypeOf(sources)===Array.prototype,'additionalSources must be an ordinary dense array');
 const keys=Reflect.ownKeys(sources);assert.equal(keys.length,sources.length+1,'additionalSources cannot have extra fields or holes');
 const paths=['original.rs','consumer.rs','liboracle_source.rlib','consumer','consumer.exe'];
 const bundle=[];
 for(let index=0;index<sources.length;index++){
  const item=Object.getOwnPropertyDescriptor(sources,String(index));assert.ok(item&&Object.hasOwn(item,'value')&&item.enumerable,'additionalSources cannot have accessors or holes');
  const source=item.value;assert.ok(source&&Object.getPrototypeOf(source)===Object.prototype,'additional source must be a plain data record');
  assert.deepEqual(Reflect.ownKeys(source).sort(),['relativePath','source','sourceBytes','sourceSha256'],'additional source fields must be exact');
  const fields=Object.getOwnPropertyDescriptors(source);for(const field of Object.values(fields))assert.ok(Object.hasOwn(field,'value')&&field.enumerable,'additional source cannot contain accessors');
  const {relativePath,source:content,sourceBytes,sourceSha256}=source;
  assert.ok(typeof relativePath==='string'&&relativePath.split('/').every(part=>/^[A-Za-z0-9_-]+(?:\.[A-Za-z0-9_-]+)*$/u.test(part)&&!/^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/iu.test(part)),'additional source requires a safe relative path');
  const canonical=relativePath.toLowerCase();assert.ok(paths.every(path=>path!==canonical&&!path.startsWith(canonical+'/')&&!canonical.startsWith(path+'/')),'additional source path collides with another source or artifact');paths.push(canonical);
  const bytes=textBytes(content);assert.equal(bytes.length,sourceBytes,'additional source byte count differs');assert.equal(sha256(bytes),sourceSha256,'additional source hash differs');
  bundle.push({relativePath,sourceBytes,sourceSha256,bytes});
 }
 // Preflight every source before the first directory/source write or process.
 assert.equal(textBytes(entry.source).length,entry.sourceBytes);assert.equal(sha256(textBytes(entry.source)),entry.sourceSha256);assert.equal(sha256(textBytes(entry.consumerSource)),entry.consumerSha256);
 return bundle;
}
function regularPath(directory,relativePath){
 assert.ok(lstatSync(directory).isDirectory()&&!lstatSync(directory).isSymbolicLink(),'private source root must be an actual directory');
 const components=relativePath.split('/');let current=directory;
 for(const[index,component]of components.entries()){
  current=join(current,component);const status=lstatSync(current);assert.ok(!status.isSymbolicLink(),'source bundle paths cannot be symbolic links');
  assert.ok(index===components.length-1?status.isFile():status.isDirectory(),'source bundle path has the wrong file/directory role');
 }
 return current;
}
function checkFiles(entry,files){
 if(files.additionalSources){
  regularPath(files.directory,'original.rs');regularPath(files.directory,'consumer.rs');
  for(const name of['liboracle_source.rlib','consumer']){
   try{lstatSync(join(files.directory,name));}catch(error){if(error.code==='ENOENT')continue;throw error;}
   regularPath(files.directory,name);
  }
  for(const source of files.additionalSources){const path=regularPath(files.directory,source.relativePath);assert.deepEqual(readFileSync(path),source.bytes,entry.id+' additional source bytes changed: '+source.relativePath);assert.equal(sha256(readFileSync(path)),source.sourceSha256,entry.id+' additional source hash changed');}
 }

 assert.deepEqual(readFileSync(files.source),textBytes(entry.source),entry.id+' original module bytes changed');
 assert.equal(sha256(readFileSync(files.source)),entry.sourceSha256,entry.id+' source receipt changed');
 assert.equal(sha256(readFileSync(files.consumer)),entry.consumerSha256,entry.id+' generated external consumer changed');
}
function runProcess(spawn,command,args,options,label){
 const result=spawn(command,args,{...options,shell:false,encoding:'utf8',maxBuffer:1024*1024});
 const reason=result.error?(result.error.code==='ETIMEDOUT'?'timed out after '+options.timeout+'ms':String(result.error.message??result.error.code??'process failed').slice(0,4096))
  :result.signal?'terminated by '+result.signal:result.status!==0?'exit '+result.status+'; '+String(result.stderr??'').slice(0,4096):null;
 if(reason!==null||(typeof result.stderr==='string'||Buffer.isBuffer(result.stderr))&&result.stderr.length>0){
  let diagnostics;
  try{diagnostics=retainSourceNativeProcessDiagnostics(result,{command,args,options,label,processMode:spawn===spawnSync?'native subprocesses':'injected process contract'});}
  catch(error){throw new Error(label+': '+reason+'; retaining process diagnostics failed: '+String(error.message).slice(0,1024),{cause:result.error??error});}
  if(reason!==null)throw new Error(label+': '+reason+'; process diagnostics: '+diagnostics.receiptPath,{cause:result.error});
  process.stderr.write(label+': compiler diagnostics retained: '+diagnostics.receiptPath+'\n');
 }
 if(typeof result.stdout!=='string')throw new TypeError(label+': stdout must be actual UTF-8 text');
 return result.stdout;
}
function runCheckedProcess(entry,files,spawn,command,args,options,label){
 if(!files.additionalSources)return runProcess(spawn,command,args,options,label);
 checkFiles(entry,files);try{return runProcess(spawn,command,args,options,label);}finally{checkFiles(entry,files);}
}
/** Authorization precedes every process, including injected contract processes. */
return function runSourceNativeObservations(plan,{environment=process.env,spawn=spawnSync,compiler='rustc',temporaryRoot=tmpdir(),compilerTimeout=30000,executionTimeout=10000,diagnosticsDirectory}={}){
 if(!nativeMode(environment))throw new TypeError('native observations require the explicit CI-only switch');
 if(!checkedPlan(plan))throw new TypeError('native execution requires the exact checked in-memory observation plan');
 if(typeof spawn!=='function'||typeof compiler!=='string'||!compiler)throw new TypeError('the native runner requires an actual process function and compiler path');
 const bundles=new Map(),invalid=[];
 for(const entry of plan.entries){try{const bundle=prepareBundle(entry);if(bundle!==null)bundles.set(entry,bundle);}catch(error){invalid.push(new Error(entry.id+': '+error.message,{cause:error}));}}
 if(invalid.length)throw new AggregateError(invalid,failureLabel+':\n'+invalid.map(error=>error.message).join('\n'));
 const observations=[],errors=[],processCounts={libraryCompilations:0,consumerCompilations:0,executions:0};
 for(const entry of plan.entries){
  let directory;
  try{
   assert.equal(textBytes(entry.source).length,entry.sourceBytes);assert.equal(sha256(textBytes(entry.source)),entry.sourceSha256);assert.equal(sha256(entry.consumerSource),entry.consumerSha256);
   directory=mkdtempSync(join(temporaryRoot,temporaryPrefix));
   const files={source:join(directory,'original.rs'),consumer:join(directory,'consumer.rs'),library:join(directory,'liboracle_source.rlib'),executable:join(directory,'consumer')};
   if(bundles.has(entry)){
    files.directory=directory;files.additionalSources=bundles.get(entry);
    assert.ok(lstatSync(directory).isDirectory()&&!lstatSync(directory).isSymbolicLink(),'private source root must be an actual directory');
    for(const source of files.additionalSources){const path=join(directory,source.relativePath);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,source.bytes,{flag:'wx'});regularPath(directory,source.relativePath);}
   }
   writeFileSync(files.source,textBytes(entry.source));writeFileSync(files.consumer,entry.consumerSource);checkFiles(entry,files);
   const options={cwd:directory,env:environment,timeout:compilerTimeout,...(diagnosticsDirectory===undefined?{}:{diagnosticsDirectory})};
   const libraryArgs=['--edition=2024',...compilerArguments,'--crate-type=rlib','--crate-name','oracle_source',files.source,'-o',files.library];
   processCounts.libraryCompilations++;runCheckedProcess(entry,files,spawn,compiler,libraryArgs,options,entry.id+' original library compiler');checkFiles(entry,files);assert.ok(existsSync(files.library),'the compiler did not create the original library');
   const consumerArgs=['--edition=2024',...compilerArguments,files.consumer,'--extern','oracle_source='+files.library,'-o',files.executable];
   processCounts.consumerCompilations++;runCheckedProcess(entry,files,spawn,compiler,consumerArgs,options,entry.id+' external consumer compiler');checkFiles(entry,files);assert.ok(existsSync(files.executable),'the compiler did not create the external consumer');
   processCounts.executions++;const stdout=runCheckedProcess(entry,files,spawn,files.executable,[],{...options,timeout:executionTimeout},entry.id+' external original function execution');checkFiles(entry,files);
   observations.push({id:entry.id,kind:entry.kind,fixtureId:entry.fixtureId,sourceBytes:entry.sourceBytes,sourceSha256:entry.sourceSha256,replaySha256:entry.replaySha256,
    originalReceipt:entry.originalReceipt,...entryReceipt(entry),...(bundles.has(entry)?{additionalSources:bundles.get(entry).map(({relativePath,sourceBytes,sourceSha256})=>({relativePath,sourceBytes,sourceSha256}))}:{}),consumerSha256:entry.consumerSha256,javaScriptSha256:entry.javaScriptSha256,
    compiler,compilerArguments:[libraryArgs,consumerArgs],observations:parseObservations(stdout,entry.observations)});
  }catch(error){errors.push(new Error(entry.id+': '+error.message,{cause:error}));}
  finally{if(directory)rmSync(directory,{recursive:true,force:true});}
 }
 if(errors.length)throw new AggregateError(errors,failureLabel+':\n'+errors.map(error=>error.message).join('\n'));
 return{model:plan.model,modelCommit:plan.modelCommit,modelSources:plan.modelSources,repositoryCommit:environment.GITHUB_SHA??'',originalCount:plan.originalCount,processMode:spawn===spawnSync?'native subprocesses':'injected process contract',processCounts,observations};
}

}
