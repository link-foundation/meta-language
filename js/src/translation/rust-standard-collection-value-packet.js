// This executable library module depends only on shared scalar/state kernels.
// Compiler graph linkage lives in the separate source call adapter.
import {executeRustStandardCollectionOperation,projectRustStandardCollectionOption} from './rust-standard-collection-state.js';
import {executeRustStandardCollectionOrderedBorrow,executeRustStandardCollectionOrderedMove} from './rust-standard-collection-ordered-borrow.js';
import {executeRustStandardCollectionOrderedSnapshot} from './rust-standard-collection-ordered-snapshot.js';
import {executeRustStandardCollectionOrdering} from './rust-standard-collection-ordering.js';
import {executeRustStandardCollectionEquality} from './rust-standard-collection-equality.js';
/** @typedef {{$:'rustCollectionPacketNormal',values:[RustStandardCollectionValue,RustStandardCollectionState],writeback:boolean}|{$:'rustCollectionInvalid',reason:string}|{$:'rustCollectionResourceLimit',state?:RustStandardCollectionState,reason:string}} RustStandardCollectionPacketTransition */
/** @param {RustStandardCollectionTransition} result @returns {RustStandardCollectionPacketTransition} */
function createRustStandardCollectionPacket(result){
 if(result.$==='rustCollectionNormal')return {$:'rustCollectionPacketNormal',values:[result.value,result.state],writeback:result.writeback};
 if(result.$==='rustCollectionInvalid')return {$:'rustCollectionInvalid',reason:result.reason};
 if(result.$==='rustCollectionResourceLimit'){
  const state=result.state;
  if(state===undefined)return {$:'rustCollectionResourceLimit',reason:result.reason};
  return {$:'rustCollectionResourceLimit',state,reason:result.reason};
 }
 return {$:'rustCollectionInvalid',reason:'collection method packets cannot substitute constructor completion'};
}
/** A real callable body for later automatic source-library projection. Both
 * reads and writes return the original result-zero/state-one tuple recipe.
 * Bare get references cannot escape this adapter; projection is immediate.
 * @param {RustStandardCollectionState} state @param {string} method
 * @param {RustStandardCollectionScalar[]} arguments_ @param {RustStandardCollectionCallOrigin} call
 * @param {string} projection @returns {RustStandardCollectionPacketTransition} */
export function executeRustStandardCollectionValuePacket(state,method,arguments_,call,projection){
 if(method==='into_iter'){
  if(arguments_.length!==0||projection!=='owned')return {$:'rustCollectionInvalid',reason:'ordered IntoIterator retains its actual empty argument list and owned result'};
  return createRustStandardCollectionPacket(executeRustStandardCollectionOrderedMove(state,call));
 }
 const ordered=['iter','keys','values'].includes(method);
 if(ordered){if(arguments_.length!==0)return {$:'rustCollectionInvalid',reason:'ordered iterator projection retains its actual empty argument list'};const result=projection==='borrowed'?executeRustStandardCollectionOrderedBorrow(state,method,call):executeRustStandardCollectionOrderedSnapshot(state,method,call,projection);return createRustStandardCollectionPacket(result);}
 if(method==='get'&&!['copied','cloned'].includes(projection)||method!=='get'&&projection!=='')return {$:'rustCollectionInvalid',reason:'collection source packets require an actual immediate get copied/cloned projection or no projection'};
 const result=executeRustStandardCollectionOperation(state,method,arguments_,call);
 if(result.$!=='rustCollectionNormal')return createRustStandardCollectionPacket(result);
 if(method==='get'){
  if(result.value.$!=='rustCollectionOption')return {$:'rustCollectionInvalid',reason:'get packets retain their actual borrowed Option result'};
  const projected=projectRustStandardCollectionOption(result.state,result.value,projection);
  if(projected.$==='rustCollectionCreated')return {$:'rustCollectionInvalid',reason:'collection projection retains an actual normal or refusal transition'};
  return createRustStandardCollectionPacket(projected);
 }
 return createRustStandardCollectionPacket(result);
}
/** Exact boundary encoding of an actual owned Rust String. Its source domain
 * has already excluded lone surrogates; no Eq normalization is performed.
 * @param {string} value @returns {RustStandardCollectionScalar} */
export function encodeRustStandardCollectionText(value){const units=[];for(let index=0;index<value.length;index++)units.push(value.charCodeAt(index));return {$:'rustCollectionString',units};}
/** @param {number[]} units @returns {string} */
export function decodeRustStandardCollectionText(units){let value='';for(const unit of units)value+=String.fromCharCode(unit);return value;}
/** @param {RustStandardCollectionState} left @param {RustStandardCollectionState} right
 * @param {RustStandardCollectionEqualityOrigin} origin @param {number} budget
 * @returns {RustStandardCollectionEqualityTransition} */
export function compareRustStandardCollectionValuePackets(left,right,origin,budget){return executeRustStandardCollectionEquality(left,right,origin,budget);}

/** @param {RustStandardCollectionState} left @param {RustStandardCollectionState} right
 * @param {RustStandardCollectionOrderingOrigin} origin @param {number} budget
 * @returns {RustStandardCollectionOrderingTransition} */
export function compareRustStandardCollectionOrderedValuePackets(left,right,origin,budget){return executeRustStandardCollectionOrdering(left,right,origin,budget);}
