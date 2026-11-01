// Deferred controls only. This module must not execute during source drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {RustSourceOrigins} from '../src/translation/rust-source-origins.js';
import {BOOL,STRING} from '../src/translation/types.js';
import {registerRustOwnedBinding,expireRustOwnedBindings} from '../src/translation/rust-owned-bindings.js';
import {readRustStandardSumOwnershipFacts,captureRustStandardSumConstructor,projectRustStandardSumPatternField,requireRustStandardSumPatternTransfer,validateRustStandardSumBorrowedResult,finishRustStandardSumFieldDereference,markRustStandardSumOwnedCallResult} from '../src/translation/rust-standard-sum-ownership.js';
import * as rules from '../src/translation/rust-standard-sum-ownership-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustStandardSumOwnershipDecisions as decisions,rustStandardSumOwnershipOriginals as originals,rustStandardSumOwnershipRefusals as refusals} from './fixtures/rust-standard-sum-ownership-transitions.mjs';
const versions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'];
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered ownership control Node '+process.versions.node);
for(const [index,row]of decisions.entries())test('owned sum source decision '+index+' '+row.operation,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
test('independent ownership inputs remain represented',()=>{assert.equal(decisions.length,41);assert.equal(originals.length,3);assert.equal(refusals.length,2);});

// Fault-injection fixtures exercise host transport premises, not source
// admission. Production origins/namespace lookup and ownership maps are used;
// these tiny catalog rows deliberately bypass parsing to probe stale evidence.
function catalog(){
 const table=new RustSourceOrigins(new Map()),text=table.typeOrigin(STRING,[]),scalar=table.typeOrigin(BOOL,[]);
 const origin={...table.unknownOrigin([]),proven:true,copyEvidence:false,type:{kind:'named',path:['actual_sum']},standardSumSource:{owner:'std::option::Option',arguments:[text],provisional:false}};
 const entry={k:'data',fullName:'actual_sum',standardSumSourceOrigin:origin};table.registerCatalogEntry('actual_sum',entry);
 return {table,text,scalar,origin,entry};
}
test('owned facts require the actual admitted catalog row independently of a supplied marker',()=>{
 const {table,origin,entry}=catalog();assert.equal(readRustStandardSumOwnershipFacts(table,origin).profile,'owned');assert.equal(origin.copyEvidence,false);
 for(const changed of [{proven:false},{wrappers:['reference']},{standardSumSource:{...origin.standardSumSource,provisional:true}}]){
  entry.standardSumSourceOrigin={...origin,...changed};assert.equal(readRustStandardSumOwnershipFacts(table,origin).profile,'');
 }
 entry.standardSumSourceOrigin=origin;
 assert.equal(readRustStandardSumOwnershipFacts(table,{...origin,wrappers:['reference']}).profile,'');
 assert.equal(readRustStandardSumOwnershipFacts(table,{...origin,type:{kind:'named',path:['missing']}}).profile,'');
});
test('borrowed fields retain complete actual root loans without extending scrutinee temporaries',()=>{
 const {table,origin,text}=catalog(),parent={...origin,wrappers:['reference'],sourceLifetimes:["'root"],sourceBindingIdentity:'root',sourcePlaceBindingIdentity:'place',sourceLoanBindingIdentities:['prior'],sourcePlaceLoanBindingIdentities:['other'],sourceExtendingTemporaryBindings:['candidate']};
 const field=projectRustStandardSumPatternField(table,parent,text,true);
 assert.deepEqual(field.wrappers,['reference']);assert.equal(field.sourceLifetimes[0],"'root");assert.equal(field.sourceStandardSumBorrowedField,true);assert.equal(field.sourceBorrowedPlace,true);
 assert.deepEqual(field.sourceLoanBindingIdentities,['root','prior','place','other']);assert.deepEqual(field.sourceExtendingTemporaryBindings,[]);
 assert.equal(field.sourceOwnedResultProof,false);assert.equal(field.copyEvidence,false);
});
test('a nested Copy sum keeps the outer owned root loan until its scalar is copied',()=>{
 const {table,origin,scalar}=catalog(),inner={...scalar,type:{kind:'named',path:['actual_copy_sum']},standardSumSource:{owner:'std::option::Option',arguments:[scalar],provisional:false}};
 table.registerCatalogEntry('actual_copy_sum',{k:'data',fullName:'actual_copy_sum',standardSumSourceOrigin:inner});
 const root=registerRustOwnedBinding(table,origin),borrowedInner=projectRustStandardSumPatternField(table,{...root,wrappers:['reference']},inner,true);
 const field=projectRustStandardSumPatternField(table,borrowedInner,scalar,true);
 assert.equal(field.sourceStandardSumBorrowedField,true);assert.equal(field.sourceBindingIdentity,root.sourceBindingIdentity);assert.deepEqual(field.sourceLoanBindingIdentities,[root.sourceBindingIdentity]);
 table.sourceBindingStates.set(root.sourceBindingIdentity,'moved');assert.throws(()=>finishRustStandardSumFieldDereference(table,{...field,wrappers:[]}),/moved source binding/);
});
test('owned pattern transfer needs its real root or fresh parent production contract',()=>{
 const {table,origin,text}=catalog(),field=projectRustStandardSumPatternField(table,origin,text,false);
 assert.equal(field.sourceStandardSumPatternMove,true);assert.throws(()=>requireRustStandardSumPatternTransfer(table,field),/fresh parent producer/);
 assert.doesNotThrow(()=>requireRustStandardSumPatternTransfer(table,{...field,sourceBindingIdentity:'actual root'}));
 const fresh=projectRustStandardSumPatternField(table,{...origin,sourceOwnedResultProof:true},text,false);assert.doesNotThrow(()=>requireRustStandardSumPatternTransfer(table,fresh));
});
test('escaping borrowed field validates actual loan states after lexical expiry',()=>{
 const {table,origin,text}=catalog(),root=registerRustOwnedBinding(table,origin),field=projectRustStandardSumPatternField(table,{...root,wrappers:['reference']},text,true);
 assert.doesNotThrow(()=>validateRustStandardSumBorrowedResult(table,field));
 assert.throws(()=>expireRustOwnedBindings(table,new Map(),field),/does not outlive/);
 assert.equal(table.sourceBindingStates.get(root.sourceBindingIdentity),'dropped');
 const parameter={...field,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[]};assert.doesNotThrow(()=>validateRustStandardSumBorrowedResult(table,parameter));
});
test('only an actual Copy field dereference detaches a live loan',()=>{
 const {table,origin,text,scalar}=catalog(),root=registerRustOwnedBinding(table,origin),parent={...root,wrappers:['reference']};
 const field=projectRustStandardSumPatternField(table,parent,scalar,true),dereferenced={...field,wrappers:[]};
 const value=finishRustStandardSumFieldDereference(table,dereferenced);assert.equal(value.sourceBindingIdentity,null);assert.deepEqual(value.sourceLoanBindingIdentities,[]);assert.equal(value.sourceStandardSumBorrowedField,false);
 const owned=projectRustStandardSumPatternField(table,parent,text,true),ownedDeref=finishRustStandardSumFieldDereference(table,{...owned,wrappers:[]});assert.equal(ownedDeref.sourceBorrowedPlace,true);assert.equal(ownedDeref.sourceBindingIdentity,root.sourceBindingIdentity);
 table.sourceBindingStates.set(root.sourceBindingIdentity,'dropped');assert.throws(()=>finishRustStandardSumFieldDereference(table,dereferenced),/does not outlive/);
});
test('constructor transfer bookkeeping prevents a second move and header capture stays provisional',()=>{
 const {table,origin,text}=catalog(),argument={k:'name',path:['payload']},node={k:'app',fn:{k:'name',path:['ActualSome']},args:[argument]};
 table.registerCatalogEntry('ActualSome',{k:'ctor',owner:'actual_sum',fields:[text],result:origin});
 let transfers=0;table.prepareOwnedArgument=actual=>{transfers+=1;table.ownedArgumentTransfers.add(actual);};
 captureRustStandardSumConstructor({origins:table},node,origin,[argument],[]);assert.equal(transfers,1);assert.equal(table.expressions.get(node).sourceOwnedResultProof,true);assert.equal(table.expressions.get(node).copyEvidence,false);
 captureRustStandardSumConstructor({origins:table},node,origin,[argument],[]);assert.equal(transfers,1);
 const provisional={...origin,proven:false,standardSumSource:{...origin.standardSumSource,provisional:true}};captureRustStandardSumConstructor({origins:table,headerDiscovery:true},node,provisional,[argument],[]);assert.equal(transfers,1);assert.equal(table.expressions.get(node).proven,false);assert.notEqual(table.expressions.get(node).sourceOwnedResultProof,true);
});
test('owned call freshness needs both recorded syntax and the actual function result owner',()=>{
 const {table,origin}=catalog(),node={k:'app',fn:{k:'name',path:['make']},args:[],span:{start:1,end:2}};
 table.registerCatalogEntry('make',{k:'fn',parameters:[],result:origin});assert.notEqual(markRustStandardSumOwnedCallResult(table,node,[],origin).sourceOwnedResultProof,true);
 table.freshCalls.add('1:2');assert.equal(markRustStandardSumOwnedCallResult(table,node,[],origin).sourceOwnedResultProof,true);
 table.registerCatalogEntry('make',{k:'const',result:origin});assert.notEqual(markRustStandardSumOwnedCallResult(table,node,[],origin).sourceOwnedResultProof,true);
});
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const sourceVersion of ['1.89.0','1.100.0'])test('owned sum frontend reports uncovered source version '+sourceVersion,()=>{
 const source='fn main(){let value:Option<String>=None;}';
 for(const read of [()=>parseRust(source,{sourceVersion}),()=>createTypedProgramLinks(source,'Rust',{rustSourceVersion:sourceVersion})])assert.throws(read,error=>error.kind==='unsupported'&&error.reason.includes('uncovered'));
});
for(const row of originals)for(const sourceVersion of versions)test('actual ownership source '+row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const concrete=LinkNetwork.parse(row.source,'Rust');
 for(const graph of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(graph.reconstructText(),row.source);
 const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(stdout(emitJavaScript(checked).text),row.stdout);
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,provenance:{fixture:row.name}}),typedText=packet.network.toLino(),typedBinary=encodeLinkNetworkBinary(packet.network);
 const baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(typedText),decodeLinkNetworkBinary(typedBinary)]){
  assert.equal(graph.toLino(),typedText);const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);
  for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);assert.ok(emitted.code.length>0);if(target==='JavaScript')assert.equal(stdout(emitted.code),row.stdout);}
 }
});
for(const row of refusals)for(const sourceVersion of versions)test('actual ownership refusal '+row.name+'/'+sourceVersion,()=>{
 for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})])assert.throws(read,error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));assert.ok(error.reason.length>0);return true;});
});
