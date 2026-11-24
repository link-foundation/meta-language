// Deferred internal region ownership controls; declarations are not evidence.
import test from 'node:test';import assert from 'node:assert/strict';
import {insertJavaScriptIteratorRegion,updateJavaScriptIteratorRegion,removeJavaScriptIteratorRegion,readJavaScriptIteratorRegionsError} from '../src/translation/javascript-iterator-regions.js';
import {insertJavaScriptOrdinaryObject} from '../src/translation/javascript-ordinary-heap.js';
const object=(identity,callable=false)=>({$:'object',identity,callable});
function setup(){let heap={$:'ordinaryHeap',objects:[],symbols:[]};for(const id of [1n,2n,3n])heap=insertJavaScriptOrdinaryObject(heap,id,{$:'null'}).heap;heap={...heap,objects:heap.objects.map(row=>row.identity===2n||row.identity===3n?{...row,callable:true}:row)};return {heap,record:{$:'synchronousIteratorRecord',iterator:object(1n),nextMethod:object(2n,true),done:false}};}
test('source regions retain ordered unique opened slots and release them explicitly',()=>{
 const {heap,record}=setup(),first=insertJavaScriptIteratorRegion(heap,[],7,record),second=insertJavaScriptIteratorRegion(heap,first.regions,3,record);assert.equal(second.$,'iteratorRegionsUpdated');assert.deepEqual(second.regions.map(row=>row.slot),[3,7]);assert.equal(readJavaScriptIteratorRegionsError(heap,second.regions),'');assert.equal(insertJavaScriptIteratorRegion(heap,second.regions,3,record).$,'iteratorRegionsInvalid');const released=removeJavaScriptIteratorRegion(heap,second.regions,3);assert.deepEqual(released.regions.map(row=>row.slot),[7]);assert.equal(second.regions.length,2);
});
test('source regions preserve cached identities and cannot revive an exhausted record',()=>{
 const {heap,record}=setup(),first=insertJavaScriptIteratorRegion(heap,[],1,record),done=updateJavaScriptIteratorRegion(heap,first.regions,1,{...record,done:true});assert.equal(done.$,'iteratorRegionsUpdated');assert.equal(updateJavaScriptIteratorRegion(heap,done.regions,1,record).$,'iteratorRegionsInvalid');assert.equal(updateJavaScriptIteratorRegion(heap,first.regions,1,{...record,nextMethod:object(3n,true)}).$,'iteratorRegionsInvalid');assert.equal(updateJavaScriptIteratorRegion(heap,first.regions,1,{...record,iterator:object(3n,true)}).$,'iteratorRegionsInvalid');assert.equal(updateJavaScriptIteratorRegion(heap,[],1,record).$,'iteratorRegionsInvalid');
});
test('region nesting and coordinate refusal preserve the original records',()=>{
 const {heap,record}=setup(),regions=Array.from({length:128},(_,slot)=>({$:'callIteratorRegion',slot,record}));assert.equal(insertJavaScriptIteratorRegion(heap,regions,128,record).$,'iteratorRegionsResourceLimit');for(const slot of [-1,-0,65536,NaN])assert.equal(insertJavaScriptIteratorRegion(heap,[],slot,record).$,'iteratorRegionsInvalid');assert.equal(regions.length,128);assert.equal(removeJavaScriptIteratorRegion(heap,regions,129).$,'iteratorRegionsInvalid');
});
