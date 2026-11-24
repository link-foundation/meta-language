// Deferred graph transport controls. This does not assert frontend admission.
import test from 'node:test';
import assert from 'node:assert/strict';
import {checkProgram} from '../src/translation/check.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

function surface(){
 const element={kind:'named',path:['Node']};
 const owner={kind:'declaration',path:['Node'],arguments:[]};
 const position={type:{kind:'array',element},wrappers:[],lifetimes:[],owner:{kind:'library',path:['std','vec','Vec'],arguments:[owner]},arrayElementPosition:{type:structuredClone(element),wrappers:['reference'],lifetimes:['a'],owner:structuredClone(owner)}};
 return {language:'Rust',items:[
  {k:'data',name:'Node',ctors:[{name:'Node',fields:[{name:'value',type:{kind:'fixed',bits:8,signed:false}}]}]},
  {k:'fn',name:'observe',params:[{name:'values',type:structuredClone(position.type)}],ret:{kind:'unit'},body:{k:'unit'},sourceSignature:{model:'Rust source function signature 3',language:'Rust',parameters:[position],result:{type:{kind:'unit'},wrappers:[],lifetimes:[],owner:{kind:'unit',path:[],arguments:[]}},lifetimeBindings:['a']}},
 ]};
}

test('recursive source positions own nominal datatype references through text and binary',()=>{
 const checked=checkProgram(surface()),packet=encodeTypedProgramLinks(checked);
 const original=checked.declarations.get('observe').sourceSignature;
 assert.deepEqual(original.parameters[0].arrayElementPosition,{type:{kind:'data',name:'Node'},wrappers:['reference'],lifetimes:['a'],owner:{kind:'declaration',path:['Node'],arguments:[]}});
 const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){
  assert.equal(graph.toLino(),text);
  assert.deepEqual(encodeLinkNetworkBinary(graph),binary);
  const restored=decodeTypedProgramLinks(graph,packet.root);
  assert.deepEqual(restored.declarations.get('observe').sourceSignature,original);
  assert.deepEqual(restored.items,checked.items);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});

test('source position ownership cannot be replaced by a same-shaped nominal declaration',()=>{
 const input=surface();
 input.items.unshift({k:'data',name:'Other',ctors:[{name:'Other',fields:[{name:'value',type:{kind:'fixed',bits:8,signed:false}}]}]});
 input.items.at(-1).sourceSignature.parameters[0].arrayElementPosition.owner.path=['Other'];
 assert.throws(()=>checkProgram(input),error=>error.kind==='type');
});
