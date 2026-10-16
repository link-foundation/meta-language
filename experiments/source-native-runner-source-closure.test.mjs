// Source-only mechanical conservation. No compiler, native process or graph planner.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {SOURCE_NATIVE_RUNNER_FILE,SOURCE_NATIVE_RUNNER_BASELINE,SOURCE_NATIVE_RUNNER_CORRECTIONS,SOURCE_NATIVE_RUNNER_ADDITIONS,SOURCE_NATIVE_RUNNER_VERIFIER_FILES,SOURCE_NATIVE_RUNNER_HISTORICAL_BINDINGS,
 readSourceNativeHistoricalBindings,replaySourceNativeRunnerCorrections,verifySourceNativeRunnerSourceClosure} from './source-native-runner-source-closure.mjs';
import {readOriginalLinkRawHistoricalSources} from './original-link-complete-table-projection/raw-schema-source-closure.mjs';
const sha256=value=>createHash('sha256').update(value).digest('hex');
const root=new URL('../',import.meta.url);

function fixture(complete=false){
 const currentBytes=readFileSync(new URL(SOURCE_NATIVE_RUNNER_FILE,root));
 let baseline=currentBytes.toString('utf8');
 for(const correction of [...SOURCE_NATIVE_RUNNER_CORRECTIONS].reverse()){
  const at=baseline.indexOf(correction.after);assert.ok(at>=0&&at===baseline.lastIndexOf(correction.after));
  baseline=baseline.slice(0,at)+correction.before+baseline.slice(at+correction.after.length);
 }
 assert.equal(sha256(baseline),SOURCE_NATIVE_RUNNER_BASELINE,'inverse reconstruction is the real historical source, not an invented baseline');
 const source=readFileSync(new URL('./original-link-complete-table-projection/'+(complete?'native-observations.mjs':'raw-native-observations.mjs'),import.meta.url),'utf8');
 const historicalSources=readSourceNativeHistoricalBindings(source,complete?'digests':'sourceDigests');
 if(!complete)assert.deepEqual(historicalSources,readOriginalLinkRawHistoricalSources(source));
 const producerCommit=complete?'9a835e469572341e6a8e5c6e9fe6f40b8655dfbe':'9123dd5d19fc26c95a9d31eff47dd9655c33da53';
 const additionalSources=Object.fromEntries(SOURCE_NATIVE_RUNNER_ADDITIONS.map(row=>[row.file,readFileSync(new URL(row.file,root))]));
 const verifierSources=Object.fromEntries(SOURCE_NATIVE_RUNNER_VERIFIER_FILES.map(file=>[file,readFileSync(new URL(file,root))]));
 const receipt={version:1,producerCommit,historicalSources:{...historicalSources},historicalBinding:{...SOURCE_NATIVE_RUNNER_HISTORICAL_BINDINGS.find(row=>row.producerCommit===producerCommit)},
  baseline:{commit:producerCommit,file:SOURCE_NATIVE_RUNNER_FILE,source:baseline,bytes:Buffer.byteLength(baseline),sha256:SOURCE_NATIVE_RUNNER_BASELINE},
  current:{file:SOURCE_NATIVE_RUNNER_FILE,bytes:currentBytes.length,sha256:sha256(currentBytes)},
  corrections:structuredClone(SOURCE_NATIVE_RUNNER_CORRECTIONS),additionalSources:structuredClone(SOURCE_NATIVE_RUNNER_ADDITIONS),
  verifierSources:Object.entries(verifierSources).map(([file,bytes])=>({file,bytes:bytes.length,sha256:sha256(bytes)})),
  scope:'source closure only; no native execution or performance claim'};
 return{receipt,currentBytes,historicalSources,options:{producerCommit,additionalSources,verifierSources}};
}
const verify=value=>verifySourceNativeRunnerSourceClosure(value.receipt,value.currentBytes,value.historicalSources,value.options);

test('runner source closure conserves both complete historical producer tables and only the four reviewed source contexts',()=>{
 for(const complete of[false,true]){
  const value=fixture(complete),proof=verify(value);
  assert.equal(proof.baselineSha256,SOURCE_NATIVE_RUNNER_BASELINE);assert.equal(proof.sha256,sha256(value.currentBytes));
  assert.equal(proof.producerCommit,value.options.producerCommit);assert.equal(proof.corrections.length,4);
  assert.equal(value.receipt.baseline.bytes,10188);assert.equal(Object.keys(value.historicalSources).includes('../source-native-observations.mjs'),true);
  if(!complete)assert.equal(Object.keys(value.historicalSources).length,47);
  assert.equal(replaySourceNativeRunnerCorrections(value.receipt.baseline.source),value.currentBytes.toString('utf8'));
 }
});

test('coherent new runner hashes cannot excuse changed limits, authorization, source checks or success conditions',()=>{
 for(const[before,after]of[
  ['executionTimeout=10000','executionTimeout=10001'],['compilerTimeout=30000','compilerTimeout=30001'],
  ['maxBuffer:1024*1024','maxBuffer:2*1024*1024'],['if(!nativeMode(environment))','if(false)'],
  ['if(!checkedPlan(plan))','if(false)'],['checkFiles(entry,files);assert.ok(existsSync(files.library)','assert.ok(existsSync(files.library)'],
  ["if(typeof result.stdout!=='string')","if(false)"],['observations:parseObservations(stdout,entry.observations)','observations:entry.observations'],
 ]){
  const value=fixture();let source=value.currentBytes.toString('utf8');assert.ok(source.includes(before));source=source.replace(before,after);
  value.currentBytes=Buffer.from(source);value.receipt.current.bytes=value.currentBytes.length;value.receipt.current.sha256=sha256(value.currentBytes);
  assert.throws(()=>verify(value),assert.AssertionError);
 }
});

test('runner closure receipts cannot relabel old bindings, omit corrections or forge source and dependency pins',()=>{
 for(const mutate of[
  value=>{value.receipt.version=2;},value=>{value.receipt.producerCommit='0'.repeat(40);},
  value=>{value.receipt.baseline.source+='\n';},value=>{value.receipt.baseline.bytes--;},value=>{value.receipt.baseline.sha256='0'.repeat(64);},
  value=>{value.receipt.historicalSources['policy.mjs']='0'.repeat(64);},value=>{delete value.receipt.historicalSources['policy.mjs'];},
  value=>{value.receipt.historicalSources=Object.fromEntries(Object.entries(value.receipt.historicalSources).reverse());},
  value=>{value.receipt.corrections.pop();},value=>{value.receipt.corrections.reverse();},value=>{value.receipt.corrections[0].after+='\n';},
  value=>{value.receipt.current.file='another-runner.mjs';},value=>{value.receipt.current.bytes--;},value=>{value.receipt.current.sha256='0'.repeat(64);},
  value=>{value.receipt.additionalSources.pop();},value=>{value.receipt.additionalSources[0].sha256='0'.repeat(64);},
  value=>{value.receipt.verifierSources[0].sha256='0'.repeat(64);},value=>{value.receipt.verifierSources=[];},
  value=>{value.receipt.scope='native performance passed';},
 ]){const value=fixture();mutate(value);assert.throws(()=>verify(value),assert.AssertionError);}
});

test('coherent helper and compiler policy hashes cannot introduce warning suppression or unreviewed diagnostic behavior',()=>{
 for(const file of SOURCE_NATIVE_RUNNER_ADDITIONS.map(row=>row.file)){
  const value=fixture();const bytes=Buffer.concat([value.options.additionalSources[file],Buffer.from('\ncoherent but unreviewed edit\n')]);
  value.options.additionalSources[file]=bytes;
  const receipt=value.receipt.additionalSources.find(row=>row.file===file);receipt.bytes=bytes.length;receipt.sha256=sha256(bytes);
  assert.throws(()=>verify(value),assert.AssertionError);
 }
});

test('coherently edited caller and receipt tables cannot rewrite, remove or reorder any original producer binding',()=>{
 for(const complete of[false,true])for(const mutate of[
  sources=>({...sources,'policy.mjs':'0'.repeat(64)}),
  sources=>Object.fromEntries(Object.entries(sources).reverse()),
  sources=>Object.fromEntries(Object.entries(sources).slice(1)),
  sources=>({...sources,'unexpected-source.mjs':'0'.repeat(64)}),
 ]){
  const value=fixture(complete);value.historicalSources=mutate(value.historicalSources);value.receipt.historicalSources={...value.historicalSources};
  assert.throws(()=>verify(value),assert.AssertionError);
 }
});
