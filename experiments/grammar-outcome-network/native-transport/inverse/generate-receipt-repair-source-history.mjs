// Authenticate all predecessor source definitions behind the complete CI repair batch.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
const root=new URL('../../../../',import.meta.url),revision='6f32c2924d6284e0a43130f225ed817450f1f2f8';
const sources=[
  "experiments/original-link-complete-table-projection/raw-borrowed-emission.mjs",
  "experiments/original-link-complete-table-projection/raw-emission-profiles.test.mjs",
  "js/scripts/complete-declaration-receipts.mjs",
  "js/scripts/current-translation-source-descriptors.mjs",
  "js/scripts/translation-source-history.mjs",
  "js/tests/translation-complete-declaration-receipts.test.js"
];
const fixtures=[
  "experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino",
  "experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino"
];
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
/** Capture exact old modules and current digests; this proves preservation only. */
export function generateReceiptRepairSourceHistory({check=false}={}) {
  const packets=[];
  for(const [kind,paths]of [['sources',sources],['fixtures',fixtures]]){
    const rows=paths.map(file=>{
      const original=captureGitArtifactCertificate(revision,file);
      verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
      const sourceText=original.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),original.blob);
      const current=readFileSync(new URL(file,root));assert.deepEqual(Buffer.from(current.toString('utf8')),current);
      return{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current)};
    });
    const packet={schemaVersion:1,revision,run:39372455987,sources:rows,
      boundary:'Complete authenticated original modules and fixtures remain available after CI repairs. Different current operations have separate identities. Source conservation establishes neither native execution nor general semantic equivalence; unchanged fixture inputs and independent controls remain mandatory.'};
    const output=new URL('parity/self-translation/ci-39372455987-receipt-'+kind+'-history.lino',root);
    if(check)checkProjectData(output,packet);else writeProjectData(output,packet);packets.push(packet);
  }
  return packets;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packets=generateReceiptRepairSourceHistory({check:process.argv.includes('--check')});
  console.log('Authenticated '+packets.reduce((count,packet)=>count+packet.sources.length,0)+' complete predecessor files and current source digests.');
}
