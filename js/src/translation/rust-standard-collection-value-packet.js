// This executable library module depends only on shared scalar/state kernels.
// Compiler graph linkage lives in the separate source call adapter.
import {executeRustStandardCollectionOperation,projectRustStandardCollectionOption} from './rust-standard-collection-state.js';
import {executeRustStandardCollectionEquality} from './rust-standard-collection-equality.js';
/** @typedef {{$:'rustCollectionPacketNormal',values:[RustStandardCollectionScalar|RustStandardCollectionOption|RustStandardCollectionState|{$:'rustCollectionUnit'},RustStandardCollectionState],writeback:boolean}|{$:'rustCollectionInvalid',reason:string}|{$:'rustCollectionResourceLimit',state?:RustStandardCollectionState,reason:string}} RustStandardCollectionPacketTransition */
/** A real callable body for later automatic source-library projection. Both
 * reads and writes return the original result-zero/state-one tuple recipe.
 * Bare get references cannot escape this adapter; projection is immediate.
 * @param {RustStandardCollectionState} state @param {string} method
 * @param {RustStandardCollectionScalar[]} arguments_ @param {RustStandardCollectionCallOrigin} call
 * @param {string} projection @returns {RustStandardCollectionPacketTransition} */
export function executeRustStandardCollectionValuePacket(state,method,arguments_,call,projection){
 if(method==='get'&&!['copied','cloned'].includes(projection)||method!=='get'&&projection!=='')return {$:'rustCollectionInvalid',reason:'collection source packets require an actual immediate get copied/cloned projection or no projection'};
 const result=executeRustStandardCollectionOperation(state,method,arguments_,call);
 if(result.$==='rustCollectionInvalid'||result.$==='rustCollectionResourceLimit')return result;
 if(result.$!=='rustCollectionNormal')return {$:'rustCollectionInvalid',reason:'collection method packets cannot substitute constructor completion'};
 if(method==='get'){
  if(result.value.$!=='rustCollectionOption')return {$:'rustCollectionInvalid',reason:'get packets retain their actual borrowed Option result'};
  const projected=projectRustStandardCollectionOption(result.state,result.value,projection);
  return projected.$==='rustCollectionNormal'?{$:'rustCollectionPacketNormal',values:[projected.value,projected.state],writeback:false}:projected.$==='rustCollectionResourceLimit'||projected.$==='rustCollectionInvalid'?projected:{$:'rustCollectionInvalid',reason:'collection projection retains an actual normal or refusal transition'};
 }
 return {$:'rustCollectionPacketNormal',values:[result.value,result.state],writeback:result.writeback};
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
