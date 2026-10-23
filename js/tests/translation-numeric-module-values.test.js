import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {acceptNumericModuleBindingSignature} from '../src/translation/numeric-module-binding-rules.js';
import {acceptNumericModuleBindingDeclarationSignature} from '../src/translation/numeric-module-bindings.js';

const fixture=readProjectData(new URL('../../parity/fixtures/numeric-module-values.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const moduleSource=row=>row.source+'\nexport function verify(){return '+row.predicate+';}\n';
const observe=source=>runInNewContext('"use strict";\n'+source.replace(/^export /gmu,'')+'\nverify();',{}, {timeout:1000});

test('actual checked Number static signatures require their complete original CST owner',()=>{
 const source='const LIMIT=128 * 1024;',signature={k:'const',name:'LIMIT',type:{kind:'float'},literal:false};
 assert.equal(acceptNumericModuleBindingDeclarationSignature(source,signature),true);
 for(const changed of [{...signature,name:'other'},{...signature,k:'fn'},{...signature,type:{kind:'array'}},{...signature,literal:true}]){
  assert.equal(acceptNumericModuleBindingDeclarationSignature(source,changed),false);
 }
 for(const other of ['const LIMIT=read();','let LIMIT=128*1024;','const LIMIT=1n+2n;','const LIMIT="1"+2;','const LIMIT=[1,2];','const LIMIT=1+2;effect();']){
  assert.equal(acceptNumericModuleBindingDeclarationSignature(other,signature),false,other);
 }
 for(const [kind,type,literal,owner,name] of [['fn','float',false,'LIMIT','LIMIT'],['const','fixed',false,'LIMIT','LIMIT'],['const','float',true,'LIMIT','LIMIT'],['const','float',false,'','LIMIT'],['const','float',false,'LIMIT','other']]){
  assert.equal(acceptNumericModuleBindingSignature(kind,type,literal,owner,name),false);
 }
});

test('arithmetic module values supply real static reads and preserve complete sources',()=>{
 assert.equal(fixture.cases.length,6);
 for(const row of fixture.cases){
  const source=moduleSource(row);assert.equal(observe(source),true,source);
  const result=selfTranslate(source,'JavaScript','Rust',{retainDiagnostics:true});
  assert.ok(result.items.every(item=>item.status==='translated'),JSON.stringify(result.items));
  assert.match(result.code,/pub static \w+: std::sync::LazyLock<f64>/u);
  assert.match(result.code,/pub fn verify\(\) -> bool/u);
  assert.equal(selfTranslationSignatures(source,'JavaScript').find(entry=>entry.name==='verify')?.ret.kind,'bool');
  assert.equal(selfTranslate(result.code,'Rust','JavaScript').code,source);
 }
});

test('an arithmetic static remains available beside carried code and under lexical shadowing',()=>{
 const source='const VALUE=128 * 1024;\nexport function verify(){return VALUE===131072;}\nfunction deferred(){return /unsupported/u;}\n';
 const result=selfTranslate(source,'JavaScript','Rust',{retainDiagnostics:true});
 assert.ok(result.items.some(item=>item.term==='export_statement'&&item.status==='translated'));
 assert.ok(result.items.some(item=>item.status==='carried'&&item.diagnostic?.details.construct==='regular expression'));
 assert.equal(selfTranslate(result.code,'Rust','JavaScript').code,source);
 const shadow='const VALUE=128 * 1024;\n/** @param {number} VALUE @returns {number} */\nexport function choose(VALUE){return VALUE+2;}\n';
 const output=selfTranslate(shadow,'JavaScript','Rust');
 assert.ok(output.items.every(item=>item.status==='translated'||item.status==='comment'));
 const body=/pub fn choose\([\s\S]*?\n\}/u.exec(output.code)?.[0];assert.ok(body);
 assert.doesNotMatch(body,/\*crate::VALUE/u,'the actual local parameter takes precedence over the module static');
 assert.equal(selfTranslate(output.code,'Rust','JavaScript').code,shadow);
});

test('numeric source admission retains eager-effect duplicate-owner and unsupported-operator obligations',()=>{
 for(const prefix of ['let VALUE=1+2;','const VALUE=read()+2;','const VALUE=1+2;effect();','const VALUE=1+2;const VALUE=3;']){
  const source=prefix+'\nexport function verify(){return VALUE===3;}\n';
  assert.deepEqual(selfTranslationSignatures(source,'JavaScript'),[],prefix);
  assert.equal(selfTranslate(selfTranslate(source,'JavaScript','Rust').code,'Rust','JavaScript').code,source);
 }
 for(const expression of ['2 ** 8','1 << 8']){
  const source='const VALUE='+expression+';\nexport function verify(){return VALUE===256;}\n';
  const result=selfTranslate(source,'JavaScript','Rust',{retainDiagnostics:true});
  // Keep both original inputs. The shared Number bitwise interpretation now
  // discharges the shift; exponentiation remains an explicit obligation.
  if(expression==='1 << 8'){
   assert.ok(result.items.every(item=>item.status==='translated'));
   assert.equal(observe(source),true);
  }else assert.ok(result.items.some(item=>item.status==='carried'));
  assert.equal(selfTranslate(result.code,'Rust','JavaScript').code,source);
 }
});

test('TypeScript erasure retains the actual arithmetic constant source and scalar signature',()=>{
 assert.equal(fixture.typedCases.length,1);
 for(const row of fixture.typedCases){
  const source=moduleSource(row),result=selfTranslate(source,'TypeScript','Rust',{retainDiagnostics:true});
  assert.ok(result.items.every(item=>item.status==='translated'),JSON.stringify(result.items));
  assert.equal(selfTranslate(result.code,'Rust','TypeScript').code,source);
  assert.equal(selfTranslationSignatures(source,'TypeScript').find(entry=>entry.name==='verify')?.ret.kind,'bool');
 }
});

test('the exact native arithmetic observation script covers every shared source and signature',()=>{
 const native=readFileSync(new URL('../../rust/tests/unit/self_translation_inert_module_bindings.rs',import.meta.url),'utf8');
 for(const [pattern,language,cases] of [[/let numeric_script = r#"([\s\S]*?)"#;/gu,'JavaScript',fixture.cases],[/let typed_script = r#"([\s\S]*?)"#;/gu,'TypeScript',fixture.typedCases]]){
  const scripts=[...native.matchAll(pattern)];assert.equal(scripts.length,1);
  for(const row of cases){
   const source=moduleSource(row);
   const output=execFileSync(process.execPath,['--max-old-space-size=160','--input-type=module','-e',scripts[0][1],source],{cwd:new URL('../../',import.meta.url),encoding:'utf8',timeout:10000,maxBuffer:1024*1024});
   assert.deepEqual(readStructuredDataLino(output),{translation:selfTranslate(source,language,'Rust'),signatures:selfTranslationSignatures(source,language)});
  }
 }
});

test('ordinary recursive components retain the same source-owned numeric static',()=>{
 const source='const VALUE=128*1024;\n'
  +'/** @param {number} n @returns {number} */\nfunction left(n){if(n===0)return VALUE;return right(n-1);}\n'
  +'/** @param {number} n @returns {number} */\nfunction right(n){if(n===0)return VALUE;return left(n-1);}\n'
  +'export function verify(){return left(4)===131072;}\n';
 assert.equal(observe(source),true);
 const result=selfTranslate(source,'JavaScript','Rust',{retainDiagnostics:true});
 assert.ok(result.items.every(item=>item.status==='translated'||item.status==='comment'),JSON.stringify(result.items));
 assert.equal(selfTranslate(result.code,'Rust','JavaScript').code,source);
 assert.equal(selfTranslationSignatures(source,'JavaScript').find(entry=>entry.name==='verify')?.ret.kind,'bool');
});
