// A source-authenticated budget for this one complete naming data document.
// Generic codecs and every other data caller retain their original limits.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from './project-data.mjs';
import {inspectBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
export const NAMING_SOURCE_BINDING_CODEC_POLICY=Object.freeze({
  schemaVersion:1,model:'naming source binding codec profile 1',
  source:'parity/naming/source-bindings.lino',profile:'parity/naming/source-binding-codec-profile.lino',
  inputByteLimit:128*1024*1024,nodeLimit:1_000_000,depthLimit:256,expandedByteLimit:192*1024*1024,
});
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exact=(value,keys)=>{assert.ok(value!==null&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),keys.slice().sort());};
const integer=(value,min,max)=>assert.ok(Number.isSafeInteger(value)&&value>=min&&value<=max,'finite reviewed source binding bound');
export function verifyNamingSourceBindingCodecProfile(profile){
  const policy=NAMING_SOURCE_BINDING_CODEC_POLICY;
  exact(profile,['schemaVersion','model','policy','profiles']);
  assert.equal(profile.schemaVersion,policy.schemaVersion);assert.equal(profile.model,policy.model);
  assert.deepEqual(profile.policy,{inputByteLimit:policy.inputByteLimit,nodeLimit:policy.nodeLimit,depthLimit:policy.depthLimit,expandedByteLimit:policy.expandedByteLimit});
  assert.ok(Array.isArray(profile.profiles)&&profile.profiles.length>0&&profile.profiles.length<=4096,'bounded append-only source epochs');
  const seen=new Set();
  for(const epoch of profile.profiles){
    exact(epoch,['source','records','cost']);exact(epoch.source,['path','bytes','sha256']);exact(epoch.cost,['nodes','bytes']);
    assert.equal(epoch.source.path,policy.source);integer(epoch.source.bytes,1,policy.inputByteLimit);
    assert.match(epoch.source.sha256,/^[0-9a-f]{64}$/u);assert.ok(!seen.has(epoch.source.sha256),'unique complete source epochs');seen.add(epoch.source.sha256);
    integer(epoch.records,1,policy.nodeLimit);integer(epoch.cost.nodes,1,policy.nodeLimit);integer(epoch.cost.bytes,0,policy.expandedByteLimit);
  }
  return profile;
}
/** Hash and physical-input admission precede the explicitly larger expanded-byte budget. */
export function decodeProfiledNamingSourceBindingDocument(source,profile){
  verifyNamingSourceBindingCodecProfile(profile);const policy=NAMING_SOURCE_BINDING_CODEC_POLICY,epoch=profile.profiles.at(-1);
  assert.equal(typeof source,'string');assert.ok(Buffer.byteLength(source)<=policy.inputByteLimit,'the original physical input byte ceiling remains required');
  assert.deepEqual(pin(source),{bytes:epoch.source.bytes,sha256:epoch.source.sha256},'unknown or stale complete naming source binding document');
  const inspected=inspectBootstrapStructuredDataLino(source,{nodeLimit:policy.nodeLimit,depthLimit:policy.depthLimit,byteLimit:policy.expandedByteLimit});
  assert.ok(Array.isArray(inspected.value));assert.equal(inspected.value.length,epoch.records);
  assert.deepEqual(inspected.cost,epoch.cost,'the complete validated graph retains its declared exact expansion cost');
  return inspected.value;
}
export function readProfiledNamingSourceBindingDocument(){
  const policy=NAMING_SOURCE_BINDING_CODEC_POLICY;
  return decodeProfiledNamingSourceBindingDocument(readFileSync(new URL(policy.source,root),'utf8'),readProjectData(new URL(policy.profile,root)));
}
