// Deferred closed-domain controls. Do not execute before complete source drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {resolveRustStandardCollectionOwner,readRustStandardCollectionSourceOriginError,readRustStandardCollectionMethodArity,readRustStandardCollectionMethodResult,readRustStandardCollectionDomainError,planRustStandardCollectionStatePacket} from '../src/translation/rust-standard-collection-rules.js';
import {createRustStandardCollectionState,readRustStandardCollectionStateError,executeRustStandardCollectionOperation,readRustStandardCollectionReference,projectRustStandardCollectionOption,testRustStandardCollectionScalarEquality} from '../src/translation/rust-standard-collection-state.js';
import {planRustOwnedRecordStatePacket} from '../src/translation/rust-owned-record-state-rules.js';
import {rustStandardCollectionOriginals,rustStandardCollectionRefusals} from './fixtures/rust-standard-collections.mjs';
const integer=value=>({$:'rustCollectionInteger',value:BigInt(value)}),boolean=value=>({$:'rustCollectionBoolean',value}),text=value=>({$:'rustCollectionString',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
const owner=domain=>({kind:domain==='String'?'library':'primitive',path:domain==='String'?['std','string','String']:[domain],arguments:[]});
function origin(kind='map',keyDomain='String',valueDomain='i32',pointerWidth=64){
 const resolvedOwner='std::collections::'+(kind==='map'?'HashMap':'HashSet'),domains=kind==='map'?[keyDomain,valueDomain]:[keyDomain];
 return {$:'rustStandardCollectionOrigin',ordinary:true,proven:true,shadowed:false,typeIdentity:resolvedOwner+'<'+domains.join(',')+'>',resolvedOwner,sourceOwner:{kind:'library',path:resolvedOwner.split('::'),arguments:domains.map(owner)},arguments:domains.map(domain=>({proven:true,wrappers:[],sourceOwner:owner(domain),...(domain==='String'?{textSourceModel:'owned string'}:{})})),sourceVersionMinor:90,pointerWidth,customHasher:false,customEquality:false};
}
function call(state,method,domains=[],changes={}){
 const constructor=['new','default'].includes(method),borrowed=method!=='insert',canonical=resolveRustStandardCollectionOwner(state.origin.resolvedOwner,true,false);
 return {$:'rustStandardCollectionCallOrigin',owner:canonical,typeIdentity:state.origin.typeIdentity,receiverRoot:state.root,method,dispatchOwner:method==='clone'?'core::clone::Clone':method==='default'?'core::default::Default':canonical,proven:true,shadowed:false,mutable:true,available:!constructor,overlappingLoan:false,arguments:domains.map(domain=>({domain,proven:true,available:true,borrowed,ownedTransfer:!borrowed&&domain==='String',sourceOwner:owner(domain)})),...(method==='clone'?{outputRoot:state.root+' clone'}:{}),...changes};
}
function setup(kind='map',keyDomain='String',valueDomain='i32',pointerWidth=64){const actual=origin(kind,keyDomain,valueDomain,pointerWidth),request={origin:actual,root:'actual lexical collection root'},result=createRustStandardCollectionState(actual,keyDomain,kind==='map'?valueDomain:'unit',pointerWidth,request.root,call(request,'new'));assert.equal(result.$,'rustCollectionCreated',result.reason);return result.state;}
function apply(state,method,args=[],changes={}){const domains=args.map((_,index)=>index===0?state.keyDomain:state.valueDomain),result=executeRustStandardCollectionOperation(state,method,args,call(state,method,domains,changes));assert.equal(result.$,'rustCollectionNormal',result.reason);return result;}
function copied(state,key){const result=apply(state,'get',[key]),projected=projectRustStandardCollectionOption(state,result.value,'copied');assert.equal(projected.$,'rustCollectionNormal',projected.reason);return projected.value;}

test('only actual unshadowed std collection owner paths are eligible',()=>{
 for(const path of ['std::collections::HashMap','std::collections::hash_map::HashMap'])assert.equal(resolveRustStandardCollectionOwner(path,true,false),'std::collections::HashMap');
 for(const path of ['std::collections::HashSet','std::collections::hash_set::HashSet'])assert.equal(resolveRustStandardCollectionOwner(path,true,false),'std::collections::HashSet');
 for(const path of ['HashMap','core::collections::HashMap','alloc::collections::HashMap','local::HashMap'])assert.equal(resolveRustStandardCollectionOwner(path,true,false),'');
 // The former BTreeMap refusal input is preserved as a positive owner: its
 // distinct closed Ord kernel and source integration are now authored.
 assert.equal(resolveRustStandardCollectionOwner('std::collections::BTreeMap',true,false),'std::collections::BTreeMap');
 assert.equal(resolveRustStandardCollectionOwner('std::collections::HashMap',true,true),'');assert.equal(resolveRustStandardCollectionOwner('std::collections::HashMap',false,false),'');
 const actual=origin();for(const changed of [{...actual,proven:false},{...actual,shadowed:true},{...actual,typeIdentity:''},{...actual,customHasher:true},{...actual,customEquality:true},{...actual,sourceVersionMinor:100},{...actual,sourceOwner:{...actual.sourceOwner,kind:'declaration'}},{...actual,arguments:[{...actual.arguments[0],sourceOwner:{...owner('String'),kind:'declaration'}},actual.arguments[1]]}])assert.notEqual(readRustStandardCollectionSourceOriginError(changed,'String','i32',64),'');
});
test('closed domain and method inventories reject float hashing iteration and guessed aliases',()=>{
 for(const domain of ['String','bool','i8','i16','i32','i64','i128','isize','u8','u16','u32','u64','u128','usize'])assert.equal(readRustStandardCollectionDomainError(domain,64),'');for(const domain of ['f32','f64','char','UserKey','Vec'])assert.notEqual(readRustStandardCollectionDomainError(domain,64),'');assert.notEqual(readRustStandardCollectionDomainError('usize',16),'');
 for(const method of ['iter','keys','values','drain','entry','capacity','reserve','with_hasher','take'])assert.equal(readRustStandardCollectionMethodArity('std::collections::HashMap',method),-1);
 assert.equal(readRustStandardCollectionMethodArity('std::collections::HashMap','contains'),-1);assert.equal(readRustStandardCollectionMethodArity('std::collections::HashSet','contains_key'),-1);assert.equal(readRustStandardCollectionMethodResult('std::collections::HashMap','get'),'borrowed Option');assert.equal(readRustStandardCollectionMethodResult('std::collections::HashMap','remove'),'owned Option');assert.equal(readRustStandardCollectionMethodResult('std::collections::HashSet','remove'),'bool');
});
test('new and actual Default create empty owned state with independent roots',()=>{
 const state=setup();assert.equal(apply(state,'len').value.value,0n);assert.equal(apply(state,'is_empty').value.value,true);const actual=origin(),request={origin:actual,root:'default root'},created=createRustStandardCollectionState(actual,'String','i32',64,request.root,call(request,'default'));assert.equal(created.$,'rustCollectionCreated');assert.equal(created.state.root,'default root');assert.equal(created.state.entries.length,0);const invalid=createRustStandardCollectionState(actual,'String','i32',64,request.root,call(request,'default',[],{dispatchOwner:'user::Default'}));assert.equal(invalid.$,'rustCollectionInvalid');
});
test('HashMap insert replacement removal and absent queries preserve complete state and old Option values',()=>{
 let state=setup();const first=apply(state,'insert',[text('same'),integer(3)]);assert.deepEqual(first.value,{$:'rustCollectionOption',present:false});assert.equal(first.writeback,true);assert.equal(state.entries.length,0);state=first.state;
 const next=apply(state,'insert',[text('same'),integer(7)]);assert.deepEqual(next.value,{$:'rustCollectionOption',present:true,value:integer(3)});state=next.state;assert.equal(apply(state,'len').value.value,1n);assert.equal(apply(state,'contains_key',[text('same')]).value.value,true);assert.deepEqual(copied(state,text('same')),{$:'rustCollectionOption',present:true,value:integer(7)});
 const missing=apply(state,'remove',[text('absent')]);assert.deepEqual(missing.value,{$:'rustCollectionOption',present:false});assert.equal(missing.state.entries.length,1);const removed=apply(missing.state,'remove',[text('same')]);assert.deepEqual(removed.value,{$:'rustCollectionOption',present:true,value:integer(7)});assert.equal(removed.state.entries.length,0);assert.deepEqual(copied(removed.state,text('same')),{$:'rustCollectionOption',present:false});
});
test('String keys use exact valid Unicode equality without normalization',()=>{
 let state=setup();state=apply(state,'insert',[text('\u00e9'),integer(1)]).state;state=apply(state,'insert',[text('e\u0301'),integer(2)]).state;state=apply(state,'insert',[text('\ud83d\ude00'),integer(3)]).state;state=apply(state,'insert',[text('a\0b'),integer(4)]).state;assert.equal(state.entries.length,4);assert.equal(copied(state,text('\u00e9')).value.value,1n);assert.equal(copied(state,text('e\u0301')).value.value,2n);assert.equal(copied(state,text('\ud83d\ude00')).value.value,3n);assert.equal(copied(state,text('a\0b')).value.value,4n);assert.equal(testRustStandardCollectionScalarEquality(text('\u00e9'),text('e\u0301')),false);
 const invalid=executeRustStandardCollectionOperation(state,'insert',[text('\ud800'),integer(0)],call(state,'insert',['String','i32']));assert.equal(invalid.$,'rustCollectionInvalid');
});
test('Boolean and exact large integer keys retain distinct primitive Eq values',()=>{
 let bools=setup('map','bool','i32');bools=apply(bools,'insert',[boolean(false),integer(3)]).state;bools=apply(bools,'insert',[boolean(true),integer(7)]).state;assert.equal(copied(bools,boolean(false)).value.value,3n);assert.equal(copied(bools,boolean(true)).value.value,7n);
 let integers=setup('map','u128','String');integers=apply(integers,'insert',[integer(9007199254740992n),text('first')]).state;integers=apply(integers,'insert',[integer(9007199254740993n),text('second')]).state;assert.equal(integers.entries.length,2);assert.equal(apply(integers,'contains_key',[integer(9007199254740993n)]).value.value,true);
});
test('every integer domain enforces exact signed unsigned and active pointer bounds',()=>{
 for(const [domain,width] of [['i8',8],['i16',16],['i32',32],['i64',64],['i128',128],['isize',32],['u8',8],['u16',16],['u32',32],['u64',64],['u128',128],['usize',32]]){
  let state=setup('set',domain,'unit',32);const signed=domain[0]==='i',bound=1n<<BigInt(signed?width-1:width),minimum=signed?-bound:0n,maximum=bound-1n;state=apply(state,'insert',[integer(minimum)]).state;state=apply(state,'insert',[integer(maximum)]).state;
  for(const input of [minimum-1n,maximum+1n])assert.equal(executeRustStandardCollectionOperation(state,'insert',[integer(input)],call(state,'insert',[domain])).$,'rustCollectionInvalid');
 }
});
test('HashSet duplicate insert and remove use booleans and never map Option returns',()=>{
 let state=setup('set','String','unit');const first=apply(state,'insert',[text('x')]);assert.deepEqual(first.value,boolean(true));state=first.state;const repeated=apply(state,'insert',[text('x')]);assert.deepEqual(repeated.value,boolean(false));state=repeated.state;assert.equal(apply(state,'len').value.value,1n);assert.equal(apply(state,'contains',[text('x')]).value.value,true);assert.deepEqual(apply(state,'remove',[text('absent')]).value,boolean(false));const removed=apply(state,'remove',[text('x')]);assert.deepEqual(removed.value,boolean(true));assert.equal(removed.state.entries.length,0);
});
test('get preserves its actual borrow and projects Copy and owned Clone explicitly',()=>{
 let state=setup('map','String','String');state=apply(state,'insert',[text('key'),text('owned')]).state;const queried=apply(state,'get',[text('key')]),reference=queried.value.value;assert.equal(reference.$,'rustCollectionReference');assert.equal(reference.root,state.root);assert.equal(readRustStandardCollectionReference(state,reference,'copied').$,'rustCollectionInvalid');const cloned=projectRustStandardCollectionOption(state,queried.value,'cloned');assert.deepEqual(cloned.value,{$:'rustCollectionOption',present:true,value:text('owned')});assert.notEqual(cloned.value.value.units,state.entries[0].value.units);assert.equal(projectRustStandardCollectionOption(state,{$:'rustCollectionOption',present:false},'copied').$,'rustCollectionInvalid');
 const changed=apply(state,'insert',[text('other'),text('next')]).state;assert.equal(readRustStandardCollectionReference(changed,reference,'cloned').$,'rustCollectionInvalid');assert.equal(readRustStandardCollectionReference(state,{...reference,root:'wrong root'},'cloned').$,'rustCollectionInvalid');
});
test('standard Borrow str query compares String contents without creating owned transfers',()=>{
 let state=setup();state=apply(state,'insert',[text('borrow'),integer(7)]).state;const facts=call(state,'contains_key',['str']),result=executeRustStandardCollectionOperation(state,'contains_key',[text('borrow')],facts);assert.equal(result.$,'rustCollectionNormal');assert.deepEqual(result.value,boolean(true));assert.equal(facts.arguments[0].ownedTransfer,false);
});
test('clone deep copies owned String entries and never writes its source packet slot',()=>{
 let state=setup('map','String','String');state=apply(state,'insert',[text('key'),text('old')]).state;const clone=apply(state,'clone');assert.equal(clone.writeback,false);assert.equal(clone.state,state);const cloned=clone.value;assert.equal(cloned.root,state.root+' clone');assert.notEqual(cloned.entries[0].key.units,state.entries[0].key.units);assert.notEqual(cloned.entries[0].value.units,state.entries[0].value.units);const changed=apply(cloned,'insert',[text('key'),text('new')]).state;assert.deepEqual(state.entries[0].value,text('old'));assert.deepEqual(changed.entries[0].value,text('new'));
 assert.equal(executeRustStandardCollectionOperation(state,'clone',[],call(state,'clone',[],{outputRoot:state.root})).$,'rustCollectionInvalid');
});
test('clear publishes an empty next state without mutating the previous snapshot',()=>{
 let state=setup('set','bool','unit');state=apply(state,'insert',[boolean(true)]).state;const cleared=apply(state,'clear');assert.equal(cleared.writeback,true);assert.deepEqual(cleared.value,{$:'rustCollectionUnit'});assert.equal(cleared.state.entries.length,0);assert.equal(state.entries.length,1);assert.equal(cleared.state.revision,state.revision+1);
});
test('actual call owner type mutable root loan and owned transfer evidence cannot be substituted',()=>{
 const state=setup(),args=[text('key'),integer(1)],facts=call(state,'insert',['String','i32']);for(const changes of [{owner:'user::HashMap'},{typeIdentity:'different concrete type'},{receiverRoot:'other'},{proven:false},{shadowed:true},{available:false},{mutable:false},{overlappingLoan:true},{dispatchOwner:'local::insert'},{arguments:[{...facts.arguments[0],ownedTransfer:false},facts.arguments[1]]},{arguments:[{...facts.arguments[0],borrowed:true},facts.arguments[1]]}])assert.equal(executeRustStandardCollectionOperation(state,'insert',args,{...facts,...changes}).$,'rustCollectionInvalid');assert.equal(executeRustStandardCollectionOperation(state,'iter',[],call(state,'iter')).$,'rustCollectionInvalid');
});
test('entry and revision resource refusals retain original complete state for retry',()=>{
 let state=setup('set','u32','unit');state={...state,entries:Array.from({length:4096},(_,index)=>({key:integer(index)}))};assert.equal(readRustStandardCollectionStateError(state),'');const refused=executeRustStandardCollectionOperation(state,'insert',[integer(4096)],call(state,'insert',['u32']));assert.equal(refused.$,'rustCollectionResourceLimit');assert.equal(refused.state,state);assert.equal(state.entries.length,4096);const full={...setup('set','bool','unit'),revision:Number.MAX_SAFE_INTEGER},limited=executeRustStandardCollectionOperation(full,'insert',[boolean(true)],call(full,'insert',['bool']));assert.equal(limited.$,'rustCollectionResourceLimit');assert.equal(limited.state,full);
});
test('state packet positions reuse the actual owned-record result and writeback recipe',()=>{
 for(const count of [0,1,2,16,256])assert.deepEqual(planRustStandardCollectionStatePacket(count),planRustOwnedRecordStatePacket(count));for(const count of [-1,257,1.5])assert.deepEqual(planRustStandardCollectionStatePacket(count),[]);
});
test('independent originals and refusal fixtures retain explicit Rust source version bounds',()=>{
 assert.ok(rustStandardCollectionOriginals.length>=20);for(const fixture of rustStandardCollectionOriginals){assert.equal(fixture.sourceVersionRange,'>=1.90.0 <1.100.0');assert.equal(typeof fixture.source,'string');assert.equal(typeof fixture.stdout,'string');}assert.ok(rustStandardCollectionRefusals.length>=6);for(const fixture of rustStandardCollectionRefusals)assert.equal(typeof fixture.source,'string');
});
