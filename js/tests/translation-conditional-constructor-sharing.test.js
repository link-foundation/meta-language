// Deferred source-construction controls. Never execute during drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {shareSourceConditionalConstructors} from '../scripts/source-conditional-constructors.mjs';
import {shareSourceClosedConstructorMatches} from '../scripts/source-boolean-match-sharing.mjs';
import {testConditionalConstructorOwners,planConditionalConstructorFieldSelection} from '../src/translation/conditional-constructor-sharing-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/conditional-constructor-sharing.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.node);
const checked=source=>shareSourceClosedConstructorMatches(checkProgram(parseJavaScript(source,{referenceSemantics:false}))).program;
function normalize(value){
 if(typeof value==='number'){const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,value);return {numberBits:bytes.getBigUint64(0).toString(16).padStart(16,'0')};}
 if(typeof value==='bigint')return {integer:String(value)};
 if(Array.isArray(value))return Array.from(value,normalize);
 if(value&&typeof value==='object')return Object.fromEntries(Object.keys(value).map(key=>[key,normalize(value[key])]));
 return value;
}
function observe(source,choice){
 const trace=[];
 try{const value=runInNewContext(source.replace(/^export /gmu,'')+'\ncompute('+choice+',"retained",-0);',{console:{log:value=>trace.push(String(value))}},{timeout:1000});return {kind:'value',value:normalize(value),trace};}
 catch(error){return {kind:'throw',name:error.name,message:error.message,trace};}
}
function instrument(program){
 let condition=0;
 function walk(value){
  if(!value||typeof value!=='object')return value;
  if(Array.isArray(value))return value.map(walk);
  const node=Object.fromEntries(Object.entries(value).map(([key,child])=>[key,['type','domain','from','to','span','pattern'].includes(key)?child:walk(child)]));
  if(node.k==='ctor'&&node.ctor==='record')node.args=node.args.map((argument,index)=>index<=1?{k:'print',text:{k:'lit',value:index===0?'field:alpha':'field:bravo',type:{kind:'string'}},body:argument,type:argument.type}:argument);
  return node;
 }
 // Visit each original condition in source order before visiting its branches.
 function mark(value){
  if(!value||typeof value!=='object')return value;
  if(Array.isArray(value))return value.map(mark);
  if(value.k==='if'&&value.type?.kind==='data'){
   const ordinal=condition++;
   return {...value,cond:{k:'print',text:{k:'lit',value:'condition:'+ordinal,type:{kind:'string'}},body:value.cond,type:value.cond.type},then:mark(value.then),else:mark(value.else)};
  }
  return Object.fromEntries(Object.entries(value).map(([key,child])=>[key,['type','domain','from','to','span','pattern'].includes(key)?child:mark(child)]));
 }
 const declarations=new Map(program.declarations);
 const entry=declarations.get('compute');declarations.set('compute',{...entry,body:walk(mark(entry.body))});
 const items=values=>values.map(item=>item.k==='module'?{...item,items:items(item.items)}:declarations.get(item.fullName));
 return {...program,declarations,items:items(program.items)};
}
for(const row of fixture.owners)test(row.name,()=>assert.equal(testConditionalConstructorOwners(row.owner,row.constructor,row.fields,row.owners,row.constructors,row.arities),row.expected));
for(const [index,row]of fixture.selections.entries())test('finite field selection '+index,()=>assert.deepEqual(planConditionalConstructorFieldSelection(row.classes),row.expected));
for(const row of fixture.cases){
 test(row.name+': actual source and complete typed reload',()=>{
  const original=checked(row.source),selected=shareSourceConditionalConstructors(original);
  assert.equal(selected.proof.proofs.some(proof=>proof.owner==='compute'),row.expectShared);
  const packet=encodeTypedProgramLinks(selected.program);
  const products=[selected.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))];
  for(const choice of row.choices){const expected=observe(row.source,choice);
   assert.deepEqual(observe(emitJavaScript(original).text,choice),expected);
   for(const product of products)assert.deepEqual(observe(emitJavaScript(product).text,choice),expected);}
  for(const product of products)for(const emit of [emitRust,emitLean,emitRocq])assert.ok(emit(product).definitions.length>0);
 });
 test(row.name+': retained condition and argument traces',()=>{
  const original=instrument(checked(row.source)),selected=shareSourceConditionalConstructors(original);
  assert.equal(selected.proof.proofs.some(proof=>proof.owner==='compute'),row.expectShared);
  for(const choice of row.choices){const expected=observe(row.tracedSource,choice);
   assert.deepEqual(observe(emitJavaScript(original).text,choice),expected);
   assert.deepEqual(observe(emitJavaScript(selected.program).text,choice),expected);}
 });
}
test('reference and foreign-language profiles retain their explicit boundary',()=>{
 const original=checked(fixture.cases[0].source);
 for(const changed of [{...original,referenceSemantics:true},{...original,referenceSemantics:'false'},{...original,sourceLanguage:'Rust'}])
  assert.throws(()=>shareSourceConditionalConstructors(changed),/checked JavaScript value model/u);
});

test('generated selection bindings avoid active source names',()=>{
 const source=fixture.cases[0].source.replace(/\bvalue\b/gu,'shared_constructor_selection_1');
 const original=checked(source),selected=shareSourceConditionalConstructors(original);
 const proof=selected.proof.proofs.find(value=>value.owner==='compute');
 assert.ok(proof);assert.notEqual(proof.selectionBinding,'shared_constructor_selection_1');
 for(const choice of fixture.cases[0].choices)assert.deepEqual(observe(emitJavaScript(selected.program).text,choice),observe(source,choice));
});
