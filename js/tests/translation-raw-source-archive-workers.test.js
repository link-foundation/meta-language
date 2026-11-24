import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {EventEmitter} from 'node:events';
import {mkdtempSync, readFileSync, rmSync, writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {Worker} from 'node:worker_threads';
import test from 'node:test';
import {runOriginalLinkRawArchiveJobs,runOriginalLinkRawArchiveWorker,replayOriginalLinkRawSourceArchives,
  readOriginalLinkRawArchiveWorkerSources,prepareOriginalLinkRawArchiveBatch,canOverlapOriginalLinkRawSourcePreparation} from '../../experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs';
import {createOriginalLinkRawEmissionArtifactReader, readOriginalLinkRawArchiveSources} from '../../experiments/original-link-complete-table-projection/raw-native-observations.mjs';
import {structuredDataToLino} from '../src/structured-data-links.js';

test('raw emission artifact reuse depends on every current file byte', t => {
  const directory = mkdtempSync(join(tmpdir(), 'original-raw-emission-'));
  t.after(() => rmSync(directory, {recursive: true, force: true}));
  const path = join(directory, 'artifact.lino');
  const read = createOriginalLinkRawEmissionArtifactReader(path);
  const firstBytes = structuredDataToLino({definitions: ['one']});
  const secondBytes = structuredDataToLino({definitions: ['two']});
  assert.equal(Buffer.byteLength(firstBytes), Buffer.byteLength(secondBytes));

  writeFileSync(path, firstBytes);
  const first = read();
  assert.deepEqual(first, {definitions: ['one']});
  assert.ok(Object.isFrozen(first));
  assert.ok(Object.isFrozen(first.definitions));
  assert.strictEqual(read(), first);

  writeFileSync(path, secondBytes);
  const second = read();
  assert.deepEqual(second, {definitions: ['two']});
  assert.strictEqual(read(), second);
  writeFileSync(path, 'invalid artifact');
  assert.throws(read, TypeError);
  rmSync(path);
  assert.throws(read, {code: 'ENOENT'});

  writeFileSync(path, firstBytes);
  assert.deepEqual(read(), first);
});

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

test('parallel source preparation admits only identical sources and waits for every check',async()=>{
  const sources=['current source','consuming source','borrowed source'];
  const batch={sources};
  const receipts=['current','consuming','borrowed'];
  const calls=[];
  const prepare=()=>{calls.push('prepare');return batch;};
  const replay=inputs=>{calls.push('replay');assert.deepEqual(inputs,sources);return receipts;};
  assert.deepEqual(await prepareOriginalLinkRawArchiveBatch(prepare,replay),{batch,receipts});
  assert.deepEqual(calls.splice(0),['prepare','replay']);
  assert.deepEqual(await prepareOriginalLinkRawArchiveBatch(prepare,replay,[...sources]),{batch,receipts});
  assert.deepEqual(calls.splice(0),['replay','prepare']);
  await assert.rejects(prepareOriginalLinkRawArchiveBatch(()=>({sources:sources.toReversed()}),replay,sources),/every regenerated source/u);
  const release=Promise.withResolvers();
  const preparationFailed=new Error('source changed'),replayFailed=new Error('archive changed');
  const pending=prepareOriginalLinkRawArchiveBatch(()=>{throw preparationFailed;},async()=>{await release.promise;throw replayFailed;},sources);
  const rejection=assert.rejects(pending,error=>{assert.deepEqual(error.errors,[replayFailed,preparationFailed]);return true;});
  release.resolve();
  await rejection;
  assert.equal(canOverlapOriginalLinkRawSourcePreparation({},4,5*1024**3),true);
  assert.equal(canOverlapOriginalLinkRawSourcePreparation({},3,5*1024**3),false);
  assert.equal(canOverlapOriginalLinkRawSourcePreparation({},4,5*1024**3-1),false);
  assert.equal(canOverlapOriginalLinkRawSourcePreparation({META_LANGUAGE_JOBS:'2'},4,5*1024**3),false);
});

test('one checked current producer supplies both complete archive emissions', () => {
  const source = readFileSync(new URL('../../experiments/original-link-complete-table-projection/raw-native-observations.mjs', import.meta.url), 'utf8');
  const start = source.indexOf('export async function planOriginalLinkRawNative(');
  const end = source.indexOf('\nfunction originalLinkRawRecordDigest(', start);
  assert.ok(start >= 0 && end > start);
  const plan = source.slice(start, end);
  assert.doesNotMatch(plan, /\bgenerateOriginalLinkRawConsumingEmission\(/u, 'the standalone driver would prepare the whole current program again');
  assert.equal([...plan.matchAll(/\bprepareCurrentOriginalLinkRawEmission\(/gu)].length, 1);
  assert.match(plan, /const currentEmission = prepareCurrentOriginalLinkRawEmission\(/u);
  assert.match(plan, /prepareOriginalLinkRawSource\(currentEmission, /u);
  assert.match(plan, /prepareOriginalLinkRawConsumingEmission\(currentEmission\)/u);
  assert.match(plan, /assert\.deepEqual\(readOriginalLinkRawConsumingProfile\(generated\.artifact\.predecessor\), generated\.profile\)/u);
});

test('provisional archive source reads retain the three complete inputs and reject a changed receipt', () => {
  const sources = readOriginalLinkRawArchiveSources();
  assert.deepEqual(sources.map(source => [Buffer.byteLength(source), createHash('sha256').update(source).digest('hex')]), [
    [351680, '54388dca2da4b7f1a5bcbb7308d31d5f049997898b89a4bf940b2f7bb664aec5'],
    [351559, 'b0820a709afb3c66fc88321ceaa84e00baa795b0f07290896db6d00c740d9c7c'],
    [345744, 'bc87e15215c99ba3694f0d08e66b30252908fd2ce776f7f875b76b82af103095'],
  ]);
  assert.throws(() => readOriginalLinkRawArchiveSources((file, encoding) => {
    const text = readFileSync(file, encoding);
    return file.pathname.endsWith('/raw-borrowed-parameter-receipt.lino')
      ? text.replace('(def: 345744)', '(def: 345745)') : text;
  }), {name: 'AssertionError', actual: {bytes: 345744, sha256: 'bc87e15215c99ba3694f0d08e66b30252908fd2ce776f7f875b76b82af103095'}});
  assert.deepEqual(readOriginalLinkRawArchiveSources(), sources);
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
