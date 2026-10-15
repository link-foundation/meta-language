import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks} from '../src/index.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {validateLinkedTypedProgram} from '../src/typed-program-links-validation.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';

const owned=readProjectData(new URL('../../experiments/rust-owned-local-state-fixture.lino',import.meta.url));
const namespace=readProjectData(new URL('../../parity/fixtures/rust-type-namespace.lino',import.meta.url));
const models=[
 {source:'const fn keep(value:bool)->bool{value}fn main(){}',options:{rustConstFunctionModel:'closed pure Rust const predicates 1'},markers:['sourceConstFunction']},
 {source:owned.cases[0].source,options:{rustOwnedLocalStateModel:owned.models.moves},markers:['sourceLocalState']},
 {source:namespace.cases[0].source,options:{rustTypeNamespaceModel:namespace.typeNamespaceModel},markers:['sourceTypeNamespace','sourceUnitCloneDerivation']},
];
const create=entry=>{const packet=createTypedProgramLinks(entry.source,'Rust',entry.options);return decodeTypedProgramLinks(packet.network,packet.root);};
function removeMarkers(program,markers){
 const visited=new Set(),found=[];
 function visit(value){if(!value||typeof value!=='object'||visited.has(value))return;visited.add(value);if(value instanceof Map){for(const child of value.values())visit(child);return;}for(const[key,child]of Object.entries(value)){if(markers.includes(key)){found.push([key,child]);delete value[key];}else visit(child);}}
 visit(program);assert.ok(found.length);return found[0];
}

test('actual public const, owned text and namespace programs refuse function roots and callback proofs at their guarded boundaries',()=>{
 for(const entry of models)for(const form of['root','nested','getter']){
  let program=create(entry),calls=0;
  if(form==='root')program=Object.assign(function host(){},program);
  else{const[marker,proof]=removeMarkers(program,entry.markers);program.hidden=function host(){};if(form==='getter')Object.defineProperty(program.hidden,marker,{get(){calls++;return proof;},enumerable:true});else program.hidden[marker]=proof;}
  const checks=[()=>validateLinkedTypedProgram(program),()=>recheckLinkedTypedProgram(program),()=>encodeTypedProgramLinks(program),()=>emitRust(program),()=>emitJavaScript(program)];
  for(const check of checks)assert.throws(check,entry.markers[0]+' '+form);
  assert.equal(calls,0,'source marker accessors must not run');
 }
});

test('actual source-free namespace facts are revalidated before either direct emitter',()=>{
 const entry=models[2],mutations=[
  program=>{program.declarations.get('nested.header').sourceTypeNamespace=null;},
  program=>{program.declarations.get('nested.header').sourceTypeNamespace.result=['unknown'];},
  program=>{program.declarations.get('nested.header').sourceTypeNamespace.parameters[0]=['bool'];},
  program=>{program.declarations.get('nested.header').generated=true;},
 ];
 for(const mutate of mutations){const program=create(entry);mutate(program);assert.throws(()=>emitRust(program));assert.throws(()=>emitJavaScript(program));}
 const program=create(entry),packet=encodeTypedProgramLinks(program),restored=decodeTypedProgramLinks(packet.network,packet.root);
 assert.deepEqual(emitRust(restored),emitRust(program));assert.deepEqual(emitJavaScript(restored),emitJavaScript(program));
});

test('ordinary unselected callback metadata retains the existing direct-emitter fallback without observing accessors',()=>{
 const packet=createTypedProgramLinks('fn keep(value:i32)->i32{value}fn main(){}','Rust'),program=decodeTypedProgramLinks(packet.network,packet.root),expected=[emitRust(program),emitJavaScript(program)];let calls=0;
 program.hidden=function host(){};Object.defineProperty(program.hidden,'irrelevant',{get(){calls++;return false;}});
 assert.deepEqual([emitRust(program),emitJavaScript(program)],expected);assert.equal(calls,0);
});
