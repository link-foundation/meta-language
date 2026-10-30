// Future public/native/original/graph controls; no execution during drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {execFileSync} from 'node:child_process';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {createJavaScriptRecordOrderProgramLinks} from '../src/javascript-record-order-program.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {acceptJavaScriptRecordOrderPermutation,acceptJavaScriptRecordPropertyOrder,createJavaScriptRecordPropertyOrder,mergeJavaScriptRecordPropertyOrder} from '../src/translation/javascript-record-order-rules.js';
test('every actual order and source error matches declarations and ordinary Object.keys',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/javascript-record-order.lino',import.meta.url));
 const observed=readStructuredDataLino(execFileSync(process.execPath,['../experiments/record-property-order/produce.mjs'],{cwd:new URL('../',import.meta.url),maxBuffer:64*1024*1024}).toString('utf8'));
 assert.equal(observed.cases.length,fixture.cases.length);
 for(const [index,row]of fixture.cases.entries()){
  const actual=observed.cases[index];assert.equal(actual.name,row.name);
  const expected=row.error===null?{kind:'value',value:row.expected}:{kind:'error',className:'TypeError',message:row.error};
  assert.deepEqual(actual.observed,expected,row.name);
  if(row.error===null)assert.deepEqual(actual.originalObserved,expected,row.name);
 }
 const packet=createJavaScriptRecordOrderProgramLinks();
 assert.equal(observed.links,packet.network.toLino());
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);assert.deepEqual(observed.emissions[target],expected);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
 }
});
test('canonical position values refuse every nonfinite value and negative zero',()=>{
 for(const value of [NaN,Infinity,-Infinity,-0]){
  assert.equal(acceptJavaScriptRecordOrderPermutation(1,[value]),false);
  assert.equal(acceptJavaScriptRecordPropertyOrder([[97]],[value]),false);
  assert.throws(()=>createJavaScriptRecordPropertyOrder([[97]],[value]),{name:'TypeError',message:'invalid ordered record field sequence'});
  assert.throws(()=>mergeJavaScriptRecordPropertyOrder([[97]],[[value]]),{name:'TypeError',message:'invalid ordered record source field'});
 }
 assert.equal(acceptJavaScriptRecordOrderPermutation(-0,[]),false);
 assert.equal(acceptJavaScriptRecordOrderPermutation(0,[]),true);
});
