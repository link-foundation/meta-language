// Exact current-source views for the existing relative-array historical readers.
// These source identities do not replace current parsing, emission or native gates.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreCurrentOwnCodebaseRawSource} from './current-own-codebase-raw-source-descriptors.mjs';
import {restoreReviewedOverlayProviderModuleSource} from '../../experiments/rust-text-certificate-adapter/provider-succession.mjs';
import {RAW_CONSUMING_SNAPSHOT_FILES,restoreRawConsumingSnapshotSource} from './raw-consuming-snapshot-source-profile.mjs';
const root=new URL('../../',import.meta.url);
export const RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE='relative array';
export const RELATIVE_ARRAY_OWNED_RULE_SOURCE='js/src/translation/owned-array-append-rules.js';
export const RELATIVE_ARRAY_OWNED_RULE_REVISION='8d77e476d097448eb381ebe03e2aa173402bc03d';
export const RELATIVE_ARRAY_OWNED_RULE_OUTPUT='parity/self-translation/relative-array-owned-rule-source-history.lino';
export const RELATIVE_ARRAY_OWNED_RULE_PREVIOUS=Object.freeze({bytes:4830,sha256:'eb349133f29d2e5944ace3b96975d508a92d265231518ac2b99a604129c7b48a'});
export const RELATIVE_ARRAY_OWNED_RULE_BASELINE=Object.freeze({bytes:5765,sha256:'b367a8b068e4f59c26eaba8b7f8a8c5c039b01922aaf93bd0c312accce7fc5f3'});
export const RELATIVE_ARRAY_OWNED_RULE_BASELINE_PACKET=Object.freeze({bytes:277300,sha256:'4be14bb53c2111ac4963f37b37ac0b65e0a06f79c226ea84eee2f77a265fc029'});
export const RELATIVE_ARRAY_OWNED_RULE_CURRENT=Object.freeze({bytes:7301,sha256:'6447386be67ac8a6f9ad13c4650eda4c06cc1d93f26282515f6388d483f2930d'});
export const RELATIVE_ARRAY_OWNED_RULE_BOUNDARY='One complete certified8d owned-array-rule source and its exact current UTF-8 source are retained. The finite source inverse is available only to the relative-array historical profile. Both earlier rule behavior and current expanded iteration/self-slice behavior require their own semantic tests. No native body, descriptor gate, source fixture or previous history packet is rewritten.';
const packetSha256='f2a5321b13673248b6a34e36face5802fb24f1319be56bddb4a901d4dfd6732e';
let cached;
export const pinRelativeArrayCurrentSource=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const keys=(value,names)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),[...names].sort());};

export function deriveRelativeArrayOwnedRuleSourceIntervals(previous,current){
  assert.equal(Buffer.from(previous).toString('utf8'),previous);assert.equal(Buffer.from(current).toString('utf8'),current);
  const before=[...previous],after=[...current];let start=0,oldEnd=before.length,end=after.length;
  while(start<oldEnd&&start<end&&before[start]===after[start])start++;
  while(oldEnd>start&&end>start&&before[oldEnd-1]===after[end-1]){oldEnd--;end--;}
  return[{start:before.slice(0,start).join('').length,removed:before.slice(start,oldEnd).join(''),inserted:after.slice(start,end).join('')}];
}

export function verifyRelativeArrayOwnedRuleSourceHistory(packet){
  keys(packet,['schemaVersion','model','revision','source','previous','current','coordinateUnit','edits','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'relative array complete owned rule source history 1');
  assert.equal(packet.revision,RELATIVE_ARRAY_OWNED_RULE_REVISION);assert.equal(packet.source,RELATIVE_ARRAY_OWNED_RULE_SOURCE);
  assert.equal(packet.boundary,RELATIVE_ARRAY_OWNED_RULE_BOUNDARY);assert.equal(packet.coordinateUnit,'UTF-16 code units');
  keys(packet.previous,['source','bytes','sha256','certificate']);keys(packet.current,['source','bytes','sha256']);
  for(const[side,expected]of [['previous',RELATIVE_ARRAY_OWNED_RULE_PREVIOUS],['current',RELATIVE_ARRAY_OWNED_RULE_CURRENT]]){
    const row=packet[side];assert.equal(typeof row.source,'string');assert.deepEqual(pinRelativeArrayCurrentSource(row.source),expected);
    assert.deepEqual({bytes:row.bytes,sha256:row.sha256},expected);
  }
  verifyGitArtifactCertificate(packet.previous.certificate,Buffer.from(packet.previous.source),{revision:packet.revision,path:packet.source});
  assert.equal(packet.current.source,readFileSync(new URL(packet.source,root),'utf8'),'the complete current owned-rule source stays separately bound to the filesystem');
  assert.deepEqual(packet.edits,deriveRelativeArrayOwnedRuleSourceIntervals(packet.previous.source,packet.current.source));
  let current=packet.previous.source;
  for(const edit of packet.edits){
    keys(edit,['start','removed','inserted']);assert.ok(Number.isSafeInteger(edit.start)&&edit.start>=0&&edit.start<=current.length);
    assert.equal(current.slice(edit.start,edit.start+edit.removed.length),edit.removed);
    current=current.slice(0,edit.start)+edit.inserted+current.slice(edit.start+edit.removed.length);
  }
  assert.equal(current,packet.current.source);
  for(const edit of packet.edits.toReversed()){
    assert.equal(current.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
    current=current.slice(0,edit.start)+edit.removed+current.slice(edit.start+edit.inserted.length);
  }
  assert.equal(current,packet.previous.source,'every complete certified predecessor byte survives');
  restoreRelativeArrayOwnedRuleBaselinePacket(packet);return packet;
}

/** The new rules append to the complete prior source; reconstruct the entire
 * original packet rather than replacing its historical identity with a pin. */
export function restoreRelativeArrayOwnedRuleBaselinePacket(packet){
  const bytes=Buffer.from(packet.current.source),prefix=bytes.subarray(0,RELATIVE_ARRAY_OWNED_RULE_BASELINE.bytes);
  const source=prefix.toString('utf8');assert.ok(Buffer.from(source).equals(prefix));
  assert.deepEqual(pinRelativeArrayCurrentSource(source),RELATIVE_ARRAY_OWNED_RULE_BASELINE);
  const original={...packet,current:{source,...RELATIVE_ARRAY_OWNED_RULE_BASELINE},
    edits:deriveRelativeArrayOwnedRuleSourceIntervals(packet.previous.source,source)};
  assert.deepEqual(pinRelativeArrayCurrentSource(serializeProjectData(original,RELATIVE_ARRAY_OWNED_RULE_OUTPUT)),RELATIVE_ARRAY_OWNED_RULE_BASELINE_PACKET,
    'every complete original source, certificate, field and packet byte remains reconstructible');
  return original;
}

export function readRelativeArrayOwnedRuleSourceHistory(){
  if(!cached){
    assert.match(packetSha256,/^[a-f0-9]{64}$/u,'only a normally captured, independently pinned source packet is readable');
    const source=readFileSync(new URL(RELATIVE_ARRAY_OWNED_RULE_OUTPUT,root));assert.equal(pinRelativeArrayCurrentSource(source).sha256,packetSha256);
    cached=verifyRelativeArrayOwnedRuleSourceHistory(readBootstrapStructuredDataLino(source.toString('utf8')));
  }
  assert.equal(readFileSync(new URL(RELATIVE_ARRAY_OWNED_RULE_SOURCE,root),'utf8'),cached.current.source);
  return structuredClone(cached);
}

/** Other profiles and files are identities; unknown selected bytes stay visible
 * to the original descriptor/freshness guards rather than gaining an old owner. */
export function restoreRelativeArrayCurrentSourceView(source,file,profile){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');assert.equal(typeof profile,'string');
  if(profile!==RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE)return source;
  if(RAW_CONSUMING_SNAPSHOT_FILES.some(row=>row.file===file)){
    const bytes=Buffer.from(source),snapshot=restoreRawConsumingSnapshotSource(bytes,file,true);
    if(snapshot!==bytes)return snapshot.toString('utf8');
  }
  if(file==='js/src/translation/emit-javascript.js')return restoreReviewedOverlayProviderModuleSource(source,file);
  if(file==='js/src/typed-program-link-rules.js')return restoreCurrentOwnCodebaseRawSource(source,file,true);
  if(file!==RELATIVE_ARRAY_OWNED_RULE_SOURCE)return source;
  const packet=readRelativeArrayOwnedRuleSourceHistory();
  return source===packet.current.source?packet.previous.source:source;
}
