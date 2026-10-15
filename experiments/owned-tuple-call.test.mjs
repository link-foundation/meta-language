import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {checkRustOwnedLocalStateProof} from '../js/src/translation/rust-owned-local-state.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {ownedTupleCallFixture} from './generate-owned-tuple-call-fixture.mjs';
import {rustOwnedTupleCallRules} from './generate-rust-owned-tuple-call-rules.mjs';
const fixture=readProjectData(new URL('./owned-tuple-call-fixture.lino',import.meta.url));
const parse=entry=>checkProgram(parseRust(entry.source,{localValueStateModel:fixture.model}));
function reload(program){const archive=encodeTypedProgramLinks(program);return{...archive,network:LinkNetwork.fromLino(archive.network.toLino())};}
function execute(program,observations,sourceName='choose'){
 const emitted=emitJavaScript(program),context={};vm.runInNewContext(emitted.text,context,{timeout:1000});
 const name=emitted.mappings.find(row=>row.source===sourceName).target;
 for(const[text,count,flag,expected]of observations)assert.equal(context[name](text,BigInt(count),flag),expected);
 return{context,emitted,name};
}
function call(entry){let node=entry.body;while(node.k==='let'){if(node.value.k==='call')return node.value;node=node.body;}throw new Error('missing actual initializer call');}

test('all complete selected programs execute from checked Links with one real declaration-owned initializer',()=>{
 assert.equal(fixture.cases.length,5);
 for(const entry of fixture.cases){
  const archive=reload(parse(entry)),program=decodeTypedProgramLinks(archive.network,archive.root);
  assert.ok(archive.network.links().every(link=>link.metadata().definition!==entry.source));
  assert.equal(archive.network.fieldRelations().filter(row=>row.label===TYPED_PROGRAM_LINKS_SCHEMA.references.declaration).length,1);
  const target=program.declarations.get('choose').sourceLocalState.initializers[0];
  assert.equal(target.target,'seed');assert.deepEqual(target.fieldModes,['move','copy']);
  assert.equal(call(program.declarations.get('choose')).fn,'seed');
  const {context,name}=execute(program,entry.observations),seed=context.seed;let calls=0;
  context.seed=(...args)=>{calls++;return seed(...args);};
  const[text,count,flag,expected]=entry.observations[0];assert.equal(context[name](text,BigInt(count),flag),expected);assert.equal(calls,1);
 }
});

test('source-free target edits change actual execution only with matching checked owner and move/Copy facts',()=>{
 const entry=fixture.cases.find(row=>row.name==='source-free target'),archive=reload(parse(entry));
 const program=decodeTypedProgramLinks(archive.network,archive.root),fn=program.declarations.get('choose'),originalHeader=structuredClone(fn.sourceSignature);
 call(fn).fn='alternate';fn.sourceLocalState.initializers[0].target='alternate';
 const changed=reload(program),decoded=decodeTypedProgramLinks(changed.network,changed.root);
 assert.equal(call(decoded.declarations.get('choose')).fn,'alternate');
 assert.deepEqual(decoded.declarations.get('choose').sourceSignature,originalHeader);
 assert.deepEqual(decoded.declarations.get('choose').sourceLocalState.initializers[0].fieldModes,['move','copy']);
 execute(decoded,[['before','1',false,'second'],['before','1',true,'selected']]);
 const corruptions=[
  value=>{call(value.declarations.get('choose')).fn='alternate';},
  value=>{value.declarations.get('choose').sourceLocalState.initializers[0].target='alternate';},
  value=>{value.declarations.get('choose').sourceLocalState.initializers[0].fieldModes.reverse();},
  value=>{value.declarations.get('choose').sourceLocalState.initializers[0].fields.reverse();},
  value=>{value.declarations.get('choose').sourceLocalState.reads=[];},
  value=>{value.declarations.get('choose').sourceLocalState.bindings.find(row=>row.role==='tuple field'&&row.index===1).mutable=true;},
  value=>{call(value.declarations.get('choose')).borrowEscape=true;},
  value=>{const target=value.declarations.get('seed');target.body={...call(value.declarations.get('choose')),fn:'alternate'};},
 ];
 for(const mutate of corruptions){const value=decodeTypedProgramLinks(archive.network,archive.root);mutate(value);assert.throws(()=>encodeTypedProgramLinks(value),/tuple|state|owner|domain|read|call|expression|binder|replacement/u);}
});

test('the new concept rejects its exact source counterexamples and preserves previous source-model oracles',()=>{
 assert.equal(fixture.refusals.length,13);
 for(const entry of fixture.refusals)assert.throws(()=>parse(entry),error=>{assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason,entry.reason,entry.name);assert.deepEqual(error.span??null,entry.span,entry.name);return true;});
 const previous=readProjectData(new URL('./rust-owned-local-state-fixture.lino',import.meta.url));
 for(const entry of previous.cases){const archive=reload(checkProgram(parseRust(entry.source,{localValueStateModel:previous.models.moves})));execute(decodeTypedProgramLinks(archive.network,archive.root),entry.observations,entry.name);}
});

test('direct selected checking and emission reject a forged tuple item before relying on Links encoding',()=>{
 const program=parse(fixture.cases[0]),helper=program.declarations.get('seed');
 helper.body.items[0]=structuredClone(helper.body.items[1]);helper.sourceLocalState.reads=[];
 const expected=/actual tuple item differs from its declared owned field type/u;
 assert.throws(()=>checkRustOwnedLocalStateProof(program),expected);
 assert.throws(()=>emitJavaScript(program),expected);
 assert.throws(()=>encodeTypedProgramLinks(program),/tuple|owned field/u);
});

test('direct selected checking and emission validate the actual conditional and local result types',()=>{
 const conditional=parse(fixture.cases[0]),helper=conditional.declarations.get('seed');
 const text=helper.body.items[0],count=helper.body.items[1];
 helper.body.items[0]={k:'if',cond:{k:'lit',value:true,type:{kind:'bool'},span:text.span},then:structuredClone(count),else:text,type:text.type,span:text.span};
 helper.sourceLocalState.reads[0].path=['items',0,'else'];
 for(const run of[checkRustOwnedLocalStateProof,emitJavaScript])assert.throws(()=>run(conditional),/actual conditional branches differ/u);
 assert.throws(()=>encodeTypedProgramLinks(conditional),/conditional|branch/u);
 const local=parse(fixture.cases[0]),fn=local.declarations.get('choose'),path=[];let last=fn.body;
 while(last.body.k==='let'){path.push('body');last=last.body;}
 last.body={k:'var',name:'count',type:fn.params[1].type};
 fn.sourceLocalState.reads=fn.sourceLocalState.reads.filter(read=>JSON.stringify(read.path)!==JSON.stringify([...path,'body']));
 for(const run of[checkRustOwnedLocalStateProof,emitJavaScript])assert.throws(()=>run(local),/actual local body differs/u);
 assert.throws(()=>encodeTypedProgramLinks(local),/type|result/u);
});

test('new shared rule decisions themselves replay from checked Links without a native implementation',()=>{
 const source=rustOwnedTupleCallRules();assert.equal(readFileSync(new URL('../js/src/translation/rust-owned-tuple-call-rules.js',import.meta.url),'utf8'),source);
 const archive=reload(checkProgram(parseJavaScript(source))),program=decodeTypedProgramLinks(archive.network,archive.root),emission=emitJavaScript(program),context={};
 vm.runInNewContext(emission.text,context,{timeout:1000});
 const model=context[emission.mappings.find(row=>row.source==='acceptRustOwnedTupleCallModel').target],accept=context[emission.mappings.find(row=>row.source==='acceptRustOwnedTupleCallInitializer').target];
 assert.equal(model(fixture.model),true);for(const value of['','closed owned text local state 1','unknown'])assert.equal(model(value),false);
 for(const target of[false,true])for(const argumentsMatch of[false,true])for(const fieldsMatch of[false,true])assert.equal(accept(target,argumentsMatch,fieldsMatch),target&&argumentsMatch&&fieldsMatch);
});

test('the complete authenticated formatter stays an unresolved embedding with its original mutable tuple and library calls',()=>{
 assert.deepEqual(ownedTupleCallFixture(),fixture);
 const original=readProjectData(new URL('./complete-binary-formatter-probe.lino',import.meta.url));
 assert.deepEqual(fixture.authentic.files,original.files);assert.equal(fixture.authentic.combined.bytes,132002);assert.equal(fixture.authentic.combined.sha256,original.combined.sha256);
 assert.equal(fixture.authentic.initializer.originalText,'let (mut digits, n) = decimal(&format!("{:e}", x.abs()));');
 assert.equal(fixture.authentic.definition.originalText,'fn decimal(text: &str) -> (String, i32)');
 assert.equal(fixture.authentic.selectedRefusal.kind,'unsupported');assert.match(fixture.authentic.selectedRefusal.reason,/direct standard String, i32 and Boolean interfaces/u);
 assert.ok(fixture.authentic.occurrences.some(row=>row.name==='n'));assert.ok(fixture.authentic.unresolved.length>=4);
});
