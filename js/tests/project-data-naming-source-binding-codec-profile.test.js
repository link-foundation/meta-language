import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {defaults} from '../src/structured-data-model.js';
import {readBootstrapStructuredDataLino,inspectBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {NAMING_SOURCE_BINDING_CODEC_POLICY as policy,verifyNamingSourceBindingCodecProfile,decodeProfiledNamingSourceBindingDocument} from '../scripts/naming-source-binding-codec-profile.mjs';
import {generateNamingSourceBindingCodecProfile} from '../scripts/generate-naming-source-binding-codec-profile.mjs';
function profileFor(source){
  const inspected=inspectBootstrapStructuredDataLino(source,{nodeLimit:policy.nodeLimit,depthLimit:policy.depthLimit,byteLimit:policy.expandedByteLimit});
  return{schemaVersion:1,model:policy.model,policy:{inputByteLimit:policy.inputByteLimit,nodeLimit:policy.nodeLimit,depthLimit:policy.depthLimit,expandedByteLimit:policy.expandedByteLimit},profiles:[{source:{path:policy.source,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')},records:inspected.value.length,cost:inspected.cost}]};
}
test('scoped source binding profile preserves the default refusal and admits a small physically shared expanded document',()=>{
  assert.deepEqual(defaults,{nodeLimit:1_000_000,depthLimit:256,byteLimit:128*1024*1024});
  const one={sourceContext:'x'.repeat(128*1024)},rows=Array(1025).fill(one),source=structuredDataToLino(rows);
  assert.ok(Buffer.byteLength(source)<256*1024,'the boundary input is a small complete Links graph');
  const profile=profileFor(source);assert.ok(profile.profiles[0].cost.bytes>defaults.byteLimit);assert.ok(profile.profiles[0].cost.bytes<=policy.expandedByteLimit);
  assert.throws(()=>readBootstrapStructuredDataLino(source),/expanded data resource limit exceeded/u,'the original generic default refusal remains');
  const decoded=decodeProfiledNamingSourceBindingDocument(source,profile);assert.deepEqual(decoded,rows);assert.equal(decoded[0],decoded.at(-1),'internal object sharing survives the original typed decoder');
  assert.equal(inspectBootstrapStructuredDataLino(structuredDataToLino([1])).value[0],1);
});
test('scoped source binding profile refuses stale bytes unknown fields forged costs and every changed reviewer ceiling',()=>{
  const source=structuredDataToLino([{source:'first',name:'operation'}]),profile=profileFor(source);
  assert.equal(verifyNamingSourceBindingCodecProfile(profile),profile);
  for(const changed of[source+'\n',source.replace('first','other')])assert.throws(()=>decodeProfiledNamingSourceBindingDocument(changed,profile),/unknown or stale/u);
  for(const edit of[
    value=>{value.extra=true;},value=>{value.profiles[0].source.extra=true;},value=>{value.profiles[0].cost.extra=true;},
    value=>{value.profiles[0].source.path='parity/naming/canonical-concepts.lino';},value=>{value.profiles[0].source.sha256='x'.repeat(64);},
    value=>{value.profiles[0].source.bytes=policy.inputByteLimit+1;},value=>{value.profiles[0].records=policy.nodeLimit+1;},
    value=>{value.profiles[0].cost.nodes=policy.nodeLimit+1;},value=>{value.profiles[0].cost.bytes=policy.expandedByteLimit+1;},
    value=>{value.profiles.push(structuredClone(value.profiles[0]));},
    ...Object.keys(profile.policy).map(key=>value=>{value.policy[key]+=1;}),
  ]){const changed=structuredClone(profile);edit(changed);assert.throws(()=>verifyNamingSourceBindingCodecProfile(changed));}
  for(const key of['nodes','bytes']){const changed=structuredClone(profile);changed.profiles[0].cost[key]+=1;assert.throws(()=>decodeProfiledNamingSourceBindingDocument(source,changed),/declared exact expansion cost/u);}
  const changed=structuredClone(profile);changed.profiles[0].records+=1;assert.throws(()=>decodeProfiledNamingSourceBindingDocument(source,changed));
});
test('complete current source binding profile matches the normal whole-source author and original migration inverse',()=>{
  const profile=generateNamingSourceBindingCodecProfile({check:true});
  assert.ok(profile.profiles.at(-1).records>=3727);
  assert.equal(profile.policy.inputByteLimit,defaults.byteLimit);assert.equal(profile.policy.nodeLimit,defaults.nodeLimit);assert.equal(profile.policy.depthLimit,defaults.depthLimit);
});
