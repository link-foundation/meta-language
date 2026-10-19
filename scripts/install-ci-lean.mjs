#!/usr/bin/env node
// CI fallback for an unavailable release.lean-lang.org index. The exact
// official GitHub release and its published SHA-256 remain pinned:
// https://github.com/leanprover/lean4/releases/tag/v4.34.1
import {spawnSync} from 'node:child_process';
import {mkdtempSync,rmSync} from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';

export const LEAN_RELEASE_VERSION='4.34.1';
export const LEAN_RELEASE_SHA256='47bf4bbd78f70c2e9670598ab7124d92b6efb7330ff33e5fbb4030f6fd72e4e4';

export function installFixedLean({env=process.env,platform=process.platform,architecture=process.arch,run=spawnSync}={}){
 if(env.CI!=='true'||env.GITHUB_ACTIONS!=='true'||platform!=='linux'||architecture!=='x64')throw new Error('The pinned Lean fallback runs only on Linux x64 GitHub Actions runners.');
 if(!env.RUNNER_TEMP||!env.ELAN_HOME)throw new Error('The pinned Lean fallback requires RUNNER_TEMP and ELAN_HOME.');
 const directory=mkdtempSync(path.join(env.RUNNER_TEMP,'meta-language-lean-')),archive=path.join(directory,'release.tar.zst');
 const execute=(command,args,options={})=>{
  const result=run(command,args,{env,encoding:'utf8',stdio:['pipe','pipe','pipe'],...options});
  if(result.error)throw result.error;
  if(result.status!==0)throw new Error(`${command} failed (${result.status}): ${result.stderr??''}`);
  return result.stdout??'';
 };
 let linked=false;
 try{
  execute('curl',['--proto','=https','--tlsv1.2','--retry','3','--retry-all-errors','--max-time','600','--location','--silent','--show-error','--fail',`https://github.com/leanprover/lean4/releases/download/v${LEAN_RELEASE_VERSION}/lean-${LEAN_RELEASE_VERSION}-linux.tar.zst`,'--output',archive]);
  execute('sha256sum',['--check','--strict'],{input:`${LEAN_RELEASE_SHA256}  ${archive}\n`});
  execute('tar',['--extract','--zstd','--file',archive,'--directory',directory]);
  const elan=path.join(env.ELAN_HOME,'bin','elan'),name='meta-language-lean-'+LEAN_RELEASE_VERSION;
  execute(elan,['toolchain','link',name,path.join(directory,`lean-${LEAN_RELEASE_VERSION}-linux`)]);
  // Verify the selected binary before making it the default. A corrupt or
  // different release must fail the job, rather than silently changing Lean.
  const version=execute(elan,['run',name,'lean','--version']);
  if(!/^Lean \(version 4\.34\.1(?:,|\))/u.test(version))throw new Error('The pinned Lean release returned an unexpected version: '+version);
  execute(elan,['default',name]);linked=true;
  return {version:LEAN_RELEASE_VERSION,sha256:LEAN_RELEASE_SHA256,directory};
 }finally{
  rmSync(archive,{force:true});
  if(!linked)rmSync(directory,{recursive:true,force:true});
 }
}

if(process.argv[1]&&path.resolve(process.argv[1])===fileURLToPath(import.meta.url)){
 try{console.log('Pinned Lean fallback: '+JSON.stringify(installFixedLean()));}
 catch(error){console.error(error.message);process.exitCode=1;}
}
