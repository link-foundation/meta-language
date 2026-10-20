import assert from 'node:assert/strict';
import test from 'node:test';
import {readOriginalLinkRawArchiveCapacity,runOriginalLinkRawArchiveJobs} from '../../experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs';

test('complete source replay capacity follows actual CPU memory and operator bounds',()=>{
  const gib=1024**3;
  for(const [processors,bytes,jobs,expected]of [
    [1,16*gib,undefined,1],[2,16*gib,undefined,2],[4,0,undefined,1],
    [4,3*gib-1,undefined,1],[4,3*gib,undefined,2],[4,5*gib-1,undefined,2],
    [3,5*gib,undefined,3],[4,16*gib,'1',1],[4,16*gib,'2',2],[4,16*gib,'3',3],
  ])assert.equal(readOriginalLinkRawArchiveCapacity(processors,bytes,jobs),expected);
  for(const count of [0,-1,NaN,Infinity,1.5])assert.throws(()=>readOriginalLinkRawArchiveCapacity(count,16*gib));
  for(const bytes of [-1,NaN,Infinity,1.5])assert.throws(()=>readOriginalLinkRawArchiveCapacity(4,bytes));
  for(const jobs of ['0','2.5','-1','3x',''])assert.throws(()=>readOriginalLinkRawArchiveCapacity(4,16*gib,jobs));
});

test('three admitted complete sources start together and retain all sibling failures',{timeout:5000},async()=>{
  const started=[],sources=['current','consuming','borrowed'];let release;
  const barrier=new Promise(resolve=>{release=resolve;});
  const pending=runOriginalLinkRawArchiveJobs(sources,async(source,index)=>{
    started.push(source);if(started.length===3)release();await barrier;
    if(index!==1)throw new Error(source);return source;
  },3,3);
  await assert.rejects(pending,error=>{
    assert.ok(error instanceof AggregateError);
    assert.deepEqual(error.errors.map(item=>item.message),['current','borrowed']);return true;
  });
  assert.deepEqual(started,sources);
});

test('the original two-worker invocation profile and every refusal stay available',async()=>{
  const invoke=()=>assert.fail('an invalid profile cannot invoke work');
  await assert.rejects(runOriginalLinkRawArchiveJobs(['a'],invoke,3));
  for(const maximum of [0,4,NaN,Infinity,1.5,'3'])await assert.rejects(runOriginalLinkRawArchiveJobs(['a'],invoke,1,maximum));
  assert.deepEqual(await runOriginalLinkRawArchiveJobs(['a','b','c'],async value=>value,3,3),['a','b','c']);
});
