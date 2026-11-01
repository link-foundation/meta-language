// Internal iterator records belong to their actual source frame and slot.
// They are inspectable control data, never fabricated ECMAScript Objects.
import {readJavaScriptOrdinaryHeapError} from './javascript-ordinary-heap.js';
import {readJavaScriptSynchronousIteratorRecordError} from './javascript-iterator-protocol.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @typedef {{$:'callIteratorRegion',slot:number,record:JavaScriptSynchronousIteratorRecord}} JavaScriptCallIteratorRegion */
/** @typedef {{$:'iteratorRegionsUpdated',regions:JavaScriptCallIteratorRegion[]} | {$:'iteratorRegionsInvalid',reason:string} | {$:'iteratorRegionsResourceLimit',reason:string}} JavaScriptIteratorRegionsUpdate */
/** @param {JavaScriptCallIteratorRegion[]} regions @param {number} slot @returns {number} */
export function findJavaScriptIteratorRegion(regions,slot){let low=0,high=regions.length;while(low<high){const middle=Math.floor((low+high)/2),row=regions[middle];if(row===undefined)return -1;if(row.slot===slot)return middle;if(row.slot<slot)low=middle+1;else high=middle;}return -1;}
/** @param {number} slot @returns {boolean} */
export function testJavaScriptIteratorRegionSlot(slot){return Number.isSafeInteger(slot)&&slot>=0&&slot<65536&&!Object.is(slot,-0);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallIteratorRegion[]} regions @returns {string} */
export function readJavaScriptIteratorRegionsError(heap,regions){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;if(regions.length>128)return 'iterator regions exceed their actual per-frame nesting budget';let previous=-1;
 for(const region of regions){if(region.$!=='callIteratorRegion'||!testJavaScriptIteratorRegionSlot(region.slot)||region.slot<=previous)return 'iterator regions retain ordered unique actual source slot coordinates';previous=region.slot;const invalid=readJavaScriptSynchronousIteratorRecordError(heap,region.record);if(invalid!=='')return invalid;}return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallIteratorRegion[]} regions @param {number} slot @param {JavaScriptSynchronousIteratorRecord} record @returns {JavaScriptIteratorRegionsUpdate} */
export function insertJavaScriptIteratorRegion(heap,regions,slot,record){
 const error=readJavaScriptIteratorRegionsError(heap,regions),invalid=readJavaScriptSynchronousIteratorRecordError(heap,record);if(error!==''||invalid!==''||!testJavaScriptIteratorRegionSlot(slot))return {$:'iteratorRegionsInvalid',reason:error||invalid||'iterator opening retains its actual source slot'};
 if(findJavaScriptIteratorRegion(regions,slot)>=0)return {$:'iteratorRegionsInvalid',reason:'an active source iterator region cannot be overwritten by another acquisition'};
 if(regions.length>=128)return {$:'iteratorRegionsResourceLimit',reason:'iterator region opening exceeds its actual per-frame nesting budget'};
 let at=0;while(at<regions.length&&(regions[at]?.slot??slot)<slot)at++;
 return {$:'iteratorRegionsUpdated',regions:regions.slice(0,at).concat([{$:'callIteratorRegion',slot,record:{...record}}],regions.slice(at))};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallIteratorRegion[]} regions @param {number} slot @param {JavaScriptSynchronousIteratorRecord} record @returns {JavaScriptIteratorRegionsUpdate} */
export function updateJavaScriptIteratorRegion(heap,regions,slot,record){
 const error=readJavaScriptIteratorRegionsError(heap,regions),invalid=readJavaScriptSynchronousIteratorRecordError(heap,record);if(error!==''||invalid!==''||!testJavaScriptIteratorRegionSlot(slot))return {$:'iteratorRegionsInvalid',reason:error||invalid||'iterator advancement retains its actual source slot'};
 const index=findJavaScriptIteratorRegion(regions,slot),prior=regions[index];if(prior===undefined)return {$:'iteratorRegionsInvalid',reason:'iterator advancement requires its actual opened source region'};
 if(!testJavaScriptSemanticSameValue(prior.record.iterator,record.iterator)||!testJavaScriptSemanticSameValue(prior.record.nextMethod,record.nextMethod)||prior.record.done&&!record.done)return {$:'iteratorRegionsInvalid',reason:'iterator advancement preserves its actual cached identities and cannot revive exhaustion'};
 return {$:'iteratorRegionsUpdated',regions:regions.map((row,at)=>at===index?{...row,record:{...record}}:row)};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallIteratorRegion[]} regions @param {number} slot @returns {JavaScriptIteratorRegionsUpdate} */
export function removeJavaScriptIteratorRegion(heap,regions,slot){
 const error=readJavaScriptIteratorRegionsError(heap,regions);if(error!==''||!testJavaScriptIteratorRegionSlot(slot))return {$:'iteratorRegionsInvalid',reason:error||'iterator region release retains its actual source slot'};
 const index=findJavaScriptIteratorRegion(regions,slot);return index<0?{$:'iteratorRegionsInvalid',reason:'iterator release requires its actual opened source region'}:{$:'iteratorRegionsUpdated',regions:regions.slice(0,index).concat(regions.slice(index+1))};
}
