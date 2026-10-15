import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdtempSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import test from 'node:test';
import vm from 'node:vm';
import {publicDeclarationConsumers} from '../scripts/check-public-declaration-contracts.mjs';
import {readProjectData} from '../scripts/project-data.mjs';

const modulePath=process.env.META_LANGUAGE_OWNED_TUPLE_PUBLIC_MODULE??fileURLToPath(new URL('../src/index.js',import.meta.url));
const api=await import(pathToFileURL(modulePath).href);
const fixture=readProjectData(new URL('../../experiments/owned-tuple-call-fixture.lino',import.meta.url));
const consumers=new Map(publicDeclarationConsumers(modulePath).map(entry=>[entry.name,entry]));
async function runConsumer(name,context){
 const consumer=consumers.get(name);assert.ok(consumer,'the actual declared owned-tuple consumer exists: '+name);
 const directory=mkdtempSync(join(tmpdir(),'meta-language-owned-tuple-public-consumer-'));context.after(()=>rmSync(directory,{recursive:true,force:true}));
 const file=join(directory,'consumer.mjs');writeFileSync(file,api.eraseTypeScript(consumer.source).code);return import(pathToFileURL(file).href);
}
function findTupleInitializer(entry){
 let node=entry.body;while(node.k==='let'){if(node.value.k==='call')return node.value;node=node.body;}
 assert.fail('the actual complete choose function must retain its initializer call');
}
function executeTupleCall(evidence,observations){
 const context={};vm.runInNewContext(evidence.javascript.code,context,{timeout:1000});
 const entry=evidence.javascript.mappings.find(row=>row.source==='choose').target;
 for(const[text,count,flag,expected]of observations)assert.equal(context[entry](text,BigInt(count),flag),expected);
 const seed=context.seed,alternate=context.alternate;let calls=0;
 context.seed=(...args)=>{calls++;return seed(...args);};if(alternate)context.alternate=(...args)=>{calls++;return alternate(...args);};
 const[text,count,flag,expected]=observations[0];assert.equal(context[entry](text,BigInt(count),flag),expected);assert.equal(calls,1);
}
function checkTupleCallPacket(evidence,target='seed'){
 const program=api.decodeTypedProgramLinks(evidence.network,evidence.root),choose=program.declarations.get('choose');
 assert.equal(findTupleInitializer(choose).fn,target);
 assert.equal(choose.sourceLocalState.model,fixture.model);assert.equal(choose.sourceLocalState.initializers.length,1);
 assert.equal(choose.sourceLocalState.initializers[0].target,target);assert.deepEqual(choose.sourceLocalState.initializers[0].fieldModes,['move','copy']);
 const helper=program.declarations.get(target);assert.equal(helper.ret.kind,'tuple');assert.equal(helper.ret.elements[0].kind,'string');assert.deepEqual(helper.ret.elements[1],{kind:'fixed',bits:32,signed:true});
 assert.equal(helper.sourceSignature.result.type.elements[1].bits,32);assert.equal(helper.sourceSignature.result.owner.kind,'tuple');
 const rows=evidence.network.fieldRelations(),calls=rows.filter(row=>row.label==='declaration reference');assert.equal(calls.length,1);
 const identity=rows.find(row=>row.label==='full name'&&evidence.network.link(row.child).metadata().definition===target).parent;
 assert.ok(calls[0].child.equals(identity),'the actual initializer references the actual owned declaration');
 assert.ok(evidence.rust.code.includes('pub fn choose('));assert.ok(evidence.rust.code.includes('crate::'+target+'('));
 return program;
}

for(const[index,entry]of fixture.cases.entries()){
 test('the exact public owned-tuple consumer executes the complete '+entry.name+' program with one owned call and retained provenance',async context=>{
  const consumer=await runConsumer('valid-owned-tuple-call-whole-program-'+index,context),evidence=consumer.contractEvidence;
  checkTupleCallPacket(evidence);executeTupleCall(evidence,entry.observations);
  assert.ok(evidence.network.links().every(link=>link.metadata().definition!==entry.source));
  const rows=evidence.network.fieldRelations(),profile=rows.find(row=>row.label==='source profile');assert.ok(profile);
  const selected=rows.find(row=>row.parent.equals(profile.child)&&row.label==='rust owned local state model');assert.ok(selected);assert.equal(evidence.network.link(selected.child).metadata().definition,fixture.model);
  const provenance=rows.find(row=>row.label==='user provenance');assert.ok(provenance);
  const caller=rows.find(row=>row.parent.equals(provenance.child)&&row.label==='consumer');assert.ok(caller);assert.equal(evidence.network.link(caller.child).metadata().definition,entry.name);
  if(index===0){
   assert.equal(Buffer.byteLength(evidence.rust.code),fixture.emittedArtifact.bytes);
   assert.equal(createHash('sha256').update(evidence.rust.code).digest('hex'),fixture.emittedArtifact.sha256);
   for(const[readerIndex,expected]of fixture.emittedArtifact.readers.entries())assert.throws(()=>consumer.reimportActualEmittedRust(evidence.rust.code,readerIndex===1),error=>{
    assert.deepEqual({kind:error.kind,reason:error.reason,span:error.span??null},{kind:expected.kind,reason:expected.reason,span:expected.span});return true;
   });
  }
 });
}

test('the exact public saved-link retarget changes the actual owned declaration and matching fact while retaining source headers and move/Copy modes',async context=>{
 const consumer=await runConsumer('valid-source-free-owned-tuple-call-declaration-retarget',context),evidence=consumer.contractEvidence,program=checkTupleCallPacket(evidence,'alternate');
 const choose=program.declarations.get('choose'),original=evidence.originalProgram.declarations.get('choose');
 assert.equal(findTupleInitializer(original).fn,'seed');assert.equal(original.sourceLocalState.initializers[0].target,'seed');
 assert.deepEqual(choose.sourceSignature,original.sourceSignature);assert.deepEqual(choose.sourceLocalState.bindings,original.sourceLocalState.bindings);
 assert.deepEqual(choose.sourceLocalState.initializers[0].fieldModes,original.sourceLocalState.initializers[0].fieldModes);
 executeTupleCall(evidence,[['before','1',false,'second'],['before','1',true,'selected']]);
});

for(const[index,expected]of fixture.refusals.entries()){
 test('the exact public selected owned-tuple source retains the frozen '+expected.name+' refusal',async context=>{
  await assert.rejects(runConsumer('valid-owned-tuple-call-selector-with-refused-source-'+index,context),error=>{
   assert.deepEqual({kind:error.kind,reason:error.reason,span:error.span??null},{kind:expected.kind,reason:expected.reason,span:expected.span});return true;
  });
 });
}

for(const selection of['call-only-retarget','fact-only-retarget','detached-declaration','string-header-width','wrong-tuple-item-type','reversed-field-modes']){
 test('the exact public saved owned-tuple '+selection+' counterfeit refuses decoding and both target emitters',async context=>{
  const consumer=await runConsumer('valid-owned-tuple-call-selector-with-malformed-links-'+selection,context),packet=consumer.malformedPacket;
  assert.throws(()=>api.decodeTypedProgramLinks(packet.network,packet.root));
  for(const target of['Rust','JavaScript'])assert.throws(()=>api.emitTypedProgramLinks(packet.network,packet.root,target));
 });
}

test('the exact undeclared public owned-tuple selector retains the unknown source-model refusal',async context=>{
 await assert.rejects(runConsumer('reject-unregistered-owned-tuple-call-model-version',context),error=>{
  assert.equal(error.kind,'unsupported');assert.equal(error.reason,'Rust owned local state model: select the original move profile or canonical String copy profile');return true;
 });
});

test('the exact declared owned-tuple selector requires Rust source',async context=>{
 const consumer=await runConsumer('valid-owned-tuple-call-selector-type-with-wrong-source-language',context);
 assert.throws(()=>consumer.createWrongLanguagePacket(),{name:'TypeError',message:'a Rust owned local state model requires Rust source'});
});

test('the exact undeclared public owned-tuple signature selector retains its independent header-model refusal',async context=>{
 await assert.rejects(runConsumer('reject-unregistered-owned-tuple-call-source-signature-model',context),error=>{
  assert.equal(error.kind,'unsupported');assert.equal(error.reason,'Rust source signature model: select the legacy model or Rust source function signature 1');return true;
 });
});

test('the exact declared owned-tuple selector refuses an unproved source-profile composition',async context=>{
 const consumer=await runConsumer('valid-owned-tuple-call-selector-type-with-independent-source-profile',context);
 assert.throws(()=>consumer.createIncompatibleSourcePacket(),error=>{
  assert.equal(error.kind,'unsupported');assert.equal(error.reason,'Rust owned local state model: other source state and completion profiles require independent composition proofs');return true;
 });
});
