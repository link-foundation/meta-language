// CI scheduling only. Each supplied worker retains its complete measurement.
import assert from 'node:assert/strict';
export async function measureReportModules(modules,{workers,resolveContext,record,failure,start=()=>{}}){
 assert.ok(Array.isArray(workers)&&workers.length>0&&workers.length<=2);
 assert.equal(new Set(modules).size,modules.length,'each complete report module is scheduled exactly once');
 const pending=new Map(),completed=new Map(),available=workers.map((_,index)=>index);let published=0,ordinal=0;
 const publish=()=>{
  while(completed.has(published)){
   const entry=completed.get(published);completed.delete(published++);
   if(entry.skip)continue;
   if(Object.hasOwn(entry,'error'))failure(entry.module,entry.error);
   else record(entry.module,entry.row,entry.started);
  }
 };
 const finish=async()=>{
  if(!pending.size)return;
  const entry=await Promise.race([...pending.values()]);
  pending.delete(entry.index);available.push(entry.owner);completed.set(entry.index,entry);publish();
 };
 for(const module of modules){
  const index=ordinal++;
  const started=performance.now();
  try{
   start(module,'resolving',started);
   // Import resolution overlaps earlier measurements, as in the serial host.
   const context=resolveContext(module);
   if(!available.length)await finish();
   start(module,'measuring',started);
   const owner=available.shift();let value;
   try{value=workers[owner](module,context);}catch(error){available.push(owner);throw error;}
   pending.set(index,Promise.resolve(value).then(row=>({module,started,index,owner,row}),error=>({module,started,index,owner,error})));
  }catch(error){failure(module,error);completed.set(index,{skip:true});publish();}
 }
 while(pending.size)await finish();
 publish();
}
