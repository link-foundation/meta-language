// Immediate owned snapshots of actual standard BTree iterators. Borrowed live
// iterators never escape this entry point and Hash storage order is unavailable.
import {readRustStandardCollectionStateError} from './rust-standard-collection-state.js';
import {readRustStandardCollectionCallOriginError,readRustStandardCollectionKind,testRustStandardCollectionOrdered} from './rust-standard-collection-rules.js';
/** @param {RustStandardCollectionState} state @param {string} method
 * @param {RustStandardCollectionCallOrigin} call @param {string} projection
 * @returns {RustStandardCollectionTransition} */
export function executeRustStandardCollectionOrderedSnapshot(state,method,call,projection){
 const source=readRustStandardCollectionStateError(state);if(source!=='')return {$:'rustCollectionInvalid',reason:source};
 const actual=readRustStandardCollectionCallOriginError(state.origin,state.root,method,0,call);if(actual!=='')return {$:'rustCollectionInvalid',reason:actual};
 const owner=state.origin.resolvedOwner,kind=readRustStandardCollectionKind(owner);
 if(!testRustStandardCollectionOrdered(owner)||!(kind==='set'&&method==='iter'||kind==='map'&&['keys','values'].includes(method)))return {$:'rustCollectionInvalid',reason:'ordered snapshots require the actual BTreeSet iterator or BTreeMap key/value iterator owner'};
 const domain=method==='values'?state.valueDomain:state.keyDomain;
 if(!['copied','cloned'].includes(projection)||projection==='copied'&&domain==='String')return {$:'rustCollectionInvalid',reason:'ordered snapshots require actual primitive Copy or owned Clone before immediate Vec collection'};
 const values=[];
 for(const entry of state.entries){const value=method==='values'?entry.value:entry.key;if(value===undefined)return {$:'rustCollectionInvalid',reason:'ordered map values retain their actual owned scalar'};values.push(value.$==='rustCollectionString'?{$:'rustCollectionString',units:[...value.units]}:{...value});}
 return {$:'rustCollectionNormal',state,value:{$:'rustCollectionOrderedValues',values},writeback:false};
}
