// Authenticate all predecessor source definitions behind the complete CI repair batch.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
const root=new URL('../../../../',import.meta.url),revision='231c6610fc93225fccb1682ab2943729e0c0e52e';
const sources=[
  "rust/tests/unit/mod.rs",
  "js/tests/translation-current-ci-source-history.test.js",
  "js/tests/translation-scheduling-source-history.test.js",
  "experiments/grammar-outcome-network/native-transport/inverse/generate-scheduling-repair-archive.mjs"
];
const fixtures=[];
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
/** Capture exact old modules and current digests; this proves preservation only. */
export function generateNativeGraphSourceHistory({check=false}={}) {
  const packets=[];
  for(const [kind,paths]of [['sources',sources],['fixtures',fixtures]]){
    const rows=paths.map(file=>{
      const original=captureGitArtifactCertificate(revision,file);
      verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
      const sourceText=original.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),original.blob);
      const current=Buffer.from(restoreCurrentCiRepairSource(readFileSync(new URL(file,root),'utf8'),file));assert.deepEqual(Buffer.from(current.toString('utf8')),current);
      return{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current)};
    });
    const packet={schemaVersion:1,revision,sources:rows,
      boundary:'Complete authenticated original modules and fixtures remain available after CI repairs. Different current operations have separate identities. Source conservation establishes neither native execution nor general semantic equivalence; unchanged fixture inputs and independent controls remain mandatory.'};
    const output=new URL('parity/self-translation/concrete-program-native-source-'+kind+'-history.lino',root);
    if(check)checkProjectData(output,packet);else writeProjectData(output,packet);packets.push(packet);
  }
  const helper=new URL('js/scripts/native-graph-source-history.mjs',root),source=readFileSync(helper,'utf8');
  const path=new URL('parity/self-translation/concrete-program-native-source-sources-history.lino',root);
  const expected=source.replace(/const archiveHash='(?:PENDING|[a-f0-9]{64})';/u,"const archiveHash='"+pin(readFileSync(path)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helper,expected);
  return packets;
}

import {restoreCurrentCiRepairSource} from '../../../../js/scripts/current-ci-repair-source-history.mjs';
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packets=generateNativeGraphSourceHistory({check:process.argv.includes('--check')});
  console.log('Authenticated '+packets.reduce((count,packet)=>count+packet.sources.length,0)+' complete predecessor files and current source digests.');
}
