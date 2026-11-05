// Isolate full CST heaps while retaining all complete source replays before fixtures.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {availableParallelism,freemem} from 'node:os';
import {Worker} from 'node:worker_threads';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

export function readOriginalLinkRawArchiveWorkerSources() {
  return ['raw-source-archive-workers.mjs','raw-source-archive-worker.mjs'].map(name=>{
    const source=readFileSync(new URL('./'+name,import.meta.url));
    return{file:'experiments/original-link-complete-table-projection/'+name,...pin(source)};
  });
}

/** Preserve ordering and execute every source even when a sibling fails. */
export async function runOriginalLinkRawArchiveJobs(sources,invoke,concurrency,maximum=2) {
  assert.ok(Array.isArray(sources)&&sources.length>0&&sources.length<=3);
  assert.equal(typeof invoke,'function');assert.ok(Number.isSafeInteger(maximum)&&maximum>=1&&maximum<=3);
  assert.ok(Number.isSafeInteger(concurrency)&&concurrency>=1&&concurrency<=maximum);
  for(let index=0;index<sources.length;index++){
    assert.ok(Object.hasOwn(sources,index));const source=sources[index];
    assert.equal(typeof source,'string');assert.equal(source.isWellFormed(),true);assert.ok(Buffer.byteLength(source)<=1024*1024);
  }
  const input=[...sources],results=new Array(input.length),failures=[];let next=0;
  await Promise.all(Array.from({length:Math.min(concurrency,input.length)},async()=>{
    while(next<input.length){const index=next++;try{results[index]=await invoke(input[index],index);}
      catch(error){failures.push({index,error});}}
  }));
  if(failures.length)throw new AggregateError(failures.sort((a,b)=>a.index-b.index).map(row=>row.error),'complete source archive replay failed');
  return results;
}

/** A receipt is accepted only after a successful worker exit releases its heap. */
export function runOriginalLinkRawArchiveWorker(source,index,makeWorker=(...args)=>new Worker(...args)) {
  return new Promise((resolve,reject)=>{
    const expected=pin(source);let receipt,failure,messages=0;
    const worker=makeWorker(new URL('./raw-source-archive-worker.mjs',import.meta.url),{
      workerData:{source,index,expected},execArgv:[],name:'complete-source-archive-'+index,
      resourceLimits:{maxOldGenerationSizeMb:1024,maxYoungGenerationSizeMb:16},
    });
    worker.on('message',value=>{
      messages++;try{
        assert.equal(messages,1,'one complete archive receipt per worker');
        assert.deepEqual(Object.keys(value).sort(),['binaryReplaySha256','sourceSha256','textReplaySha256']);
        for(const actual of Object.values(value))assert.equal(actual,expected.sha256,'each complete source replay retains all original bytes');
        receipt=structuredClone(value);
      }catch(error){failure=error;}
    });
    worker.on('error',error=>{failure=error;});
    worker.on('exit',code=>{
      if(failure)return reject(failure);
      if(code!==0)return reject(new Error('complete source archive worker '+index+' exited '+code));
      if(messages!==1||!receipt)return reject(new Error('complete source archive worker '+index+' exited without its receipt'));
      resolve(receipt);
    });
  });
}

/** Select three heaps only when actual capacity leaves room for the parent. */
export function readOriginalLinkRawArchiveCapacity(processors,availableBytes,configured) {
  assert.ok(Number.isSafeInteger(processors)&&processors>=1);
  assert.ok(Number.isSafeInteger(availableBytes)&&availableBytes>=0);
  assert.ok(configured===undefined||/^[1-9][0-9]*$/u.test(configured),'the source archive job bound is a positive integer');
  const jobs=configured===undefined?3:Math.min(3,Number(configured));
  const memory=availableBytes>=5*1024**3?3:availableBytes>=3*1024**3?2:1;
  return Math.min(jobs,processors,memory);
}

/** Only CI runs complete archives. Each heap keeps its original one-GiB bound;
 * actual memory, CPUs and explicit job bounds select one, two or three. */
export async function replayOriginalLinkRawSourceArchives(sources,environment=process.env) {
  assert.equal(environment.CI,'true','complete source archive workers require CI=true');
  const processors=availableParallelism(),availableBytes=freemem();
  const concurrency=readOriginalLinkRawArchiveCapacity(processors,availableBytes,environment.META_LANGUAGE_JOBS);
  process.stdout.write('Raw source archive capacity: workers='+concurrency+'; processors='+processors+'; freeBytes='+availableBytes+'; heapMiB=1024\n');
  return runOriginalLinkRawArchiveJobs(sources,runOriginalLinkRawArchiveWorker,concurrency,3);
}

/** Prepare sources serially while already prepared sources use bounded workers.
 * No result escapes until every factory and complete worker has settled. */
export async function runOriginalLinkRawArchivePreparationJobs(prepare,invoke,concurrency){
 assert.ok(Array.isArray(prepare));const count=prepare.length;assert.ok(count>0&&count<=3);
 assert.equal(typeof invoke,'function');assert.ok(Number.isSafeInteger(concurrency)&&concurrency>=1&&concurrency<=3);
 const input=[];
 for(let index=0;index<count;index++){assert.ok(Object.hasOwn(prepare,index));const factory=prepare[index];assert.equal(typeof factory,'function');input.push(factory);}
 const sources=new Array(count),receipts=new Array(count),failures=[],active=new Set();
 for(let index=0;index<count;index++){
  while(active.size>=concurrency)await Promise.race(active);
  try{
   const source=await input[index]();
   assert.equal(typeof source,'string');assert.equal(source.isWellFormed(),true);assert.ok(Buffer.byteLength(source)<=1024*1024);
   sources[index]=source;
   // Invoke synchronously so the real worker starts before the next CPU-bound
   // factory runs on the coordinator. Only its result is asynchronous.
   const result=invoke(source,index);
   const pending=Promise.resolve(result).then(receipt=>{receipts[index]=receipt;},error=>{failures.push({index,error});}).finally(()=>active.delete(pending));
   active.add(pending);
  }catch(error){failures.push({index,error});}
 }
 await Promise.all(active);
 if(failures.length)throw new AggregateError(failures.sort((a,b)=>a.index-b.index).map(row=>row.error),'complete source archive preparation or replay failed');
 return{sources,receipts};
}

/** Every archive still completes before fixture/matrix allocation. Reserve one
 * additional GiB for overlapping coordinator preparation; worker heaps stay. */
export async function prepareOriginalLinkRawSourceArchives(prepare,environment=process.env){
 assert.equal(environment.CI,'true','complete source archive workers require CI=true');
 const processors=availableParallelism(),availableBytes=freemem();
 const concurrency=readOriginalLinkRawArchiveCapacity(processors,Math.max(0,availableBytes-1024**3),environment.META_LANGUAGE_JOBS);
 process.stdout.write('Raw source archive preparation capacity: workers='+concurrency+'; processors='+processors+'; freeBytes='+availableBytes+'; coordinatorReserveMiB=1024; heapMiB=1024\n');
 return runOriginalLinkRawArchivePreparationJobs(prepare,runOriginalLinkRawArchiveWorker,concurrency);
}
