// Deferred field-move contracts; no execution observations during drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-owned-record-update-rules.js';
import {rustOwnedRecordFieldState,rustOwnedRecordFieldSurvives} from '../src/translation/rust-owned-record-updates.js';
import {rustOwnedRecordUpdateDecisions as decisions,rustOwnedRecordUpdateOriginals as originals,rustOwnedRecordUpdateRefusals as refusals} from './fixtures/rust-owned-record-updates.mjs';
import {rustNominalStandardSumOwnershipRefusalHistory as history} from './fixtures/rust-nominal-standard-sum-ownership.mjs';
const versions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'];
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered record field move Node '+process.versions.node);
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const [index,row]of decisions.entries())test('actual record field state decision '+index+' '+row.operation,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
test('all independent field move inputs and exact prior refusal stay represented',()=>{
 assert.equal(decisions.length,29);assert.equal(originals.length,11);assert.equal(refusals.length,10);
 const prior=history.find(row=>row.previousExport==='rustNominalStandardSumOwnershipRefusals'&&row.previousIndex===6);
 assert.equal(prior.previousFixture,'rust-nominal-standard-sum-ownership.mjs');assert.equal(prior.control.originalCompilerOutcome,'accept');assert.equal(prior.control.source,originals[0].source);assert.equal(prior.control.sourceVersionRange,originals[0].sourceVersionRange);assert.equal(originals[0].stdout,'');
});
test('missing branch field states require that branch actual root availability',()=>{
 assert.equal(rustOwnedRecordFieldState(new Map([['7','available']]),'record field 7 0'),'available');
 assert.equal(rustOwnedRecordFieldState(new Map([['7','moved']]),'record field 7 0'),'absent');
 assert.equal(rustOwnedRecordFieldState(new Map([['7','available'],['record field 7 0','moved']]),'record field 7 0'),'moved');
 assert.equal(rustOwnedRecordFieldState(new Map([['7','available']]),'ordinary other identity'),'absent');
});
test('field-state lifetime follows the actual root rather than selection scope',()=>{
 const table={sourceBindingStates:new Map([['7','available']])},id='record field 7 0';
 assert.equal(rustOwnedRecordFieldSurvives(table,id,new Map([['7','available']]),{}),true);
 assert.equal(rustOwnedRecordFieldSurvives(table,id,new Map(),{}),false);
 assert.equal(rustOwnedRecordFieldSurvives(table,id,new Map(),{sourceBindingIdentity:'7'}),true);
 table.sourceBindingStates.set('7','dropped');assert.equal(rustOwnedRecordFieldSurvives(table,id,new Map([['7','available']]),{}),false);
});
for(const row of originals)for(const sourceVersion of versions)test('actual destructor-free record update '+row.name+'/'+sourceVersion,()=>{
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
for(const row of refusals)for(const sourceVersion of versions)test('actual record update refusal '+row.name+'/'+sourceVersion,()=>{
 assert.ok(['accept','reject'].includes(row.originalCompilerOutcome));
 for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})])assert.throws(read,error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));assert.ok(error.reason.length>0);return true;});
});
