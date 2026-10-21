import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {MUTABLE_ARRAY_FRONTEND_FILE,MUTABLE_ARRAY_FRONTEND_PREVIOUS,MUTABLE_ARRAY_FRONTEND_CURRENT,MUTABLE_ARRAY_FRONTEND_HOST_PACKET,
  pinMutableArrayFrontendSource,deriveMutableArrayFrontendSourceEdit,readMutableArrayFrontendSourceHistory,verifyMutableArrayFrontendSourceHistory,restoreMutableArrayFrontendSource} from '../scripts/javascript-mutable-array-frontend-source-history.mjs';
import {generateJavaScriptMutableArrayFrontendSourceHistory} from '../scripts/generate-javascript-mutable-array-frontend-source-history.mjs';
import {generateJavaScriptMutableArrayBindings} from '../scripts/generate-javascript-mutable-array-bindings.mjs';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from '../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const root=new URL('../../',import.meta.url),reader='mutable array binding decision source';
test('the actual optional-tuple frontend has one certified complete mutable-array historical source view',()=>{
  const packet=readMutableArrayFrontendSourceHistory(),current=readFileSync(new URL(MUTABLE_ARRAY_FRONTEND_FILE,root),'utf8');
  assert.deepEqual(pinMutableArrayFrontendSource(current),MUTABLE_ARRAY_FRONTEND_CURRENT);assert.equal(current,packet.current.source);
  assert.deepEqual(pinMutableArrayFrontendSource(packet.previous.source),MUTABLE_ARRAY_FRONTEND_PREVIOUS);
  assert.equal(restoreMutableArrayFrontendSource(current,MUTABLE_ARRAY_FRONTEND_FILE,reader),packet.previous.source);
  assert.equal(restoreMutableArrayFrontendSource(packet.previous.source,MUTABLE_ARRAY_FRONTEND_FILE,reader),packet.previous.source);
  assert.deepEqual(packet.edit,deriveMutableArrayFrontendSourceEdit(packet.previous.source,current));
  assert.match(current,/sourceKind === 'array' \|\| sourceKind === 'tuple'/u);
  const {start,removed,inserted}=packet.edit;
  assert.equal(current.slice(0,start)+removed+current.slice(start+inserted.length),packet.previous.source);
  assert.deepEqual(pinMutableArrayFrontendSource(readFileSync(new URL(MUTABLE_ARRAY_FRONTEND_HOST_PACKET.file,root))),{bytes:MUTABLE_ARRAY_FRONTEND_HOST_PACKET.bytes,sha256:MUTABLE_ARRAY_FRONTEND_HOST_PACKET.sha256});
});
test('mutable-array frontend views preserve unknown source refusals paths reader scopes and certificate integrity',()=>{
  const packet=readMutableArrayFrontendSourceHistory(),current=packet.current.source;
  for(const[source,file,scope]of[[current+' ',MUTABLE_ARRAY_FRONTEND_FILE,reader],[current.replace('itemCount ===','itemCount !=='),MUTABLE_ARRAY_FRONTEND_FILE,reader],[current,'js/src/translation/javascript.js',reader],[current,MUTABLE_ARRAY_FRONTEND_FILE,'other historical source author']]){
    assert.equal(restoreMutableArrayFrontendSource(source,file,scope),source);
  }
  for(const mutate of[value=>{value.previous.source+=' ';},value=>{value.current.source+=' ';},value=>{value.edit.inserted+=' ';},value=>{value.previous.certificate.commitText+=' ';},value=>{value.current.certificate.commitText+=' ';},value=>{value.hostPacket.bytes++;},value=>{value.file+=' ';}]){
    const forged=structuredClone(packet);mutate(forged);assert.throws(()=>verifyMutableArrayFrontendSourceHistory(forged));
  }
  const saved=encodeStructuredDataLinks(packet),canonical=LinkNetwork.fromLino(saved.network.toLino());
  for(const network of[canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const value=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(verifyMutableArrayFrontendSourceHistory(value),packet);
  }
});
test('normal frontend source capture and the complete original mutable-array host producer remain fresh without native overwrites',()=>{
  const before=readFileSync(new URL(MUTABLE_ARRAY_FRONTEND_HOST_PACKET.file,root));
  generateJavaScriptMutableArrayFrontendSourceHistory({check:true});
  const archive=generateJavaScriptMutableArrayBindings({check:true});
  assert.equal(archive.decision.sourceSha256,MUTABLE_ARRAY_FRONTEND_PREVIOUS.sha256);
  assert.deepEqual(readFileSync(new URL(MUTABLE_ARRAY_FRONTEND_HOST_PACKET.file,root)),before);
});
