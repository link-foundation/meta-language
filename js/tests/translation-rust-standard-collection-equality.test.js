// Deferred controls: this file is authored during bulk source drafting only.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {beginRustStandardCollectionEquality,stepRustStandardCollectionEquality,runRustStandardCollectionEquality,executeRustStandardCollectionEquality,readRustStandardCollectionEqualityStateError} from '../src/translation/rust-standard-collection-equality.js';
import {compareRustStandardCollectionValuePackets} from '../src/translation/rust-standard-collection-value-packet.js';
import {readRustStandardCollectionStateError} from '../src/translation/rust-standard-collection-state.js';
import {parseRust} from '../src/translation/rust.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {createRustStandardCollectionSourceLibrary} from '../src/translation/rust-standard-collection-library.js';
import {readSourceLibraryClosure,readSourceLibraryDeclarationOwnership} from '../scripts/read-source-library-closure.mjs';
import {encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {rustStandardCollectionEqualityOriginals,rustStandardCollectionEqualityRefusals} from './fixtures/rust-standard-collection-equality.mjs';
const integer=value=>({$:'rustCollectionInteger',value:BigInt(value)}),boolean=value=>({$:'rustCollectionBoolean',value}),text=value=>({$:'rustCollectionString',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
function state(kind,keyDomain,valueDomain,entries,root='left',revision=0){
 const resolvedOwner='std::collections::'+(kind==='map'?'HashMap':'HashSet'),domains=kind==='map'?[keyDomain,valueDomain]:[keyDomain],owner=domain=>({kind:domain==='String'?'library':'primitive',path:domain==='String'?['std','string','String']:[domain],arguments:[]}),origin={$:'rustStandardCollectionOrigin',ordinary:true,proven:true,shadowed:false,typeIdentity:resolvedOwner+'<'+domains.join(',')+'>@64',resolvedOwner,sourceOwner:{kind:'library',path:resolvedOwner.split('::'),arguments:domains.map(owner)},arguments:domains.map(domain=>({proven:true,wrappers:[],sourceOwner:owner(domain),...(domain==='String'?{textSourceModel:'owned string'}:{})})),sourceVersionMinor:90,pointerWidth:64,customHasher:false,customEquality:false};
 const result={$:'rustStandardCollectionState',origin,root,keyDomain,valueDomain:kind==='map'?valueDomain:'unit',pointerWidth:64,revision,entries};assert.equal(readRustStandardCollectionStateError(result),'');return result;
}
const evidence=(left,operation='equal')=>({$:'rustStandardCollectionEqualityOrigin',owner:'core::cmp::PartialEq',typeIdentity:left.origin.typeIdentity,operation,sourceVersionMinor:90,proven:true,leftAvailable:true,rightAvailable:true,customImplementation:false});
function equal(left,right,operation='equal',budget=16777216){const result=executeRustStandardCollectionEquality(left,right,evidence(left,operation),budget);assert.equal(result.$,'rustCollectionEqualityCompleted',result.reason);return result.value;}
test('map equality ignores reverse insertion order allocation root and revision',()=>{
 const first=state('map','u8','bool',[{key:integer(1),value:boolean(true)},{key:integer(2),value:boolean(false)}]),second=state('map','u8','bool',[{key:integer(2),value:boolean(false)},{key:integer(1),value:boolean(true)}],'right',91);
 assert.equal(equal(first,second),true);assert.equal(equal(second,first),true);assert.equal(equal(first,second,'not equal'),false);assert.equal(compareRustStandardCollectionValuePackets(first,second,evidence(first),100).value,true);
});
test('sets compare complete membership rather than entry vectors or sizes',()=>{
 const first=state('set','i128','unit',[{key:integer(-9007199254740993n)},{key:integer(7)}]),second=state('set','i128','unit',[{key:integer(7)},{key:integer(-9007199254740993n)}]);assert.equal(equal(first,second),true);
 const changed=state('set','i128','unit',[{key:integer(7)},{key:integer(-9007199254740992n)}]);assert.equal(equal(first,changed),false);assert.equal(equal(first,changed,'not equal'),true);
});
test('matching keys still compare exact owned map values',()=>{
 const first=state('map','bool','String',[{key:boolean(false),value:text('a\0😀')}]),same=state('map','bool','String',[{key:boolean(false),value:text('a\0😀')}]),changed=state('map','bool','String',[{key:boolean(false),value:text('a\0😁')}]);assert.equal(equal(first,same),true);assert.equal(equal(first,changed),false);
 const composed=state('set','String','unit',[{key:text('é')}]),decomposed=state('set','String','unit',[{key:text('e\u0301')}]);assert.equal(equal(composed,decomposed),false);
});
test('empty and different-sized operands need zero scalar work',()=>{
 const empty=state('set','u8','unit',[]),nonempty=state('set','u8','unit',[{key:integer(1)}]);assert.equal(equal(empty,empty,'equal',0),true);assert.equal(equal(empty,nonempty,'equal',0),false);assert.equal(equal(nonempty,empty,'not equal',0),true);
});
test('resource transition retains complete state and resumes at the exact comparison',()=>{
 const first=state('map','String','String',[{key:text('key'),value:text('value')}]),second=state('map','String','String',[{key:text('key'),value:text('value')}]),started=beginRustStandardCollectionEquality(first,second,evidence(first));assert.equal(started.$,'rustCollectionEqualityContinue');
 const stopped=runRustStandardCollectionEquality(started.state,6);assert.equal(stopped.$,'rustCollectionEqualityResourceLimit');assert.equal(stopped.state.phase,'value');assert.equal(stopped.state.work,6);assert.equal(stopped.state.left,first);assert.equal(stopped.state.right,second);assert.equal(readRustStandardCollectionEqualityStateError(stopped.state),'');assert.deepEqual(runRustStandardCollectionEquality(stopped.state,10),{$:'rustCollectionEqualityCompleted',value:true});
 const single=stepRustStandardCollectionEquality(started.state);assert.equal(single.state.phase,'value');assert.equal(single.state.work,6);
});
test('forged continuation cannot skip mismatched values matching keys or charged String work',()=>{
 const first=state('map','u8','bool',[{key:integer(1),value:boolean(true)}]),second=state('map','u8','bool',[{key:integer(1),value:boolean(false)}]),started=beginRustStandardCollectionEquality(first,second,evidence(first)).state;
 for(const changed of [{...started,leftIndex:1,work:2},{...started,rightIndex:1,work:1},{...started,work:1},{...started,phase:'value',work:0},{...started,leftIndex:-0}])assert.notEqual(readRustStandardCollectionEqualityStateError(changed),'');
});
test('actual domains owners availability custom implementations and source bounds remain checked',()=>{
 const first=state('set','u8','unit',[]),different=state('set','u16','unit',[]);assert.equal(executeRustStandardCollectionEquality(first,different,evidence(first)).$,'rustCollectionEqualityInvalid');
 for(const changed of [{proven:false},{leftAvailable:false},{rightAvailable:false},{customImplementation:true},{owner:'user::PartialEq'},{typeIdentity:'different'},{sourceVersionMinor:89},{sourceVersionMinor:100},{operation:'less'}])assert.equal(executeRustStandardCollectionEquality(first,first,{...evidence(first),...changed}).$,'rustCollectionEqualityInvalid');
 assert.equal(executeRustStandardCollectionEquality(first,first,{...evidence(first),sourceVersionMinor:99}).value,true);
 for(const budget of [-1,-0,NaN,Infinity,67108865])assert.equal(executeRustStandardCollectionEquality(first,first,evidence(first),budget).$,'rustCollectionEqualityInvalid');
});
function actualLibrary(){
 const entryPath='js/src/translation/rust-standard-collection-value-packet.js',closure=readSourceLibraryClosure(entryPath,{aliases:true}),program=checkProgram(parseJavaScript(closure.source)),linked=encodeTypedProgramLinks(program),body=linked.network.toLino(),receipt={model:'Complete source library asset receipt 1',schemaVersion:1,entryPath,referenceSemantics:false,sourceClosure:closure,sourceDeclarationOwners:readSourceLibraryDeclarationOwnership(closure,program),programAsset:{path:'js/src/data/rust-standard-collection-value-packet-program.lino',bytes:Buffer.byteLength(body),sha256:createHash('sha256').update(body).digest('hex')}};
 // Separate non-returning test continuations preserve refusal/resource stops;
 // neither can certify a successful collection comparison.
 return createRustStandardCollectionSourceLibrary(body,receipt,{invalid:()=>({k:'abort',message:'invalid collection source packet'}),resource:()=>({k:'abort',message:'collection resource budget exhausted'})});
}
test('public opt-in parser lowers genuine equality originals through actual projected shared bodies',()=>{
 const library=actualLibrary(),options={standardCollectionSourceModel:'Rust closed standard collections 1',standardCollectionSourceLibrary:library,standardCollectionPointerWidth:64,sourceVersion:'1.90.0',exclusiveSourceModel:'Rust exclusive source loans 1'};
 for(const fixture of rustStandardCollectionEqualityOriginals){const program=parseRust(fixture.source,options);checkProgram(program);assert.ok(program.items.some(item=>item.sourceStandardCollectionLibraryOwner?.role==='equality'),fixture.name);assert.ok(program.items.some(item=>item.sourceStandardCollectionCarrier),fixture.name);}
 for(const fixture of rustStandardCollectionEqualityRefusals)assert.throws(()=>parseRust(fixture.source,options),undefined,fixture.name);
 assert.equal(library.program.declarations.get(library.functions.find(row=>row.role==='equality').checkedName).sourceStandardCollectionLibraryOwner,undefined);
});
test('independent original literals retain native version bounds and observations',()=>{
 assert.equal(rustStandardCollectionEqualityOriginals.length,24);for(const fixture of rustStandardCollectionEqualityOriginals){assert.equal(fixture.sourceVersionRange,'>=1.90.0 <1.100.0');assert.match(fixture.stdout,/true\n$/);assert.match(fixture.source,/fn main/);}assert.equal(rustStandardCollectionEqualityRefusals.length,6);
});
