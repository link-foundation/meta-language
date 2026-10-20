// Exact source views for the current CI repair; current execution stays separate.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
const file='parity/self-translation/ci-39422441941-repair-source-history.lino';
const archiveSha256='1e73f33e00ffcc85d751ceefd8b589d286ce9c2bc6f2b64d64d5864e6a42ecdd';
const revision='90636f9fb52b1b9095defbac041c8089608b8576';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let history;

/** Unknown bytes remain visible to the original historical validator. Only a
 * complete independently pinned current source gets an original source view. */
export function restoreCurrentCiRepairSource(source,path) {
  assert.equal(typeof source,'string');assert.equal(typeof path,'string');
  source=restoreRawPlanningWorkflowHistory(source,path);
  source=restoreGrammarContextMetadataHistory(source,path);
  if(!history){const bytes=readFileSync(new URL(file,root));assert.equal(pin(bytes).sha256,archiveSha256);history=readBootstrapStructuredDataLino(bytes.toString('utf8'));assert.equal(history.revision,revision);}
  const row=history.sources.find(row=>row.file===path);if(!row)return source;
  const actual=pin(source);
  if(actual.sha256!==row.current.sha256)return source;
  assert.deepEqual(actual,row.current);
  const bytes=Buffer.from(source),edit=row.edit;
  assert.equal(edit.offsetUnit,'UTF-8 bytes');
  assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=0&&edit.end>=edit.start&&edit.end<=bytes.length);
  assert.equal(bytes.subarray(edit.start,edit.end).toString('utf8'),edit.inserted);
  const original=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
  assert.deepEqual(pin(original),{bytes:row.bytes,sha256:row.sha256});assert.equal(original.toString('utf8'),row.sourceText);
  verifyGitArtifactCertificate(row.certificate,original,{revision,path});
  return row.sourceText;
}

import {restoreGrammarContextMetadataHistory} from './grammar-context-metadata-history.mjs';
import {restoreRawPlanningWorkflowHistory} from './raw-planning-workflow-history.mjs';
