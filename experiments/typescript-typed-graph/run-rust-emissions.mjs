// Dependency-free native process transport for the Rust package's own tests.
// The Rust unit executes the semantic assertions and checks these observations.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {readFileSync,writeFileSync,mkdirSync,mkdtempSync,rmSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readBootstrapStructuredDataLino,structuredDataToLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url));
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const readNativeControlData=file=>readBootstrapStructuredDataLino(readFileSync(file,'utf8'));
const writeNativeControlData=(file,value)=>writeFileSync(file,structuredDataToLino(value));

function processObservation(result) {
  return{exitCode:result.status,signal:result.signal,
    stdoutHex:(result.stdout??Buffer.alloc(0)).toString('hex'),
    stderrHex:(result.stderr??Buffer.alloc(0)).toString('hex'),
    error:result.error?{code:result.error.code??null,message:result.error.message}:null};
}

export function runRustRuntimeTypeScriptGraphCase(index,resultsDirectory) {
  if(process.env.CI!=='true'||process.env.GITHUB_ACTIONS!=='true'){
    throw new Error('native TypeScript graph execution is GitHub CI-only');
  }
  const manifest=readNativeControlData(new URL('../../parity/fixtures/typescript-typed-graph/manifest.lino',import.meta.url));
  assert.ok(Number.isSafeInteger(index)&&index>=0&&index<manifest.cases.length);
  const row=manifest.cases[index],results=resolve(root,resultsDirectory);
  assert.equal(results,resolve(root,'issue-195-results'));
  const artifacts=join(root,'issue-195-artifacts','typescript-typed-graph-native');
  const expected=Buffer.from(row.expectedLines.map(line=>line+'\n').join(''));
  const graph=readFileSync(resolve(root,row.path));assert.deepEqual(pin(graph),{bytes:row.bytes,sha256:row.sha256});
  mkdirSync(artifacts,{recursive:true});mkdirSync(join(results,'work'),{recursive:true});
  const work=mkdtempSync(join(results,'work','typescript-rust-runtime-'));
  const git=spawnSync('git',['rev-parse','HEAD'],{cwd:root,timeout:10_000,maxBuffer:1024*1024});
  const report={case:index,sourceGraph:row,compiler:processObservation(spawnSync('rustc',['-vV'],
    {cwd:root,timeout:10_000,maxBuffer:1024*1024})),nodeVersion:process.version,
    commit:git.status===0?git.stdout.toString('utf8').trim():null,git:processObservation(git),
    run:process.env.GITHUB_RUN_ID??null,attempt:process.env.GITHUB_RUN_ATTEMPT??null,
    platform:process.platform,observations:[],failures:[]};
  try{
    if(git.status!==0||!/^[0-9a-f]{40}$/u.test(report.commit??'')||report.compiler.exitCode!==0){
      report.failures.push({provenance:'native compiler and exact checkout identities are required'});
    }
    for(const target of ['Rust','JavaScript']){
      try{
        const path=join(artifacts,'rust-runtime-case-'+index+(target==='Rust'?'.rs':'.js'));
        const bytes=readFileSync(path),binary=join(work,process.platform==='win32'?'program.exe':'program');
        const compilation=target==='Rust'?spawnSync('rustc',['--edition=2024',path,'-o',binary],
          {cwd:root,timeout:60_000,maxBuffer:1024*1024}):null;
        const execution=target==='JavaScript'?spawnSync(process.execPath,[path],{cwd:root,timeout:1000,maxBuffer:1024*1024})
          :compilation.status===0?spawnSync(binary,[],{cwd:root,timeout:1000,maxBuffer:1024*1024}):null;
        assert.deepEqual(readFileSync(path),bytes,'the entire actual compiler/interpreter source remains unchanged');
        const observation={target,source:{path,...pin(bytes)},compilation:compilation?processObservation(compilation):null,
          execution:execution?processObservation(execution):null,expectedStdoutHex:expected.toString('hex')};
        report.observations.push(observation);
        if(compilation&&compilation.status!==0||!execution||execution.status!==0||!execution.stdout.equals(expected)){
          report.failures.push({target,observation});
        }
      }catch(failure){report.failures.push({target,error:{name:failure.name,message:failure.message}});}
    }
  }finally{
    writeNativeControlData(join(artifacts,'rust-runtime-case-'+index+'.execution.lino'),report);
    rmSync(work,{recursive:true});
  }
  assert.equal(report.failures.length,0,'native package TypeScript graph execution failed');
  return report;
}


if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.equal(process.argv[2],'--case');assert.match(process.argv[3],/^(?:0|[1-9][0-9]*)$/u);
  assert.deepEqual(process.argv.slice(4),['--results-dir','issue-195-results']);
  runRustRuntimeTypeScriptGraphCase(Number(process.argv[3]),process.argv[5]);
}
