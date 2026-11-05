import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {readInertModuleBindingDeclaration,readInertModuleSourceFacts} from '../src/translation/inert-module-bindings.js';
import {readEmptyStandardCollectionBindingDeclaration} from '../src/translation/empty-module-collections.js';
import {readEmptyStandardCollectionModuleBinding} from '../src/translation/empty-module-collection-rules.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/empty-module-collections.lino',import.meta.url));
const read=facts=>readEmptyStandardCollectionModuleBinding(facts.terms,facts.fields,facts.parents,facts.starts,facts.ends,facts.texts,facts.errors,facts.sourceBytes);

test('empty standard collection siblings retain real module scope beside an unsupported deferred body',()=>{
 for(const {source:prefix} of fixture.cases){
  const source=prefix+'\nexport function create(){return new Map([["left",1],["right",2]]);}\nfunction deferred(){return /unsupported/u;}\n';
  const actual=selfTranslate(source,'JavaScript','Rust',{retainDiagnostics:true});
  assert.ok(actual.items.some(item=>item.term==='export_statement'&&item.status==='translated'),JSON.stringify(actual.items));
  assert.equal(selfTranslationSignatures(source,'JavaScript').find(entry=>entry.name==='create')?.k,'fn');
  assert.ok(actual.items.some(item=>item.status==='carried'&&item.diagnostic?.message.includes('regular expression')),'the unsupported function retains its original obligation');
  assert.equal(selfTranslate(actual.code,'Rust','JavaScript').code,source);
 }
});

test('all shared empty allocation fixtures retain exact actual owners and UTF-8 name ranges',()=>{
 assert.equal(fixture.cases.length,6);
 for(const {source,name,owner} of fixture.cases){
  assert.equal(readInertModuleBindingDeclaration(source),null,'the original interpretation remains unchanged');
  const actual=readEmptyStandardCollectionBindingDeclaration(source);assert.ok(actual,source);
  assert.equal(actual.name,name);assert.equal(actual.owner,owner);assert.equal(actual.coordinateUnit,'UTF-8 bytes');
  assert.equal(Buffer.from(source).subarray(actual.nameRange.start,actual.nameRange.end).toString('utf8'),name);
 }
});

test('the additive empty interpretation refuses arguments eager siblings computed owners and source damage',()=>{
 for(const source of ['const cache=new Map([]);','const cache=new Set([]);','const cache=new Map(undefined);','const cache=new Set(null);','const cache=new Map(effect());','const cache=new Set(...values);','const cache=new custom.Map;','const cache=new globalThis.Set();','const cache=new (Map)();','const cache=new WeakMap();','let cache=new Map();','const {cache}=new Map();','const cache=new Map(),other=new Set();','const cache=new Map();effect();','const cache=new Map(;']){
  assert.equal(readEmptyStandardCollectionBindingDeclaration(source),null,source);
 }
 const source='/*λ*/ export const cache=new Set(/* empty */);\r\n',facts=readInertModuleSourceFacts(source);assert.equal(read(facts).length,2);
 for(const mutate of [value=>{value.errors[value.errors.length-1]=1;},value=>{value.parents[1]=1;},value=>{value.ends[0]--;},value=>{value.sourceBytes++;},value=>{value.fields.pop();},value=>{value.starts[2]=-1;},value=>{value.parents[2]=NaN;},value=>{value.texts[value.texts.indexOf('Set')]='custom';}]){
  const changed=structuredClone(facts);mutate(changed);assert.deepEqual(read(changed),[]);
 }
});

test('complete module ownership still refuses hidden standard owners and eager effects',()=>{
 for(const prefix of ['const Map={};const cache=new Map();','class Map{}const cache=new Map;','import {Map} from "./custom.mjs";const cache=new Map();','const Set={};const cache=new Set;','const cache=new Map();effect();','const cache=new Set();class Owner{static value=effect();}']){
  const source=prefix+'\nexport function create(){return new Map([["left",1]]);}\nfunction deferred(){return /unsupported/u;}\n';
  assert.deepEqual(selfTranslationSignatures(source,'JavaScript'),[],prefix);
  assert.equal(selfTranslate(selfTranslate(source,'JavaScript','Rust').code,'Rust','JavaScript').code,source);
 }
});

test('the actual JavaScript contract embedded in native tests observes every complete fixture',()=>{
 const native=readFileSync(new URL('../../rust/tests/unit/self_translation_inert_module_bindings.rs',import.meta.url),'utf8');
 const scripts=[...native.matchAll(/let script = r#"([\s\S]*?)"#;/gu)];assert.equal(scripts.length,1);
 for(const {source:prefix} of fixture.cases){
  const source=prefix+'\nexport function create(){return new Map([["a",1],["b",2]]);}\nfunction deferred(){return /unsupported/u;}\n';
  const output=execFileSync(process.execPath,['--max-old-space-size=160','--input-type=module','-e',scripts[0][1],source],{cwd:new URL('../../',import.meta.url),encoding:'utf8',timeout:10000,maxBuffer:1024*1024});
  const actual=readStructuredDataLino(output);
  assert.deepEqual(actual,{translation:selfTranslate(source,'JavaScript','Rust'),signatures:selfTranslationSignatures(source,'JavaScript')});
 }
});
