// Deferred source contracts. Nothing here is recorded execution evidence.
import test from 'node:test';import assert from 'node:assert/strict';import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';import {checkProgram} from '../src/translation/check.js';import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';import {LinkNetwork} from '../src/network.js';import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-owned-record-state-rules.js';
import {beginRustOwnedRecordStateBlock,endRustOwnedRecordStateBlock,registerRustOwnedRecordMutableBinding,readRustOwnedRecordMutableName} from '../src/translation/rust-owned-record-state.js';
import {RustSourceOrigins} from '../src/translation/rust-source-origins.js';import {STRING} from '../src/translation/types.js';import {cloneRustCallableTypeOrigin} from '../src/translation/rust-function-pointers.js';
import {rustOwnedRecordStateDecisions as decisions,rustOwnedRecordStateOriginals as originals,rustOwnedRecordStateRefusals as refusals,rustOwnedRecordStateReferences as references,rustOwnedRecordStateRefusalHistory as refusalHistory} from './fixtures/rust-owned-record-state.mjs';
const versions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'];
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered record SSA state Node '+process.versions.node);
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const [index,row]of decisions.entries())test('record state source decision '+index+' '+row.operation,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
test('all independent state inputs and source rationale stay represented',()=>{assert.equal(decisions.length,37);assert.equal(originals.length,13);assert.equal(refusals.length,9);assert.equal(refusalHistory.length,10);assert.deepEqual(refusals.map(row=>row.previousIndex),[0,1,2,3,4,5,6,8,9]);assert.equal(references.length,3);assert.ok(references.every(value=>value.startsWith('https://doc.rust-lang.org/reference/')));});
test('private collecting sites preserve scope identity without supplying runtime ownership',()=>{
 // This hand-authored catalog probes transport premises. It is not evidence
 // that a source program or nominal ownership graph has been admitted.
 const table=new RustSourceOrigins(),text=table.typeOrigin(STRING,[]),origin={...table.unknownOrigin([]),type:{kind:'named',path:['Pair']},proven:false,copyEvidence:false,sourceOwner:{kind:'declaration',path:['Pair'],arguments:[]}};
 table.registerCatalogEntry('Pair',{k:'data',fullName:'Pair'});table.registerCatalogEntry('Pair.Pair',{k:'ctor',owner:'Pair',fields:[text,text],result:origin});table.scope.set('value',origin);
 let serial=0;const parser={origins:table,namedRecords:new Map([['Pair',{owner:'Pair',constructor:'Pair.Pair',names:['left','right'],syntax:'named',update:true}]]),ordinaryCompletionContext:()=>true,freshProductBinding:()=>`private_test_value${serial++}`};
 const frame=beginRustOwnedRecordStateBlock(parser,true),name=registerRustOwnedRecordMutableBinding(parser,'value',{start:0,end:7});
 assert.notEqual(name,'value');assert.equal(table.sourceBindingStates.size,0);assert.ok(!table.scope.get('value').sourceBindingIdentity);assert.ok(!table.scope.get('value').sourceLexicalBindingIdentity);assert.equal(table.scope.get('value').proven,false);assert.equal(table.scope.get('value').copyEvidence,false);assert.equal(table.standardSumSourceNeedsFinalAdmission,true);
 const before=table.enter();assert.deepEqual(readRustOwnedRecordMutableName(parser,{k:'name',path:['value']},[]).path,[name]);
 table.bind('value',{k:'bool',value:true},[]);assert.deepEqual(readRustOwnedRecordMutableName(parser,{k:'name',path:['value']},[]).path,['value']);assert.ok(!table.scope.get('value').sourceRecordStateCollectingSite);
 table.scope=before;const type=cloneRustCallableTypeOrigin(table.scope.get('value'));assert.ok(!type.sourceRecordStateCollectingSite);assert.equal(type.proven,false);
 endRustOwnedRecordStateBlock(parser,frame);
});
for(const row of originals)for(const sourceVersion of versions)test('actual pure-core record SSA '+row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const concrete=LinkNetwork.parse(row.source,'Rust'),text=concrete.toLino(),binary=encodeLinkNetworkBinary(concrete);
 for(const graph of [concrete,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)])assert.equal(graph.reconstructText(),row.source);
 const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(stdout(emitJavaScript(checked).text),row.stdout);
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,provenance:{fixture:row.name}}),typedText=packet.network.toLino(),typedBinary=encodeLinkNetworkBinary(packet.network),baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(typedText),decodeLinkNetworkBinary(typedBinary)]){assert.equal(graph.toLino(),typedText);const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);assert.ok(emitted.code.length>0);if(target==='JavaScript')assert.equal(stdout(emitted.code),row.stdout);}assert.deepEqual(encodeLinkNetworkBinary(graph),typedBinary);}
 const pending=[checked],seen=new Set();while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;assert.ok(seen.size<100000);seen.add(node);assert.ok(!['cellAllocate','cellRead','cellWrite'].includes(node.k),'record state must use pure value carriers in every declaration and main');pending.push(...(node instanceof Map?[...node.values()]:Object.values(node)));}
});
for(const row of refusals)for(const sourceVersion of versions)test('record state retained boundary '+row.name+'/'+sourceVersion,()=>{
 assert.ok(['accept','reject'].includes(row.originalCompilerOutcome));for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})])assert.throws(read,error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));assert.ok(error.reason.length>0);if(row.name==='old mutable scalar boundary retains its exact complete input')assert.match(error.reason,/mutable binding: mutation is outside the portable core/u);return true;});
});
