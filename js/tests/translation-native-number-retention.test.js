import assert from 'node:assert/strict';
import {mkdtempSync,readFileSync,writeFileSync,readdirSync,rmSync,truncateSync} from 'node:fs';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {structuredDataToLino,readStructuredDataLino} from '../src/structured-data-links.js';
import {structuredDataRecordToLino} from '../src/structured-data-records.js';
import {executePhysicalHarness,readPhysicalShardGroups,readPhysicalShardPolicy,retainNativeNumberPublication,selectPhysicalTests} from '../scripts/rust-test-shards.mjs';
import {generateNativeNumberPublicationRunner} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-publication-runner.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';

const root=new URL('../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const testName='grammar_outcome_original_numbers::original_borrowed_number_predicates_payloads_and_wire_positions_match_every_independent_control';

// Synthetic disk-backed host transport controls execute no native process.
async function retentionControl() {
  const directory=mkdtempSync(path.join(tmpdir(),'meta-language-number-retention-')),name='c'.repeat(24);
  const identity={commit:'a'.repeat(40),tree:'b'.repeat(40),run:'123',attempt:'2',profile:'evidence',platform:'ubuntu-latest',suite:'all'};
  const policy=readPhysicalShardPolicy(),inventory=[{name:testName,kind:'test'}];
  const shard=Array.from({length:policy.evidence.shards},(_,index)=>index+1).find(index=>selectPhysicalTests(inventory,index,policy.evidence.shards,policy.evidence.sharedScopes).length);
  const fixture=readProjectData(new URL('parity/fixtures/original-native-number-observations.lino',root));
  const report={schemaVersion:1,profile:'original native number controls',testName,commit:identity.commit,run:identity.run,attempt:identity.attempt,targetUsizeBits:64,
    sources:Object.fromEntries(Object.entries({test:'rust/tests/unit/grammar_outcome_original_numbers.rs',model:'rust/tests/unit/native_json_number_observation_model.rs',
      visitors:'rust/tests/unit/native_json_number_observation_visitors.rs',publisher:'rust/tests/unit/native_json_number_publication.rs',
      fixture:'parity/fixtures/original-native-number-observations.lino',lock:'rust/Cargo.lock'}).map(([key,file])=>[key,pin(readFileSync(new URL(file,root)))])),observations:fixture.observations};
  const standardError='meta-language original native number controls: '+structuredDataRecordToLino(report),standardOutput='test '+testName+' ... ok\n';
  const receipt=await executePhysicalHarness({binary:'/synthetic/rust/target/debug/deps/unit-abcdef01',arguments:['--nocapture'],identity,shard,count:policy.evidence.shards,
    invoke:async(_binary,_arguments,listing)=>({stdout:listing?testName+': test\n':standardOutput,stderr:listing?'':standardError,exitCode:0,signal:null})});
  const prefix=path.join(directory,name);
  writeFileSync(prefix+'.lino',structuredDataToLino(receipt));writeFileSync(prefix+'.stderr.log',standardError);writeFileSync(prefix+'.stdout.log',standardOutput);
  writeFileSync(prefix+'.inventory.log',testName+': test\n');writeFileSync(prefix+'.all-inventory.log',testName+': test\n');
  writeFileSync(path.join(directory,'identity.lino'),structuredDataToLino({identity,shard,count:policy.evidence.shards,
    policyDigest:pin(readFileSync(new URL('parity/rust-test-physical-shards.lino',root))).sha256,host:'synthetic-host',expectedBinaries:['unit']}));
  const receipts=['identity.lino',name+'.lino'].sort().map(file=>({file,sha256:pin(readFileSync(path.join(directory,file))).sha256}));
  writeFileSync(path.join(directory,'seal.lino'),structuredDataToLino({identity,shard,count:policy.evidence.shards,result:'success',receipts}));
  return{directory,name,prefix,receipt,standardError,standardOutput,report};
}

test('automatic native retention preserves every original runner command selection result and seal byte',()=>{
  const history=generateNativeNumberPublicationRunner({check:true}),original=history.sources[0];
  verifyGitArtifactCertificate(original.certificate,Buffer.from(original.sourceText),{revision:original.commit,path:original.file});
  let current=readFileSync(new URL(original.file,root),'utf8');assert.deepEqual(pin(current),original.current);
  for(const action of [...original.actions].reverse()){assert.equal(current.split(action.to).length,2);current=current.replace(action.to,action.from);}
  assert.equal(current,original.sourceText);assert.deepEqual(pin(current),{bytes:original.bytes,sha256:original.sha256});
  const source=readFileSync(new URL(original.file,root),'utf8');
  assert.ok(source.includes('verifyHarnessExecution(receipt);\n  retainNativeNumberPublication(current.directory, name, receipt);\n  return receipt.exitCode;'));
  assert.ok(source.includes("readNativePublicationText(prefix + '.lino')"));
  const directory=mkdtempSync(path.join(tmpdir(),'meta-language-retention-cli-'));
  try{
    const result=spawnSync(process.execPath,[fileURLToPath(new URL(original.file,root)),'--verify',directory],
      {encoding:'utf8',timeout:10000,maxBuffer:65536,env:{...process.env,META_LANGUAGE_SHARD_EXPECTED_PROFILES:'evidence',
        GITHUB_SHA:'a'.repeat(40),GITHUB_RUN_ID:'123',GITHUB_RUN_ATTEMPT:'2'}});
    assert.equal(result.status,1);assert.equal(result.signal,null);assert.equal(result.error,undefined);
    assert.match(result.stderr,/missing or unregistered physical cells/u,'the real JavaScript CLI loads and retains its original missing-union refusal');
  }finally{rmSync(directory,{recursive:true,force:true});}
});

test('automatic retention emits checked Links after successful complete logs without changing physical receipt admission',async()=>{
  const control=await retentionControl();
  try{
    const original=readPhysicalShardGroups(control.directory),seal=readFileSync(path.join(control.directory,'seal.lino'));
    assert.equal(retainNativeNumberPublication(control.directory,control.name,control.receipt),true);
    const filename=control.prefix+'.original-number-publication.lino.log',report=readStructuredDataLino(readFileSync(filename,'utf8'));
    assert.deepEqual(report.observations,control.report.observations);assert.deepEqual(report.identity,control.receipt.identity);
    assert.deepEqual(report.execution.receipt,pin(readFileSync(control.prefix+'.lino')));
    assert.deepEqual(readPhysicalShardGroups(control.directory),original);assert.ok(readFileSync(path.join(control.directory,'seal.lino')).equals(seal));
    assert.throws(()=>retainNativeNumberPublication(control.directory,control.name,control.receipt),/EEXIST/u);
    assert.deepEqual(readStructuredDataLino(readFileSync(filename,'utf8')),report);
  }finally{rmSync(control.directory,{recursive:true,force:true});}
});

test('other native profiles and tests retain their original receipt inventory without requiring publication',async()=>{
  const control=await retentionControl();
  try{
    const files=readdirSync(control.directory).sort();
    for(const receipt of [{...control.receipt,identity:{...control.receipt.identity,profile:'test'}},{...control.receipt,selected:[]}])
      assert.equal(retainNativeNumberPublication(control.directory,control.name,receipt),false);
    assert.deepEqual(readdirSync(control.directory).sort(),files);
    assert.throws(()=>retainNativeNumberPublication(control.directory,'../foreign',control.receipt),/identity/u);
  }finally{rmSync(control.directory,{recursive:true,force:true});}
});

test('missing changed malformed oversized and detached retained native inputs refuse before artifact writes',async()=>{
  for(const mutate of [control=>{rmSync(control.prefix+'.stderr.log');},control=>{writeFileSync(control.prefix+'.stderr.log','changed');},
    control=>{writeFileSync(control.prefix+'.stderr.log',Buffer.from([0xc3]));},control=>{truncateSync(control.prefix+'.stderr.log',16*1024*1024+1);},
    control=>{writeFileSync(control.prefix+'.lino',structuredDataToLino({...control.receipt,detached:true}));},
    control=>{writeFileSync(control.prefix+'.stdout.log',control.standardOutput.replace('ok','FAILED'));}]){
    const control=await retentionControl();
    try{mutate(control);assert.throws(()=>retainNativeNumberPublication(control.directory,control.name,control.receipt));
      assert.ok(!readdirSync(control.directory).some(file=>file.endsWith('.original-number-publication.lino.log')));
    }finally{rmSync(control.directory,{recursive:true,force:true});}
  }
});
