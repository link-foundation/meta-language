// Deferred complete source/body/nullable parameter controls. Never run in draft mode.
import assert from 'node:assert/strict';
import test from 'node:test';
import {reorderWrappedRecordPropertyValues} from '../src/translation/record-value-order-template.js';
import {specializeRecordValueOrder} from '../src/translation/record-value-order-specialization.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
test('the original opaque source algorithm preserves every nullish payload and refuses every malformed position sequence',()=>{
 const payloads=[undefined,null,false,'',0,-0,NaN,3n,{value:2}];
 const wrapped=payloads.map(value=>({$: 'wrappedValue',value}));const order=[8,7,6,5,4,3,2,1,0];
 const result=reorderWrappedRecordPropertyValues(wrapped,order);
 for(const [index,position]of order.entries())assert.equal(Object.is(result[index].value,payloads[position]),true);
 for(const input of [[0],[0,0],[0,2],[-1,0],[0,1.5],[0,NaN],[0,Infinity],[-0,1]])assert.throws(()=>reorderWrappedRecordPropertyValues(wrapped.slice(0,2),input),{name:'TypeError',message:'invalid ordered record property order'});
});
test('every payload specialization retains its source declarations and both codecs in all targets',()=>{
 for(const element of [{kind:'float'},{kind:'bool'},{kind:'string'},{kind:'unit'},{kind:'option',element:{kind:'float'},absence:'both'},{kind:'record',fields:[{name:'text',type:{kind:'string'}}],ordered:true}]){
  const specialized=specializeRecordValueOrder(element,'specializedValues');
  const owner=specialized.library.declarations.get(specialized.type.name);
  assert.deepEqual(owner.ctors[0].fields[0].type,element);
  const names=[...specialized.library.declarations.keys()];
  assert.ok(names.some(name=>name.endsWith('.readOpaqueValue')));assert.ok(names.some(name=>name.endsWith('.refuseOpaqueValue')));
  const packet=encodeTypedProgramLinks(specialized.library);
  for(const target of ['JavaScript','Rust','Lean','Rocq']){
   const expected=emitTypedProgramLinks(packet.network,packet.root,target);
   for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
  }
 }
});
