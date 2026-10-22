// CI scheduling only. Each supplied worker retains its complete measurement.
import assert from 'node:assert/strict';
export async function measureReportModules(modules,{workers,resolveContext,record,failure,start=()=>{}}){
 assert.ok(Array.isArray(workers)&&workers.length>0&&workers.length<=2);
 assert.equal(new Set(modules).size,modules.length,'each complete report module is scheduled exactly once');
 const pending=[];let next=0;
 const finish=async()=>{
  const entry=pending.shift();if(!entry)return;
  const outcome=await entry.result;
  if(Object.hasOwn(outcome,'error'))failure(entry.module,outcome.error);
  else record(entry.module,outcome.row,entry.started);
 };
 for(const module of modules){
  const started=performance.now();
  try{
   start(module,'resolving',started);
   // Import resolution overlaps earlier measurements, as in the serial host.
   const context=resolveContext(module);
   if(pending.length===workers.length)await finish();
   start(module,'measuring',started);
   const value=workers[next](module,context);next=(next+1)%workers.length;
   pending.push({module,started,result:Promise.resolve(value).then(row=>({row}),error=>({error}))});
  }catch(error){failure(module,error);}
 }
 while(pending.length)await finish();
}
