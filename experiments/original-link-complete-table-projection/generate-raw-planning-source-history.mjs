import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
const root=new URL('../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

/** Preserve complete predecessors before qualifying the current planning rules. */
export function generateOriginalLinkRawPlanningHistory({check=false}={}) {
  const revision='c7469b0ef86bed0aed2497a16243c842b184a204';
  const files=['experiments/original-link-complete-table-projection/raw-native-observations.mjs',
    'experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs',
    'js/scripts/grammar-context-metadata-history.mjs'];
  const sources=files.map(file=>{
    const original=captureGitArtifactCertificate(revision,file);
    verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
    const sourceText=original.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),original.blob);
    return{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,
      current:pin(readFileSync(new URL(file,root)))};
  });
  const value={schemaVersion:1,revision,sources,
    boundary:'Complete original planning source definitions retain independent Git membership and every byte. Current resource admission and immutable-data traversal have separate source-qualified identities. This source archive supplies no native execution, workload timing or semantic equivalence evidence.'};
  const output=new URL('parity/naming/raw-planning-source-history.lino',root);
  if(check)checkProjectData(output,value);else writeProjectData(output,value);
  return value;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const value=generateOriginalLinkRawPlanningHistory({check:process.argv.includes('--check')});
  console.log('Retained '+value.sources.length+' complete original planning sources and their current contexts.');
}
