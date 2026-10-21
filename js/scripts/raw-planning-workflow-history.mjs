// Only a complete reviewed current workflow acquires its immediate predecessor.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreGrammarTestPartitionHistory} from './grammar-test-partition-history.mjs';
const archive=new URL('../../parity/self-translation/raw-planning-workflow-history.lino',import.meta.url);
const archiveHash='23e42299350a76de06cd2c986c8739c63f1213835c9f5cae59d11ed9bb950e46';
let history;

/** Preserve unknown changes for the original guard; authenticate only exact bytes. */
export function restoreRawPlanningWorkflowHistory(source,file) {
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');
  source=restoreGrammarTestPartitionHistory(source,file);
  // Legacy guards identify one of these workflows by its complete digest.
  // The certificate below always authenticates the actual JavaScript owner.
  if(!['.github/workflows/js.yml','.github/workflows/ci.yml','.github/workflows/rust.yml'].includes(file))return source;
  if(!history){
    const bytes=readFileSync(archive);assert.equal(createHash('sha256').update(bytes).digest('hex'),archiveHash);
    history=readProjectData(archive);
  }
  if(createHash('sha256').update(source).digest('hex')!==history.current.sha256)return source;
  assert.equal(Buffer.byteLength(source),history.current.bytes);
  const bytes=Buffer.from(source),edit=history.edit;
  assert.equal(edit.offsetUnit,'UTF-8 bytes');
  assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=0&&edit.end>=edit.start&&edit.end<=bytes.length);
  assert.equal(bytes.subarray(edit.start,edit.end).toString(),edit.inserted);
  const original=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
  assert.equal(original.toString(),history.source);
  verifyGitArtifactCertificate(history.certificate,original,{
    revision:'76e27c44c5854a2cb547aea5cef06ea80adf729c',path:'.github/workflows/js.yml'});
  return history.source;
}
