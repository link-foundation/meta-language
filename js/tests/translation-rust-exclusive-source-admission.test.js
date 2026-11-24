// Deferred complete-source controls. These declarations are not observations.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustExclusiveSourceAdmissionOriginals as originals,rustExclusiveSourceAdmissionRefusals as refusals,rustExclusiveSourceAdmissionVersions as versions} from './fixtures/rust-exclusive-source-admission.mjs';
const model='Rust exclusive source loans 1',rustVersions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'];
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
function objects(root){const result=[],pending=[root],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;assert.ok(seen.size<100000);seen.add(value);result.push(value);pending.push(...(value instanceof Map?[...value.values()]:Object.values(value)));}return result;}
for(const row of originals)for(const sourceVersion of rustVersions)test('exclusive actual source '+row.name+'/'+sourceVersion,()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered exclusive source Node '+process.versions.node+'; '+versions.node);
 const concrete=LinkNetwork.parse(row.source,'Rust'),text=concrete.toLino(),binary=encodeLinkNetworkBinary(concrete);for(const graph of [concrete,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)])assert.equal(graph.reconstructText(),row.source);
 const checked=checkProgram(parseRust(row.source,{sourceVersion,exclusiveSourceModel:model}));assert.equal(stdout(emitJavaScript(checked).text),row.stdout);
 const declarations=[...checked.declarations.values()].filter(entry=>entry.k==='fn'),exclusive=declarations.filter(entry=>entry.sourceExclusiveSignature?.packet.length);assert.ok(exclusive.length>0);assert.ok(declarations.every(entry=>entry.sourceExclusiveSignature?.model===model&&entry.sourceSignature===undefined));
 for(const entry of exclusive){const signature=entry.sourceExclusiveSignature;assert.equal(entry.ret.kind,'tuple');assert.equal(entry.ret.elements.length,signature.packet.length+1);assert.deepEqual(signature.packet,signature.parameters.flatMap((row,index)=>row.mode==='exclusive'?[index]:[]));}
 for(const value of objects(checked)){assert.ok(!String(value.op??'').startsWith('rustExclusiveSource'));assert.ok(!['cellAllocate','cellRead','cellWrite'].includes(value.k),'exclusive source state uses ordinary value packets');}
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,rustExclusiveSourceModel:model,provenance:{fixture:row.name}}),typedText=packet.network.toLino(),typedBinary=encodeLinkNetworkBinary(packet.network),baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(typedText),decodeLinkNetworkBinary(typedBinary)]){assert.equal(graph.toLino(),typedText);const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);assert.ok(emitted.code.length>0);if(target==='JavaScript')assert.equal(stdout(emitted.code),row.stdout);}assert.deepEqual(encodeLinkNetworkBinary(graph),typedBinary);}
});
for(const row of refusals)for(const sourceVersion of rustVersions)test('exclusive source retained boundary '+row.name+'/'+sourceVersion,()=>{
 assert.equal(typeof row.compilerAccepted,'boolean');assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion,exclusiveSourceModel:model})),error=>{assert.ok(['unsupported','type'].includes(error.kind),'valid source profile boundaries need an explicit source diagnostic');assert.ok(error.reason.length>0);return true;});
});
test('the original mutable-reference refusal stays unchanged without selecting the new profile',()=>{
 const source='fn change(value: &mut u8) { *value = 7; } fn main() {}';assert.throws(()=>parseRust(source,{sourceVersion:'1.90.0'}),error=>error.kind==='unsupported'&&error.reason==='mutable reference: mutation is outside the portable core');
});
test('exclusive profile cannot reinterpret old immutable signature profiles or uncovered Rust versions',()=>{
 const source='fn main() {}';for(const sourceVersion of ['1.89.0','1.100.0'])assert.throws(()=>parseRust(source,{sourceVersion,exclusiveSourceModel:model}));for(const sourceSignatureModel of ['Rust source function signature 1','Rust source function signature 3'])assert.throws(()=>parseRust(source,{sourceVersion:'1.90.0',exclusiveSourceModel:model,sourceSignatureModel}));
});
test('source packet metadata cannot substitute exclusive order or writeback type after JSON transport',()=>{
 const checked=checkProgram(parseRust(originals[0].source,{sourceVersion:'1.90.0',exclusiveSourceModel:model})),entry=[...checked.declarations.values()].find(row=>row.k==='fn'&&row.sourceExclusiveSignature?.packet.length);assert.ok(entry);
 const signature=structuredClone(entry.sourceExclusiveSignature);signature.packet=[];const surface=parseRust(originals[0].source,{sourceVersion:'1.90.0',exclusiveSourceModel:model}),item=objects(surface).find(value=>value.k==='fn'&&value.name===entry.name);item.sourceExclusiveSignature=signature;assert.throws(()=>checkProgram(surface),error=>error.kind==='type');
});
