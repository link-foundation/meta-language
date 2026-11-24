// Deferred actual frontend/transport controls; never execute while drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-owned-standard-sum-method-rules.js';
import {rustOwnedStandardSumMethodDecisions as decisions,rustOwnedStandardSumMethodOriginals as originals,rustOwnedStandardSumMethodRefusals as refusals,rustOwnedStandardSumMethodPreviousBoundaries as previousBoundaries} from './fixtures/rust-owned-standard-sum-methods.mjs';
const versions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'];
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered owned consuming method Node '+process.versions.node);
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const [index,row]of decisions.entries())test('owned consuming method source decision '+index+' '+row.operation,()=>{assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
test('all independently authored consuming method inputs remain represented',()=>{assert.equal(decisions.length,56);assert.equal(originals.length,12);assert.equal(refusals.length,9);});
test('former eager callback refusal retains its exact input and a positive compiler contract',()=>{
 assert.equal(previousBoundaries.length,1);const boundary=previousBoundaries[0];
 const row=originals.find(value=>value.name===boundary.row.name);
 assert.ok(row);assert.equal(row.source,boundary.row.source);assert.equal(row.sourceVersionRange,boundary.row.sourceVersionRange);
 assert.equal(row.stdout,'text\n');assert.equal(row.originalCompilerExpectation.outcome,'accept');
 assert.equal(row.expectationHistory[0].previousIndex,boundary.previousIndex);
 assert.ok(!refusals.some(value=>value.source===boundary.row.source));
});
for(const row of originals)for(const sourceVersion of versions)test('actual consuming method source '+row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const concrete=LinkNetwork.parse(row.source,'Rust'),text=concrete.toLino(),binary=encodeLinkNetworkBinary(concrete);
 for(const graph of [concrete,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)])assert.equal(graph.reconstructText(),row.source);
 const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(stdout(emitJavaScript(checked).text),row.stdout);
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,provenance:{fixture:row.name}}),typedText=packet.network.toLino(),typedBinary=encodeLinkNetworkBinary(packet.network);
 const baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(typedText),decodeLinkNetworkBinary(typedBinary)]){
  assert.equal(graph.toLino(),typedText);const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);
  for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);assert.ok(emitted.code.length>0);if(target==='JavaScript')assert.equal(stdout(emitted.code),row.stdout);}
  assert.deepEqual(encodeLinkNetworkBinary(graph),typedBinary);
 }
});
for(const row of refusals)for(const sourceVersion of versions)test('actual consuming method refusal '+row.name+'/'+sourceVersion,()=>{
 assert.ok(['accept','reject'].includes(row.originalCompilerOutcome));
 for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})])assert.throws(read,error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));assert.ok(error.reason.length>0);return true;});
});
