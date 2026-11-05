// Standard closed-domain PartialEq compares membership and owned values. Entry
// vector order, allocation roots and mutation revisions are not Rust values.
import {readRustStandardCollectionStateError,testRustStandardCollectionScalarEquality} from './rust-standard-collection-state.js';
import {resolveRustStandardCollectionOwner,readRustStandardCollectionKind,readRustStandardCollectionSourceVersionError} from './rust-standard-collection-rules.js';
/** @typedef {{$:'rustStandardCollectionEqualityOrigin',owner:string,typeIdentity:string,operation:string,sourceVersionMinor:number,proven:boolean,leftAvailable:boolean,rightAvailable:boolean,customImplementation:boolean}} RustStandardCollectionEqualityOrigin */
/** @typedef {{$:'rustCollectionEqualityState',left:RustStandardCollectionState,right:RustStandardCollectionState,origin:RustStandardCollectionEqualityOrigin,leftIndex:number,rightIndex:number,phase:string,work:number}} RustStandardCollectionEqualityState */
/** @typedef {{$:'rustCollectionEqualityContinue',state:RustStandardCollectionEqualityState}|{$:'rustCollectionEqualityCompleted',value:boolean}|{$:'rustCollectionEqualityInvalid',reason:string}|{$:'rustCollectionEqualityResourceLimit',state:RustStandardCollectionEqualityState,reason:string}} RustStandardCollectionEqualityTransition */
/** @param {RustStandardCollectionState} left @param {RustStandardCollectionState} right @param {RustStandardCollectionEqualityOrigin} origin @returns {string} */
export function readRustStandardCollectionEqualityOriginError(left,right,origin){
 const first=readRustStandardCollectionStateError(left);if(first!=='')return first;const second=readRustStandardCollectionStateError(right);if(second!=='')return second;
 if(origin?.$!=='rustStandardCollectionEqualityOrigin'||!origin.proven||!origin.leftAvailable||!origin.rightAvailable||origin.customImplementation!==false||!['core::cmp::PartialEq','std::cmp::PartialEq'].includes(origin.owner)||!['equal','not equal'].includes(origin.operation))return 'collection equality retains actual available closed standard PartialEq operands and operation';
 const version=readRustStandardCollectionSourceVersionError(origin.sourceVersionMinor);if(version!=='')return version;
 const firstOwner=resolveRustStandardCollectionOwner(left.origin.resolvedOwner,true,false),secondOwner=resolveRustStandardCollectionOwner(right.origin.resolvedOwner,true,false);
 if(firstOwner!==secondOwner||left.keyDomain!==right.keyDomain||left.valueDomain!==right.valueDomain||left.pointerWidth!==right.pointerWidth||left.origin.typeIdentity!==right.origin.typeIdentity||origin.typeIdentity!==left.origin.typeIdentity)return 'collection equality preserves the exact standard owner concrete domains and pointer width within its supported source versions';
 return '';
}
/** @param {RustStandardCollectionEqualityState} state @returns {string} */
export function readRustStandardCollectionEqualityStateError(state){
 if(state?.$!=='rustCollectionEqualityState')return 'collection equality retains its actual source state';
 const source=readRustStandardCollectionEqualityOriginError(state.left,state.right,state.origin);if(source!=='')return source;
 for(const coordinate of [state.leftIndex,state.rightIndex,state.work])if(!Number.isSafeInteger(coordinate)||coordinate<0||Object.is(coordinate,-0))return 'collection equality retains exact nonnegative cursors and charged work';
 if(state.leftIndex>state.left.entries.length||state.rightIndex>state.right.entries.length||!['search','value'].includes(state.phase))return 'collection equality cursors remain within their actual complete operands';
 if(state.phase==='value'){
  const left=state.left.entries[state.leftIndex],right=state.right.entries[state.rightIndex];
  if(readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.left.origin.resolvedOwner,true,false))!=='map'||left?.value===undefined||right?.value===undefined||!testRustStandardCollectionScalarEquality(left.key,right.key))return 'map equality value comparison retains its actual selected equal-key pair';
 }
 if(state.left.entries.length!==state.right.entries.length&&(state.leftIndex!==0||state.rightIndex!==0||state.phase!=='search'||state.work!==0))return 'collection equality size preflight precedes every key comparison';
 if(state.leftIndex===state.left.entries.length&&(state.rightIndex!==0||state.phase!=='search'))return 'completed equality prefixes retain their canonical next cursor';
 // Recompute the closed pure prefix before accepting an external continuation.
 // A forged cursor cannot skip unequal entries or invent already charged work.
 const map=readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.left.origin.resolvedOwner,true,false))==='map';let work=0;
 for(let index=0;index<state.leftIndex;index++){
  const left=state.left.entries[index];let found=false;
  for(const right of state.right.entries){work+=readRustStandardCollectionScalarEqualityWork(left.key,right.key);if(testRustStandardCollectionScalarEquality(left.key,right.key)){if(map){if(left.value===undefined||right.value===undefined||!testRustStandardCollectionScalarEquality(left.value,right.value))return 'equality prefix retains actual equal owned values';work+=readRustStandardCollectionScalarEqualityWork(left.value,right.value);}found=true;break;}}
  if(!found)return 'equality prefix retains actual complete equal membership';
 }
 if(state.leftIndex<state.left.entries.length){const left=state.left.entries[state.leftIndex];for(let index=0;index<state.rightIndex;index++){const right=state.right.entries[index];if(testRustStandardCollectionScalarEquality(left.key,right.key))return 'equality search cursor cannot skip its actual matching key';work+=readRustStandardCollectionScalarEqualityWork(left.key,right.key);}if(state.phase==='value')work+=readRustStandardCollectionScalarEqualityWork(left.key,state.right.entries[state.rightIndex].key);}
 if(work!==state.work)return 'equality charged work retains its actual closed comparison prefix';
 return '';
}
/** @param {RustStandardCollectionScalar} left @param {RustStandardCollectionScalar} right @returns {number} */
function readRustStandardCollectionScalarEqualityWork(left,right){return left.$==='rustCollectionString'&&right.$==='rustCollectionString'?Math.max(1,left.units.length+right.units.length):1;}
/** @param {RustStandardCollectionEqualityState} state @returns {number} */
export function readRustStandardCollectionEqualityWork(state){
 if(state.left.entries.length!==state.right.entries.length||state.leftIndex===state.left.entries.length||state.rightIndex===state.right.entries.length)return 0;
 const left=state.left.entries[state.leftIndex],right=state.right.entries[state.rightIndex];
 return state.phase==='value'&&left.value!==undefined&&right.value!==undefined?readRustStandardCollectionScalarEqualityWork(left.value,right.value):readRustStandardCollectionScalarEqualityWork(left.key,right.key);
}
/** @param {RustStandardCollectionEqualityState} state @param {boolean} equal @returns {RustStandardCollectionEqualityTransition} */
function finishRustStandardCollectionEquality(state,equal){return {$:'rustCollectionEqualityCompleted',value:state.origin.operation==='not equal'?!equal:equal};}
/** @param {RustStandardCollectionState} left @param {RustStandardCollectionState} right @param {RustStandardCollectionEqualityOrigin} origin @returns {RustStandardCollectionEqualityTransition} */
export function beginRustStandardCollectionEquality(left,right,origin){
 const state={$:'rustCollectionEqualityState',left,right,origin,leftIndex:0,rightIndex:0,phase:'search',work:0},error=readRustStandardCollectionEqualityStateError(state);return error===''?{$:'rustCollectionEqualityContinue',state}:{$:'rustCollectionEqualityInvalid',reason:error};
}
/** A shared step performs one primitive key/value comparison. There is no
 * observable enumeration and no call to a represented hasher or user Eq body.
 * @param {RustStandardCollectionEqualityState} state @returns {RustStandardCollectionEqualityTransition} */
export function stepRustStandardCollectionEquality(state){
 const error=readRustStandardCollectionEqualityStateError(state);if(error!=='')return {$:'rustCollectionEqualityInvalid',reason:error};
 return advanceRustStandardCollectionEquality(state);
}
/** Internal transitions preserve validated immutable operands. No represented
 * callbacks run between validation and a complete bounded run.
 * @param {RustStandardCollectionEqualityState} state @returns {RustStandardCollectionEqualityTransition} */
function advanceRustStandardCollectionEquality(state){
 if(state.left.entries.length!==state.right.entries.length)return finishRustStandardCollectionEquality(state,false);
 if(state.leftIndex===state.left.entries.length)return finishRustStandardCollectionEquality(state,true);
 if(state.rightIndex===state.right.entries.length)return finishRustStandardCollectionEquality(state,false);
 const cost=readRustStandardCollectionEqualityWork(state);if(cost>Number.MAX_SAFE_INTEGER-state.work)return {$:'rustCollectionEqualityResourceLimit',state,reason:'collection equality charged work exceeds exact source coordinates'};
 const left=state.left.entries[state.leftIndex],right=state.right.entries[state.rightIndex],next={...state,work:state.work+cost};
 if(state.phase==='value'){
  if(left.value===undefined||right.value===undefined)return {$:'rustCollectionEqualityInvalid',reason:'map equality preserves actual owned values'};
  if(!testRustStandardCollectionScalarEquality(left.value,right.value))return finishRustStandardCollectionEquality(state,false);
  return {$:'rustCollectionEqualityContinue',state:{...next,leftIndex:state.leftIndex+1,rightIndex:0,phase:'search'}};
 }
 if(!testRustStandardCollectionScalarEquality(left.key,right.key))return {$:'rustCollectionEqualityContinue',state:{...next,rightIndex:state.rightIndex+1}};
 return readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.left.origin.resolvedOwner,true,false))==='map'?{$:'rustCollectionEqualityContinue',state:{...next,phase:'value'}}:{$:'rustCollectionEqualityContinue',state:{...next,leftIndex:state.leftIndex+1,rightIndex:0}};
}
/** Budget counts closed scalar comparisons plus compared String storage units.
 * Exhaustion retains both complete operands and the exact next comparison.
 * @param {RustStandardCollectionEqualityState} state @param {number} budget @returns {RustStandardCollectionEqualityTransition} */
export function runRustStandardCollectionEquality(state,budget){
 if(!Number.isSafeInteger(budget)||budget<0||budget>67108864||Object.is(budget,-0))return {$:'rustCollectionEqualityInvalid',reason:'collection equality requires an explicit bounded nonnegative work budget'};
 const error=readRustStandardCollectionEqualityStateError(state);if(error!=='')return {$:'rustCollectionEqualityInvalid',reason:error};
 let current=state,remaining=budget;
 while(true){
  const cost=readRustStandardCollectionEqualityWork(current);if(cost>remaining)return {$:'rustCollectionEqualityResourceLimit',state:current,reason:'closed collection equality exceeds its selected scalar and String work budget'};
  const transition=advanceRustStandardCollectionEquality(current);if(transition.$!=='rustCollectionEqualityContinue')return transition;
  remaining-=cost;current=transition.state;
 }
}
/** @param {RustStandardCollectionState} left @param {RustStandardCollectionState} right @param {RustStandardCollectionEqualityOrigin} origin @param {number} [budget] @returns {RustStandardCollectionEqualityTransition} */
export function executeRustStandardCollectionEquality(left,right,origin,budget=16777216){
 const beginning=beginRustStandardCollectionEquality(left,right,origin);return beginning.$==='rustCollectionEqualityContinue'?runRustStandardCollectionEquality(beginning.state,budget):beginning;
}
