import assert from 'node:assert/strict';
import {mkdtempSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import test from 'node:test';
import vm from 'node:vm';
import {publicDeclarationConsumers,selectPublicDeclarationDiagnostics} from '../scripts/check-public-declaration-contracts.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {readAssertionVersionPolicy} from '../scripts/antlr-version-assertions.mjs';
import {assertionVersionPolicy} from '../scripts/generate-assertion-version-policy.mjs';

const modulePath=process.env.META_LANGUAGE_CONST_PUBLIC_MODULE??fileURLToPath(new URL('../src/index.js',import.meta.url));
const moduleUrl=pathToFileURL(modulePath),api=await import(moduleUrl.href);
const {emitRust}=await import(new URL('./translation/emit-rust.js',moduleUrl));
const fixture=readProjectData(new URL('../../experiments/rust-const-function-fixture.lino',import.meta.url));
const consumers=new Map(publicDeclarationConsumers(modulePath).map(entry=>[entry.name,entry]));
async function runConsumer(name,context){
 const consumer=consumers.get(name);assert.ok(consumer,'the actual declared const consumer exists: '+name);
 const directory=mkdtempSync(join(tmpdir(),'meta-language-const-public-consumer-'));context.after(()=>rmSync(directory,{recursive:true,force:true}));
 const file=join(directory,'consumer.mjs');writeFileSync(file,api.eraseTypeScript(consumer.source).code);return import(pathToFileURL(file).href);
}
function observe(code,entry,edited=false){
 const runtime={};vm.runInNewContext(code,runtime,{timeout:1000});const fn=runtime[entry.name];assert.equal(typeof fn,'function');
 if(entry.name==='accept_rust_owned_local_record_value')for(const required of[false,true])for(const value of[false,true])assert.equal(fn(required,value),edited?!required&&value:!required||value);
 else if(entry.name==='accept_rust_owned_local_source_span')for(const present of[false,true])for(const record of[false,true])for(const offset of[false,true])assert.equal(fn(present,record,offset),!present||record&&offset);
 else for(const start of[-Infinity,-1,-0,0,1,Infinity,NaN])for(const end of[-Infinity,-1,-0,0,1,Infinity,NaN])assert.equal(fn(start,end),start<=end);
}

for(const entry of fixture.cases)for(const selection of['implicit','explicit']){
 test('the exact '+selection+' public const consumer retains '+entry.name+' and actual complete definition replay',async context=>{
  const consumer=await runConsumer('valid-const-predicate-'+selection+'-'+entry.name,context),evidence=consumer.contractEvidence;
  let program=api.decodeTypedProgramLinks(evidence.network,evidence.root);
  assert.deepEqual(program.items[0].sourceConstFunction,entry.proof);assert.deepEqual(program.items[0].sourceSignature,entry.signature);
  assert.equal(evidence.code,entry.javascript);observe(evidence.code,entry);
  const relations=evidence.network.fieldRelations(),profile=relations.find(row=>row.label==='source profile');assert.ok(profile);
  const signature=relations.find(row=>row.parent.equals(profile.child)&&row.label==='rust source signature model');
  if(selection==='explicit')assert.equal(evidence.network.link(signature.child).metadata().definition,'Rust source function signature 1');else assert.equal(signature,undefined);
  assert.throws(()=>api.createTypedProgramLinks(entry.source,'Rust'),error=>error.kind===entry.originalRefusal.kind&&error.reason===entry.originalRefusal.reason);
  for(let cycle=0;cycle<3;cycle++){
   const emitted=emitRust(program);assert.ok(emitted.definitions.every(value=>value.startsWith('pub const fn ')));
   if(cycle===0){assert.equal(emitted.definitions[0],entry.rustDefinition);assert.ok(evidence.rustCode.includes(emitted.definitions.join('\n\n')));}
   const replay=consumer.reimportActualRustDefinitions(emitted.definitions.join('\n\n'));program=replay.program;
   assert.equal(program.items[0].sourceConstFunction.keyword,'const');assert.deepEqual(program.items[0].sourceSignature,entry.signature);observe(replay.code,entry);
  }
  // Empty source-header selection is an existing JavaScript runtime boundary;
  // the public TypeScript selector continues to expose its canonical spelling.
  const empty=api.createTypedProgramLinks(entry.source,'Rust',{rustConstFunctionModel:fixture.model,rustSourceSignatureModel:''});
  assert.equal(api.emitTypedProgramLinks(empty.network,empty.root,'JavaScript').code,entry.javascript);
  const emptyRows=empty.network.fieldRelations(),emptyProfile=emptyRows.find(row=>row.label==='source profile');
  assert.ok(emptyRows.some(row=>row.parent.equals(emptyProfile.child)&&row.label==='rust source signature model'&&empty.network.link(row.child).metadata().definition===''));
 });
}

test('the exact public const predicate link edit changes Boolean meaning while retaining qualification and owners',async context=>{
 const consumer=await runConsumer('valid-source-free-const-predicate-operation-edit',context),evidence=consumer.contractEvidence,entry=fixture.cases[0];
 const program=api.decodeTypedProgramLinks(evidence.network,evidence.root);
 assert.deepEqual(program.items[0].sourceConstFunction,entry.proof);assert.deepEqual(program.items[0].sourceSignature,entry.signature);assert.equal(program.items[0].body.op,'and');
 observe(evidence.originalCode,entry);observe(evidence.code,entry,true);
 const emitted=emitRust(program);assert.ok(evidence.rustCode.includes(emitted.definitions.join('\n\n')));assert.ok(emitted.definitions[0].startsWith('pub const fn '));
 observe(consumer.reimportActualRustDefinitions(emitted.definitions.join('\n\n')).code,entry,true);
});

test('the exact unknown public const selector keeps its runtime model refusal',async context=>{
 await assert.rejects(runConsumer('reject-unregistered-const-predicate-model-version',context),error=>{
  assert.equal(error.kind,'unsupported');assert.equal(error.reason,'Rust const function model: select the legacy model or closed pure Rust const predicates 1');return true;
 });
});

test('the exact declared const selector still requires Rust source',async context=>{
 const consumer=await runConsumer('valid-const-predicate-selector-type-with-wrong-source-language',context);
 assert.throws(()=>consumer.createWrongLanguagePacket(),{name:'TypeError',message:'a Rust const function model requires Rust source'});
});

test('the exact unknown explicit const signature selection keeps its independent runtime refusal',async context=>{
 await assert.rejects(runConsumer('reject-unregistered-explicit-const-predicate-signature-model',context),error=>{
  assert.equal(error.kind,'unsupported');assert.equal(error.reason,'Rust source signature model: select the legacy model or Rust source function signature 1');return true;
 });
});

test('the unchanged unknown const model selects exact compiler diagnostics with no uncovered fallback',()=>{
 const consumer=consumers.get('reject-unregistered-const-predicate-model-version');
 assert.ok(consumer);assert.match(consumer.source,/rustConstFunctionModel:'closed pure Rust const predicates 99'/u);
 assert.deepEqual(consumer.expectedDiagnostics,['TS2820']);
 assert.equal(consumer.assertionPolicy,'typescript-const-model-rejection-diagnostic');
 const registry=readAssertionVersionPolicy();assert.deepEqual(registry.policy,assertionVersionPolicy());
 const policy=registry.policy.assertions.find(entry=>entry.id===consumer.assertionPolicy);
 assert.deepEqual(policy.expectations.map(entry=>[entry.minimum,entry.maximum,entry.includeMinimum,entry.includeMaximum]),
  [['5.9.2','5.9.2',true,true],['7.0.2','7.0.2',true,true]]);
 for(const version of['5.9.2','7.0.2'])assert.deepEqual(selectPublicDeclarationDiagnostics(consumer,version,registry),['TS2820']);
 for(const version of['5.9.1','5.9.3','6.0.0','7.0.1','7.0.3','8.0.0'])assert.throws(
  ()=>selectPublicDeclarationDiagnostics(consumer,version,registry),/uncovered active version/u);
 assert.throws(()=>selectPublicDeclarationDiagnostics({...consumer,assertionPolicy:'unknown compiler policy'},'5.9.2',registry),/exactly one registered assertion policy/u);
 const duplicate={policy:{...registry.policy,assertions:[...registry.policy.assertions,policy]}};
 assert.throws(()=>selectPublicDeclarationDiagnostics(consumer,'5.9.2',duplicate),/exactly one registered assertion policy/u);
 const independent=consumers.get('reject-unregistered-explicit-const-predicate-signature-model');
 assert.deepEqual(selectPublicDeclarationDiagnostics(independent,'5.9.2',registry),['TS2322']);
});

test('the public Boolean const model executes its four declared decision outcomes',async context=>{
 const consumer=await runConsumer('valid-Boolean-const-decision-tree-model',context),evidence=consumer.contractEvidence;
 assert.equal(evidence.model,'closed Boolean Rust const decision trees 1');
 assert.match(evidence.rustCode,/pub const fn selected\(/u);
 const runtime={};vm.runInNewContext(evidence.code,runtime,{timeout:1000});
 for(const flag of[false,true])for(const value of[false,true])assert.equal(runtime.selected(flag,value),flag?value:!value);
});

test('the archive parser public factory executes its archive and refuses all three owned options',async context=>{
 const consumer=await runConsumer('valid-global-concept-archive-parser-factory',context);
 assert.equal(typeof consumer.archive,'string');assert.ok(consumer.archive.length>0);
 assert.equal(consumer.contractEvidence.accepted,true);assert.ok(consumer.contractEvidence.tree);
 for(const option of['resolveGrammar','grammarName','globalConceptSpace']){
  const name='reject-global-concept-archive-parser-owned-option-'+option;
  assert.deepEqual(consumers.get(name).expectedDiagnostics,['TS2322']);
  const rejected=await runConsumer(name,context);
  assert.throws(()=>rejected.createInvalidParser(consumer.archive),{name:'TypeError',
   message:'the archived global concept space owns parser option '+option});
 }
});
