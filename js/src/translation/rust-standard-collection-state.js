// Closed primitive collection state uses explicit entries and scalar equality.
// Hash entry order is unobservable; BTree entries retain standard primitive Ord.
// No host Map/Set or represented user comparison callback implements behavior.
import {resolveRustStandardCollectionOwner,readRustStandardCollectionKind,readRustStandardCollectionSourceOriginError,readRustStandardCollectionIntegerWidth,readRustStandardCollectionDomainError,readRustStandardCollectionCallOriginError,readRustStandardCollectionArgumentOriginError,testRustStandardCollectionMutation,testRustStandardCollectionOrdered} from './rust-standard-collection-rules.js';
import {compareRustStandardCollectionScalars} from './rust-standard-collection-order.js';
import {acceptRustOwnedLocalTextUnits} from './rust-owned-local-state-rules.js';
/** @typedef {{$:'rustCollectionBoolean',value:boolean}|{$:'rustCollectionInteger',value:bigint}|{$:'rustCollectionString',units:number[]}} RustStandardCollectionScalar */
/** @typedef {{$:'rustCollectionReference',root:string,revision:number,key:RustStandardCollectionScalar,domain:string}|RustStandardCollectionScalar} RustStandardCollectionOptionValue */
/** @typedef {{$:'rustCollectionOption',present:boolean,value?:RustStandardCollectionOptionValue}} RustStandardCollectionOption */
/** @typedef {{key:RustStandardCollectionScalar,value?:RustStandardCollectionScalar}} RustStandardCollectionEntry */
/** @typedef {{$:'rustStandardCollectionState',origin:RustStandardCollectionOrigin,root:string,keyDomain:string,valueDomain:string,pointerWidth:number,revision:number,entries:RustStandardCollectionEntry[]}} RustStandardCollectionState */
/** @typedef {{$:'rustCollectionOrderedValues',values:RustStandardCollectionScalar[]}|{$:'rustCollectionOrderedEntries',entries:RustStandardCollectionEntry[]}|{$:'rustCollectionUnit'}|RustStandardCollectionScalar|RustStandardCollectionOption|RustStandardCollectionState} RustStandardCollectionValue */
/** @typedef {{$:'rustCollectionCreated',state:RustStandardCollectionState}|{$:'rustCollectionNormal',state:RustStandardCollectionState,value:RustStandardCollectionValue,writeback:boolean}|{$:'rustCollectionInvalid',reason:string}|{$:'rustCollectionResourceLimit',state?:RustStandardCollectionState,reason:string}} RustStandardCollectionTransition */
/** @returns {number} */
function readMaximumRustStandardCollectionTextUnits(){return 1048576;}
/** @param {RustStandardCollectionScalar} value @returns {RustStandardCollectionScalar} */
function copyRustStandardCollectionScalar(value){
 if(value.$==='rustCollectionString')return {$:'rustCollectionString',units:value.units.slice()};
 if(value.$==='rustCollectionBoolean')return {$:'rustCollectionBoolean',value:value.value};
 return {$:'rustCollectionInteger',value:value.value};
}
/** @param {RustStandardCollectionScalar} key @param {RustStandardCollectionScalar|undefined} value @returns {RustStandardCollectionEntry} */
function createRustStandardCollectionEntry(key,value){
 const copiedKey=copyRustStandardCollectionScalar(key);
 if(value===undefined)return {key:copiedKey};
 return {key:copiedKey,value:copyRustStandardCollectionScalar(value)};
}
/** @param {RustStandardCollectionEntry} entry @returns {RustStandardCollectionEntry} */
function retainRustStandardCollectionEntry(entry){return {...entry};}
/** @param {RustStandardCollectionEntry} entry @param {RustStandardCollectionScalar} value @returns {RustStandardCollectionEntry} */
function replaceRustStandardCollectionEntryValue(entry,value){return {...entry,value:copyRustStandardCollectionScalar(value)};}
/** @param {RustStandardCollectionEntry|undefined} entry @returns {RustStandardCollectionScalar|undefined} */
function readRustStandardCollectionEntryValue(entry){
 if(entry===undefined)return undefined;
 return entry.value;
}
/** @param {RustStandardCollectionEntry|undefined} entry @param {string} kind @returns {RustStandardCollectionScalar|undefined} */
function readRustStandardCollectionStoredScalar(entry,kind){
 if(entry===undefined)return undefined;
 if(kind==='map')return entry.value;
 return entry.key;
}
/** @param {boolean} value @returns {RustStandardCollectionValue} */
function createRustStandardCollectionBoolean(value){return {$:'rustCollectionBoolean',value};}
/** @param {RustStandardCollectionScalar} left @param {RustStandardCollectionScalar} right @returns {boolean} */
export function testRustStandardCollectionScalarEquality(left,right){
 if(left.$==='rustCollectionString'){
  if(right.$!=='rustCollectionString')return false;
  if(left.units.length!==right.units.length)return false;
  for(let index=0;index<left.units.length;index++)if(left.units[index]!==right.units[index])return false;
  return true;
 }
 if(left.$==='rustCollectionBoolean'){
  if(right.$==='rustCollectionBoolean')return left.value===right.value;
  return false;
 }
 if(left.$==='rustCollectionInteger'){
  if(right.$==='rustCollectionInteger')return left.value===right.value;
 }
 return false;
}
/** @param {RustStandardCollectionScalar|undefined} value @param {string} domain @param {number} pointerWidth @returns {string} */
export function readRustStandardCollectionScalarError(value,domain,pointerWidth){
 const error=readRustStandardCollectionDomainError(domain,pointerWidth);if(error!=='')return error;
 if(value===undefined||value===null){
  if(domain==='String')return 'collection String values retain bounded valid Unicode scalar text without lone surrogates';
  if(domain==='bool')return 'collection Boolean domain accepts only actual Boolean scalars';
  return 'collection integer domains accept only exact integer scalars';
 }
 return readRustStandardCollectionPresentScalarError(value,domain,pointerWidth);
}
/** @param {RustStandardCollectionScalar} value @param {string} domain @param {number} pointerWidth @returns {string} */
function readRustStandardCollectionPresentScalarError(value,domain,pointerWidth){
 if(domain==='String'){
  const invalid='collection String values retain bounded valid Unicode scalar text without lone surrogates';
  if(value.$!=='rustCollectionString')return invalid;
  return value.units.length<=readMaximumRustStandardCollectionTextUnits()&&value.units.every(unit=>Number.isInteger(unit)&&unit>=0&&unit<=65535)&&acceptRustOwnedLocalTextUnits(value.units)?'':invalid;
 }
 if(domain==='bool'){
  const invalid='collection Boolean domain accepts only actual Boolean scalars';
  if(value.$!=='rustCollectionBoolean')return invalid;
  return typeof value.value==='boolean'?'':invalid;
 }
 const invalid='collection integer domains accept only exact integer scalars';
 if(value.$!=='rustCollectionInteger')return invalid;
 if(typeof value.value!=='bigint')return invalid;
 const width=readRustStandardCollectionIntegerWidth(domain,pointerWidth),signed=domain[0]==='i',magnitude=1n<<BigInt(signed?width-1:width),minimum=signed?-magnitude:0n,maximum=magnitude-1n;
 return value.value>=minimum&&value.value<=maximum?'':'collection integer scalar is outside its actual declared source domain';
}
/** @param {RustStandardCollectionEntry[]} entries @param {RustStandardCollectionScalar} key @returns {number} */
function findRustStandardCollectionEntry(entries,key){for(let index=0;index<entries.length;index++)if(testRustStandardCollectionScalarEquality(entries[index].key,key))return index;return -1;}
/** @param {RustStandardCollectionScalar} scalar @returns {number} */
function readRustStandardCollectionScalarTextLength(scalar){
 return scalar.$==='rustCollectionString'?scalar.units.length:0;
}
/** @param {RustStandardCollectionState} state @returns {string} */
export function readRustStandardCollectionStateError(state){
 if(state?.$!=='rustStandardCollectionState'||typeof state.root!=='string'||state.root.length===0||!Number.isSafeInteger(state.revision)||state.revision<0||Object.is(state.revision,-0))return 'collection state retains its actual source root and exact nonnegative revision';
 const source=readRustStandardCollectionSourceOriginError(state.origin,state.keyDomain,state.valueDomain,state.pointerWidth);if(source!=='')return source;
 if(state.entries.length>4096)return 'collection state exceeds its explicit entry storage profile';
 const kind=readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.origin.resolvedOwner,true,false));let units=0;
 for(let index=0;index<state.entries.length;index++){
  const entry=state.entries[index];
  if(entry===undefined)return 'collection state retains an actual owned entry at each position';
  const keyError=readRustStandardCollectionScalarError(entry.key,state.keyDomain,state.pointerWidth);if(keyError!=='')return keyError;
  units+=readRustStandardCollectionScalarTextLength(entry.key);
  const value=entry.value;
  if(kind==='map'){
   if(value===undefined)return 'standard Map entries retain actual owned values';
   const valueError=readRustStandardCollectionScalarError(value,state.valueDomain,state.pointerWidth);if(valueError!=='')return valueError;
   units+=readRustStandardCollectionScalarTextLength(value);
  }
  else if(value!==undefined)return 'standard Set entries do not invent stored map values';
  if(testRustStandardCollectionOrdered(state.origin.resolvedOwner)&&index>0&&compareRustStandardCollectionScalars(state.entries[index-1].key,entry.key)>=0)return 'BTree collection entries retain strict standard primitive Ord including UTF8 String ordering';
  for(let previous=0;previous<index;previous++)if(testRustStandardCollectionScalarEquality(state.entries[previous].key,entry.key))return 'collection state retains one entry for each primitive Eq key';
 }
 return units<=readMaximumRustStandardCollectionTextUnits()?'':'collection state exceeds its explicit total String storage profile';
}
/** @param {RustStandardCollectionOrigin} origin @param {string} keyDomain @param {string} valueDomain @param {number} pointerWidth @param {string} root @returns {RustStandardCollectionState} */
function initializeRustStandardCollectionState(origin,keyDomain,valueDomain,pointerWidth,root){
 return {$:'rustStandardCollectionState',origin,root,keyDomain,valueDomain,pointerWidth,revision:0,entries:[]};
}
/** @param {RustStandardCollectionOrigin} origin @param {string} keyDomain @param {string} valueDomain @param {number} pointerWidth @param {string} root @param {RustStandardCollectionCallOrigin} call @returns {RustStandardCollectionTransition} */
export function createRustStandardCollectionState(origin,keyDomain,valueDomain,pointerWidth,root,call){
 const source=readRustStandardCollectionSourceOriginError(origin,keyDomain,valueDomain,pointerWidth);if(source!=='')return {$:'rustCollectionInvalid',reason:source};
 if(!['new','default'].includes(call?.method))return {$:'rustCollectionInvalid',reason:'collection creation retains its actual new or Default constructor dispatch'};
 const actual=readRustStandardCollectionCallOriginError(origin,root,call.method,0,call);if(actual!=='')return {$:'rustCollectionInvalid',reason:actual};
 const state=initializeRustStandardCollectionState(origin,keyDomain,valueDomain,pointerWidth,root),error=readRustStandardCollectionStateError(state);return error===''?{$:'rustCollectionCreated',state}:{$:'rustCollectionInvalid',reason:error};
}
/** @param {RustStandardCollectionOptionValue|undefined} value @returns {RustStandardCollectionValue} */
function createRustStandardCollectionOption(value){return value===undefined?{$:'rustCollectionOption',present:false}:{$:'rustCollectionOption',present:true,value};}
/** @param {RustStandardCollectionScalar} value @returns {RustStandardCollectionOptionValue} */
function copyRustStandardCollectionOptionScalar(value){
 if(value.$==='rustCollectionString')return {$:'rustCollectionString',units:value.units.slice()};
 if(value.$==='rustCollectionBoolean')return {$:'rustCollectionBoolean',value:value.value};
 return {$:'rustCollectionInteger',value:value.value};
}
/** @param {RustStandardCollectionScalar} value @returns {RustStandardCollectionValue} */
function copyRustStandardCollectionResultScalar(value){
 if(value.$==='rustCollectionString')return {$:'rustCollectionString',units:value.units.slice()};
 if(value.$==='rustCollectionBoolean')return {$:'rustCollectionBoolean',value:value.value};
 return {$:'rustCollectionInteger',value:value.value};
}
/** @param {RustStandardCollectionState} state @param {RustStandardCollectionEntry[]} entries @param {RustStandardCollectionValue} value @returns {RustStandardCollectionTransition} */
function retainRustStandardCollectionMutation(state,entries,value){
 if(state.revision>=Number.MAX_SAFE_INTEGER)return {$:'rustCollectionResourceLimit',state,reason:'collection mutation revision exceeds its explicit exact coordinate profile'};
 const next={...state,revision:state.revision+1,entries},error=readRustStandardCollectionStateError(next);
 if(error!=='')return {$:'rustCollectionResourceLimit',state,reason:error};
 return {$:'rustCollectionNormal',state:next,value,writeback:true};
}
/** @param {RustStandardCollectionState} state @param {RustStandardCollectionScalar} key @param {number} position
 * @param {boolean} ordered @param {RustStandardCollectionEntry} replacement @param {RustStandardCollectionValue} value
 * @returns {RustStandardCollectionTransition} */
function retainRustStandardCollectionInsertion(state,key,position,ordered,replacement,value){
 const present=position>=0;
 let insertion=position;
 if(!present){
  insertion=state.entries.length;
  if(ordered){
   insertion=0;
   for(const entry of state.entries){
    if(compareRustStandardCollectionScalars(entry.key,key)>=0)break;
    insertion++;
   }
  }
 }
 const entries=[];
 let index=0;
 for(const entry of state.entries){
  if(index===insertion)entries.push(replacement);
  if(!present||index!==insertion)entries.push(retainRustStandardCollectionEntry(entry));
  index++;
 }
 if(insertion===state.entries.length)entries.push(replacement);
 return retainRustStandardCollectionMutation(state,entries,value);
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
  const entries=state.entries.map(entry=>createRustStandardCollectionEntry(entry.key,entry.value));
  return {$:'rustCollectionNormal',state,value:{$:'rustStandardCollectionState',origin:state.origin,root:call.outputRoot??'',keyDomain:state.keyDomain,valueDomain:state.valueDomain,pointerWidth:state.pointerWidth,revision:0,entries},writeback:false};
 }
 if(method==='clear')return retainRustStandardCollectionMutation(state,[],{$:'rustCollectionUnit'});
 const key=arguments_[0];if(key===undefined)return {$:'rustCollectionInvalid',reason:'collection lookup retains its actual complete key operand'};
 const position=findRustStandardCollectionEntry(state.entries,key),present=position>=0;
 if(method==='contains'||method==='contains_key')return {$:'rustCollectionNormal',state,value:{$:'rustCollectionBoolean',value:present},writeback:false};
 if(method==='get')return {$:'rustCollectionNormal',state,value:createRustStandardCollectionOption(present?{$:'rustCollectionReference',root:state.root,revision:state.revision,key:copyRustStandardCollectionScalar(state.entries[position].key),domain:kind==='map'?state.valueDomain:state.keyDomain}:undefined),writeback:false};
 if(!testRustStandardCollectionMutation(owner,method))return {$:'rustCollectionInvalid',reason:'collection operation requires its actual registered closed semantics'};
 if(method==='insert'){
  const ordered=testRustStandardCollectionOrdered(owner);
  if(kind==='map'){
   const input=arguments_[1];if(input===undefined)return {$:'rustCollectionInvalid',reason:'standard Map insertion retains its actual owned value'};
   const existing=state.entries[position];
   const previous=readRustStandardCollectionEntryValue(existing);
   if(present&&previous===undefined)return {$:'rustCollectionInvalid',reason:'standard Map replacement retains its actual old owned value'};
   const value=createRustStandardCollectionOption(previous===undefined?undefined:copyRustStandardCollectionOptionScalar(previous));
   const replacement=existing===undefined?createRustStandardCollectionEntry(key,input):replaceRustStandardCollectionEntryValue(existing,input);
   return retainRustStandardCollectionInsertion(state,key,position,ordered,replacement,value);
  }
  const value=createRustStandardCollectionBoolean(!present);
  if(present)return retainRustStandardCollectionMutation(state,state.entries.map(entry=>retainRustStandardCollectionEntry(entry)),value);
  return retainRustStandardCollectionInsertion(state,key,position,ordered,createRustStandardCollectionEntry(key,undefined),value);
 }
 const previous=readRustStandardCollectionEntryValue(state.entries[position]);
 if(kind==='map'&&present&&previous===undefined)return {$:'rustCollectionInvalid',reason:'standard Map removal retains its actual old owned value'};
 const value=kind==='map'?createRustStandardCollectionOption(previous===undefined?undefined:copyRustStandardCollectionOptionScalar(previous)):createRustStandardCollectionBoolean(present);
 const entries=[];
 let index=0;
 for(const entry of state.entries){
  if(index!==position)entries.push(retainRustStandardCollectionEntry(entry));
  index++;
 }
 return retainRustStandardCollectionMutation(state,entries,value);
}
/** References retain the actual lexical root/revision. The parser must also
 * retain real borrow lifetimes; runtime revision checks never grant a loan.
 * @param {RustStandardCollectionState} state @param {RustStandardCollectionOptionValue} reference @param {string} operation @returns {RustStandardCollectionTransition} */
export function readRustStandardCollectionReference(state,reference,operation){
 const error=readRustStandardCollectionStateError(state);if(error!=='')return {$:'rustCollectionInvalid',reason:error};
 const kind=readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.origin.resolvedOwner,true,false)),domain=kind==='map'?state.valueDomain:state.keyDomain;
 const invalid='collection reference projection retains actual root revision domain and Copy or Clone operation';
 if(reference===undefined||reference===null)return {$:'rustCollectionInvalid',reason:invalid};
 if(reference.$!=='rustCollectionReference')return {$:'rustCollectionInvalid',reason:invalid};
 if(reference.root!==state.root||reference.revision!==state.revision||reference.domain!==domain||!['copied','cloned'].includes(operation)||operation==='copied'&&domain==='String')return {$:'rustCollectionInvalid',reason:invalid};
 const keyError=readRustStandardCollectionScalarError(reference.key,state.keyDomain,state.pointerWidth);if(keyError!=='')return {$:'rustCollectionInvalid',reason:keyError};
 const position=findRustStandardCollectionEntry(state.entries,reference.key);if(position<0)return {$:'rustCollectionInvalid',reason:'collection reference retains its actual live entry'};
 const value=readRustStandardCollectionStoredScalar(state.entries[position],kind);
 return value===undefined?{$:'rustCollectionInvalid',reason:'collection reference retains its actual stored value'}:{$:'rustCollectionNormal',state,value:copyRustStandardCollectionResultScalar(value),writeback:false};
}
/** @param {RustStandardCollectionState} state @param {RustStandardCollectionValue} option @param {string} operation @returns {RustStandardCollectionTransition} */
export function projectRustStandardCollectionOption(state,option,operation){
 const error=readRustStandardCollectionStateError(state);if(error!=='')return {$:'rustCollectionInvalid',reason:error};
 const kind=readRustStandardCollectionKind(resolveRustStandardCollectionOwner(state.origin.resolvedOwner,true,false)),domain=kind==='map'?state.valueDomain:state.keyDomain;
 const invalid='collection Option projection requires the actual standard borrowed Option Copy or Clone contract';
 if(option===undefined||option===null)return {$:'rustCollectionInvalid',reason:invalid};
 if(option.$!=='rustCollectionOption')return {$:'rustCollectionInvalid',reason:invalid};
 if(typeof option.present!=='boolean'||!['copied','cloned'].includes(operation)||operation==='copied'&&domain==='String')return {$:'rustCollectionInvalid',reason:invalid};
 if(!option.present)return option.value===undefined?{$:'rustCollectionNormal',state,value:createRustStandardCollectionOption(undefined),writeback:false}:{$:'rustCollectionInvalid',reason:'None retains no invented payload'};
 const payload=option.value,invalidPayload='copied and cloned require actual borrowed Option payloads';
 if(payload===undefined||payload===null)return {$:'rustCollectionInvalid',reason:invalidPayload};
 if(payload.$!=='rustCollectionReference')return {$:'rustCollectionInvalid',reason:invalidPayload};
 const read=readRustStandardCollectionReference(state,payload,operation);
 if(read.$!=='rustCollectionNormal')return read;
 const value=read.value;
 if(value.$==='rustCollectionString')return {$:'rustCollectionNormal',state:read.state,value:createRustStandardCollectionOption({$:'rustCollectionString',units:value.units}),writeback:read.writeback};
 if(value.$==='rustCollectionBoolean')return {$:'rustCollectionNormal',state:read.state,value:createRustStandardCollectionOption({$:'rustCollectionBoolean',value:value.value}),writeback:read.writeback};
 if(value.$==='rustCollectionInteger')return {$:'rustCollectionNormal',state:read.state,value:createRustStandardCollectionOption({$:'rustCollectionInteger',value:value.value}),writeback:read.writeback};
 return read;
}
