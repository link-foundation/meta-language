#!/usr/bin/env node
// Private source-receipt writer: reads actual historical Git blobs; never imports a planner or runs native code.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {serializeProjectData,writeProjectData} from '../js/scripts/project-data.mjs';
import {SOURCE_NATIVE_RUNNER_FILE,SOURCE_NATIVE_RUNNER_BASELINE,SOURCE_NATIVE_RUNNER_CORRECTIONS,SOURCE_NATIVE_RUNNER_ADDITIONS,SOURCE_NATIVE_RUNNER_VERIFIER_FILES,SOURCE_NATIVE_RUNNER_HISTORICAL_BINDINGS,
 readSourceNativeHistoricalBindings,verifySourceNativeRunnerSourceClosure} from './source-native-runner-source-closure.mjs';
const root=fileURLToPath(new URL('../',import.meta.url));
const sha256=value=>createHash('sha256').update(value).digest('hex');
const models=[
 {file:'experiments/original-link-complete-table-projection/raw-native-observations.mjs',table:'sourceDigests',constant:'ORIGINAL_LINK_RAW_MODEL_COMMIT',commit:'9123dd5d19fc26c95a9d31eff47dd9655c33da53',receipt:'experiments/original-link-complete-table-projection/data/raw-runner-source-closure.lino'},
 {file:'experiments/original-link-complete-table-projection/native-observations.mjs',table:'digests',constant:'ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT',commit:'9a835e469572341e6a8e5c6e9fe6f40b8655dfbe',receipt:'experiments/original-link-complete-table-projection/data/runner-source-closure.lino'},
];

/** Preflight both real baselines and every current dependency before the first write. */
export function writeSourceNativeRunnerClosures({check=false,read=readFileSync,git=execFileSync,writeData=writeProjectData,writeSource=writeFileSync}={}){
 const currentBytes=read(resolve(root,SOURCE_NATIVE_RUNNER_FILE));
 const additionalSources=Object.fromEntries(SOURCE_NATIVE_RUNNER_ADDITIONS.map(row=>[row.file,read(resolve(root,row.file))]));
 const verifierSources=Object.fromEntries(SOURCE_NATIVE_RUNNER_VERIFIER_FILES.map(file=>[file,read(resolve(root,file))]));
 const outputs=models.map(model=>{
  const source=read(resolve(root,model.file),'utf8');
  const commit=new RegExp('^export const '+model.constant+"\\s*=\\s*'([a-f0-9]{40})';",'mu').exec(source)?.[1];
  assert.equal(commit,model.commit,'each source receipt retains its actual original producer');
  const historicalSources=readSourceNativeHistoricalBindings(source,model.table);
  const historicalBinding=SOURCE_NATIVE_RUNNER_HISTORICAL_BINDINGS.find(row=>row.producerCommit===commit);
  assert.ok(historicalBinding);assert.equal(historicalBinding.file,model.file);assert.equal(historicalBinding.table,model.table);
  const historicalBytes=git('git',['show',historicalBinding.commit+':'+historicalBinding.file],{cwd:root,maxBuffer:1024*1024});
  assert.ok(Buffer.isBuffer(historicalBytes));assert.equal(historicalBytes.length,historicalBinding.bytes);
  assert.equal(sha256(historicalBytes),historicalBinding.sha256,'the actual historical consumer Git blob is exact');
  assert.deepEqual(readSourceNativeHistoricalBindings(historicalBytes.toString('utf8'),model.table),historicalSources,'every original digest and its order remain unchanged');
  const baselineBytes=git('git',['show',commit+':'+SOURCE_NATIVE_RUNNER_FILE],{cwd:root,maxBuffer:1024*1024});
  assert.ok(Buffer.isBuffer(baselineBytes));assert.equal(sha256(baselineBytes),SOURCE_NATIVE_RUNNER_BASELINE,'the actual historical Git blob is exact');
  const receipt={version:1,producerCommit:commit,historicalSources,historicalBinding,
   baseline:{commit,file:SOURCE_NATIVE_RUNNER_FILE,source:baselineBytes.toString('utf8'),bytes:baselineBytes.length,sha256:SOURCE_NATIVE_RUNNER_BASELINE},
   current:{file:SOURCE_NATIVE_RUNNER_FILE,bytes:currentBytes.length,sha256:sha256(currentBytes)},
   corrections:SOURCE_NATIVE_RUNNER_CORRECTIONS,additionalSources:SOURCE_NATIVE_RUNNER_ADDITIONS,
   verifierSources:Object.entries(verifierSources).map(([file,bytes])=>({file,bytes:bytes.length,sha256:sha256(bytes)})),
   scope:'source closure only; no native execution or performance claim'};
  verifySourceNativeRunnerSourceClosure(receipt,currentBytes,historicalSources,{producerCommit:commit,additionalSources,verifierSources});
  const bytes=Buffer.from(serializeProjectData(receipt,resolve(root,model.receipt)),'utf8');
  const pins=[...source.matchAll(/^const runnerClosureReceiptSha256='[a-f0-9]{64}';$/gmu)];assert.equal(pins.length,1,'each verifier owns one explicit receipt pin');
  const updated=source.replace(pins[0][0],"const runnerClosureReceiptSha256='"+sha256(bytes)+"';");
  assert.deepEqual(readSourceNativeHistoricalBindings(updated,model.table),historicalSources,'writing an additive receipt cannot rewrite any old binding');
  return{...model,value:receipt,bytes,source:updated,sha256:sha256(bytes)};
 });
 if(check){
  for(const output of outputs){assert.deepEqual(read(resolve(root,output.receipt)),output.bytes,'runner source closure receipt is stale');assert.equal(read(resolve(root,output.file),'utf8'),output.source,'runner source closure receipt pin is stale');}
 }else{
  for(const output of outputs){writeData(resolve(root,output.receipt),output.value);writeSource(resolve(root,output.file),output.source);}
 }
 return outputs.map(({receipt,sha256})=>({file:receipt,sha256}));
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
 assert.ok(process.argv.slice(2).every(argument=>argument==='--check'),'only --check is supported');
 for(const row of writeSourceNativeRunnerClosures({check:process.argv.includes('--check')}))process.stdout.write(row.file+' '+row.sha256+'\n');
}
