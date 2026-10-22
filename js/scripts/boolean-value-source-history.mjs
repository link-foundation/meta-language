// The original guard observes its exact certified helper, never a partial edit.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const sourceArchiveHash='d8b43ce60d714f7bf3422e0ff373c290238a77f9f5d14086b62333e0b46c984d';
let history;

/** Authenticate the complete immediate predecessor of the raw partition guard. */
export function restoreBooleanValueGuardHistory(source,path) {
  assert.equal(typeof source,'string');assert.equal(typeof path,'string');
  if(path!=='js/tests/support/raw-contract-partition-history.js')return source;
  if(!history){
    const bytes=readFileSync(new URL('../../parity/naming/boolean-value-source-history.lino',import.meta.url));
    assert.equal(createHash('sha256').update(bytes).digest('hex'),sourceArchiveHash);
    history=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(history.revision,'76e27c44c5854a2cb547aea5cef06ea80adf729c');
  }
  const row=history.sources.find(row=>row.file===path);assert.ok(row);
  if(createHash('sha256').update(source).digest('hex')!==row.current.sha256)return source;
  assert.equal(Buffer.byteLength(source),row.current.bytes);
  const original=verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{
    revision:history.revision,path});
  assert.equal(original.length,row.bytes);
  assert.equal(createHash('sha256').update(original).digest('hex'),row.sha256);
  return row.sourceText;
}
