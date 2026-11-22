// Future public/native/original/graph controls; no execution during drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {createJavaScriptRecordOrderProgramLinks} from '../src/javascript-record-order-program.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {checkProgram} from '../src/translation/check.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {acceptJavaScriptRecordOrderPermutation,acceptJavaScriptRecordPropertyOrder,createJavaScriptPresentRecordPropertyOrder,createJavaScriptRecordPropertyOrder,mergeJavaScriptRecordPropertyOrder} from '../src/translation/javascript-record-order-rules.js';
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
test('the ordered field witness rejects omissions, duplicates and absent positions',()=>{
 const names=[[50],[97],[98]];
 assert.deepEqual(createJavaScriptPresentRecordPropertyOrder(names,[2,0],[true,false,true]),[0,2]);
 assert.deepEqual(createJavaScriptPresentRecordPropertyOrder(names,[],[false,false,false]),[]);
 for(const order of [[0],[0,0],[0,1,2],[0,1],[-0,2],[NaN,2]]){
  assert.throws(()=>createJavaScriptPresentRecordPropertyOrder(names,order,[true,false,true]),{
   name:'TypeError',message:'invalid ordered record field sequence',
  });
 }
});
test('ordered optional fields survive spread, rest, enumeration and both graph codecs',()=>{
 const source=readFileSync(new URL('../../experiments/record-property-order/optional-fields.js',import.meta.url),'utf8');
 const original=runInNewContext(source+'\ncompute');
 assert.equal(original(false),true);
 assert.equal(original(true),true);
 const {network,root}=encodeTypedProgramLinks(checkProgram(parseJavaScript(source)));
 const codecs=[LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const originalEmission=emitTypedProgramLinks(network,root,target);
  for(const decoded of codecs)assert.deepEqual(emitTypedProgramLinks(decoded,root,target),originalEmission);
  if(target==='JavaScript'){
   const entry=originalEmission.mappings.find(({kind,source})=>kind==='function'&&source==='compute').target;
   const result=runInNewContext(originalEmission.code.replace(/^export /gmu,'')+'\n'+entry+'(false) && '+entry+'(true)',{}, {timeout:2000});
   assert.equal(result,true);
  }
 }
});
test('writes create and retain ordered optional own fields on every alias',()=>{
 const source=readFileSync(new URL('../../experiments/record-property-order/optional-field-writes.js',import.meta.url),'utf8');
 const functions=['compute','compound','nullableCompound'];
 assert.equal(runInNewContext(source+'\n'+functions.map(name=>name+'()').join(' && '),{}, {timeout:2000}),true);
 const {network,root}=encodeTypedProgramLinks(checkProgram(parseJavaScript(source)));
 const codecs=[LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];
 for(const target of ['JavaScript','Rust']){
  const emission=emitTypedProgramLinks(network,root,target);
  for(const decoded of codecs)assert.deepEqual(emitTypedProgramLinks(decoded,root,target),emission);
  if(target==='JavaScript'){
   const entries=functions.map(name=>emission.mappings.find(({kind,source})=>kind==='function'&&source===name).target);
   assert.equal(runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+entries.map(entry=>entry+'()').join(' && '),{}, {timeout:2000}),true);
  }
 }
});
