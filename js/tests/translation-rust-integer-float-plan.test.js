import * as rules from '../src/translation/rust-integer-float-plan-rules.js';
// Deferred actual source/codec/product controls. Never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-integer-float-plan.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
const options={rustSourceHeaderEmissionModel:fixture.model};
test('every shared floating source term and admitted width keeps its actual premise',()=>{
 for(const row of fixture.operations)assert.equal(rules.readRustIntegerFloatPlanOperation(row.code),row.expected);
 for(const bits of fixture.invalidWidths)assert.deepEqual(rules.planRustIntegerFloatConversion(bits),[]);
 for(const row of fixture.widths){const plan=rules.planRustIntegerFloatConversion(row.bits),operations=[];assert.equal(plan.length,row.terms*5);for(let at=0;at<plan.length;at+=5)operations.push(rules.readRustIntegerFloatPlanOperation(plan[at]));assert.equal(operations.filter(value=>value==='input').length,1);assert.equal(operations.filter(value=>value==='binding').length,row.bindings);assert.equal(operations.at(-1),'binding');}
});
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),before=packet.network.toLino(),program=decodeTypedProgramLinks(packet.network,packet.root),retained=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),retained);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 const javascript=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 assert.equal(String(runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),row.expected);
 assert.equal(packet.network.toLino(),before);
});
for(const row of fixture.printedSources)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),lines=[],emission=emitTypedProgramLinks(packet.network,packet.root,'JavaScript');
 runInNewContext(emission.code.replace(/^export /gmu,''),{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});
 assert.deepEqual(lines,row.lines);
 for(const network of views(packet))for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
test('the actual associated receiver and argument calls occur once in source order',()=>{
 const row=fixture.cases.find(row=>row.name.startsWith('actual integer floating operands')),packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root),calls=[];
 let body=program.declarations.get('select').body;
 while(body.k==='let'){if(body.value.k==='call')calls.push(body.value.fn);body=body.body;}
 assert.deepEqual(calls,['input']);
 const pending=[program.declarations.get('select').body],seen=new WeakSet(),actual=[];
 while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);if(node.k==='call')actual.push(node.fn);for(const [field,child] of Object.entries(node))if(!['span','type','domain'].includes(field)){if(Array.isArray(child))pending.push(...child);else if(child&&typeof child==='object')pending.push(child);}}
 assert.deepEqual(actual.sort(),['input']);
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
