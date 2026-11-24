// Deferred controls only; source authors never import this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readOwnedArrayAppendProofs} from '../src/translation/owned-array-append.js';
import {acceptOwnedArrayFreshSliceSource} from '../src/translation/owned-array-allocation-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-fresh-array-slices.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.cases)test(row.name,()=>{
 const concrete=LinkNetwork.parse(row.source,'JavaScript');for(const network of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(network.reconstructText(),row.source);
 const program=checkProgram(parseJavaScript(row.source,{referenceSemantics:false})),packet=encodeTypedProgramLinks(program);
 const original=()=>runInNewContext(row.source+'\ncompute();',{}, {timeout:1000});
 if(row.abort===null)assert.equal(String(original()),row.value);else assert.throws(original,error=>error.message===row.abort);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.ok(decodeTypedProgramLinks(network,packet.root).declarations.has('compute'));
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  const emission=emitTypedProgramLinks(network,packet.root,'JavaScript'),name=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target,run=()=>runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000});
  if(row.abort===null)assert.equal(String(run()),row.value);else assert.throws(run,error=>error.message===row.abort);
 }
});
test('fresh slice facts require actual typed array allocation and checked arity',()=>{
 assert.equal(acceptOwnedArrayFreshSliceSource('copySlice','','name',1,0,true),true);
 assert.equal(acceptOwnedArrayFreshSliceSource('method','slice','name',1,2,true),true);
 for(const args of [['method','push','name',1,1,true],['copySlice','','name',1,0,false],['method','slice','field',1,0,true],['method','slice','name',1,3,true]])assert.equal(acceptOwnedArrayFreshSliceSource(...args),false);
});
test('an array parameter itself never becomes a private append owner',()=>{
 const object={k:'name',path:['values']},statement={s:'expr',expr:{k:'method',object,op:'push',args:[{k:'str',value:'z'}]}};
 assert.deepEqual(readOwnedArrayAppendProofs([statement],[{name:'values',type:{kind:'array',element:{kind:'string'}}}]),[]);
});
test('a captured input cannot supply a later private allocation proof',()=>{
 const input={k:'name',path:['values']};
 const statements=[{s:'const',name:'capture',value:{k:'lambda',params:[],body:input}},{s:'const',name:'copy',value:{k:'copySlice',object:input}},{s:'expr',expr:{k:'method',object:{k:'name',path:['copy']},op:'push',args:[{k:'str',value:'z'}]}}];
 assert.deepEqual(readOwnedArrayAppendProofs(statements,[{name:'values',type:{kind:'array',element:{kind:'string'}}}]),[]);
});
test('a typed string slice cannot authorize an array append',()=>{
 const statements=[{s:'const',name:'copy',value:{k:'copySlice',object:{k:'name',path:['value']}}},{s:'expr',expr:{k:'method',object:{k:'name',path:['copy']},op:'push',args:[{k:'str',value:'z'}]}}];
 assert.deepEqual(readOwnedArrayAppendProofs(statements,[{name:'value',type:{kind:'string'}}]),[]);
});
