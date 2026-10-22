// Source conservation for a finite physical partition change; no test evidence.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
export const CI_TEST_PARTITION_REVISION='491cf0caa9f014e7f4f4caef62611fa482114721';
export const CI_TEST_PARTITION_HISTORY='parity/fixtures/ci-39839936265-test-partitions.lino';
const packetSha256='1f0cd6d7e5c01ffd2e219094a3bcd82be6ae4f7a420223b64e2b8650482e4eab';
const matrix=(group,count)=>Array.from({length:count},(_,index)=>`          - {group: ${group}, partition: ${index+1}, shards: ${count}}\n`).join('');
export const CI_TEST_PARTITION_CHANGES=Object.freeze([
 {before:matrix('conformance',4),after:matrix('conformance',8)},
 {before:matrix('translation',6),after:matrix('translation',12)},
 {before:'node scripts/test-groups.mjs --run "$TEST_GROUP"',after:'node scripts/ci-test-partitions.mjs --run "$TEST_GROUP"'},
 {before:'pattern: translation-test-feedback-${{ github.sha }}-*-of-6',after:'pattern: translation-test-feedback-${{ github.sha }}-*-of-12'},
 {before:'            --partitions 6 \\\n',after:'            --partitions 12 \\\n'},
]);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let cached;
export function deriveCiTestPartitionWorkflow(source){
 let current=source;
 for(const{before,after}of CI_TEST_PARTITION_CHANGES){assert.equal(current.split(before).length,2);current=current.replace(before,()=>after);}
 return current;
}
export function verifyCiTestPartitionHistory(packet){
 assert.equal(packet.model,'complete CI physical test partition succession 1');assert.equal(packet.revision,CI_TEST_PARTITION_REVISION);
 assert.equal(packet.file,'.github/workflows/js.yml');assert.equal(packet.run,39839936265);
 assert.deepEqual(packet.changes,CI_TEST_PARTITION_CHANGES);
 assert.deepEqual(pin(packet.previous.source),{bytes:packet.previous.bytes,sha256:packet.previous.sha256});
 verifyGitArtifactCertificate(packet.previous.certificate,Buffer.from(packet.previous.source),{revision:packet.revision,path:packet.file});
 const source=deriveCiTestPartitionWorkflow(packet.previous.source);assert.equal(source,packet.current.source);
 assert.deepEqual(pin(source),{bytes:packet.current.bytes,sha256:packet.current.sha256});
 let inverse=source;for(const{before,after}of [...packet.changes].reverse()){assert.equal(inverse.split(after).length,2);inverse=inverse.replace(after,()=>before);}
 assert.equal(inverse,packet.previous.source);return packet;
}
export function readCiTestPartitionHistory(){
 if(!cached){const bytes=readFileSync(new URL('../../'+CI_TEST_PARTITION_HISTORY,import.meta.url));assert.equal(pin(bytes).sha256,packetSha256);cached=verifyCiTestPartitionHistory(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
 return structuredClone(cached);
}
export function restoreCiTestPartitionWorkflow(source){
 const packet=readCiTestPartitionHistory();return source===packet.current.source?packet.previous.source:source;
}
