// Deferred actual source lowering, original source and graph controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {assertVersions,callSourceFixture,observeCallSource,observeCallSourceRefusal} from '../../experiments/javascript-properties/call-source-controls.mjs';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {compileJavaScriptCallSource} from '../src/translation/javascript-call-source.js';
import {sourceExceptionFixture,observeCallSourceException,observeCallSourceExceptionRefusal} from '../../experiments/javascript-properties/call-source-exception-controls.mjs';
import {decodeJavaScriptCallSource,encodeJavaScriptCallSource} from '../../experiments/javascript-properties/call-source-codecs.mjs';
assertVersions();
for(const row of callSourceFixture.rows)test(row.name,()=>observeCallSource(row));
for(const row of callSourceFixture.refusals)test(row.name,()=>observeCallSourceRefusal(row));
for(const row of sourceExceptionFixture.programs)test(row.name,()=>observeCallSourceException(row));
for(const row of sourceExceptionFixture.refusals)test(row.name,()=>observeCallSourceExceptionRefusal(row));
test('unowned references and duplicate parameter owners never yield executable partial code',()=>{
 for(const result of [compileJavaScriptCallSource({$:'sourceBinding',identity:1},[]),compileJavaScriptCallSource({$:'sourceConstant',value:{$:'undefined'}},[0,0])]){
  assert.notEqual(result.error,'');assert.deepEqual(result.instructions,[]);assert.equal(result.registers,0);
 }
});
test('the call source fixture codec preserves missing and explicitly undefined properties in argument lists',()=>{
 const property={$:'sourceGet',object:{$:'sourceReceiver'},key:{$:'sourceConstant',value:{$:'undefined'}}};
 const source={$:'sourcePrimitive',operation:'identity',arguments:[property,{...property,receiver:undefined}]};
 const wire=encodeJavaScriptCallSource(source);
 assert.equal(Object.hasOwn(wire.arguments[0],'receiver'),false);
 assert.deepEqual(wire.arguments[1].receiver,{fieldPresence:'undefined'});
 assert.deepEqual(decodeJavaScriptCallSource(wire),source);
});
test('actual compiler graph retains both codecs and all target emissions',()=>{
 const packet=createSourceDefinitionProgramLinks('meta-language/source-program/javascript-call-source-program');
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
