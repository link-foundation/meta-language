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
export async function runOriginalLinkRawArchiveJobs(sources,invoke,concurrency) {
  assert.ok(Array.isArray(sources)&&sources.length>0&&sources.length<=3);
  assert.equal(typeof invoke,'function');assert.ok(Number.isSafeInteger(concurrency)&&concurrency>=1&&concurrency<=2);
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

/** Only CI runs complete archives. At most two one-GiB engine heaps run, and
 * low available memory, one CPU or a lower explicit job bound selects one. */
export async function replayOriginalLinkRawSourceArchives(sources,environment=process.env) {
  assert.equal(environment.CI,'true','complete source archive workers require CI=true');
  const configured=environment.META_LANGUAGE_JOBS;
  assert.ok(configured===undefined||/^[1-9][0-9]*$/u.test(configured),'the source archive job bound is a positive integer');
  const jobs=configured===undefined?2:Math.min(2,Number(configured));
  const concurrency=Math.min(jobs,availableParallelism(),freemem()>=3*1024**3?2:1);
  return runOriginalLinkRawArchiveJobs(sources,runOriginalLinkRawArchiveWorker,concurrency);
}
