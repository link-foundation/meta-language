import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {partitionTests,partitionGroupFiles,testFiles} from '../scripts/test-groups.mjs';
import {partitionCiTestGroupFiles} from '../scripts/ci-test-partitions.mjs';
import {readCiTestPartitionHistory,verifyCiTestPartitionHistory,restoreCiTestPartitionWorkflow,restoreCiTestPartitionSource,restorePhysicalReportWorkflow,restoreParallelPackageWorkflow} from '../scripts/ci-test-partition-history.mjs';
import {restoreWorkflowSourceHistory} from '../scripts/workflow-source-history.mjs';
test('physical CI partitions preserve every complete logical cohort exactly once under the original deadline',()=>{
 const groups=partitionTests(testFiles()).groups;
 for(const[group,oldCount]of [['translation',6],['conformance',4]]){
  const count=oldCount*2,files=groups[group],physical=Array.from({length:count},(_,index)=>partitionCiTestGroupFiles(group,files,`${index+1}/${count}`));
  assert.ok(physical.every(files=>files.length>0));assert.deepEqual(physical.flat().sort(),files);assert.equal(new Set(physical.flat()).size,files.length);
  for(let index=0;index<oldCount;index++)assert.deepEqual([...physical[index],...physical[index+oldCount]].sort(),partitionGroupFiles(files,`${index+1}/${oldCount}`),'complete original logical cohort '+group+index);
 }
 for(const[group,count]of [['grammar',6],['grammar-native',6],['tooling',4]])for(let at=1;at<=count;at++)assert.deepEqual(partitionCiTestGroupFiles(group,groups[group],`${at}/${count}`),partitionGroupFiles(groups[group],`${at}/${count}`));
 const packet=readCiTestPartitionHistory(),current=readFileSync(new URL('../../.github/workflows/js.yml',import.meta.url),'utf8');assert.equal(restoreParallelPackageWorkflow(current,'.github/workflows/js.yml'),packet.current.source);
 const region=current.slice(current.indexOf('\n  tests:\n'),current.indexOf('\n  translation-test-feedback:\n'));
 assert.match(region,/timeout-minutes: 15\n/u);assert.match(region,/fail-fast: false\n/u);
 assert.match(region,/ci-test-partitions\.mjs --run "\$TEST_GROUP"/u);
 for(const[group,count]of [['translation',12],['conformance',8]])assert.deepEqual([...region.matchAll(new RegExp('\\{group: '+group+', partition: (\\d+), shards: (\\d+)\\}','gu'))].map(([,index,total])=>[+index,+total]),Array.from({length:count},(_,index)=>[index+1,count]));
 assert.match(current,/pattern: translation-test-feedback-\$\{\{ github.sha \}\}-\*-of-12/u);assert.match(current,/--partitions 12 \\/u);
 assert.equal(restoreCiTestPartitionWorkflow(current),packet.previous.source);assert.equal(restoreCiTestPartitionWorkflow(current+'\n'),current+'\n');
 const feedback=readFileSync(new URL('../../'+packet.feedback.file,import.meta.url),'utf8');
 assert.equal(restoreCiTestPartitionSource(feedback,packet.feedback.file),packet.feedback.previous.source);
 assert.equal(restoreCiTestPartitionSource(feedback+'\n',packet.feedback.file),feedback+'\n');
 for(const mutate of [value=>{value.feedback.current.source+=' ';},value=>{value.feedback.previous.certificate.path='unregistered.mjs';},value=>{value.changes.pop();},value=>{value.current.source+=' ';},value=>{value.previous.certificate.path='unregistered.yml';}]){const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyCiTestPartitionHistory(changed));}
});
test('physical scheduling refuses invalid scope and retains the serial child invocation',()=>{
 for(const shard of ['0/12','13/12','1/0','1.2/12','1/999'])assert.throws(()=>partitionCiTestGroupFiles('translation',['one','two'],shard));
 assert.throws(()=>partitionCiTestGroupFiles('translation',['one','one'],'1/2'));
 const source=readFileSync(new URL('../scripts/ci-test-partitions.mjs',import.meta.url),'utf8');assert.match(source,/'--test-concurrency=1'/u);
});

test('physical report changes retain complete independently certified workflow predecessors',()=>{
 const packet=readCiTestPartitionHistory();assert.equal(packet.reportWorkflows.rows.length,2);
 for(const row of packet.reportWorkflows.rows){
  const actual=readFileSync(new URL('../../'+row.file,import.meta.url),'utf8');assert.equal(restoreParallelPackageWorkflow(actual,row.file),row.current.source);
  assert.equal(restorePhysicalReportWorkflow(actual,row.file),row.previous.source);
  assert.equal(restorePhysicalReportWorkflow(actual+'\n',row.file),actual+'\n');
  assert.equal(restorePhysicalReportWorkflow(actual,row.file+'.unknown'),actual);
  assert.equal(restorePhysicalReportWorkflow(row.previous.source,row.file),row.previous.source);
 }
 for(const mutate of [value=>{value.reportWorkflows.rows[0].current.source+=' ';},value=>{value.reportWorkflows.rows[0].previous.certificate.path='foreign.yml';},value=>{value.reportWorkflows.changes.pop();},value=>{value.reportWorkflows.rows.reverse();}]){
  const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyCiTestPartitionHistory(changed));
 }
});

test('parallel package checks retain every command and all JavaScript and native prerequisites',()=>{
 const packet=readCiTestPartitionHistory();assert.equal(packet.packageWorkflows.rows.length,2);
 for(const row of packet.packageWorkflows.rows){
  const published=readFileSync(new URL('../../'+row.file,import.meta.url),'utf8');
  const actual=restoreWorkflowSourceHistory(row.file,published);assert.equal(actual,row.current.source);
  assert.equal(restoreParallelPackageWorkflow(published,row.file),row.previous.source);
  assert.equal(restoreParallelPackageWorkflow(published+'\n',row.file),published+'\n');
  assert.equal(restoreParallelPackageWorkflow(published,row.file+'.unknown'),published);
  assert.equal(restoreParallelPackageWorkflow(row.previous.source,row.file),row.previous.source);
  if(row.file==='.github/workflows/js.yml'){
   assert.equal(row.previous.source.replace('    needs: [tests, report-assertions]\n',''),actual,'one dependency edge is the entire workflow change');
   const packageJob=actual.slice(actual.indexOf('\n  package-checks:\n'),actual.indexOf('\n  test:\n'));
   assert.doesNotMatch(packageJob,/^    needs:/mu);assert.doesNotMatch(packageJob,/download-artifact|\$\{\{ needs\./u);
   assert.match(actual,/needs: \[tests, report-assertions, package-checks, translation-test-feedback\]/u);
  }else{
   assert.equal(actual,row.previous.source,'the complete native and acceptance workflow remains exact');
   assert.match(actual,/needs: \[changes, js, self-translation-report-validation\]/u);
  }
 }
 for(const mutate of [value=>{value.packageWorkflows.rows[0].current.source+=' ';},value=>{value.packageWorkflows.rows[1].previous.certificate.path='foreign.yml';},value=>{value.packageWorkflows.rows[1].changes=[];},value=>{value.packageWorkflows.rows.reverse();}]){
  const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyCiTestPartitionHistory(changed));
 }
});

test('active workflows require every new holdout and source replay while retaining package gates',()=>{
 const readWorkflow = name => readFileSync(new URL(`../../.github/workflows/${name}.yml`,import.meta.url),'utf8');
 const jobs = source => new Map(parseWorkflow(source).map(job => [job.id, job]));
 const ci = jobs(readWorkflow('ci'));
 const aggregate = ci.get('acceptance');
 const holdouts = ci.get('grammar-inference-holdout');
 const versions = ci.get('rust-source-versions');
 const originals = jobs(readWorkflow('rust')).get('rust-state-source-originals');
 const javascript = jobs(readWorkflow('js'));
 assert.match(holdouts.header,/strategy: \[rpni, edsm, alergia, xvada, natgi, crucio, attributed\]/u);
 assert.match(holdouts.steps.map(step => step.text).join('\n'),/node-version: 24\.14\.0/u);
 assert.match(versions.header,/needs: \[js\]/u);
 assert.match(versions.header,/version: \['1\.90\.0', '1\.96\.0', '1\.97\.0', '1\.99\.0'\]/u);
 for(const [job, command] of [
  [versions, 'experiments/rust-source-versions/native-matrix.mjs'],
  [originals, 'js/scripts/run-rust-state-source-originals.mjs --execute'],
 ]){
  const steps = job.steps.map(step => step.text).join('\n');
  assert.match(steps,/node-version: 24\.22\.0/u);
  assert.ok(steps.includes(command),command);
  assert.match(steps,/if-no-files-found: error/u);
 }
 assert.match(originals.header,/needs: \[check\]/u);
 assert.match(originals.header,/needs\.check\.result == 'success'/u);
 assert.match(aggregate.header,/needs: \[[^\n]*rust-source-versions, grammar-inference-holdout\]/u);
 assert.match(aggregate.steps[0].text,/needs\.candidates\.result != 'success'/u);
 for (const job of ['rust-source-versions','grammar-inference-holdout']){
  assert.ok(aggregate.steps.some(step => step.text.includes(`needs.${job}.result != 'success'`) && step.text.includes('exit 1')),job);
 }
 assert.doesNotMatch(javascript.get('package-checks').header,/needs:/u);
 assert.match(javascript.get('test').header,/needs: \[tests, report-assertions, package-checks, translation-test-feedback\]/u);
});
