import assert from 'node:assert/strict';
import test from 'node:test';
import {readInertModuleBindingDeclaration,readInertModuleSourceFacts} from '../src/translation/inert-module-bindings.js';
import {readInertModuleBinding,acceptInertModuleBindingScope} from '../src/translation/inert-module-binding-rules.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const read=facts=>readInertModuleBinding(facts.terms,facts.fields,facts.parents,facts.starts,facts.ends,facts.texts,facts.errors,facts.sourceBytes);
test('complete actual literal declarations retain their source owner and precise standard-global requirements',()=>{
 for(const[source,name,owner]of[
  ['const values = [true, null, {a: -0, text: "λ"}, [1, 2,],];','values',0],
  ['export const values = {name:"café",nested:{enabled:true},empty:[]};','values',0],
  ['/*λ*/ const values = Object.freeze({a:[1,2],b:null});\r\n','values',3],
  ['const owners = new Set(["left", "right"]);','owners',2],
  ['const owners = new Map([["left", 1], ["right", 2,],]);','owners',1],
  ['const empty = new Map([]);','empty',1],
  ['const empty = Object.freeze([]);','empty',3],
 ]){
  const actual=readInertModuleBindingDeclaration(source);assert.ok(actual,source);assert.equal(actual.name,name);assert.equal(actual.owner,owner);
  assert.equal(Buffer.from(source).subarray(actual.nameRange.start,actual.nameRange.end).toString('utf8'),name);
  assert.equal(acceptInertModuleBindingScope(['named function declaration','owned inert value binding'],[-1,owner],[name]),true);
 }
});
test('actual evaluated calls getters computed keys spreads overwrites and hidden owners refuse',()=>{
 for(const source of[
  'const a=[effect()];','const a={value:effect()};','const a={[effect()]:true};','const a={get value(){return true;}};',
  'const a={...other};','const a=[...other];','const a=[,true];','const a=new Set(other);','const a=new Map([[1]]);',
  'const a=new Map([[1,2,3]]);','const a=custom.freeze({});','const a=Object["freeze"]({});','const a=Object.freeze({},effect());',
  'const a=new custom.Set([1]);','const a=new Set([effect()]);','const a=new Set([Object.freeze({})]);',
  'const a=[value];','const {a}={a:[]};','const a=[],b=[];','let a=[];','var a=[];','const a=[];effect();',
  'export default [];','const a=()=>[];','const a=Object.freeze({value:()=>true});','const a=[];class Set{}',
  'const a = [',' '.repeat(65537),
 ])assert.equal(readInertModuleBindingDeclaration(source),null,source.slice(0,120));
 for(const[owner,name]of[[1,'Map'],[2,'Set'],[3,'Object']]){
  assert.equal(acceptInertModuleBindingScope(['owned inert value binding'],[owner],[name]),false,'actual module/import/local owner '+name);
  assert.equal(acceptInertModuleBindingScope(['unknown module form','owned inert value binding'],[-1,owner],[]),false);
 }
 for(const owner of[-1,1.5,4,NaN,Infinity])assert.equal(acceptInertModuleBindingScope(['owned inert value binding'],[owner],[]),false);
 assert.equal(acceptInertModuleBindingScope(['owned inert value binding'],[],[]),false);
});
test('complete source facts refuse omitted children malformed ownership and source-extent edits',()=>{
 const source='const values=Object.freeze({left:[1,2],right:true});',facts=readInertModuleSourceFacts(source);assert.ok(read(facts).length===2);
 for(const mutate of[
  value=>{value.errors[value.errors.length-1]=1;},value=>{value.parents[1]=1;},value=>{value.ends[0]--;},
  value=>{value.sourceBytes++;},value=>{value.fields.pop();},value=>{value.starts[2]=-1;},
  value=>{value.parents[2]=NaN;},value=>{value.terms[value.terms.indexOf('pair')]='spread_element';},
 ]){const changed=structuredClone(facts);mutate(changed);assert.deepEqual(read(changed),[]);}
});

test('actual literal module siblings permit existing Map and Set semantics without erasing their source',async()=>{
 const{selfTranslate,selfTranslationSignatures}=await import('../src/self-translation.js');
 for(const declaration of ['const values=[1,2];','const values={enabled:true};','const values=new Set(["a","b"]);','const values=new Map([["a",1]]);','const values=Object.freeze({enabled:true});']){
  const source=declaration+'\nexport function create(){return new Map([["a",1],["b",2]]);}\n';
  const result=selfTranslate(source,'JavaScript','Rust');
  assert.ok(result.items.some(item=>item.status==='translated'&&item.term==='export_statement'),JSON.stringify(result.items));
  assert.equal(selfTranslationSignatures(source,'JavaScript').find(entry=>entry.name==='create')?.k,'fn');
  assert.equal(selfTranslate(result.code,'Rust','JavaScript').code,source,'the complete initializer remains in original source provenance');
 }
 for(const prefix of ['const Map={};','const Map=[];','const values=[effect()];','const values={get enabled(){return true;}};','const Set=custom;const values=new Set([1]);','const Object=custom;const values=Object.freeze({});','class Owner{static value=effect();}const values=[];']){
  const source=prefix+'\nexport function create(){return new Map([["a",1]]);}\n';
  assert.deepEqual(selfTranslationSignatures(source,'JavaScript'),[],prefix);
 }
});

test('the native rules and both complete checked archives are automatically fresh',async()=>{
 const{generateInertModuleBindingRules}=await import('../scripts/generate-inert-module-binding-rules.mjs');
 await generateInertModuleBindingRules({check:true,native:true});
});
