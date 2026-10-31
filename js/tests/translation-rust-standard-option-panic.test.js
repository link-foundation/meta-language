// Deferred complete-source controls. Do not execute during source drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as methods from '../src/translation/rust-standard-sum-method-rules.js';
import * as panic from '../src/translation/rust-panic-value-rules.js';
import {rustStandardOptionPanicMethodDecisions as decisions,rustStandardOptionPanicMethodOriginals as originals,
 rustStandardOptionPanicMethodRefusals as refusals} from './fixtures/rust-standard-option-panic-methods.mjs';

const versions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'],transports=['original','text','binary'];
const [nodeMajor,nodeMinor]=process.versions.node.split('.').map(Number);
assert.ok(nodeMajor===24||nodeMajor===22&&nodeMinor>=15,'uncovered Option panic control Node '+process.versions.node);
function expected(row){return {stdout:row.stdout,panic:row.panic??null};}
function observe(code){
 const lines=[];let failure=null;
 try{vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});}
 catch(error){assert.equal(error.name,'Error');assert.equal(typeof error.message,'string');failure=error.message;}
 return {stdout:lines.length?lines.join('\n')+'\n':'',panic:failure};
}
function assertNoPrivate(root){
 const pending=[root],seen=new Set();while(pending.length){
  const value=pending.pop();if(value===null||typeof value!=='object'||seen.has(value))continue;
  assert.ok(seen.size<100000,'finite full source traversal');seen.add(value);
  assert.notEqual(value.k,'rustStandardSumTry');assert.notEqual(value.op,'rustStandardSumTry');assert.notEqual(value.k,'rustStandardSumSourcePattern');
  if(Array.isArray(value.path))assert.ok(value.path.every(part=>typeof part!=='string'||!part.startsWith('#rust-standard-sum-source')));
  for(const child of Object.values(value))if(child!==null&&typeof child==='object')pending.push(child);
 }
}

test('all independent Option panic fixture inputs remain present',()=>{
 assert.equal(originals.length,5);assert.equal(decisions.length,18);assert.equal(refusals.length,4);
 assert.equal(originals.filter(row=>typeof row.panic==='string').length,3);
 for(const row of originals){assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');assert.ok(row.source.includes('fn main()'));assert.equal(typeof row.stdout,'string');}
});
for(const [index,row]of decisions.entries())test('standard Option panic source decision '+index+' '+row.operation,()=>{
 const operation=methods[row.operation]??panic[row.operation];assert.equal(typeof operation,'function');assert.deepEqual(operation(...row.arguments),row.expected);
});

for(const row of originals)for(const sourceVersion of versions)test('actual Option panic source '+row.name+' / '+sourceVersion,()=>{
 const concrete=LinkNetwork.parse(row.source,'Rust'),concreteText=concrete.toLino(),concreteBinary=encodeLinkNetworkBinary(concrete);
 for(const graph of [concrete,LinkNetwork.fromLino(concreteText),decodeLinkNetworkBinary(concreteBinary)]){
  assert.equal(graph.toLino(),concreteText);assert.equal(graph.reconstructText(),row.source,'complete original source retained');
 }
 // Runtime panic controls must first succeed through the real frontend/checker.
 const surface=parseRust(row.source,{sourceVersion});assertNoPrivate(surface);
 const checked=checkProgram(surface);assertNoPrivate([checked.items,checked.main]);
 assert.deepEqual(observe(emitJavaScript(checked).text),expected(row),'actual frontend output and raw panic payload');
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,provenance:{fixture:row.name}});
 const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network),graphs=[packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)];
 assert.equal(graphs.length,transports.length);
 const baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(let index=0;index<graphs.length;index+=1){
  const graph=graphs[index],label=row.name+'/'+sourceVersion+'/'+transports[index];assert.equal(graph.toLino(),text,label);
  const restored=decodeTypedProgramLinks(graph,packet.root);assertNoPrivate([restored.items,restored.main]);
  assert.deepEqual(restored.items,checked.items,label+': complete current checked declarations');assert.deepEqual(restored.main,checked.main,label+': actual main');
  for(const target of targets){
   const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target],label+'/'+target+': all emission fields');
   assert.equal(typeof emitted.code,'string');assert.ok(emitted.code.length>0);
   if(target==='JavaScript')assert.deepEqual(observe(emitted.code),expected(row),label+': actual selected runtime outcome');
  }
 }
 assert.deepEqual(encodeLinkNetworkBinary(graphs[2]),binary);
});

for(const row of refusals)for(const sourceVersion of versions)test('Option panic actual refusal '+row.name+' / '+sourceVersion,()=>{
 for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})]){
  assert.throws(read,error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));assert.equal(typeof error.reason,'string');assert.ok(error.reason.length>0);return true;});
 }
});
