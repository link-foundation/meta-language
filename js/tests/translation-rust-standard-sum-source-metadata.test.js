// Deferred source controls: no generation or execution during drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {metadataCosts,reservations} from './fixtures/rust-standard-sum-source-metadata-draft.mjs';
import {measureRustStandardSumSourceMetadata} from '../src/translation/rust-standard-sum-source-metadata.js';
import {readRustStandardSumSourceMetadataCountError,readRustStandardSumSourceMetadataReservation} from '../src/translation/rust-standard-sum-source-metadata-rules.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===24||major===22&&minor>=15,'uncovered source metadata control Node '+process.versions.node);
for(const row of metadataCosts)test(row.name,()=>assert.deepEqual(measureRustStandardSumSourceMetadata([row.origin]),row.expected));
for(const [index,row]of reservations.entries())test('semantic copy reservation boundary '+index,()=>assert.deepEqual(readRustStandardSumSourceMetadataReservation(...row.input),row.expected));
test('semantic metadata depth includes boundary and refuses next level',()=>{
 assert.equal(readRustStandardSumSourceMetadataCountError(1,0,256),'');
 assert.notEqual(readRustStandardSumSourceMetadataCountError(1,0,257),'');
 for(const depth of [-1,0.5,NaN,Infinity])assert.notEqual(readRustStandardSumSourceMetadataCountError(1,0,depth),'');
});
test('spans and incidental host bookkeeping do not change the shared projection',()=>{
 const row=metadataCosts[0];
 const origin={...row.origin,span:{start:1,end:999},sourceBindingIdentity:'incidental',standardSumSourceOrdinal:99};
 assert.deepEqual(measureRustStandardSumSourceMetadata([origin]),row.expected);
});
test('shared references count per occurrence and structural cycles refuse',()=>{
 const row=metadataCosts[0],shared={...row.origin,expandedSourceOwner:row.origin.sourceOwner};
 assert.deepEqual(measureRustStandardSumSourceMetadata([shared]),[23,224]);
 const cyclic={...row.origin};cyclic.fields=[cyclic];
 assert.throws(()=>measureRustStandardSumSourceMetadata([cyclic]),/structural cycle/);
});
