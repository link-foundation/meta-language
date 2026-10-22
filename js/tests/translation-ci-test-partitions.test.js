import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {partitionTests,partitionGroupFiles,testFiles} from '../scripts/test-groups.mjs';
import {partitionCiTestGroupFiles} from '../scripts/ci-test-partitions.mjs';
import {readCiTestPartitionHistory,verifyCiTestPartitionHistory,restoreCiTestPartitionWorkflow} from '../scripts/ci-test-partition-history.mjs';
test('physical CI partitions preserve every complete logical cohort exactly once under the original deadline',()=>{
 const groups=partitionTests(testFiles()).groups;
 for(const[group,oldCount]of [['translation',6],['conformance',4]]){
  const count=oldCount*2,files=groups[group],physical=Array.from({length:count},(_,index)=>partitionCiTestGroupFiles(group,files,`${index+1}/${count}`));
  assert.ok(physical.every(files=>files.length>0));assert.deepEqual(physical.flat().sort(),files);assert.equal(new Set(physical.flat()).size,files.length);
  for(let index=0;index<oldCount;index++)assert.deepEqual([...physical[index],...physical[index+oldCount]].sort(),partitionGroupFiles(files,`${index+1}/${oldCount}`),'complete original logical cohort '+group+index);
 }
 for(const[group,count]of [['grammar',6],['grammar-native',6],['tooling',4]])for(let at=1;at<=count;at++)assert.deepEqual(partitionCiTestGroupFiles(group,groups[group],`${at}/${count}`),partitionGroupFiles(groups[group],`${at}/${count}`));
 const packet=readCiTestPartitionHistory(),current=readFileSync(new URL('../../.github/workflows/js.yml',import.meta.url),'utf8');assert.equal(current,packet.current.source);
 const region=current.slice(current.indexOf('\n  tests:\n'),current.indexOf('\n  translation-test-feedback:\n'));
 assert.match(region,/timeout-minutes: 15\n/u);assert.match(region,/fail-fast: false\n/u);
 assert.match(region,/ci-test-partitions\.mjs --run "\$TEST_GROUP"/u);
 for(const[group,count]of [['translation',12],['conformance',8]])assert.deepEqual([...region.matchAll(new RegExp('\\{group: '+group+', partition: (\\d+), shards: (\\d+)\\}','gu'))].map(([,index,total])=>[+index,+total]),Array.from({length:count},(_,index)=>[index+1,count]));
 assert.match(current,/pattern: translation-test-feedback-\$\{\{ github.sha \}\}-\*-of-12/u);assert.match(current,/--partitions 12 \\/u);
 assert.equal(restoreCiTestPartitionWorkflow(current),packet.previous.source);assert.equal(restoreCiTestPartitionWorkflow(current+'\n'),current+'\n');
 for(const mutate of [value=>{value.changes.pop();},value=>{value.current.source+=' ';},value=>{value.previous.certificate.path='unregistered.yml';}]){const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyCiTestPartitionHistory(changed));}
});
test('physical scheduling refuses invalid scope and retains the serial child invocation',()=>{
 for(const shard of ['0/12','13/12','1/0','1.2/12','1/999'])assert.throws(()=>partitionCiTestGroupFiles('translation',['one','two'],shard));
 assert.throws(()=>partitionCiTestGroupFiles('translation',['one','one'],'1/2'));
 const source=readFileSync(new URL('../scripts/ci-test-partitions.mjs',import.meta.url),'utf8');assert.match(source,/'--test-concurrency=1'/u);
});
