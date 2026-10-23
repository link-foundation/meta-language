import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../../scripts/git-object-certificate.mjs';
import {restoreCiTestPartitionWorkflow} from '../../scripts/ci-test-partition-history.mjs';
const file=new URL('../../../parity/self-translation/translation-workflow-history.lino',import.meta.url);
const historyHash='d0789b7e0c01d83b684b71a3ff2c3a624782e6344a24fd4a891250a5e4385409';
const digest=value=>createHash('sha256').update(value).digest('hex');
let history;

/** Restore exact scheduling inputs before the retained historical CI assertions. */
export function restoreTranslationWorkflowHistory(source) {
  assert.equal(typeof source,'string');
  source=restoreCiTestPartitionWorkflow(source);
  source=restoreCurrentCiRepairSource(source,'.github/workflows/rust.yml');
  source=restoreEvidenceTranslationSourceHistory('.github/workflows/ci.yml',source);
  if(!history){assert.equal(digest(readFileSync(file)),historyHash);history=readProjectData(file);}
  const actual=digest(source),row=history.workflows.find(row=>row.current.sha256===actual||row.sha256===actual);
  if(!row)return source;
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
  if(actual===row.sha256){assert.equal(source,row.sourceText);return source;}
  assert.equal(Buffer.byteLength(source),row.current.bytes);
  const bytes=Buffer.from(source),edit=row.edit;assert.equal(bytes.subarray(edit.start,edit.end).toString('utf8'),edit.inserted);
  const restored=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
  assert.equal(restored.length,row.bytes);assert.equal(digest(restored),row.sha256);assert.equal(restored.toString('utf8'),row.sourceText);
  return row.sourceText;
}

import {restoreEvidenceTranslationSourceHistory} from '../../scripts/evidence-translation-source-history.mjs';

import {restoreCurrentCiRepairSource} from '../../scripts/current-ci-repair-source-history.mjs';
