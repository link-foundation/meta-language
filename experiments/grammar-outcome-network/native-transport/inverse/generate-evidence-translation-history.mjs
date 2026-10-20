// Authenticate all predecessor source definitions behind the complete CI repair batch.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
const root=new URL('../../../../',import.meta.url),revision='ab8881173efbb657ef139518b34de2ff910cdbe7';
const sources=[
  "js/tests/support/ci-node-runtime-history.js",
  "js/tests/issue-195-evidence-translation-feedback.test.js",
  "js/tests/support/platform-test-deadline-history.js",
  "js/scripts/ci-repair-source-history.mjs",
  "js/scripts/native-graph-source-history.mjs",
  "js/tests/support/verification-concurrency-history.js",
  "js/scripts/generate-translation-workflow-history.mjs",
  "js/tests/support/translation-workflow-history.js"
];
const fixtures=[".github/workflows/ci.yml"];
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
/** Capture exact old modules and current digests; this proves preservation only. */
export function generateEvidenceTranslationHistory({check=false}={}) {
  const packets=[];
  for(const [kind,paths]of [['sources',sources],['fixtures',fixtures]]){
    const rows=paths.map(file=>{
      const original=captureGitArtifactCertificate(revision,file);
      verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
      const sourceText=original.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),original.blob);
      const current=readFileSync(new URL(file,root));assert.deepEqual(Buffer.from(current.toString('utf8')),current);
      return{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current)};
    });
    const packet={schemaVersion:1,revision,sources:rows,
      boundary:'Complete authenticated original modules and fixtures remain available after CI repairs. Different current operations have separate identities. Source conservation establishes neither native execution nor general semantic equivalence; unchanged fixture inputs and independent controls remain mandatory.'};
    const output=new URL('parity/self-translation/evidence-translation-source-'+kind+'-history.lino',root);
    if(check)checkProjectData(output,packet);else writeProjectData(output,packet);packets.push(packet);
  }
  const helper=new URL('js/scripts/evidence-translation-source-history.mjs',root),source=readFileSync(helper,'utf8');
  const path=new URL('parity/self-translation/evidence-translation-source-fixtures-history.lino',root);
  const expected=source.replace(/const archiveHash='(?:PENDING|[a-f0-9]{64})';/u,"const archiveHash='"+pin(readFileSync(path)).sha256+"';");
  const codePath=new URL('parity/self-translation/evidence-translation-source-sources-history.lino',root);
  const complete=expected.replace(/const codeArchiveHash='(?:PENDING|[a-f0-9]{64})';/u,"const codeArchiveHash='"+pin(readFileSync(codePath)).sha256+"';");
  if(check)assert.equal(source,complete);else writeFileSync(helper,complete);
  return packets;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packets=generateEvidenceTranslationHistory({check:process.argv.includes('--check')});
  console.log('Authenticated '+packets.reduce((count,packet)=>count+packet.sources.length,0)+' complete predecessor files and current source digests.');
}
