import assert from 'node:assert/strict';
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import path from 'node:path';
import test from 'node:test';
import {installFixedLean,LEAN_RELEASE_SHA256} from '../../scripts/install-ci-lean.mjs';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';

function createLeanDownloadFixture(t){
 const root=mkdtempSync(path.join(tmpdir(),'meta-language-lean-test-'));t.after(()=>rmSync(root,{recursive:true,force:true}));
 return {root,env:{CI:'true',GITHUB_ACTIONS:'true',RUNNER_TEMP:root,ELAN_HOME:path.join(root,'elan')},platform:'linux',architecture:'x64'};
}

test('the Lean fallback refuses local and unsupported runners before creating or downloading files',t=>{
 const fixture=createLeanDownloadFixture(t),run=()=>assert.fail('No download or toolchain command may run');
 for(const overrides of [{env:{...fixture.env,CI:'false'}},{env:{...fixture.env,GITHUB_ACTIONS:'false'}},{platform:'darwin'},{architecture:'arm64'},{env:{CI:'true',GITHUB_ACTIONS:'true'}}]){
  assert.throws(()=>installFixedLean({...fixture,run,...overrides}),/only on Linux x64|requires RUNNER_TEMP/u);
  assert.deepEqual(readdirSync(fixture.root),[]);
 }
});

test('the Lean fallback authenticates the exact release before extraction linking and version selection',t=>{
 const fixture=createLeanDownloadFixture(t),calls=[];
 const result=installFixedLean({...fixture,run:(command,args,options)=>{
  calls.push({command,args,options});return {status:0,stdout:args.includes('--version')?'Lean (version 4.34.1, x86_64-unknown-linux-gnu, Release)\n':'',stderr:''};
 }});
 assert.deepEqual(calls.map(value=>path.basename(value.command)),['curl','sha256sum','tar','elan','elan','elan']);
 assert.ok(calls[0].args.includes('https://github.com/leanprover/lean4/releases/download/v4.34.1/lean-4.34.1-linux.tar.zst'));
 assert.equal(LEAN_RELEASE_SHA256,'47bf4bbd78f70c2e9670598ab7124d92b6efb7330ff33e5fbb4030f6fd72e4e4');
 assert.equal(calls[1].options.input,LEAN_RELEASE_SHA256+'  '+path.join(result.directory,'release.tar.zst')+'\n');
 assert.deepEqual(calls[3].args,['toolchain','link','meta-language-lean-4.34.1',path.join(result.directory,'lean-4.34.1-linux')]);
 assert.deepEqual(calls[4].args,['run','meta-language-lean-4.34.1','lean','--version']);
 assert.deepEqual(calls[5].args,['default','meta-language-lean-4.34.1']);assert.ok(existsSync(result.directory));
 assert.equal(result.version,'4.34.1');assert.equal(result.sha256,LEAN_RELEASE_SHA256);
});

test('download checksum and version failures stop Lean selection and clear only the owned temporary directory',t=>{
 for(const failure of ['curl','sha256sum','tar','version']){
  const fixture=createLeanDownloadFixture(t),calls=[];
  assert.throws(()=>installFixedLean({...fixture,run:(command,args)=>{
   calls.push({command,args});const failed=path.basename(command)===failure;
   return {status:failed?1:0,stderr:failed?'fixture failure':'',stdout:args.includes('--version')?'Lean (version 4.33.0, fixture)\n':''};
  }}),/failed|unexpected version/u);
  assert.ok(!calls.some(value=>value.args[0]==='default'));assert.deepEqual(readdirSync(fixture.root),[]);
  if(failure==='sha256sum')assert.ok(!calls.some(value=>path.basename(value.command)==='tar'));
 }
});

test('every Lean setup preserves the same version and forwards its selected home to later evidence steps',()=>{
 const workflow=readFileSync(new URL('../../.github/workflows/ci.yml',import.meta.url),'utf8');
 const setups=parseWorkflow(workflow).flatMap(job=>job.steps.filter(step=>step.name==='Setup Lean'));
 assert.equal(setups.length,4);
 for(const step of setups){
  assert.match(step.run,/elan-init.*--default-toolchain leanprover\/lean4:v4\.34\.1/u);
  assert.match(step.run,/echo "ELAN_HOME=\$\{ELAN_HOME\}" >> "\$GITHUB_ENV"/u);
  assert.match(step.run,/if ! "\$\{ELAN_HOME\}\/bin\/lean" --version; then\n\s+node scripts\/install-ci-lean\.mjs\n\s+fi/u);
  assert.match(step.run,/"\$\{ELAN_HOME\}\/bin\/lean" --version\s*$/u);
  assert.ok(!step.text.includes('continue-on-error: true'));
 }
});
