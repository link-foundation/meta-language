import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {readInertModuleBindingDeclaration,readInertModuleSourceFacts} from '../src/translation/inert-module-bindings.js';
import {readNumericModuleBindingDeclaration} from '../src/translation/numeric-module-bindings.js';
import {readNumericModuleBinding} from '../src/translation/numeric-module-binding-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/numeric-module-bindings.lino',import.meta.url));
const read=facts=>readNumericModuleBinding(facts.terms,facts.fields,facts.parents,facts.starts,facts.ends,facts.texts,facts.errors,facts.sourceBytes);
const moduleSource=prefix=>prefix+'\nexport function create(){return new Map([["a",1],["b",2]]);}\nfunction deferred(){return /unsupported/u;}\n';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');

test('complete arithmetic constants retain names coordinates and independent Number results',()=>{
 assert.equal(fixture.cases.length,6);
 for(const {source,name,observedText} of fixture.cases){
  assert.equal(readInertModuleBindingDeclaration(source),null,'the preceding interpretation remains exact');
  const actual=readNumericModuleBindingDeclaration(source);assert.ok(actual,source);
  assert.equal(actual.name,name);assert.equal(actual.owner,0);assert.equal(actual.coordinateUnit,'UTF-8 bytes');
  assert.equal(Buffer.from(source).subarray(actual.nameRange.start,actual.nameRange.end).toString('utf8'),name);
  const ordinary=source.replace(/\bexport\s+/u,'');
  assert.equal(vm.runInNewContext(ordinary+'\nString('+name+');',{}, {timeout:1000}),observedText);
 }
});

test('arithmetic scope admits checked collection siblings and preserves carried obligations and complete source',()=>{
 for(const {source:prefix} of fixture.cases){
  const source=moduleSource(prefix),actual=selfTranslate(source,'JavaScript','Rust',{retainDiagnostics:true});
  assert.ok(actual.items.some(item=>item.term==='export_statement'&&item.status==='translated'),JSON.stringify(actual.items));
  assert.equal(selfTranslationSignatures(source,'JavaScript').find(entry=>entry.name==='create')?.k,'fn');
  assert.ok(actual.items.some(item=>item.status==='carried'&&item.diagnostic?.message.includes('regular expression')));
  assert.equal(selfTranslate(actual.code,'Rust','JavaScript').code,source);
 }
});

test('the arithmetic rule refuses eager effects names coercion BigInt and damaged CST',()=>{
 for(const source of ['let value=1+2;','const value=read()+2;','const value=owner.value+2;','const value=other+2;','const value=1n+2n;','const value=1n+2;','const value="1"+2;','const value=true+2;','const value=null+2;','const value=+undefined;','const value=NaN+2;','const value=Infinity-2;','const value=(1,2);','const value=1<2;','const value=1&&2;','const value=(owner=2)+3;','const value=++other;','const value=1+2,other=3;','const value=1+2;effect();','const {value}=1+2;','const value=1+;']){
  assert.equal(readNumericModuleBindingDeclaration(source),null,source);
 }
 const facts=readInertModuleSourceFacts('/* λ */ export const limit=(128 * 1024) ** 2;\r\n');assert.equal(read(facts).length,2);
 for(const mutate of [value=>{value.errors[value.errors.length-1]=1;},value=>{value.parents[1]=1;},value=>{value.ends[0]--;},value=>{value.sourceBytes++;},value=>{value.fields.pop();},value=>{value.starts[2]=-1;},value=>{value.parents[2]=NaN;},value=>{value.texts[value.texts.indexOf('128')]='128n';}]){
  const changed=structuredClone(facts);mutate(changed);assert.deepEqual(read(changed),[]);
 }
});

test('strict module arithmetic rejects legacy leading-zero numeric spellings',()=>{
 for(const spelling of ['00','01','07','08','09','010','078']){
  const source='const value='+spelling+'+2;';
  assert.throws(()=>new vm.Script('"use strict";'+source));
  assert.equal(readNumericModuleBindingDeclaration(source),null,source);
 }
 for(const spelling of ['0','0.1','0e1','0xff','0XFF','0o10','0O10','0b10','0B10','1_000']){
  const source='const value='+spelling+'+2;';
  assert.doesNotThrow(()=>new vm.Script('"use strict";'+source));
  assert.ok(readNumericModuleBindingDeclaration(source),source);
 }
});

test('arithmetic bindings still count their actual owners and refuse eager module siblings',()=>{
 for(const prefix of ['const Map=1+2;','const Set=1+2;const values=new Set();','const limit=1+2;effect();','const limit=1+2;class Owner{static value=effect();}']){
  const source=moduleSource(prefix);assert.deepEqual(selfTranslationSignatures(source,'JavaScript'),[],prefix);
  assert.equal(selfTranslate(selfTranslate(source,'JavaScript','Rust').code,'Rust','JavaScript').code,source);
 }
});

test('the actual native-test JavaScript script observes all complete arithmetic fixtures',()=>{
 const native=readFileSync(new URL('../../rust/tests/unit/self_translation_inert_module_bindings.rs',import.meta.url),'utf8');
 const scripts=[...native.matchAll(/let script = r#"([\s\S]*?)"#;/gu)];assert.equal(scripts.length,1);
 for(const {source:prefix} of fixture.cases){
  const source=moduleSource(prefix);
  const output=execFileSync(process.execPath,['--max-old-space-size=160','--input-type=module','-e',scripts[0][1],source],{cwd:new URL('../../',import.meta.url),encoding:'utf8',timeout:10000,maxBuffer:1024*1024});
  assert.deepEqual(readStructuredDataLino(output),{translation:selfTranslate(source,'JavaScript','Rust'),signatures:selfTranslationSignatures(source,'JavaScript')});
 }
});
