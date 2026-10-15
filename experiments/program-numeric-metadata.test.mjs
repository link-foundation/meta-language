import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {acceptProgramIntegralMetadata} from '../js/src/program-numeric-metadata-rules.js';
import {numericMetadataArchive,numericMetadataFixture} from './generate-program-numeric-metadata-fixture.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';

const fixture=readProjectData(new URL('./program-numeric-metadata-fixture.lino',import.meta.url));
test('the generated fixed metadata role admits exactly the existing supported widths',()=>{
 for(const value of[8,16,32,64,128])assert.equal(acceptProgramIntegralMetadata('fixed','bits',value,false),true);
 for(const value of[-0,0,-1,7,32.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1,'32',32n,new Number(32),null,undefined])assert.equal(acceptProgramIntegralMetadata('fixed','bits',value,Object.is(value,-0)),false);
 for(const[kind,field,value]of[['float','value',-0],['float','value',1.25],['fixed','value',32],['record','bits',32]])assert.equal(acceptProgramIntegralMetadata(kind,field,value,Object.is(value,-0)),false);
});
test('actual decimal and exponent bit widths retain the complete checked program and both emissions',()=>{
 assert.deepEqual(numericMetadataFixture(),fixture);
 for(const entry of fixture.cases){
  const baseline=decodeTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root);
  for(const edit of entry.aliases){
   const packet=numericMetadataArchive(entry,edit),program=decodeTypedProgramLinks(packet.network,packet.root);assert.deepEqual(program,baseline,edit.id);
   const rebuilt=encodeTypedProgramLinks(program);assert.deepEqual(decodeTypedProgramLinks(LinkNetwork.fromLino(rebuilt.network.toLino()),rebuilt.root),baseline);
   for(const target of['JavaScript','Rust'])assert.equal(emitTypedProgramLinks(packet.network,packet.root,target).code,emitTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root,target).code,edit.id+target);
  }
 }
 const text=fixture.cases.find(entry=>entry.proofBindingScalar);assert.ok(text.aliases.some(edit=>edit.scalar===text.proofBindingScalar&&edit.spelling==='32.0'));
});
test('unsupported, nonintegral, unsafe and signed-zero widths retain their existing refusals',()=>{
 for(const entry of fixture.cases)for(const edit of entry.invalid){const packet=numericMetadataArchive(entry,edit);assert.throws(()=>decodeTypedProgramLinks(packet.network,packet.root),undefined,edit.id);for(const target of['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(packet.network,packet.root,target),undefined,edit.id+target);}
});
test('primitive negative zero and nonintegral floating values retain their execution meaning',()=>{
 const source='fn keep(value:f64)->f64{value}fn main(){println!("{}",keep(-0f64)==0f64);}';
 const packet=createTypedProgramLinks(source,'Rust'),before=packet.network.toLino(),program=decodeTypedProgramLinks(LinkNetwork.fromLino(before),packet.root),rebuilt=encodeTypedProgramLinks(program);
 const emitted=emitTypedProgramLinks(rebuilt.network,rebuilt.root,'JavaScript').code,context={console:{log(){}}};vm.createContext(context);vm.runInContext(emitted,context,{timeout:1000});
 assert.equal(vm.runInContext('Object.is(keep(-0),-0)',context),true);assert.equal(vm.runInContext('keep(1.25)',context),1.25);
 assert.equal(packet.network.toLino(),before);assert.equal(emitTypedProgramLinks(packet.network,packet.root,'JavaScript').code,emitted);
});
