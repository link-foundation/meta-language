// Deferred source-context controls. Never import during full requirements drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {selfTranslate,selfTranslationSignatures,selfTranslateWithDiagnostics} from '../src/self-translation.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-versions.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
function observe(translation){
 assert.ok(translation.items.every(item=>['translated','comment','provenance'].includes(item.status)),'every actual source item must translate');
 const code=translation.code.replace(/^export \{[^}]*\};?\s*$/gmu,''),target=/export \{ (\w+) as result \};/u.exec(translation.code)?.[1]??'result';
 return ()=>runInNewContext(code+'\n'+target+'();',{}, {timeout:1000});
}
for(const semanticTranslation of [false,true])for(const version of fixture.sourceVersions)for(const row of fixture.cases)test('source std '+version+' semantic '+semanticTranslation+' '+row.name,()=>{
 const translation=selfTranslate(row.source,'Rust','JavaScript',{semanticTranslation,rustSourceVersion:version});
 assert.ok(translation.code.includes(' rust-source-version='+version));
 const execute=observe(translation),expected=row.expectedByVersion[version];
 if(expected.kind==='abort')assert.throws(execute,error=>error.message===expected.message);else assert.equal(String(execute()),expected.value);
 const report=selfTranslateWithDiagnostics(row.source,'Rust','JavaScript',{semanticTranslation,rustSourceVersion:version});
 assert.deepEqual(report.translation,translation);assert.ok(report.diagnostics.every(value=>value===null));
});
for(const version of fixture.sourceVersions)test('independent library signatures use source std '+version,()=>{
 const signatures=selfTranslationSignatures(fixture.cases[0].source,'Rust',{semanticTranslation:true,rustSourceVersion:version});
 assert.equal(signatures.length,1);assert.equal(signatures[0].name,'result');assert.deepEqual(signatures[0].params,[]);assert.equal(signatures[0].ret.kind,'bool');
});
for(const version of fixture.invalidVersions)test('unsupported std cannot become carried or kept source '+version,()=>{
 for(const target of ['JavaScript','Rust'])for(const semanticTranslation of [false,true])assert.throws(()=>selfTranslate('pub fn result()->bool{true}','Rust',target,{semanticTranslation,rustSourceVersion:version}),error=>error.reason===fixture.versionError);
 assert.throws(()=>selfTranslationSignatures('pub fn result()->bool{true}','Rust',{semanticTranslation:true,rustSourceVersion:version}),error=>error.reason===fixture.versionError);
});
test('source-version selectors require their actual source language',()=>{
 for(const from of ['JavaScript','TypeScript'])for(const version of ['', '1.90.0']){
  assert.throws(()=>selfTranslate('function result(){return true;}',from,'Rust',{rustSourceVersion:version}),error=>error.message.includes('Rust source version requires Rust source'));
  assert.throws(()=>selfTranslationSignatures('function result(){return true;}',from,{rustSourceVersion:version}),error=>error.message.includes('Rust source version requires Rust source'));
 }
});
test('version getters are captured once for ordinary semantic diagnostic and signature paths',()=>{
 for(const semanticTranslation of [false,true])for(const diagnostics of [false,true]){
  let reads=0;const options=Object.defineProperty({semanticTranslation},'rustSourceVersion',{get:()=>++reads===1?'1.96.0':'1.99.0'});
  const value=(diagnostics?selfTranslateWithDiagnostics:selfTranslate)(fixture.cases[0].source,'Rust','JavaScript',options),translation=diagnostics?value.translation:value;
  assert.equal(reads,1);assert.throws(observe(translation),error=>error.message===fixture.cases[0].expectedByVersion['1.96.0'].message);
 }
 let reads=0;selfTranslationSignatures(fixture.cases[0].source,'Rust',Object.defineProperty({semanticTranslation:true},'rustSourceVersion',{get:()=>{reads+=1;return '1.96.0';}}));assert.equal(reads,1);
});
