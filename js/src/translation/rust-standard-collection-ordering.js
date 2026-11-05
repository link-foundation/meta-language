// Lexicographic standard BTree collection Ord over actual closed scalar domains.
// Iterator entries are already checked against the strict standard key order.
import {readRustStandardCollectionStateError} from './rust-standard-collection-state.js';
import {readRustStandardCollectionKind,resolveRustStandardCollectionOwner,testRustStandardCollectionOrdered,readRustStandardCollectionSourceVersionError} from './rust-standard-collection-rules.js';
import {compareRustStandardCollectionScalars} from './rust-standard-collection-order.js';
/** @typedef {{$:'rustStandardCollectionOrderingOrigin',owner:string,typeIdentity:string,sourceVersionMinor:number,proven:boolean,leftAvailable:boolean,rightAvailable:boolean,customImplementation:boolean}} RustStandardCollectionOrderingOrigin */
/** @typedef {{$:'rustCollectionOrderingState',left:RustStandardCollectionState,right:RustStandardCollectionState,origin:RustStandardCollectionOrderingOrigin,index:number,phase:string,work:number}} RustStandardCollectionOrderingState */
/** @typedef {{$:'rustCollectionOrderingContinue',state:RustStandardCollectionOrderingState}|{$:'rustCollectionOrderingCompleted',value:number}|{$:'rustCollectionOrderingInvalid',reason:string}|{$:'rustCollectionOrderingResourceLimit',state:RustStandardCollectionOrderingState,reason:string}} RustStandardCollectionOrderingTransition */
/** @param {RustStandardCollectionScalar} left @param {RustStandardCollectionScalar} right @returns {number} */
function orderingWork(left,right){return left.$==='rustCollectionString'&&right.$==='rustCollectionString'?Math.max(1,left.units.length+right.units.length):1;}
/** @param {RustStandardCollectionState} left @param {RustStandardCollectionState} right @param {RustStandardCollectionOrderingOrigin} origin @returns {string} */
export function readRustStandardCollectionOrderingOriginError(left,right,origin){
 const first=readRustStandardCollectionStateError(left);if(first!=='')return first;const second=readRustStandardCollectionStateError(right);if(second!=='')return second;
 if(!testRustStandardCollectionOrdered(left.origin.resolvedOwner)||resolveRustStandardCollectionOwner(left.origin.resolvedOwner,true,false)!==resolveRustStandardCollectionOwner(right.origin.resolvedOwner,true,false)||left.origin.typeIdentity!==right.origin.typeIdentity||left.keyDomain!==right.keyDomain||left.valueDomain!==right.valueDomain||left.pointerWidth!==right.pointerWidth)return 'ordering requires the same actual standard BTree owner and complete closed key/value domains';
 if(origin?.$!=='rustStandardCollectionOrderingOrigin'||!['core::cmp::Ord','std::cmp::Ord','core::cmp::PartialOrd','std::cmp::PartialOrd'].includes(origin.owner)||origin.typeIdentity!==left.origin.typeIdentity||!origin.proven||!origin.leftAvailable||!origin.rightAvailable||origin.customImplementation!==false)return 'ordering retains actual available closed standard Ord operands without custom implementation';
 return readRustStandardCollectionSourceVersionError(origin.sourceVersionMinor);
}
/** @param {RustStandardCollectionOrderingState} state @returns {string} */
export function readRustStandardCollectionOrderingStateError(state){
 if(state?.$!=='rustCollectionOrderingState')return 'ordering retains its actual complete source state';const source=readRustStandardCollectionOrderingOriginError(state.left,state.right,state.origin);if(source!=='')return source;
 const length=Math.min(state.left.entries.length,state.right.entries.length),map=readRustStandardCollectionKind(state.left.origin.resolvedOwner)==='map';
 if(!Number.isSafeInteger(state.index)||state.index<0||state.index>length||Object.is(state.index,-0)||!Number.isSafeInteger(state.work)||state.work<0||Object.is(state.work,-0)||!['key','value'].includes(state.phase)||state.phase==='value'&&(!map||state.index===length))return 'ordering retains bounded exact cursors charged work and map value phase';
 let work=0;for(let index=0;index<state.index;index++){const first=state.left.entries[index],second=state.right.entries[index];if(compareRustStandardCollectionScalars(first.key,second.key)!==0)return 'ordering cursor cannot skip the first unequal key';work+=orderingWork(first.key,second.key);if(map){if(first.value===undefined||second.value===undefined||compareRustStandardCollectionScalars(first.value,second.value)!==0)return 'ordering cursor cannot skip the first unequal owned map value';work+=orderingWork(first.value,second.value);}}
 if(state.phase==='value'){const first=state.left.entries[state.index],second=state.right.entries[state.index];if(compareRustStandardCollectionScalars(first.key,second.key)!==0)return 'ordering value phase retains actual equal selected keys';work+=orderingWork(first.key,second.key);}
 return work===state.work?'':'ordering charged work retains its actual complete equal prefix';
}
/** @param {RustStandardCollectionState} left @param {RustStandardCollectionState} right @param {RustStandardCollectionOrderingOrigin} origin @returns {RustStandardCollectionOrderingTransition} */
export function beginRustStandardCollectionOrdering(left,right,origin){const state={$:'rustCollectionOrderingState',left,right,origin,index:0,phase:'key',work:0},error=readRustStandardCollectionOrderingStateError(state);return error===''?{$:'rustCollectionOrderingContinue',state}:{$:'rustCollectionOrderingInvalid',reason:error};}
/** @param {RustStandardCollectionOrderingState} state @returns {number} */
function readRustStandardCollectionOrderingWork(state){if(state.index===Math.min(state.left.entries.length,state.right.entries.length))return 0;const first=state.left.entries[state.index],second=state.right.entries[state.index];return state.phase==='value'&&first.value!==undefined&&second.value!==undefined?orderingWork(first.value,second.value):orderingWork(first.key,second.key);}
/** @param {RustStandardCollectionOrderingState} state @returns {RustStandardCollectionOrderingTransition} */
function advanceRustStandardCollectionOrdering(state){
 if(state.index===Math.min(state.left.entries.length,state.right.entries.length))return {$:'rustCollectionOrderingCompleted',value:state.left.entries.length===state.right.entries.length?0:state.left.entries.length<state.right.entries.length?-1:1};
 const first=state.left.entries[state.index],second=state.right.entries[state.index];let comparison;
 if(state.phase==='value'){if(first.value===undefined||second.value===undefined)return {$:'rustCollectionOrderingInvalid',reason:'map ordering retains actual owned values'};comparison=compareRustStandardCollectionScalars(first.value,second.value);}else comparison=compareRustStandardCollectionScalars(first.key,second.key);
 if(comparison!==0)return {$:'rustCollectionOrderingCompleted',value:comparison};
 const map=readRustStandardCollectionKind(state.left.origin.resolvedOwner)==='map',next={...state,work:state.work+readRustStandardCollectionOrderingWork(state),index:state.index+(state.phase==='value'||!map?1:0),phase:state.phase==='key'&&map?'value':'key'};
 return {$:'rustCollectionOrderingContinue',state:next};
}
/** @param {RustStandardCollectionOrderingState} state @returns {RustStandardCollectionOrderingTransition} */
export function stepRustStandardCollectionOrdering(state){const error=readRustStandardCollectionOrderingStateError(state);return error===''?advanceRustStandardCollectionOrdering(state):{$:'rustCollectionOrderingInvalid',reason:error};}
/** @param {RustStandardCollectionOrderingState} state @param {number} budget @returns {RustStandardCollectionOrderingTransition} */
export function runRustStandardCollectionOrdering(state,budget){
 if(!Number.isSafeInteger(budget)||budget<0||budget>67108864||Object.is(budget,-0))return {$:'rustCollectionOrderingInvalid',reason:'ordered comparison retains its bounded exact work budget'};const error=readRustStandardCollectionOrderingStateError(state);if(error!=='')return {$:'rustCollectionOrderingInvalid',reason:error};
 let current=state,remaining=budget;while(true){const cost=readRustStandardCollectionOrderingWork(current);if(cost>remaining)return {$:'rustCollectionOrderingResourceLimit',state:current,reason:'BTree ordering exceeds its selected primitive and String work budget'};const result=advanceRustStandardCollectionOrdering(current);if(result.$!=='rustCollectionOrderingContinue')return result;remaining-=cost;current=result.state;}
}
/** @param {RustStandardCollectionState} left @param {RustStandardCollectionState} right @param {RustStandardCollectionOrderingOrigin} origin @param {number} budget @returns {RustStandardCollectionOrderingTransition} */
export function executeRustStandardCollectionOrdering(left,right,origin,budget){const initial=beginRustStandardCollectionOrdering(left,right,origin);return initial.$==='rustCollectionOrderingContinue'?runRustStandardCollectionOrdering(initial.state,budget):initial;}
