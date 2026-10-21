import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {EventEmitter} from 'node:events';
import {Worker} from 'node:worker_threads';
import test from 'node:test';
import {runOriginalLinkRawArchiveJobs,runOriginalLinkRawArchiveWorker,replayOriginalLinkRawSourceArchives,
  readOriginalLinkRawArchiveWorkerSources} from '../../experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs';

test('bounded source archive jobs retain every complete input and deterministic result order',async()=>{
  const sources=['first complete source','second complete source','third complete source'],original=[...sources];
  let active=0,peak=0;const calls=[];
  const pending=runOriginalLinkRawArchiveJobs(sources,async(source,index)=>{
    calls.push([source,index]);active++;peak=Math.max(peak,active);
    await new Promise(resolve=>setTimeout(resolve,index===0?10:1));active--;return{source,index};
  },2);
  sources[2]='caller changed input';
  const result=await pending;
  assert.equal(peak,2);assert.equal(active,0);assert.deepEqual(calls,original.map((source,index)=>[source,index]));
  assert.deepEqual(result,original.map((source,index)=>({source,index})));
  const serial=[];await runOriginalLinkRawArchiveJobs(original,async source=>serial.push(source),1);assert.deepEqual(serial,original);
});

test('source archive failures keep every sibling check and reject invalid bounds before invocation',async()=>{
  const calls=[];
  await assert.rejects(runOriginalLinkRawArchiveJobs(['first','second','third'],async(source,index)=>{
    calls.push(source);if(index!==1)throw new Error(source);return source;
  },2),error=>{assert.ok(error instanceof AggregateError);assert.deepEqual(error.errors.map(item=>item.message),['first','third']);return true;});
  assert.deepEqual(calls,['first','second','third']);
  const invoke=()=>assert.fail('invalid input cannot invoke a worker');
  for(const sources of [[],new Array(1),['\ud800'],[null],['a'.repeat(1024*1024+1)],['a','b','c','d']])
    await assert.rejects(runOriginalLinkRawArchiveJobs(sources,invoke,2));
  for(const count of [0,3,1.5,NaN,Infinity,'2'])await assert.rejects(runOriginalLinkRawArchiveJobs(['a'],invoke,count));
  for(const environment of [{},{CI:'false'},{CI:'true',META_LANGUAGE_JOBS:'0'},{CI:'true',META_LANGUAGE_JOBS:'2.5'}])
    await assert.rejects(replayOriginalLinkRawSourceArchives(['small control only'],environment));
});

test('complete archive receipts require one full matching message and a successful worker exit',async()=>{
  const source='small source control',digest=createHash('sha256').update(source).digest('hex');
  const receipt={sourceSha256:digest,textReplaySha256:digest,binaryReplaySha256:digest};
  let options;
  const make=events=>(_url,input)=>{
    options=input;const worker=new EventEmitter();
    queueMicrotask(()=>{for(const[name,...arguments_]of events)worker.emit(name,...arguments_);});return worker;
  };
  assert.deepEqual(await runOriginalLinkRawArchiveWorker(source,1,make([['message',receipt],['exit',0]])),receipt);
  assert.deepEqual(options.resourceLimits,{maxOldGenerationSizeMb:1024,maxYoungGenerationSizeMb:16});
  assert.deepEqual(options.execArgv,[]);assert.equal(options.workerData.source,source);
  for(const events of [
    [['message',receipt],['exit',1]], [['exit',0]],
    [['message',receipt],['message',receipt],['exit',0]],
    [['message',{...receipt,binaryReplaySha256:'0'.repeat(64)}],['exit',0]],
    [['message',{...receipt,hidden:'extra'}],['exit',0]],
    [['message',receipt],['error',new Error('worker failed')],['exit',0]],
  ])await assert.rejects(runOriginalLinkRawArchiveWorker(source,0,make(events)));
});

test('worker source provenance is explicit and actual complete source work refuses local execution',async()=>{
  const sources=readOriginalLinkRawArchiveWorkerSources();assert.equal(sources.length,2);
  for(const row of sources){assert.ok(row.bytes>0);assert.match(row.sha256,/^[a-f0-9]{64}$/u);assert.match(row.file,/raw-source-archive-worker/u);}
  sources[0].sha256='changed';assert.notEqual(readOriginalLinkRawArchiveWorkerSources()[0].sha256,'changed');
  await assert.rejects(replayOriginalLinkRawSourceArchives(['small control only'],{CI:'false'}),/require CI=true/u);
});

test('the active Node worker API exposes bounded engine heaps before any application source loads',{timeout:5000},async t=>{
  const worker=new Worker("const {parentPort,resourceLimits}=require('node:worker_threads');parentPort.postMessage(resourceLimits);parentPort.close();",{
    eval:true,execArgv:[],resourceLimits:{maxOldGenerationSizeMb:64,maxYoungGenerationSizeMb:16},
  });
  t.after(()=>worker.terminate());
  const actual=await new Promise((resolve,reject)=>{
    let value;worker.on('message',message=>{value=message;});worker.on('error',reject);
    worker.on('exit',code=>code===0?resolve(value):reject(new Error('bounded Node API control exited '+code)));
  });
  assert.ok(actual.maxOldGenerationSizeMb>0&&actual.maxOldGenerationSizeMb<=1024);
  assert.ok(actual.maxYoungGenerationSizeMb>0&&actual.maxYoungGenerationSizeMb<=16);
});
