// Only historical readers select this finite committed source view.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
export const SHARED_ARRAY_NATIVE_FRONTEND_HISTORY='parity/self-translation/shared-array-native-frontend-history.lino';
export const SHARED_ARRAY_NATIVE_FRONTEND_PREVIOUS='56822853bb29d6f1002a1d5293535447342d675d';
export const SHARED_ARRAY_NATIVE_FRONTEND_CURRENT='491cf0caa9f014e7f4f4caef62611fa482114721';
export const SHARED_ARRAY_NATIVE_FRONTEND_FILES=Object.freeze(['rust/src/translation/rust/expressions.rs','rust/src/translation/rust/expressions/control.rs','rust/src/typed_program_links/projection.rs','js/src/translation/check.js','rust/src/translation/surface/nodes.rs','rust/src/translation/check/expressions.rs','js/src/typed-program-expression-projection.js']);
const packetSha256='dc846f2292d28a461f8e75803f61eb710c444a65c3892dd4fb0f9ffc348ed88e';
export const pinSharedArrayNativeFrontend=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let cached;
export function deriveSharedArrayNativeFrontendEdit(before,after){
 const original=[...before],current=[...after];let start=0,end=original.length,finish=current.length;
 while(start<end&&start<finish&&original[start]===current[start])start++;
 while(end>start&&finish>start&&original[end-1]===current[finish-1]){end--;finish--;}
 return{start:original.slice(0,start).join('').length,removed:original.slice(start,end).join(''),inserted:current.slice(start,finish).join('')};
}
export function verifySharedArrayNativeFrontendHistory(packet){
 assert.deepEqual(Object.keys(packet).sort(),['boundary','currentRevision','model','previousRevision','rows','schemaVersion']);
 assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'complete shared array native frontend source succession 1');
 assert.equal(packet.previousRevision,SHARED_ARRAY_NATIVE_FRONTEND_PREVIOUS);assert.equal(packet.currentRevision,SHARED_ARRAY_NATIVE_FRONTEND_CURRENT);
 assert.deepEqual(packet.rows.map(row=>row.file),SHARED_ARRAY_NATIVE_FRONTEND_FILES);
 for(const row of packet.rows){
  assert.deepEqual(Object.keys(row).sort(),['current','edit','file','previous']);
  for(const[side,revision]of[['previous',packet.previousRevision],['current',packet.currentRevision]]){
   const value=row[side];assert.deepEqual(Object.keys(value).sort(),['bytes','certificate','sha256','source']);
   assert.deepEqual(pinSharedArrayNativeFrontend(value.source),{bytes:value.bytes,sha256:value.sha256});
   verifyGitArtifactCertificate(value.certificate,Buffer.from(value.source),{revision,path:row.file});
  }
  assert.deepEqual(row.edit,deriveSharedArrayNativeFrontendEdit(row.previous.source,row.current.source));
  const{start,removed,inserted}=row.edit;assert.equal(row.current.source.slice(start,start+inserted.length),inserted);
  assert.equal(row.current.source.slice(0,start)+removed+row.current.source.slice(start+inserted.length),row.previous.source);
 }
 return packet;
}
export function readSharedArrayNativeFrontendHistory(){
 if(!cached){const bytes=readFileSync(new URL('../../'+SHARED_ARRAY_NATIVE_FRONTEND_HISTORY,import.meta.url));assert.equal(pinSharedArrayNativeFrontend(bytes).sha256,packetSha256);cached=verifySharedArrayNativeFrontendHistory(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
 return structuredClone(cached);
}
export function restoreSharedArrayNativeFrontendSource(source,file){
 assert.equal(typeof source,'string');assert.equal(typeof file,'string');if(!SHARED_ARRAY_NATIVE_FRONTEND_FILES.includes(file))return source;
 const row=readSharedArrayNativeFrontendHistory().rows.find(row=>row.file===file);return source===row.current.source?row.previous.source:source;
}
