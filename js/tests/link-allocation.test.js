// Future complete controls. Never execute during the whole requirements draft.
import assert from'node:assert/strict';
import test from'node:test';
import{readProjectData}from'../scripts/project-data.mjs';
import{acceptLinkAllocationCapacity,acceptLinkAllocationWord}from'../src/link-allocation-rules.js';
import{createLinkAllocationProgramLinks}from'../src/link-allocation-program.js';
import{emitTypedProgramLinks}from'../src/typed-program-links.js';
import{LinkNetwork}from'../src/network.js';
import{encodeLinkNetworkBinary,decodeLinkNetworkBinary}from'../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/link-allocation.lino',import.meta.url));
const[major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const[index,row]of fixture.cases.entries())test('exact native-capacity source case '+index,()=>{
 const valid=row.words.every(value=>Number.isInteger(value)&&value>=0&&value<=65535)&&Number.isSafeInteger(row.count)&&row.count>=0&&row.count<=4294967295;
 const lower=valid?row.words.reduce((sum,value,index)=>sum+(BigInt(value)<<BigInt(16*index)),0n):0n;
 const current=row.exhausted?(1n<<64n):lower;
 const expected=valid&&(!row.exhausted||lower===0n)&&current>0n&&current+BigInt(row.count)<=(1n<<64n);
 assert.equal(expected,row.expected);
 assert.equal(acceptLinkAllocationCapacity(...row.words,row.exhausted,row.count),row.expected);
});
test('complete allocation definition survives both codecs and all four emissions',()=>{
 const packet=createLinkAllocationProgramLinks();
 for(const target of['JavaScript','Rust','Lean','Rocq']){
  const original=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),original);
 }
 for(const value of[NaN,Infinity,-Infinity,-1,0.5,65536])assert.equal(acceptLinkAllocationWord(value),false);
});
