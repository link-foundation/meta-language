import {mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
import {materializeProjectTranslationInputs} from '../scripts/draft-project-translation.mjs';
import {createProjectTranslationArchive,readProjectTranslationArchive} from '../src/project-translation-archive.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {runCommandLine} from '../src/cli.js';
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

for(const version of fixture.sourceVersions)test('command source snapshot '+version,()=>{
 const row=fixture.cases[0],options={semanticTranslation:true,rustSourceVersion:version},expected=selfTranslate(row.source,'Rust','JavaScript',options);
 const output=runCommandLine(['translate','--to','JavaScript','--semantic','--rust-source-version',version,'source.rs'],{readFile:path=>{assert.equal(path,'source.rs');return row.source;}});
 assert.equal(output.exitCode,0,output.stderr);assert.equal(output.stdout,expected.code);
 assert.throws(observe({...expected,code:output.stdout}),error=>error.message===row.expectedByVersion[version].message);
});
test('command rejects missing duplicate invalid and wrong-language source snapshots',()=>{
 for(const args of [['--rust-source-version'],['--rust-source-version','--semantic'],['--rust-source-version','1.90.0','--rust-source-version','1.99.0']])assert.equal(runCommandLine(['translate','--to','JavaScript','source.rs',...args],{readFile:()=>assert.fail('invalid argument plans cannot read')}).exitCode,2);
 for(const version of fixture.invalidVersions){const output=runCommandLine(['translate','--to','JavaScript','--rust-source-version',version,'source.rs'],{readFile:()=>fixture.cases[0].source});assert.equal(output.exitCode,1);assert.equal(output.stdout,'');assert.ok(output.stderr.includes(fixture.versionError));}
 const output=runCommandLine(['translate','--to','Rust','--rust-source-version','1.90.0','source.js'],{readFile:()=>''});assert.equal(output.exitCode,1);assert.ok(output.stderr.includes('Rust source version requires Rust source'));
});

for(const version of fixture.sourceVersions)test('mixed source archive retains per-module snapshot '+version,()=>{
 const inputs=[{identifier:'lib.rs',language:'Rust',source:fixture.cases[0].source,options:{rustSourceVersion:version}},{identifier:'plain.js',language:'JavaScript',source:'export function plain(){return true;}'}];
 const packet=createProjectTranslationArchive(inputs,{targetLanguage:'JavaScript'});
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const view=readProjectTranslationArchive(network,packet.root),rust=view.draft.modules.find(row=>row.identifier==='lib.rs'),plain=view.draft.modules.find(row=>row.identifier==='plain.js');
  assert.equal(typeof rust.emittedCode,'string');assert.equal(typeof plain.emittedCode,'string');
  assert.equal(view.configuration.modules.find(row=>row.identifier==='lib.rs').options.rustSourceVersion,version);
  assert.ok(!Object.hasOwn(view.configuration.modules.find(row=>row.identifier==='plain.js').options,'rustSourceVersion'));
  const original=view.sources.find(row=>row.path==='lib.rs');assert.equal(original.source,fixture.cases[0].source);assert.equal(original.projection,'ConcreteSyntax');
  assert.throws(observe({code:rust.emittedCode,items:rust.items??[]}),error=>error.message===fixture.cases[0].expectedByVersion[version].message);
 }
});

test('tracked mixed-language inventories attach source std only to original Rust entries',()=>{
 const directory=mkdtempSync(join(tmpdir(),'meta-language-source-std-inventory-'));
 const git=args=>execFileSync('git',['-c','gc.auto=0','-c','core.hooksPath=/dev/null','-C',directory,...args],{stdio:'pipe'});
 try{
  writeFileSync(join(directory,'library.rs'),'pub fn result()->bool{true}');writeFileSync(join(directory,'library.js'),'export function result(){return true;}');writeFileSync(join(directory,'library.ts'),'export function result():boolean{return true;}');
  git(['init']);git(['add','--','library.rs','library.js','library.ts']);git(['-c','user.name=Fixture author','-c','user.email=fixture@example.invalid','commit','-m','Independent tracked source inputs']);
  for(const version of ['1.90.0','1.99.0','']){
   const inventory=materializeProjectTranslationInputs(directory,{rustSourceVersion:version});assert.equal(inventory.inputs.length,3);
   for(const entry of inventory.inputs){if(entry.language==='Rust')assert.equal(entry.options.rustSourceVersion,version||'1.99.0');else assert.ok(!Object.hasOwn(entry.options,'rustSourceVersion'));}
  }
  for(const entry of materializeProjectTranslationInputs(directory).inputs)assert.ok(!Object.hasOwn(entry.options,'rustSourceVersion'));
  assert.throws(()=>materializeProjectTranslationInputs(directory,{rustSourceVersion:'1.89.0'}),error=>error.reason===fixture.versionError);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
