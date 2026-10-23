// Scheduling only: actual complete archive exits AND fresh compiler validation
// must succeed before any source receipts or observation matrix can publish.
import assert from 'node:assert/strict';
import {availableParallelism,freemem} from 'node:os';
import {readOriginalLinkRawArchiveCapacity,runOriginalLinkRawArchivePreparationJobs,runOriginalLinkRawArchiveWorker} from './raw-source-archive-workers.mjs';

export async function runOriginalLinkRawArchiveValidationJobs(sources,validate,invoke,concurrency){
 assert.ok(Array.isArray(sources)&&sources.length>0&&sources.length<=3);
 assert.equal(typeof validate,'function');assert.equal(typeof invoke,'function');
 assert.ok(Number.isSafeInteger(concurrency)&&concurrency>=1&&concurrency<=3);
 const owned=[];
 for(let index=0;index<sources.length;index++){
  assert.ok(Object.hasOwn(sources,index));const source=sources[index];
  assert.equal(typeof source,'string');assert.equal(source.isWellFormed(),true);assert.ok(Buffer.byteLength(source)<=1024*1024);owned.push(source);
 }
 Object.freeze(owned);
 const ready=Promise.withResolvers(),wanted=Math.min(concurrency,owned.length);let started=0;
 const archives=runOriginalLinkRawArchivePreparationJobs(owned.map(source=>()=>source),(source,index)=>{
  try{return invoke(source,index);}
  finally{started++;if(started>=wanted)ready.resolve();}
 },concurrency);
 // A constructor failure cannot strand the validation barrier. The original
 // archive scheduler still attempts every later source and retains all errors.
 archives.then(()=>ready.resolve(),()=>ready.resolve());
 await ready.promise;
 const results=await Promise.allSettled([archives,Promise.resolve().then(()=>validate(owned))]);
 const failures=results.filter(result=>result.status==='rejected').map(result=>result.reason);
 if(failures.length)throw new AggregateError(failures,'complete source archive replay or fresh source validation failed');
 assert.deepEqual(results[0].value.sources,owned,'every complete replay belongs to the original owned snapshot');
 return{sources:owned,receipts:results[0].value.receipts,validated:results[1].value};
}

export async function validateOriginalLinkRawSourceArchives(sources,validate,environment=process.env){
 assert.equal(environment.CI,'true','complete source archives require CI=true');
 const processors=availableParallelism(),availableBytes=freemem();
 const concurrency=readOriginalLinkRawArchiveCapacity(processors,Math.max(0,availableBytes-1024**3),environment.META_LANGUAGE_JOBS);
 process.stdout.write('Raw source archive validation capacity: workers='+concurrency+'; processors='+processors+'; freeBytes='+availableBytes+'; coordinatorReserveMiB=1024; heapMiB=1024\n');
 return runOriginalLinkRawArchiveValidationJobs(sources,validate,runOriginalLinkRawArchiveWorker,concurrency);
}
