import assert from 'node:assert/strict';
import {mkdtempSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import test from 'node:test';
import vm from 'node:vm';
import {publicDeclarationConsumers} from '../scripts/check-public-declaration-contracts.mjs';
import {readProjectData} from '../scripts/project-data.mjs';

const modulePath=process.env.META_LANGUAGE_RUNTIME_SHELL_PUBLIC_MODULE??fileURLToPath(new URL('../src/index.js',import.meta.url));
const api=await import(pathToFileURL(modulePath).href);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-runtime-shell.lino',import.meta.url));
const consumers=new Map(publicDeclarationConsumers(modulePath).map(entry=>[entry.name,entry]));
async function runConsumer(name,context){
 const consumer=consumers.get(name);assert.ok(consumer,'the actual declared whole-runtime consumer exists: '+name);
 const directory=mkdtempSync(join(tmpdir(),'meta-language-runtime-shell-public-consumer-'));context.after(()=>rmSync(directory,{recursive:true,force:true}));
 const file=join(directory,'consumer.mjs');writeFileSync(file,api.eraseTypeScript(consumer.source).code);return import(pathToFileURL(file).href);
}
function executeWholeJavaScript(code,names=[]){
 const lines=[],runtime=vm.createContext({console:{log:(...args)=>lines.push(args.join(' '))}});
 vm.runInContext(code,runtime,{timeout:1000});
 const calls=[];for(const name of names){assert.equal(typeof runtime[name],'function');runtime[name]=()=>{calls.push(name);return null;};}
 if(names.length)vm.runInContext('main();',runtime,{timeout:1000});
 return {output:lines.join('\n'),calls};
}
function readOwnedRuntimeOperations(node,result=[]){
 if(node.kind==='source operation'){result.push(node);node.parts.forEach(child=>readOwnedRuntimeOperations(child,result));}
 return result;
}
function checkRuntimeEvidence(evidence,entry,shift='28'){
 const program=api.decodeTypedProgramLinks(evidence.network,evidence.root),proof=program.sourceRuntimeShell;
 assert.deepEqual([...program.declarations.keys()],entry.names);
 assert.deepEqual(program.main.effects.map(effect=>effect.value.fn),entry.calls);
 assert.equal(proof.model,fixture.model);assert.equal(proof.observationModel,fixture.observationModel);
 assert.equal(proof.span.start,0);assert.equal(proof.tokens.at(-1).kind,'eof');assert.equal(proof.tokens.at(-1).span.end,proof.span.end);
 assert.equal(new Set(proof.tokens).size,proof.tokens.length);
 for(const token of proof.tokens){
  if(token.declaration)assert.equal(program.declarations.get(token.declaration.fullName),token.declaration);
  if(token.binding)assert.ok(proof.tokens.includes(token.binding));
 }
 assert.equal(program.main.sourceRuntimeWorker.model,fixture.model);assert.equal(program.main.sourceRuntimeWorker.observationModel,fixture.observationModel);
 assert.deepEqual(program.main.sourceRuntimeWorker.span,program.main.span);
 const operations=readOwnedRuntimeOperations(proof.operations);assert.equal(proof.assumptions.length,4);
 for(const row of proof.assumptions)assert.ok(operations.includes(row.operationNode),'the actual assumption owns an actual operation link');
 for(const emitted of[evidence.javascript,evidence.rust]){
  assert.equal(emitted.assumptions.length,4);assert.deepEqual(emitted.assumptions.map(row=>row.id),proof.assumptions.map(row=>row.id));
  for(const [index,row]of emitted.assumptions.entries()){
   const saved=proof.assumptions[index],operation=saved.operationNode;
   assert.equal(row.statement,saved.meaning);assert.deepEqual(row.details,['source operation '+operation.op+' at '+operation.span.start+'..'+operation.span.end]);
  }
  assert.ok(emitted.encodings.some(row=>row.id==='closed-pure-rust-runtime-shell'));
 }
 for(const fragment of['std::env::args().any','std::thread::Builder::new()','.stack_size(1 << '+shift+')','.spawn(move ||','.expect("spawn the program thread")','worker.join().is_err()','std::process::exit(101);'])assert.ok(evidence.rust.code.includes(fragment),fragment);
 const observed=executeWholeJavaScript(evidence.javascript.code,entry.names);assert.equal(observed.output,entry.output);assert.deepEqual(observed.calls,entry.calls);
 return program;
}

for(const entry of fixture.cases.filter(row=>['empty-program','ordered-repeated'].includes(row.id))){
 test('the exact public whole-runtime consumer retains '+entry.id+' through three complete Rust artifacts and actual JavaScript executions',async context=>{
  const consumer=await runConsumer('valid-runtime-shell-whole-artifact-'+entry.id,context);
  assert.equal(executeWholeJavaScript(entry.original).output,entry.output);
  assert.equal(consumer.contractEvidence.originalRustCode,entry.fullEmittedRust);
  let evidence=consumer.contractEvidence;checkRuntimeEvidence(evidence,entry);
  const rows=evidence.network.fieldRelations(),profile=rows.find(row=>row.label==='source profile');assert.ok(profile);
  const selection=rows.find(row=>row.parent.equals(profile.child)&&row.label==='rust runtime shell model');assert.ok(selection);
  assert.equal(evidence.network.link(selection.child).metadata().definition,fixture.model);
  const provenance=rows.find(row=>row.label==='user provenance');assert.ok(provenance);
  const owner=rows.find(row=>row.parent.equals(provenance.child)&&row.label==='consumer');assert.ok(owner);assert.equal(evidence.network.link(owner.child).metadata().definition,entry.id);
  for(let cycle=0;cycle<3;cycle++){
   const source=evidence.rust.code;evidence=consumer.reimportWholeRustArtifact(source);
   const program=checkRuntimeEvidence(evidence,entry);assert.equal(program.sourceRuntimeShell.span.end,source.length);
  }
 });
}

test('the exact public saved-link resource edit retains owned tokens and four conditional assumptions through complete replay',async context=>{
 const consumer=await runConsumer('valid-source-free-runtime-shell-resource-edit',context),entry=fixture.cases.find(row=>row.id==='ordered-repeated');
 let evidence=consumer.contractEvidence;const program=checkRuntimeEvidence(evidence,entry,'20');
 const shift=readOwnedRuntimeOperations(program.sourceRuntimeShell.operations).find(node=>node.op==='unsigned stack size shift').parts.at(-1);
 assert.ok(program.sourceRuntimeShell.tokens.includes(shift));assert.equal(shift.value,'20');
 const replay=await runConsumer('valid-runtime-shell-whole-artifact-ordered-repeated',context);
 for(let cycle=0;cycle<3;cycle++){evidence=replay.reimportWholeRustArtifact(evidence.rust.code);checkRuntimeEvidence(evidence,entry,'20');}
});

test('the exact undeclared public whole-runtime model keeps its runtime refusal',async context=>{
 await assert.rejects(runConsumer('reject-unregistered-runtime-shell-model-version',context),error=>{
  assert.equal(error.kind,'unsupported');assert.equal(error.reason,'Rust runtime shell model: select the legacy model or closed pure Rust unit runtime shell 1');return true;
 });
});

test('the exact declared whole-runtime selector requires Rust source',async context=>{
 const consumer=await runConsumer('valid-runtime-shell-selector-type-with-wrong-source-language',context);
 assert.throws(()=>consumer.createWrongLanguagePacket(),{name:'TypeError',message:'a Rust runtime shell model requires Rust source'});
});

test('the exact declared whole-runtime selector refuses an unproved source-profile composition',async context=>{
 const consumer=await runConsumer('valid-runtime-shell-selector-type-with-independent-source-profile',context);
 assert.throws(()=>consumer.createIncompatibleSourcePacket(),error=>{
  assert.equal(error.kind,'unsupported');assert.equal(error.reason,'closed Rust runtime shell: the whole runtime interpretation requires its independent source profile');return true;
 });
});

test('the exact declared whole-runtime selector refuses an unproved target-profile composition',async context=>{
 const consumer=await runConsumer('valid-runtime-shell-selector-type-with-independent-target-profile',context);
 assert.throws(()=>consumer.emitIncompatibleTargetPacket(),error=>{
  assert.equal(error.kind,'unsupported');assert.equal(error.reason,'closed Rust runtime shell: target header changes require an independent runtime composition proof');return true;
 });
});

test('the exact whole-artifact consumers preserve current default Rust and generated JavaScript replay refusals',async context=>{
 const consumer=await runConsumer('valid-runtime-shell-legacy-replay-boundaries',context);
 assert.equal(consumer.replayBoundaryEvidence.rustSource,fixture.cases.find(row=>row.id==='ordered-repeated').fullEmittedRust);
 assert.equal(consumer.replayBoundaryEvidence.javaScriptSource,`// Translated from Rust by meta-language: portable core, JavaScript target.
'use strict';

function alpha() {
  return null;
}

function beta() {
  return null;
}

function main() {
  const ml_main_ignored1 = alpha();
  const ml_main_ignored2 = beta();
  const ml_main_ignored3 = alpha();
}

main();
`);
 assert.throws(()=>consumer.createWithoutSelectedRustModel(),error=>{
  assert.deepEqual({kind:error.kind,reason:error.reason,span:error.span},{kind:'unsupported',reason:'function without a result: ml_main returns (); only value-returning functions are portable',span:{start:245,end:258}});return true;
 });
 assert.throws(()=>consumer.reimportCompleteJavaScriptWithoutGeneratedBindingModel(),error=>{
  assert.deepEqual({kind:error.kind,reason:error.reason,span:error.span},{kind:'unsupported',reason:"reserved identifier: ml_main_ignored1 uses the translator's reserved ml_ prefix",span:{start:190,end:206}});return true;
 });
});
