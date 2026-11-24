// Exact archive metadata succession; every historical semantic field is retained.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
export const RAW_ARCHIVE_RECEIPT_HISTORY='parity/self-translation/ci-39830836214-raw-archive-history.lino';
const packetSha256='4e9225f045ff899a2bfb3b400619dad914fad2f5f4dce1ace11929cac8c54bef';
const revision='f73a10b22f8d0bcfbf4707621bcc9edd6989533d';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let cached;
export function verifyRawArchiveReceiptHistory(packet){
 assert.equal(pin(serializeProjectData(packet,RAW_ARCHIVE_RECEIPT_HISTORY)).sha256,packetSha256,'the complete archive metadata review is pinned');
 assert.equal(packet.revision,revision);assert.equal(packet.model,'complete raw archive metadata succession 1');
 assert.deepEqual(packet.rows.map(row=>row.file),['experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino']);
 for(const[index,row]of packet.rows.entries()){
  const paths=[['archive','textBytes'],['archive','textSha256'],['archive','binarySha256']];
  assert.deepEqual(row.changes.map(change=>change.path),paths,'no semantic or replay field can change');
  assert.match(row.current.sha256,/^[a-f0-9]{64}$/u);assert.ok(Number.isSafeInteger(row.current.bytes)&&row.current.bytes>0);
 }
 return packet;
}
export function readRawArchiveReceiptHistory(){
 if(!cached){const bytes=readFileSync(new URL(RAW_ARCHIVE_RECEIPT_HISTORY,root));assert.equal(pin(bytes).sha256,packetSha256);cached=verifyRawArchiveReceiptHistory(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
 return structuredClone(cached);
}
/** Unknown complete bytes remain unknown to the existing downstream gate. */
export function restoreRawArchiveReceiptText(source,file){
 if(!file.endsWith('/raw-consuming-current-native-definitions.lino'))return source;
 const row=readRawArchiveReceiptHistory().rows.find(row=>row.file===file);if(!row)return source;
 if(pin(source).sha256!==row.current.sha256)return source;
 assert.deepEqual(pin(source),row.current);
 const value=readBootstrapStructuredDataLino(source);
 for(const change of row.changes){
  const parent=value[change.path[0]],field=change.path[1];assert.ok(Object.hasOwn(parent,field));assert.deepEqual(parent[field],change.after);parent[field]=change.before;
 }
 const original=serializeProjectData(value,file);assert.deepEqual(pin(original),row.previous);
 verifyGitArtifactCertificate(row.certificate,Buffer.from(original),{revision,path:file});
 return original;
}
