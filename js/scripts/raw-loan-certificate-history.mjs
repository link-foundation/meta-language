// Only the old certificate control reads this exact historical source view.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

const archive=new URL('../../parity/self-translation/ci-39389240046-check-sources-history.lino',import.meta.url);
const archiveHash='23a020717f26277f0571d94f0176fa05868a355cd8c67b6220a66242c958777d';
const digest=value=>createHash('sha256').update(value).digest('hex');

/** Preserve the certificate transition's whole original input after loan edits. */
export function restoreRawLoanCertificateSource(source) {
  assert.equal(typeof source,'string');
  assert.equal(digest(readFileSync(archive)),archiveHash,'the complete reviewed source archive stays exact');
  const history=readProjectData(archive);
  const file='experiments/original-link-complete-table-projection/raw-borrowed-emission.test.mjs';
  const rows=history.sources.filter(row=>row.file===file);assert.equal(rows.length,1);
  const row=rows[0];
  assert.equal(Buffer.byteLength(source),row.current.bytes,'the complete current raw loan control stays exact');
  assert.equal(digest(source),row.current.sha256,'changed raw loan controls require a new reviewed source view');
  const bytes=verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.equal(bytes.length,row.bytes);assert.equal(digest(bytes),row.sha256);
  return row.sourceText;
}
