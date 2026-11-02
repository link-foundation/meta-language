// Closed primitive collection state uses explicit entries and scalar equality.
// Hash entry order is unobservable; BTree entries retain standard primitive Ord.
// No host Map/Set or represented user comparison callback implements behavior.
import {resolveRustStandardCollectionOwner,readRustStandardCollectionKind,readRustStandardCollectionSourceOriginError,readRustStandardCollectionIntegerWidth,readRustStandardCollectionDomainError,readRustStandardCollectionCallOriginError,readRustStandardCollectionArgumentOriginError,testRustStandardCollectionMutation,testRustStandardCollectionOrdered} from './rust-standard-collection-rules.js';
import {compareRustStandardCollectionScalars} from './rust-standard-collection-order.js';
import {acceptRustOwnedLocalTextUnits} from './rust-owned-local-state-rules.js';
/** @typedef {{$:'rustCollectionBoolean',value:boolean}|{$:'rustCollectionInteger',value:bigint}|{$:'rustCollectionString',units:number[]}} RustStandardCollectionScalar */
/** @typedef {{$:'rustCollectionReference',root:string,revision:number,key:RustStandardCollectionScalar,domain:string}} RustStandardCollectionReference */
/** @typedef {{$:'rustCollectionOption',present:boolean,value?:RustStandardCollectionScalar|RustStandardCollectionReference}} RustStandardCollectionOption */
/** @typedef {{key:RustStandardCollectionScalar,value?:RustStandardCollectionScalar}} RustStandardCollectionEntry */
/** @typedef {{$:'rustStandardCollectionState',origin:RustStandardCollectionOrigin,root:string,keyDomain:string,valueDomain:string,pointerWidth:number,revision:number,entries:RustStandardCollectionEntry[]}} RustStandardCollectionState */
/** @typedef {{$:'rustCollectionCreated',state:RustStandardCollectionState}|{$:'rustCollectionNormal',state:RustStandardCollectionState,value:RustStandardCollectionScalar|RustStandardCollectionOption|RustStandardCollectionState|{$:'rustCollectionOrderedValues',values:RustStandardCollectionScalar[]}|{$:'rustCollectionUnit'},writeback:boolean}|{$:'rustCollectionInvalid',reason:string}|{$:'rustCollectionResourceLimit',state?:RustStandardCollectionState,reason:string}} RustStandardCollectionTransition */
const maximumEntries=4096,maximumTextUnits=1048576;
/** @param {RustStandardCollectionScalar} value @returns {RustStandardCollectionScalar} */
function copyRustStandardCollectionScalar(value){return value.$==='rustCollectionString'?{$:'rustCollectionString',units:[...value.units]}:{...value};}
/** @param {RustStandardCollectionScalar} left @param {RustStandardCollectionScalar} right @returns {boolean} */
export function testRustStandardCollectionScalarEquality(left,right){
 if(left.$!==right.$)return false;
 if(left.$==='rustCollectionString'&&right.$==='rustCollectionString'){if(left.units.length!==right.units.length)return false;for(let index=0;index<left.units.length;index++)if(left.units[index]!==right.units[index])return false;return true;}
 return left.$==='rustCollectionBoolean'&&right.$==='rustCollectionBoolean'?left.value===right.value:left.$==='rustCollectionInteger'&&right.$==='rustCollectionInteger'&&left.value===right.value;
}
/** @param {RustStandardCollectionScalar} value @param {string} domain @param {number} pointerWidth @returns {string} */
export function readRustStandardCollectionScalarError(value,domain,pointerWidth){
 const error=readRustStandardCollectionDomainError(domain,pointerWidth);if(error!=='')return error;
 if(domain==='String')return value?.$==='rustCollectionString'&&value.units.length<=maximumTextUnits&&value.units.every(unit=>Number.isInteger(unit)&&unit>=0&&unit<=65535)&&acceptRustOwnedLocalTextUnits(value.units)?'':'collection String values retain bounded valid Unicode scalar text without lone surrogates';
 if(domain==='bool')return value?.$==='rustCollectionBoolean'&&typeof value.value==='boolean'?'':'collection Boolean domain accepts only actual Boolean scalars';
 if(value?.$!=='rustCollectionInteger'||typeof value.value!=='bigint')return 'collection integer domains accept only exact integer scalars';
 const width=readRustStandardCollectionIntegerWidth(domain,pointerWidth),signed=domain[0]==='i',magnitude=1n<<BigInt(signed?width-1:width),minimum=signed?-magnitude:0n,maximum=magnitude-1n;
 return value.value>=minimum&&value.value<=maximum?'':'collection integer scalar is outside its actual declared source domain';
}
/** @param {RustStandardCollectionEntry[]} entries @param {RustStandardCollectionScalar} key @returns {number} */
function findRustStandardCollectionEntry(entries,key){for(let index=0;index<entries.length;index++)if(testRustStandardCollectionScalarEquality(entries[index].key,key))return index;return -1;}
/** @param {RustStandardCollectionState} state @returns {string} */
export function readRustStandardCollectionStateError(state){
 if(state?.$!=='rustStandardCollectionState'||typeof state.root!=='string'||state.root.length===0||!Number.isSafeInteger(state.revision)||state.revision<0||Object.is(state.revision,-0))return 'collection state retains its actual source root and exact nonnegative revision';
 const source=readRustStandardCollectionSourceOriginError(state.origin,state.keyDomain,state.valueDomain,state.pointerWidth);if(source!=='')return source;
 if(state.entries.length>maximumEntries)return 'collection state exceeds its explicit entry storage profile';
 const kind=readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.origin.resolvedOwner,true,false));let units=0;
 for(let index=0;index<state.entries.length;index++){
  const entry=state.entries[index],keyError=readRustStandardCollectionScalarError(entry.key,state.keyDomain,state.pointerWidth);if(keyError!=='')return keyError;
  if(entry.key.$==='rustCollectionString')units+=entry.key.units.length;
  if(kind==='map'){if(entry.value===undefined)return 'standard Map entries retain actual owned values';const valueError=readRustStandardCollectionScalarError(entry.value,state.valueDomain,state.pointerWidth);if(valueError!=='')return valueError;if(entry.value.$==='rustCollectionString')units+=entry.value.units.length;}
  else if(entry.value!==undefined)return 'standard Set entries do not invent stored map values';
  if(testRustStandardCollectionOrdered(state.origin.resolvedOwner)&&index>0&&compareRustStandardCollectionScalars(state.entries[index-1].key,entry.key)>=0)return 'BTree collection entries retain strict standard primitive Ord including UTF8 String ordering';
  for(let previous=0;previous<index;previous++)if(testRustStandardCollectionScalarEquality(state.entries[previous].key,entry.key))return 'collection state retains one entry for each primitive Eq key';
 }
 return units<=maximumTextUnits?'':'collection state exceeds its explicit total String storage profile';
}
/** @param {RustStandardCollectionOrigin} origin @param {string} keyDomain @param {string} valueDomain @param {number} pointerWidth @param {string} root @param {RustStandardCollectionCallOrigin} call @returns {RustStandardCollectionTransition} */
export function createRustStandardCollectionState(origin,keyDomain,valueDomain,pointerWidth,root,call){
 const source=readRustStandardCollectionSourceOriginError(origin,keyDomain,valueDomain,pointerWidth);if(source!=='')return {$:'rustCollectionInvalid',reason:source};
 if(!['new','default'].includes(call?.method))return {$:'rustCollectionInvalid',reason:'collection creation retains its actual new or Default constructor dispatch'};
 const actual=readRustStandardCollectionCallOriginError(origin,root,call.method,0,call);if(actual!=='')return {$:'rustCollectionInvalid',reason:actual};
 const state={$:'rustStandardCollectionState',origin,root,keyDomain,valueDomain,pointerWidth,revision:0,entries:[]},error=readRustStandardCollectionStateError(state);return error===''?{$:'rustCollectionCreated',state}:{$:'rustCollectionInvalid',reason:error};
}
/** @param {RustStandardCollectionScalar|RustStandardCollectionReference|undefined} value @returns {RustStandardCollectionOption} */
function createRustStandardCollectionOption(value){return value===undefined?{$:'rustCollectionOption',present:false}:{$:'rustCollectionOption',present:true,value};}
/** @param {RustStandardCollectionState} state @param {RustStandardCollectionEntry[]} entries @returns {RustStandardCollectionTransition} */
function retainRustStandardCollectionMutation(state,entries){
 if(state.revision>=Number.MAX_SAFE_INTEGER)return {$:'rustCollectionResourceLimit',state,reason:'collection mutation revision exceeds its explicit exact coordinate profile'};
 const next={...state,revision:state.revision+1,entries},error=readRustStandardCollectionStateError(next);
 if(error!=='')return {$:'rustCollectionResourceLimit',state,reason:error};
 return {$:'rustCollectionNormal',state:next,value:{$:'rustCollectionUnit'},writeback:true};
}
/** @param {RustStandardCollectionState} state @param {string} method @param {RustStandardCollectionScalar[]} arguments_ @param {RustStandardCollectionCallOrigin} call @returns {RustStandardCollectionTransition} */
export function executeRustStandardCollectionOperation(state,method,arguments_,call){
 const error=readRustStandardCollectionStateError(state);if(error!=='')return {$:'rustCollectionInvalid',reason:error};
 const actual=readRustStandardCollectionCallOriginError(state.origin,state.root,method,arguments_.length,call);if(actual!=='')return {$:'rustCollectionInvalid',reason:actual};
 if(['iter','keys','values'].includes(method))return {$:'rustCollectionInvalid',reason:'ordered iterator snapshots require actual copied/cloned projection through the immediate value-packet adapter'};
 if(method==='new'||method==='default')return {$:'rustCollectionInvalid',reason:'collection constructors use the explicit source creation entry point'};
 const owner=resolveRustStandardCollectionOwner(state.origin.resolvedOwner,true,false),kind=readRustStandardCollectionKind(owner),borrowed=method!=='insert';
 for(let index=0;index<arguments_.length;index++){
  const domain=index===0?state.keyDomain:state.valueDomain,argument=readRustStandardCollectionArgumentOriginError(call.arguments[index],domain,borrowed);if(argument!=='')return {$:'rustCollectionInvalid',reason:argument};
  const scalar=readRustStandardCollectionScalarError(arguments_[index],domain,state.pointerWidth);if(scalar!=='')return {$:'rustCollectionInvalid',reason:scalar};
 }
 if(method==='len')return {$:'rustCollectionNormal',state,value:{$:'rustCollectionInteger',value:BigInt(state.entries.length)},writeback:false};
 if(method==='is_empty')return {$:'rustCollectionNormal',state,value:{$:'rustCollectionBoolean',value:state.entries.length===0},writeback:false};
 if(method==='clone'){
  const entries=state.entries.map(entry=>({key:copyRustStandardCollectionScalar(entry.key),...(entry.value===undefined?{}:{value:copyRustStandardCollectionScalar(entry.value)})})),value={...state,root:call.outputRoot??'',revision:0,entries};
  return {$:'rustCollectionNormal',state,value,writeback:false};
 }
 if(method==='clear')return retainRustStandardCollectionMutation(state,[]);
 const key=arguments_[0];if(key===undefined)return {$:'rustCollectionInvalid',reason:'collection lookup retains its actual complete key operand'};
 const position=findRustStandardCollectionEntry(state.entries,key),present=position>=0;
 if(method==='contains'||method==='contains_key')return {$:'rustCollectionNormal',state,value:{$:'rustCollectionBoolean',value:present},writeback:false};
 if(method==='get')return {$:'rustCollectionNormal',state,value:createRustStandardCollectionOption(present?{$:'rustCollectionReference',root:state.root,revision:state.revision,key:copyRustStandardCollectionScalar(state.entries[position].key),domain:kind==='map'?state.valueDomain:state.keyDomain}:undefined),writeback:false};
 if(!testRustStandardCollectionMutation(owner,method))return {$:'rustCollectionInvalid',reason:'collection operation requires its actual registered closed semantics'};
 const entries=state.entries.map(entry=>({...entry}));let value;
 if(method==='insert'){
  if(kind==='map'){
   const input=arguments_[1];if(input===undefined)return {$:'rustCollectionInvalid',reason:'standard Map insertion retains its actual owned value'};
   const previous=present?entries[position].value:undefined;if(present&&previous===undefined)return {$:'rustCollectionInvalid',reason:'standard Map replacement retains its actual old owned value'};
   value=createRustStandardCollectionOption(previous===undefined?undefined:copyRustStandardCollectionScalar(previous));
   if(present)entries[position]={...entries[position],value:copyRustStandardCollectionScalar(input)};
   else entries.push({key:copyRustStandardCollectionScalar(key),value:copyRustStandardCollectionScalar(input)});
  }else{value={$:'rustCollectionBoolean',value:!present};if(!present)entries.push({key:copyRustStandardCollectionScalar(key)});}
 }else{const previous=present?entries[position].value:undefined;if(kind==='map'&&present&&previous===undefined)return {$:'rustCollectionInvalid',reason:'standard Map removal retains its actual old owned value'};value=kind==='map'?createRustStandardCollectionOption(previous===undefined?undefined:copyRustStandardCollectionScalar(previous)):{$:'rustCollectionBoolean',value:present};if(present)entries.splice(position,1);}
 if(method==='insert'&&!present&&testRustStandardCollectionOrdered(owner)){const inserted=entries.pop();if(inserted===undefined)return {$:'rustCollectionInvalid',reason:'ordered insertion retains its actual new entry'};let index=0;while(index<entries.length&&compareRustStandardCollectionScalars(entries[index].key,inserted.key)<0)index++;entries.splice(index,0,inserted);}
 const changed=retainRustStandardCollectionMutation(state,entries);return changed.$==='rustCollectionNormal'?{...changed,value}:changed;
}
/** References retain the actual lexical root/revision. The parser must also
 * retain real borrow lifetimes; runtime revision checks never grant a loan.
 * @param {RustStandardCollectionState} state @param {RustStandardCollectionReference} reference @param {string} operation @returns {RustStandardCollectionTransition} */
export function readRustStandardCollectionReference(state,reference,operation){
 const error=readRustStandardCollectionStateError(state);if(error!=='')return {$:'rustCollectionInvalid',reason:error};
 const kind=readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.origin.resolvedOwner,true,false)),domain=kind==='map'?state.valueDomain:state.keyDomain;
 if(reference?.$!=='rustCollectionReference'||reference.root!==state.root||reference.revision!==state.revision||reference.domain!==domain||!['copied','cloned'].includes(operation)||operation==='copied'&&domain==='String')return {$:'rustCollectionInvalid',reason:'collection reference projection retains actual root revision domain and Copy or Clone operation'};
 const keyError=readRustStandardCollectionScalarError(reference.key,state.keyDomain,state.pointerWidth);if(keyError!=='')return {$:'rustCollectionInvalid',reason:keyError};
 const position=findRustStandardCollectionEntry(state.entries,reference.key);if(position<0)return {$:'rustCollectionInvalid',reason:'collection reference retains its actual live entry'};
 const entry=state.entries[position],value=kind==='map'?entry.value:entry.key;
 return value===undefined?{$:'rustCollectionInvalid',reason:'collection reference retains its actual stored value'}:{$:'rustCollectionNormal',state,value:copyRustStandardCollectionScalar(value),writeback:false};
}
/** @param {RustStandardCollectionState} state @param {RustStandardCollectionOption} option @param {string} operation @returns {RustStandardCollectionTransition} */
export function projectRustStandardCollectionOption(state,option,operation){
 const error=readRustStandardCollectionStateError(state);if(error!=='')return {$:'rustCollectionInvalid',reason:error};
 const kind=readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.origin.resolvedOwner,true,false)),domain=kind==='map'?state.valueDomain:state.keyDomain;
 if(option?.$!=='rustCollectionOption'||typeof option.present!=='boolean'||!['copied','cloned'].includes(operation)||operation==='copied'&&domain==='String')return {$:'rustCollectionInvalid',reason:'collection Option projection requires the actual standard borrowed Option Copy or Clone contract'};
 if(!option.present)return option.value===undefined?{$:'rustCollectionNormal',state,value:createRustStandardCollectionOption(undefined),writeback:false}:{$:'rustCollectionInvalid',reason:'None retains no invented payload'};
 if(option.value?.$!=='rustCollectionReference')return {$:'rustCollectionInvalid',reason:'copied and cloned require actual borrowed Option payloads'};
 const read=readRustStandardCollectionReference(state,option.value,operation);return read.$==='rustCollectionNormal'&&read.value.$!=='rustStandardCollectionState'&&read.value.$!=='rustCollectionOption'&&read.value.$!=='rustCollectionUnit'&&read.value.$!=='rustCollectionOrderedValues'?{...read,value:createRustStandardCollectionOption(read.value)}:read;
}
