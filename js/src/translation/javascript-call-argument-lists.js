// Abstract Call argument Lists are frame-owned control data, not hidden Arrays.
import {readJavaScriptOrdinaryValueError} from './javascript-ordinary-heap.js';
import {testJavaScriptIteratorRegionSlot} from './javascript-iterator-regions.js';
/** @typedef {{$:'callArgumentList',slot:number,depth:number,values:JavaScriptSemanticValue[]}} JavaScriptCallArgumentList */
/** @typedef {{$:'argumentListsUpdated',lists:JavaScriptCallArgumentList[]} | {$:'argumentListsInvalid',reason:string} | {$:'argumentListsResourceLimit',reason:string}} JavaScriptCallArgumentListsUpdate */
/** @param {JavaScriptCallArgumentList[]} lists @param {number} slot @returns {number} */
export function findJavaScriptCallArgumentList(lists,slot){for(let at=0;at<lists.length;at++)if(lists[at]?.slot===slot)return at;return -1;}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallArgumentList[]} lists @param {number} depth @returns {string} */
export function readJavaScriptCallArgumentListsError(heap,lists,depth){
 if(!Number.isSafeInteger(depth)||depth<0||depth>1024||Object.is(depth,-0)||lists.length>128)return 'argument Lists retain bounded actual handler depth and nesting';let previous=-1,count=0;
 for(const list of lists){if(list.$!=='callArgumentList'||!testJavaScriptIteratorRegionSlot(list.slot)||list.slot<=previous||!Number.isSafeInteger(list.depth)||Object.is(list.depth,-0)||list.depth<0||list.depth>depth)return 'argument Lists retain ordered unique slots and actual enclosing handler depths';previous=list.slot;count+=list.values.length;if(count>1048576)return 'argument Lists exceed their shared frame argument budget';for(const value of list.values){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}}
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallArgumentList[]} lists @param {number} slot @param {number} depth @returns {JavaScriptCallArgumentListsUpdate} */
export function openJavaScriptCallArgumentList(heap,lists,slot,depth){
 const error=readJavaScriptCallArgumentListsError(heap,lists,depth);if(error!==''||!testJavaScriptIteratorRegionSlot(slot))return {$:'argumentListsInvalid',reason:error||'argument opening requires its actual source slot'};
 if(findJavaScriptCallArgumentList(lists,slot)>=0)return {$:'argumentListsInvalid',reason:'argument opening cannot overwrite an unfinished source Call'};
 if(lists.length>=128)return {$:'argumentListsResourceLimit',reason:'argument Lists exceed their actual frame nesting budget'};
 let at=0;while(at<lists.length&&(lists[at]?.slot??slot)<slot)at++;return {$:'argumentListsUpdated',lists:lists.slice(0,at).concat([{$:'callArgumentList',slot,depth,values:[]}],lists.slice(at))};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallArgumentList[]} lists @param {number} slot @param {number} depth @param {JavaScriptSemanticValue} value @returns {JavaScriptCallArgumentListsUpdate} */
export function appendJavaScriptCallArgument(heap,lists,slot,depth,value){
 const error=readJavaScriptCallArgumentListsError(heap,lists,depth),invalid=readJavaScriptOrdinaryValueError(heap,value),at=findJavaScriptCallArgumentList(lists,slot),list=lists[at];if(error!==''||invalid!==''||list===undefined)return {$:'argumentListsInvalid',reason:error||invalid||'argument append requires its actual opened Call List'};
 let count=1;for(const row of lists)count+=row.values.length;if(count>1048576)return {$:'argumentListsResourceLimit',reason:'argument append exceeds the shared actual argument storage budget'};
 return {$:'argumentListsUpdated',lists:lists.map((row,index)=>index===at?{...row,values:row.values.concat([value])}:row)};
}
/** Abandon only evaluation owned by a crossed handler boundary. An outer
 * argument List survives an inner catch which completes its argument.
 * @param {JavaScriptCallArgumentList[]} lists @param {number} depth @returns {JavaScriptCallArgumentList[]} */
export function retainJavaScriptCallArgumentLists(lists,depth){return lists.filter(list=>list.depth<=depth);}
