// The public selected profile retains genuine source and source-free execution.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {checkRustOwnedLocalStateProof} from '../js/src/translation/rust-owned-local-state.js';
import {rustOwnedLocalSourceOperationFixture} from './generate-rust-owned-local-source-operation-fixture.mjs';
const fixture=readProjectData(new URL('./rust-owned-local-state-fixture.lino',import.meta.url));
test('the public factory selects original and canonical owned text models with exact reloadable facts',()=>{
 for(const entry of fixture.cases){
  for(const [source,model,key]of [[entry.source,fixture.models.moves,'javascript'],[entry.replaySource,fixture.models.copies,'replayJavaScript']]){
   const created=createTypedProgramLinks(source,'Rust',{rustOwnedLocalStateModel:model,provenance:{caller:'owned text test'}});
   const network=LinkNetwork.fromLino(created.network.toLino()),program=decodeTypedProgramLinks(network,created.root);
   assert.equal(program.items[0].sourceLocalState.model,model);
   assert.deepEqual(program.items[0].sourceSignature.parameters[1].type,{kind:'fixed',bits:32,signed:true});
   const emitted=emitTypedProgramLinks(network,created.root,'JavaScript');
   assert.equal(emitted.code,entry[key]);
   const runtime={};vm.runInNewContext(emitted.code,runtime,{timeout:1000});
   for(const[text,count,flag,expected]of entry.observations)assert.equal(runtime[entry.name](text,BigInt(count),flag),expected);
   const relations=network.fieldRelations(),profile=relations.find(relation=>relation.label==='source profile');
   assert.ok(profile);assert.ok(relations.some(relation=>relation.parent.equals(profile.child)&&relation.label==='rust owned local state model'&&network.link(relation.child).metadata().definition===model));
   assert.ok(relations.some(relation=>relation.label==='caller'&&network.link(relation.child).metadata().definition==='owned text test'));
   assert.ok(network.links().every(link=>!link.metadata().definition?.includes(source)));
  }
 }
});
test('public model selection preserves default, foreign language and incompatible profile refusals',()=>{
 const source=fixture.cases[0].source;
 assert.throws(()=>createTypedProgramLinks(source,'Rust'),/mutable tuple binding/u);
 assert.throws(()=>createTypedProgramLinks(source,'Rust',{rustOwnedLocalStateModel:''}),/mutable tuple binding/u);
 assert.throws(()=>createTypedProgramLinks(source,'Rust',{rustOwnedLocalStateModel:'unknown'}),/Rust owned local state model/u);
 assert.throws(()=>createTypedProgramLinks('function empty(){return 1;}','JavaScript',{rustOwnedLocalStateModel:fixture.models.moves}),/a Rust owned local state model requires Rust source/u);
 for(const[field,value]of [['rustArrayResultModel','fresh scalar vector result 1'],['rustCompletionModel','closed guarded function returns 1'],['rustScalarLoopModel','closed primitive scalar loop 1'],['rustPrimitiveOwnerModel','Rust standard primitive owner 1']])assert.throws(()=>createTypedProgramLinks(source,'Rust',{rustOwnedLocalStateModel:fixture.models.moves,[field]:value}),/other source state and completion profiles require independent composition proofs/u);
});

test('tuple field receiver records retain their exact shared owned fields before direct emission',()=>{
 for(const field of ['borrowEscape','opaqueBinding']){
  const entry=fixture.cases[0],program=decodeTypedProgramLinks(LinkNetwork.fromLino(entry.links),entry.root);
  const receiver=program.items[0].body.body.value.object;assert.equal(receiver.k,'var');receiver[field]=true;
  assert.throws(()=>checkRustOwnedLocalStateProof(program),/tuple field layout or lexical ownership changed/u);
  assert.throws(()=>emitJavaScript(program),/tuple field layout or lexical ownership changed/u);
 }
});

test('actual Box and borrowed source operations refuse before names erase their source owners',()=>{
 const recorded=readProjectData(new URL('./rust-owned-local-source-operation-fixture.lino',import.meta.url));assert.deepEqual(rustOwnedLocalSourceOperationFixture(),recorded);assert.equal(recorded.cases.length,6);
 const bodyCases=[
  [fixture.models.moves,'let(mut digits,n)=(text,count);let boxed=Box::new(digits);digits=String::from("new");boxed'],
  [fixture.models.moves,'let(mut digits,n)=(text,count);let borrowed=&&digits;digits=String::from("new");digits'],
  [fixture.models.copies,'let boxed=Box::new(text);boxed'],
  [fixture.models.copies,'*Box::new(text)'],
  [fixture.models.copies,'let borrowed=&text;String::from("value")'],
  [fixture.models.copies,'let borrowed=&&text;String::from("value")'],
 ];
 for(const[model,body]of bodyCases){const source=`fn bad(text:String,count:i32,flag:bool)->String{${body}}fn main(){}`;assert.throws(()=>createTypedProgramLinks(source,'Rust',{rustOwnedLocalStateModel:model}),/Box allocation|source indirection|references and mutable expression/u);}
});

test('source indirection refusal archives retain exact full source diagnostics for both text models',()=>{
 const recorded=readProjectData(new URL('./rust-owned-local-source-operation-fixture.lino',import.meta.url));
 for(const entry of recorded.cases)assert.throws(()=>createTypedProgramLinks(entry.source,'Rust',{rustOwnedLocalStateModel:entry.model}),error=>{assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason,entry.reason,entry.name);assert.deepEqual(error.span??null,entry.span,entry.name);return true;});
});
