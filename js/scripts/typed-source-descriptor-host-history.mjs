// Historical source views only; runtime providers continue to use actual code.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
export const TYPED_SOURCE_DESCRIPTOR_HOST_REVISION='d3e4076667add3e19e4cf5f10a3cea60255dd2d7';
export const TYPED_SOURCE_DESCRIPTOR_HOST_OUTPUT='parity/self-translation/typed-source-descriptor-host-history.lino';
export const TYPED_SOURCE_DESCRIPTOR_HOST_FILES=Object.freeze(['js/scripts/program-string-source-descriptors.mjs','js/scripts/generate-program-string-source-descriptors.mjs','js/scripts/generate-own-codebase-raw-source-descriptors.mjs','js/tests/translation-own-codebase-raw-source-descriptors.test.js','js/scripts/test-groups.mjs','js/tests/issue-195-evidence-node-dependency.test.js']);
const packetSha256='2172621bcbf5df42f79c124b968f4abb9f67df809b6950bf637cbbb46436a0ee';
export const pinTypedSourceDescriptorHost=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let cached;
export function deriveTypedSourceDescriptorHostEdit(before,after){
 const left=[...before],right=[...after];let start=0,end=left.length,finish=right.length;
 while(start<end&&start<finish&&left[start]===right[start])start++;
 while(end>start&&finish>start&&left[end-1]===right[finish-1]){end--;finish--;}
 return{start:left.slice(0,start).join('').length,removed:left.slice(start,end).join(''),inserted:right.slice(start,finish).join('')};
}
export function verifyTypedSourceDescriptorHostHistory(packet){
 assert.deepEqual(Object.keys(packet).sort(),['boundary','model','revision','rows','schemaVersion']);
 assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'complete typed source descriptor host history 1');assert.equal(packet.revision,TYPED_SOURCE_DESCRIPTOR_HOST_REVISION);
 assert.deepEqual(packet.rows.map(row=>row.file),TYPED_SOURCE_DESCRIPTOR_HOST_FILES);
 for(const row of packet.rows){
  assert.deepEqual(Object.keys(row).sort(),['current','edit','file','previous']);
  assert.deepEqual(Object.keys(row.previous).sort(),['bytes','certificate','sha256','source']);assert.deepEqual(Object.keys(row.current).sort(),['bytes','sha256','source']);
  assert.deepEqual(pinTypedSourceDescriptorHost(row.previous.source),{bytes:row.previous.bytes,sha256:row.previous.sha256});
  assert.deepEqual(pinTypedSourceDescriptorHost(row.current.source),{bytes:row.current.bytes,sha256:row.current.sha256});
  verifyGitArtifactCertificate(row.previous.certificate,Buffer.from(row.previous.source),{revision:packet.revision,path:row.file});
  assert.deepEqual(row.edit,deriveTypedSourceDescriptorHostEdit(row.previous.source,row.current.source));
  const {start,removed,inserted}=row.edit;assert.equal(row.current.source.slice(start,start+inserted.length),inserted);
  assert.equal(row.current.source.slice(0,start)+removed+row.current.source.slice(start+inserted.length),row.previous.source);
 }
 return packet;
}
export function readTypedSourceDescriptorHostHistory(){
 if(!cached){const bytes=readFileSync(new URL('../../'+TYPED_SOURCE_DESCRIPTOR_HOST_OUTPUT,import.meta.url));assert.equal(pinTypedSourceDescriptorHost(bytes).sha256,packetSha256);cached=verifyTypedSourceDescriptorHostHistory(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
 return structuredClone(cached);
}
export function restoreTypedSourceDescriptorHostSource(source,file){
 assert.equal(typeof source,'string');assert.equal(typeof file,'string');if(!TYPED_SOURCE_DESCRIPTOR_HOST_FILES.includes(file))return source;
 const row=readTypedSourceDescriptorHostHistory().rows.find(row=>row.file===file);return source===row.current.source?row.previous.source:source;
}
