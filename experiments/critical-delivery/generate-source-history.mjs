// Preserve every original conversation obligation before adding gate observations.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {restoreCriticalDeliveryConversationSource} from '../../js/scripts/tooling-frozen-source-history.mjs';
import {checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';

const root=new URL('../../',import.meta.url),revision='e5d0ff8265bcd3136cffd42d53942c2e4b4eb2ea';
const pin=bytes=>({bytes:Buffer.byteLength(bytes),sha256:createHash('sha256').update(bytes).digest('hex')});
export function generateCriticalDeliverySourceHistory({check=false}={}) {
  const file='js/scripts/issue-195-conversation-requirements.mjs',original=captureGitArtifactCertificate(revision,file);
  verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
  const sourceText=original.blob.toString('utf8'),current=restoreCriticalDeliveryConversationSource(readFileSync(new URL(file,root),'utf8'));
  const addition="  'I195-CONVERSATION-CRITICAL-DELIVERY': [\n    'js/scripts/issue-195-requirements.mjs', 'js/scripts/issue-195-acceptance-lib.mjs',\n    'js/tests/issue-195-evidence-critical-delivery.test.js',\n  ],\n";
  const anchor='const sourceTooling = {\n';assert.equal(sourceText.split(anchor).length,2);
  assert.equal(current,sourceText.replace(anchor,anchor+addition),'only the executable tooling entry point appends');
  assert.equal(current.split(addition).length,2);assert.equal(current.replace(addition,''),sourceText,'every complete original obligation and source byte survives');
  const history={schemaVersion:1,revision,sources:[{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current)}],
    addition,anchor,requirement:'I195-CONVERSATION-CRITICAL-DELIVERY',
    boundary:'The complete preceding conversation source is independently Git-certified. One tooling implementation entry point is added without removing or changing any obligation, source, scope, fixture, assertion, runtime, checkpoint or gate. Source preservation supplies no passing execution evidence.'};
  const output=new URL('../../parity/self-translation/critical-delivery-source-history.lino',import.meta.url);
  if(check)checkProjectData(output,history);else writeProjectData(output,history);return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateCriticalDeliverySourceHistory({check:process.argv.includes('--check')});
  console.log('Retained every complete original conversation source byte while adding critical delivery gate controls.');
}
