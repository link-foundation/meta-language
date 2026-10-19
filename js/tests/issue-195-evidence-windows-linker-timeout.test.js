import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {restoreWindowsLinkerLookupHistory} from './support/windows-linker-lookup-history.js';

function readActualWindowsLinkerLookup(file,identifier) {
  const workflow=readFileSync(new URL('../../'+file,import.meta.url),'utf8');
  const job=parseWorkflow(workflow).find(job=>job.id===identifier);assert.ok(job);
  const steps=job.steps.filter(step=>step.name==='Verify Windows native linker resolution');assert.equal(steps.length,1);
  const source=/node --input-type=commonjs <<'NODE'\n([\s\S]*?)\n {10}NODE(?:\n|$)/u.exec(steps[0].text)?.[1];assert.ok(source);
  return source.replace(/^ {10}/gmu,'');
}

function executeWindowsLinkerLookup(source,results,selected) {
  const calls=[],output=[],diagnostics=[];let error;
  try {
    runInNewContext(source,{
      require:name=>{
        if(name==='node:path')return path;
        assert.equal(name,'node:child_process');
        return{spawnSync:(command,args,options)=>{
          calls.push({command,args:[...args],options:{...options}});
          assert.ok(calls.length<=results.length,'the actual script cannot invent additional process results');
          return results[calls.length-1];
        }};
      },
      process:{env:{CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER:selected}},
      console:{log:value=>output.push(value),error:value=>diagnostics.push(value)},
    },{timeout:1000});
  }catch(caught){error=caught;}
  return{calls,output,diagnostics,error};
}

test('actual Windows linker timeout repair preserves complete workflow scope and every first-resolution refusal',()=>{
  const history=readProjectData(new URL('../../parity/naming/windows-linker-timeout-source-history.lino',import.meta.url));
  for(const source of history.sources){
    assert.equal(Buffer.byteLength(source.sourceText),source.bytes);
    assert.equal(createHash('sha256').update(source.sourceText).digest('hex'),source.sha256);
  }
  assert.equal(history.capture.job.id,118178414273);
  assert.equal(history.capture.job.steps.find(step=>step.name==='Verify Windows native linker resolution').conclusion,'failure');
  assert.equal(history.capture.job.steps.find(step=>step.name==='Run tests').conclusion,'success');
  assert.equal(history.capture.log.sha256,'841acc502f1b12c0ba8925c7561b37404e938cf24bdf0336b767dd606a3749f3');
  assert.ok(history.capture.diagnosticLines.some(line=>line.includes('status=null, signal=SIGTERM, spawnSync where.exe ETIMEDOUT')));
  assert.ok(history.capture.diagnosticLines.some(line=>line.includes('41 passed; 0 failed')));
  for(const [file,identifier]of [['.github/workflows/rust.yml','test'],['.github/workflows/ci.yml','consumers']]){
    const current=readFileSync(new URL('../../'+file,import.meta.url),'utf8');
    const original=history.sources.find(row=>row.file===file);assert.ok(original);
    assert.equal(current,original.sourceText.replace(history.before,history.after));
    assert.equal(restoreWindowsLinkerLookupHistory(current),original.sourceText);
    assert.equal(readActualWindowsLinkerLookup(file,identifier),readActualWindowsLinkerLookup('.github/workflows/rust.yml','test'));
  }
  const source=readActualWindowsLinkerLookup('.github/workflows/rust.yml','test');
  const expected='C:\\Program Files\\Visual Studio\\VC\\bin\\Hostx64\\x64\\link.exe';
  const other='C:\\Program Files\\Git\\usr\\bin\\link.exe';
  const success={status:0,signal:null,stdout:expected+'\r\n'+other+'\r\n',stderr:''};
  const timeout={status:null,signal:'SIGTERM',stdout:'',stderr:'',error:Object.assign(new Error('spawnSync where.exe ETIMEDOUT'),{code:'ETIMEDOUT'})};
  for(let failed=0;failed<=2;failed++){
    const result=executeWindowsLinkerLookup(source,[...Array(failed).fill(timeout),success],expected);
    assert.equal(result.error,undefined);assert.equal(result.calls.length,failed+1);
    assert.deepEqual(result.output,['Validated Windows native child linker: '+expected]);
    assert.equal(result.diagnostics.length,failed);
    for(const [at,message]of result.diagnostics.entries())assert.equal(message,'Retrying timed-out Windows linker lookup ('+(at+1)+'/3): spawnSync where.exe ETIMEDOUT');
    for(const call of result.calls)assert.deepEqual(call,{command:'where.exe',args:['link.exe'],options:{encoding:'utf8',timeout:10000,maxBuffer:65536,windowsHide:true}});
  }
  const exhausted=executeWindowsLinkerLookup(source,[timeout,timeout,timeout],expected);
  assert.match(exhausted.error?.message??'',/Windows linker lookup failed/u);
  assert.equal(exhausted.calls.length,3);assert.equal(exhausted.diagnostics.length,2);assert.deepEqual(exhausted.output,[]);
  for(const failure of [
    {...success,stdout:other+'\r\n'+expected+'\r\n'},
    {...success,stdout:''},{...success,stdout:'link.exe\n'},
    {...success,status:1,stderr:'lookup failed'},{...success,signal:'SIGTERM'},
    {...success,error:new Error('lookup timeout')},{...success,stdout:null},
    {...timeout,error:Object.assign(new Error('permission denied'),{code:'EACCES'})},
    {...timeout,signal:'SIGKILL'},{...timeout,status:1},
  ])for(const preceding of [[],[timeout]]){
    const result=executeWindowsLinkerLookup(source,[...preceding,failure],expected);
    assert.match(result.error?.message??'',/Windows (?:linker lookup failed|native child resolves)/u);
    assert.equal(result.calls.length,preceding.length+1);assert.deepEqual(result.output,[]);
  }
  for(const selected of ['','relative/link.exe']){
    const result=executeWindowsLinkerLookup(source,[],selected);
    assert.match(result.error?.message??'',/validated absolute MSVC linker path/u);
    assert.deepEqual(result.calls,[]);assert.deepEqual(result.output,[]);assert.deepEqual(result.diagnostics,[]);
  }
});

test('Windows linker source inverse refuses unbounded or weakened retries and retains unrelated workflow changes',()=>{
  const history=readProjectData(new URL('../../parity/naming/windows-linker-timeout-source-history.lino',import.meta.url));
  const source=readFileSync(new URL('../../.github/workflows/rust.yml',import.meta.url),'utf8');
  for(const [before,after]of [['attempt <= 3','attempt <= 99'],["'ETIMEDOUT'","'EACCES'"],
    ["resolved.signal !== 'SIGTERM'",'false'],['resolved.status !== null','false'],['timeout: 10000','timeout: 0']]){
    assert.throws(()=>restoreWindowsLinkerLookupHistory(source.replace(history.after,history.after.replace(before,after))));
  }
  assert.throws(()=>restoreWindowsLinkerLookupHistory(source+'\n'+history.after));
  assert.throws(()=>restoreWindowsLinkerLookupHistory(source.replace(history.after,history.before)));
  const original=history.sources.find(row=>row.file==='.github/workflows/rust.yml').sourceText;
  assert.equal(restoreWindowsLinkerLookupHistory(source.replace('timeout-minutes: 15','timeout-minutes: 99')),
    original.replace('timeout-minutes: 15','timeout-minutes: 99'));
});
