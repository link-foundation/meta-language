// This worker invokes the exact original public CST/text/binary replay algorithm.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {parentPort,workerData,isMainThread,resourceLimits} from 'node:worker_threads';
assert.equal(isMainThread,false);assert.equal(process.env.CI,'true');
assert.ok(resourceLimits.maxOldGenerationSizeMb>0&&resourceLimits.maxOldGenerationSizeMb<=1024,'inherited Node options cannot raise the original source heap bound');
assert.ok(resourceLimits.maxYoungGenerationSizeMb>0&&resourceLimits.maxYoungGenerationSizeMb<=16);
assert.ok(parentPort);assert.equal(typeof workerData.source,'string');
assert.equal(workerData.source.isWellFormed(),true);assert.ok(Buffer.byteLength(workerData.source)<=1024*1024);
assert.deepEqual(workerData.expected,{bytes:Buffer.byteLength(workerData.source),sha256:createHash('sha256').update(workerData.source).digest('hex')});
process.stdout.write('Raw source archive worker: index='+workerData.index+'; bytes='+workerData.expected.bytes+'; begin\n');
const{replayOriginalLinkRawRustSource}=await import('./raw-native-observations.mjs');
const receipt=await replayOriginalLinkRawRustSource(workerData.source);
parentPort.postMessage(receipt);parentPort.close();
