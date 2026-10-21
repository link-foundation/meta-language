// A finite source-only view for the immutable mutable-array host author.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
export const MUTABLE_ARRAY_FRONTEND_FILE='js/src/translation/frontend-rules.js';
export const MUTABLE_ARRAY_FRONTEND_PREVIOUS_REVISION='6ffff3b2dc530a5f24c128dadf877f3fa106932b';
export const MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION='3fe3a9f9decfa6431b5b0718729a5d123b5f236b';
export const MUTABLE_ARRAY_FRONTEND_OUTPUT='parity/self-translation/javascript-mutable-array-frontend-source-history.lino';
export const MUTABLE_ARRAY_FRONTEND_PREVIOUS=Object.freeze({"bytes": 32262, "sha256": "7dbdc1f0c6dac091c794e9c0934cc5a55ed00d444cf037eb46843e8ddd515e63"});
export const MUTABLE_ARRAY_FRONTEND_CURRENT=Object.freeze({"bytes": 32290, "sha256": "44fde15e3f286c8d891519819251433295c5a7ef1e4a6e1424fd846800809ce6"});
export const MUTABLE_ARRAY_FRONTEND_HOST_PACKET=Object.freeze({file:'parity/self-translation/javascript-mutable-array-binding-hosts.lino',...{"bytes": 1625786, "sha256": "bb2f5ca4e690014ed0a08c984854b39e13cbc7a8a7dbc9be8f7d292b30a21e7b"}});
const packetSha256='872547aaae96f2d83ccab99b30f2e4d7265f596b2640f16c592cad46570b4cf5';
const beforeLine="  return language === 'JavaScript' && sourceKind === 'array' && expectedKind === 'option' && presentKind === 'tuple';";
const afterLine="  return language === 'JavaScript' && (sourceKind === 'array' || sourceKind === 'tuple') && expectedKind === 'option' && presentKind === 'tuple';";
const declaration='export function acceptJavaScriptOptionalTupleContext(language, sourceKind, expectedKind, presentKind) {\n';
let cached;
export const pinMutableArrayFrontendSource=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),[...keys].sort());};
export function deriveMutableArrayFrontendSourceEdit(previous,current){
  assert.deepEqual(pinMutableArrayFrontendSource(previous),MUTABLE_ARRAY_FRONTEND_PREVIOUS);
  assert.deepEqual(pinMutableArrayFrontendSource(current),MUTABLE_ARRAY_FRONTEND_CURRENT);
  const before=declaration+beforeLine+'\n}',after=declaration+afterLine+'\n}';
  assert.equal(previous.split(before).length,2);assert.equal(current.split(after).length,2);
  assert.equal(current,previous.replace(before,()=>after),'one complete optional-tuple declaration changes; every mutable decision byte remains exact');
  const start=previous.indexOf(before)+declaration.length;
  assert.equal(current.slice(0,start)+beforeLine+current.slice(start+afterLine.length),previous,'the entire independently certified predecessor is recovered');
  return{start,removed:beforeLine,inserted:afterLine};
}
export function verifyMutableArrayFrontendSourceHistory(packet,revision=MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION){
  assert.match(revision,/^[a-f0-9]{40}$/u,'a full actual current committed source revision is required');
  exactKeys(packet,['schemaVersion','model','file','previousRevision','revision','coordinateUnit','previous','current','edit','hostPacket','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'mutable array frontend historical source 1');
  assert.equal(packet.file,MUTABLE_ARRAY_FRONTEND_FILE);assert.equal(packet.previousRevision,MUTABLE_ARRAY_FRONTEND_PREVIOUS_REVISION);assert.equal(packet.revision,revision);
  assert.equal(packet.coordinateUnit,'UTF-16 code units');
  for(const[key,pin,commit]of[['previous',MUTABLE_ARRAY_FRONTEND_PREVIOUS,MUTABLE_ARRAY_FRONTEND_PREVIOUS_REVISION],['current',MUTABLE_ARRAY_FRONTEND_CURRENT,revision]]){
    const row=packet[key];exactKeys(row,['source','bytes','sha256','certificate']);assert.equal(typeof row.source,'string');
    assert.equal(Buffer.from(row.source).toString('utf8'),row.source);
    assert.deepEqual(pinMutableArrayFrontendSource(row.source),pin);assert.deepEqual({bytes:row.bytes,sha256:row.sha256},pin);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:commit,path:packet.file});
  }
  assert.deepEqual(packet.edit,deriveMutableArrayFrontendSourceEdit(packet.previous.source,packet.current.source));
  assert.deepEqual(packet.hostPacket,MUTABLE_ARRAY_FRONTEND_HOST_PACKET);
  assert.deepEqual(pinMutableArrayFrontendSource(readFileSync(new URL(packet.hostPacket.file,root))),{bytes:packet.hostPacket.bytes,sha256:packet.hostPacket.sha256},'every immutable old mutable-array host/archive byte remains unchanged');
  assert.equal(packet.boundary,'Only the mutable-array historical source author receives this certified complete predecessor view. Actual optional-tuple semantics, generated frontend Rust, styles, native hosts and all other source readers stay current; unknown sources remain visible to their original guards.');
  return packet;
}
export function readMutableArrayFrontendSourceHistory(){
  if(!cached){const bytes=readFileSync(new URL(MUTABLE_ARRAY_FRONTEND_OUTPUT,root));assert.equal(pinMutableArrayFrontendSource(bytes).sha256,packetSha256);cached=verifyMutableArrayFrontendSourceHistory(readProjectData(new URL(MUTABLE_ARRAY_FRONTEND_OUTPUT,root)));}
  return structuredClone(cached);
}
export function restoreMutableArrayFrontendSource(source,file,reader){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');assert.equal(typeof reader,'string');
  if(file!==MUTABLE_ARRAY_FRONTEND_FILE||reader!=='mutable array binding decision source')return source;
  const actual=pinMutableArrayFrontendSource(source);
  if(actual.bytes!==MUTABLE_ARRAY_FRONTEND_CURRENT.bytes||actual.sha256!==MUTABLE_ARRAY_FRONTEND_CURRENT.sha256)return source;
  const packet=readMutableArrayFrontendSourceHistory();assert.equal(source,packet.current.source);
  return packet.previous.source;
}
