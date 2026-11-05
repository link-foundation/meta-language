import assert from 'node:assert/strict';
import test from 'node:test';
import {runOriginalLinkRawArchivePreparationJobs} from '../../experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs';

test('source preparation overlaps active replays and publishes only after every worker releases its heap',{timeout:5000},async()=>{
 const sources=['first whole source','second whole source','third whole source'];
 const gates=sources.map(()=>Promise.withResolvers()),second=Promise.withResolvers(),third=Promise.withResolvers();
 const events=[];let active=0,peak=0,published=false;
 const prepare=sources.map((source,index)=>()=>{events.push('prepare '+index);if(index===1)assert.equal(active,1,'the first worker starts before the second source is prepared');return source;});
 const pending=runOriginalLinkRawArchivePreparationJobs(prepare,(source,index)=>{
  assert.equal(source,sources[index]);active++;peak=Math.max(peak,active);events.push('start '+index);
  if(index===1)second.resolve();if(index===2)third.resolve();
  return gates[index].promise.then(()=>{active--;events.push('exit '+index);return{source};});
 },2).then(result=>{published=true;return result;});
 await second.promise;assert.equal(peak,2);assert.equal(published,false);assert.ok(!events.includes('prepare 2'));
 gates[1].resolve();await third.promise;assert.equal(peak,2);assert.equal(published,false);
 gates[2].resolve();await Promise.resolve();assert.equal(published,false,'a finished sibling cannot publish before the first worker exits');
 gates[0].resolve();assert.deepEqual(await pending,{sources,receipts:sources.map(source=>({source}))});
 assert.equal(active,0);assert.ok(events.indexOf('prepare 1')>events.indexOf('start 0'));
});

test('serial source preparation waits for complete replay before preparing the next source',{timeout:5000},async()=>{
 const gate=Promise.withResolvers(),started=Promise.withResolvers(),events=[];
 const pending=runOriginalLinkRawArchivePreparationJobs([()=>{events.push('prepare 0');return'first';},()=>{events.push('prepare 1');return'second';}],(source,index)=>{
  events.push('start '+index);if(index===0){started.resolve();return gate.promise;}return source;
 },1);
 await started.promise;assert.deepEqual(events,['prepare 0','start 0']);gate.resolve('first');
 assert.deepEqual(await pending,{sources:['first','second'],receipts:['first','second']});
});

test('all preparation and replay failures retain original identity and every later task settles',async()=>{
 const first=new Error('first replay'),second=new Error('second preparation'),calls=[];
 await assert.rejects(runOriginalLinkRawArchivePreparationJobs([()=>{calls.push('prepare 0');return'first';},()=>{calls.push('prepare 1');throw second;},()=>{calls.push('prepare 2');return'third';}],(source,index)=>{
  calls.push('start '+index);return index===0?Promise.reject(first):Promise.resolve().then(()=>{calls.push('exit 2');return source;});
 },2),error=>{assert.ok(error instanceof AggregateError);assert.deepEqual(error.errors,[first,second]);return true;});
 assert.deepEqual(calls.filter(value=>value.startsWith('prepare')),['prepare 0','prepare 1','prepare 2']);
 assert.ok(calls.includes('exit 2'));
});

test('factory inventory and worker bounds refuse before any preparation or replay starts',async()=>{
 let calls=0;const operation=()=>{calls++;return'whole source';};
 for(const prepare of [[],new Array(2),[operation,undefined],[operation,operation,operation,operation]])await assert.rejects(runOriginalLinkRawArchivePreparationJobs(prepare,operation,2));
 for(const count of [0,4,1.5,NaN,Infinity,'2'])await assert.rejects(runOriginalLinkRawArchivePreparationJobs([operation],operation,count));
 assert.equal(calls,0);
 for(const value of ['', 'whole source'])assert.deepEqual(await runOriginalLinkRawArchivePreparationJobs([()=>value],source=>source,1),{sources:[value],receipts:[value]});
 for(const value of [undefined,1,'\ud800','x'.repeat(1024*1024+1)])await assert.rejects(runOriginalLinkRawArchivePreparationJobs([()=>value],operation,1),AggregateError);
 assert.equal(calls,0);
});

test('prepared factory inventory is owned before a factory can mutate the caller array',async()=>{
 const events=[];
 const prepare=[()=>{prepare[1]=()=>{throw new Error('foreign replacement');};prepare.length=1;return'first';},()=>{events.push('original second');return'second';}];
 assert.deepEqual(await runOriginalLinkRawArchivePreparationJobs(prepare,value=>value,2),{sources:['first','second'],receipts:['first','second']});
 assert.deepEqual(events,['original second']);
});
