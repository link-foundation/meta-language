import assert from 'node:assert/strict';
import test from 'node:test';
import {measureReportModules} from '../scripts/self-translation-report-queue.mjs';
test('two bounded report workers preserve every complete module and deterministic output order',async()=>{
 const releases=new Map(),active=new Set(),events=[],rows=[],failures=[];
 const worker=owner=>(module,context)=>{
  assert.equal(active.has(owner),false,'one pending measurement per actual worker');active.add(owner);
  assert.equal(context.source,module+' complete source');events.push(module);
  return new Promise(resolve=>releases.set(module,()=>{active.delete(owner);resolve({module,source:context.source});}));
 };
 const running=measureReportModules(['first','second','third','fourth'],{
  workers:[worker(0),worker(1)],resolveContext:module=>({source:module+' complete source'}),record:(_,row)=>rows.push(row),failure:(...args)=>failures.push(args),
 });
 assert.deepEqual(events,['first','second']);assert.equal(active.size,2);
 releases.get('second')();await Promise.resolve();assert.deepEqual(rows,[],'a later worker cannot reorder the published report');
 releases.get('first')();
 for(let index=0;index<10&&!releases.has('fourth');index++)await Promise.resolve();
 assert.deepEqual(events,['first','second','third','fourth']);
 releases.get('fourth')();releases.get('third')();await running;
 assert.deepEqual(rows,['first','second','third','fourth'].map(module=>({module,source:module+' complete source'})));assert.deepEqual(failures,[]);assert.equal(active.size,0);
});
test('context and worker failures remain failures while all other complete measurements finish',async()=>{
 for(const count of[1,2]){
  const rows=[],failures=[],calls=[];
  await measureReportModules(['first','bad context','bad worker','bad synchronous worker','last'],{
   workers:Array.from({length:count},()=>module=>{calls.push(module);if(module==='bad synchronous worker')throw new Error(module);return module==='bad worker'?Promise.reject(new Error(module)):Promise.resolve({module});}),
   resolveContext:module=>{if(module==='bad context')throw new Error(module);return{};},record:(_,row)=>rows.push(row.module),failure:(module,error)=>failures.push([module,error.message]),
  });
  assert.deepEqual(rows,['first','last']);assert.deepEqual(calls,['first','bad worker','bad synchronous worker','last']);
  assert.deepEqual(failures.sort(),['bad context','bad worker','bad synchronous worker'].map(module=>[module,module]).sort());
 }
});
test('invalid report worker counts and duplicate module inventories refuse before measurement',async()=>{
 for(const workers of[[],[()=>{},()=>{},()=>{}]])await assert.rejects(measureReportModules(['first'],{workers}),assert.AssertionError);
 await assert.rejects(measureReportModules(['first','first'],{workers:[()=>{throw new Error('must not run');}]}),/exactly once/u);
});

test('a completed worker continues the complete queue while an earlier slow module retains publication order',async()=>{
 let releaseFirst;const events=[],rows=[],contexts=[],active=new Set();
 const worker=owner=>(module,context)=>{
  assert.equal(active.has(owner),false);active.add(owner);events.push(module);assert.equal(context.module,module);
  if(module==='first')return new Promise(resolve=>{releaseFirst=()=>{active.delete(owner);resolve({module});};});
  return Promise.resolve().then(()=>{active.delete(owner);return{module};});
 };
 const running=measureReportModules(['first','second','third','fourth'],{
  workers:[worker(0),worker(1)],resolveContext:module=>{contexts.push(module);return{module};},record:(_,row)=>rows.push(row.module),failure:(_,error)=>{throw error;},
 });
 try{
  for(let tick=0;tick<40&&!events.includes('fourth');tick++)await Promise.resolve();
  assert.deepEqual(events,['first','second','third','fourth'],'the free worker cannot idle behind an earlier slow measurement');
  assert.deepEqual(contexts,events,'complete import contexts retain their original resolution order');
  assert.deepEqual(rows,[],'publication still waits for the complete preceding row');
 }finally{releaseFirst();await running;}
 assert.deepEqual(rows,['first','second','third','fourth']);assert.equal(active.size,0);
});
