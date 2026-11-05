// Deferred executable packet controls; no projected library graph is invented.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createRustStandardCollectionState} from '../src/translation/rust-standard-collection-state.js';
import {executeRustStandardCollectionValuePacket,resumeRustStandardCollectionSourcePacket,readRustStandardCollectionProjectedLibraryError,lowerRustStandardCollectionProjectedSourcePacket,testRustStandardCollectionProjectedGraphEquality} from '../src/translation/rust-standard-collection-call-adapter.js';
const integer=value=>({$:'rustCollectionInteger',value:BigInt(value)}),owner=domain=>({kind:'primitive',path:[domain],arguments:[]});
const origin={$:'rustStandardCollectionOrigin',ordinary:true,proven:true,shadowed:false,typeIdentity:'actual std HashMap u8 i32 concrete source type',resolvedOwner:'std::collections::HashMap',sourceOwner:{kind:'library',path:['std','collections','HashMap'],arguments:[owner('u8'),owner('i32')]},arguments:['u8','i32'].map(domain=>({proven:true,wrappers:[],sourceOwner:owner(domain)})),sourceVersionMinor:90,pointerWidth:64,customHasher:false,customEquality:false};
function call(root,method,domains=[],changes={}){return {$:'rustStandardCollectionCallOrigin',owner:'std::collections::HashMap',typeIdentity:origin.typeIdentity,receiverRoot:root,method,dispatchOwner:method==='clone'?'core::clone::Clone':'std::collections::HashMap',proven:true,shadowed:false,mutable:true,available:true,overlappingLoan:false,arguments:domains.map(domain=>({domain,proven:true,available:true,borrowed:method!=='insert',ownedTransfer:false,sourceOwner:owner(domain)})),...changes};}
function setup(){const result=createRustStandardCollectionState(origin,'u8','i32',64,'actual root',call('actual root','new'));assert.equal(result.$,'rustCollectionCreated');return result.state;}
function packet(state,method,args=[],changes={},projection=''){return executeRustStandardCollectionValuePacket(state,method,args,call(state.root,method,args.map((_,index)=>index===0?'u8':'i32'),changes),projection);}

test('actual kernel insertion packet retains old Option result zero and updated whole state one',()=>{
 const state=setup(),first=packet(state,'insert',[integer(1),integer(7)]);assert.equal(first.$,'rustCollectionPacketNormal');assert.equal(first.writeback,true);assert.deepEqual(first.values[0],{$:'rustCollectionOption',present:false});assert.equal(first.values[1].entries.length,1);assert.equal(state.entries.length,0);const replaced=packet(first.values[1],'insert',[integer(1),integer(9)]);assert.deepEqual(replaced.values[0],{$:'rustCollectionOption',present:true,value:integer(7)});assert.equal(replaced.values[1].entries[0].value.value,9n);
});
test('caller writeback publishes latest root before continuation evaluates the result',()=>{
 const state=setup(),next=packet(state,'insert',[integer(1),integer(7)]),roots=[{binder:1,collection:state}];let entered=false;
 const result=resumeRustStandardCollectionSourcePacket(next,roots,1,(value,published)=>{entered=true;assert.equal(published[0].collection.entries[0].value.value,7n);assert.deepEqual(value,{$:'rustCollectionOption',present:false});return published[0].collection;});assert.equal(entered,true);assert.equal(result,next.values[1]);assert.equal(roots[0].collection.entries.length,0);
});
test('get packet immediately copies the actual borrowed result and never exposes its reference',()=>{
 const inserted=packet(setup(),'insert',[integer(1),integer(7)]),state=inserted.values[1],read=packet(state,'get',[integer(1)],{},'copied');assert.equal(read.$,'rustCollectionPacketNormal');assert.equal(read.writeback,false);assert.deepEqual(read.values[0],{$:'rustCollectionOption',present:true,value:integer(7)});assert.equal(read.values[1],state);assert.equal(packet(state,'get',[integer(1)]).$,'rustCollectionInvalid');assert.equal(packet(state,'len',[],{},'copied').$,'rustCollectionInvalid');
});
test('clone packet returns independent owned value while retaining source state slot',()=>{
 const source=packet(setup(),'insert',[integer(1),integer(7)]).values[1],cloned=packet(source,'clone',[],{outputRoot:'actual fresh clone root'});assert.equal(cloned.$,'rustCollectionPacketNormal');assert.equal(cloned.writeback,false);assert.equal(cloned.values[0].root,'actual fresh clone root');assert.equal(cloned.values[1],source);const roots=[{binder:1,collection:source}],published=resumeRustStandardCollectionSourcePacket(cloned,roots,1,(value,updated)=>({value,updated}));assert.equal(published.updated[0],roots[0]);assert.equal(published.value,cloned.values[0]);
});
test('invalid ownership resource refusal and mismatched roots never run the continuation',()=>{
 const state=setup();assert.equal(packet(state,'insert',[integer(1),integer(7)],{mutable:false}).$,'rustCollectionInvalid');const limited=packet({...state,revision:Number.MAX_SAFE_INTEGER},'insert',[integer(1),integer(7)]);assert.equal(limited.$,'rustCollectionResourceLimit');assert.equal(limited.state.revision,Number.MAX_SAFE_INTEGER);let entered=false;const invalid=resumeRustStandardCollectionSourcePacket({$:'rustCollectionPacketNormal',values:[integer(1),{...state,root:'substituted'}],writeback:true},[{binder:1,collection:state}],1,()=>{entered=true;});assert.equal(invalid.$,'rustCollectionInvalid');assert.equal(entered,false);
});
test('projected source seam refuses body promises and metadata-only graph descriptors',()=>{
 for(const library of [undefined,{}, {functions:[{role:'packet',checkedName:'executeRustStandardCollectionValuePacket',bodyAvailable:true}]},{program:{declarations:new Map()},sourceProgram:{items:[]},functions:[]}])assert.notEqual(readRustStandardCollectionProjectedLibraryError(library),'');const result=lowerRustStandardCollectionProjectedSourcePacket({}, {}, {}, [], [], {}, ()=>{throw new Error('not reached');});assert.equal(result.$,'rustCollectionSourceLoweringInvalid');
});
test('complete graph correspondence checks real bodies Map contents exact scalars and aliases',()=>{
 const body={k:'lit',value:9007199254740993n,type:{kind:'int'}},declaration={k:'fn',name:'actual',params:[],ret:{kind:'int'},body},actual={items:[declaration],declarations:new Map([['actual',declaration]])},otherBody={k:'lit',value:9007199254740993n,type:{kind:'int'}},otherDeclaration={k:'fn',name:'actual',params:[],ret:{kind:'int'},body:otherBody},expected={items:[otherDeclaration],declarations:new Map([['actual',otherDeclaration]])};
 assert.equal(testRustStandardCollectionProjectedGraphEquality(actual,expected),true);
 const changed={...otherDeclaration,body:{...otherBody,value:9007199254740992n}};assert.equal(testRustStandardCollectionProjectedGraphEquality(actual,{items:[changed],declarations:new Map([['actual',changed]])}),false);
 assert.equal(testRustStandardCollectionProjectedGraphEquality(actual,{items:[otherDeclaration],declarations:new Map([['actual',{...otherDeclaration}]])}),false);
 assert.equal(testRustStandardCollectionProjectedGraphEquality({value:-0},{value:0}),false);assert.equal(testRustStandardCollectionProjectedGraphEquality({value:NaN},{value:NaN}),true);
 const map=new Map([['key',1]]);map.extra='unrepresented';assert.equal(testRustStandardCollectionProjectedGraphEquality(map,new Map([['key',1]])),false);
 assert.equal(testRustStandardCollectionProjectedGraphEquality(Object.freeze({value:7}),{value:7}),true);let reads=0;const accessor={get value(){reads++;return 7;}};assert.equal(testRustStandardCollectionProjectedGraphEquality(accessor,{value:7}),false);assert.equal(reads,0);
});
