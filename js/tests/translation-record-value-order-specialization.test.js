// Deferred complete source/body/nullable parameter controls. Never run in draft mode.
import assert from 'node:assert/strict';
import test from 'node:test';
import {reorderWrappedRecordPropertyValues,pairWrappedOwnPropertyValues} from '../src/translation/record-value-order-template.js';
import {specializeRecordValueOrder} from '../src/translation/record-value-order-specialization.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0');
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

test('the complete pairing source keeps opaque nullish payloads and exact keys',()=>{
 const object={value:3},payloads=[undefined,null,false,-0,'',object],keys=['undefined','null','false','zero','\ud800','object'];
 const values=payloads.map(value=>({$:'wrappedValue',value})),entries=pairWrappedOwnPropertyValues(keys,values);
 assert.equal(entries.length,payloads.length);
 for(let index=0;index<entries.length;index+=1){assert.equal(entries[index].$,'wrappedEntry');assert.equal(entries[index].key,keys[index]);assert.equal(entries[index].value,values[index]);assert.ok(Object.is(entries[index].value.value,payloads[index]));}
 assert.deepEqual(pairWrappedOwnPropertyValues([],[]),[]);
 assert.throws(()=>pairWrappedOwnPropertyValues(['a'],[]),{name:'TypeError',message:'invalid own entry key and value sequence'});
 assert.throws(()=>pairWrappedOwnPropertyValues([],values),{name:'TypeError',message:'invalid own entry key and value sequence'});
});
