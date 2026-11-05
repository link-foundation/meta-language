// Actual ordered standard iterator storage. This callable exposes only private
// shared entries; the source adapter supplies genuine receiver-derived regions.
import {readRustStandardCollectionStateError} from './rust-standard-collection-state.js';
import {readRustStandardCollectionCallOriginError,readRustStandardCollectionKind,testRustStandardCollectionOrdered} from './rust-standard-collection-rules.js';
/** @param {RustStandardCollectionState} state @param {string} method
 * @param {RustStandardCollectionCallOrigin} call @returns {RustStandardCollectionTransition} */
export function executeRustStandardCollectionOrderedBorrow(state,method,call){
 const source=readRustStandardCollectionStateError(state);if(source!=='')return {$:'rustCollectionInvalid',reason:source};
 const actual=readRustStandardCollectionCallOriginError(state.origin,state.root,method,0,call);if(actual!=='')return {$:'rustCollectionInvalid',reason:actual};
 const kind=readRustStandardCollectionKind(state.origin.resolvedOwner);
 if(!testRustStandardCollectionOrdered(state.origin.resolvedOwner)||!(kind==='set'&&method==='iter'||kind==='map'&&['iter','keys','values'].includes(method)))return {$:'rustCollectionInvalid',reason:'a shared ordered iterator requires its actual standard BTree owner and projection'};
 return {$:'rustCollectionNormal',state,value:{$:'rustCollectionOrderedEntries',entries:state.entries},writeback:false};
}
