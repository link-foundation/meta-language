// Shared process mechanics only; each experiment supplies its checked-plan and CI authorization.
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
const sha256=value=>createHash('sha256').update(value).digest('hex');
/** Configure the existing whole-library byte-receipt/process protocol, with no process at import. */
export function createSourceNativeObservationRunner({nativeMode,checkedPlan,textBytes,parseObservations,entryReceipt,temporaryPrefix,failureLabel}){
function checkFiles(entry,files){
 assert.deepEqual(readFileSync(files.source),textBytes(entry.source),entry.id+' original module bytes changed');
 assert.equal(sha256(readFileSync(files.source)),entry.sourceSha256,entry.id+' source receipt changed');
 assert.equal(sha256(readFileSync(files.consumer)),entry.consumerSha256,entry.id+' generated external consumer changed');
}
function runProcess(spawn,command,args,options,label){
 const result=spawn(command,args,{...options,shell:false,encoding:'utf8',maxBuffer:1024*1024});
 if(result.error)throw new Error(label+': '+(result.error.code==='ETIMEDOUT'?'timed out after '+options.timeout+'ms':result.error.message),{cause:result.error});
 if(result.signal)throw new Error(label+': terminated by '+result.signal);
 if(result.status!==0)throw new Error(label+': exit '+result.status+'; '+String(result.stderr??'').slice(0,4096));
 if(typeof result.stdout!=='string')throw new TypeError(label+': stdout must be actual UTF-8 text');
 return result.stdout;
}
/** Authorization precedes every process, including injected contract processes. */
return function runSourceNativeObservations(plan,{environment=process.env,spawn=spawnSync,compiler='rustc',temporaryRoot=tmpdir(),compilerTimeout=30000,executionTimeout=10000}={}){
 if(!nativeMode(environment))throw new TypeError('native observations require the explicit CI-only switch');
 if(!checkedPlan(plan))throw new TypeError('native execution requires the exact checked in-memory observation plan');
 if(typeof spawn!=='function'||typeof compiler!=='string'||!compiler)throw new TypeError('the native runner requires an actual process function and compiler path');
 const observations=[],errors=[],processCounts={libraryCompilations:0,consumerCompilations:0,executions:0};
 for(const entry of plan.entries){
  let directory;
  try{
   assert.equal(textBytes(entry.source).length,entry.sourceBytes);assert.equal(sha256(textBytes(entry.source)),entry.sourceSha256);assert.equal(sha256(entry.consumerSource),entry.consumerSha256);
   directory=mkdtempSync(join(temporaryRoot,temporaryPrefix));
   const files={source:join(directory,'original.rs'),consumer:join(directory,'consumer.rs'),library:join(directory,'liboracle_source.rlib'),executable:join(directory,'consumer')};
   writeFileSync(files.source,textBytes(entry.source));writeFileSync(files.consumer,entry.consumerSource);checkFiles(entry,files);
   const options={cwd:directory,env:environment,timeout:compilerTimeout};
   const libraryArgs=['--edition=2024','--crate-type=rlib','--crate-name','oracle_source',files.source,'-o',files.library];
   processCounts.libraryCompilations++;runProcess(spawn,compiler,libraryArgs,options,entry.id+' original library compiler');checkFiles(entry,files);assert.ok(existsSync(files.library),'the compiler did not create the original library');
   const consumerArgs=['--edition=2024',files.consumer,'--extern','oracle_source='+files.library,'-o',files.executable];
   processCounts.consumerCompilations++;runProcess(spawn,compiler,consumerArgs,options,entry.id+' external consumer compiler');checkFiles(entry,files);assert.ok(existsSync(files.executable),'the compiler did not create the external consumer');
   processCounts.executions++;const stdout=runProcess(spawn,files.executable,[],{...options,timeout:executionTimeout},entry.id+' external original function execution');checkFiles(entry,files);
   observations.push({id:entry.id,kind:entry.kind,fixtureId:entry.fixtureId,sourceBytes:entry.sourceBytes,sourceSha256:entry.sourceSha256,replaySha256:entry.replaySha256,
    originalReceipt:entry.originalReceipt,...entryReceipt(entry),consumerSha256:entry.consumerSha256,javaScriptSha256:entry.javaScriptSha256,
    compiler,compilerArguments:[libraryArgs,consumerArgs],observations:parseObservations(stdout,entry.observations)});
  }catch(error){errors.push(new Error(entry.id+': '+error.message,{cause:error}));}
  finally{if(directory)rmSync(directory,{recursive:true,force:true});}
 }
 if(errors.length)throw new AggregateError(errors,failureLabel+':\n'+errors.map(error=>error.message).join('\n'));
 return{model:plan.model,modelCommit:plan.modelCommit,modelSources:plan.modelSources,repositoryCommit:environment.GITHUB_SHA??'',originalCount:plan.originalCount,processMode:spawn===spawnSync?'native subprocesses':'injected process contract',processCounts,observations};
}

}
