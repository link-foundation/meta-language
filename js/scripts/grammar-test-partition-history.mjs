// Only the exact complete successor supplies a certified historical matrix.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreRawOracleContractPartitionHistory} from './raw-oracle-contract-partition-history.mjs';
const archiveHash='262cdbb79fdb8831799e5e5e255031c700c5b283a222c414c9277858fca289f1';
let history;

/** Preserve unknown edits; authenticate the whole reviewed grammar schedule. */
export function restoreGrammarTestPartitionHistory(source,path) {
  assert.equal(typeof source,'string');assert.equal(typeof path,'string');
  source=restoreRawOracleContractPartitionHistory(source,path);
  if(!['.github/workflows/js.yml','.github/workflows/ci.yml','.github/workflows/rust.yml'].includes(path))return source;
  if(!history){
    const bytes=readFileSync(new URL('../../parity/self-translation/grammar-test-partition-history.lino',import.meta.url));
    assert.equal(createHash('sha256').update(bytes).digest('hex'),archiveHash);
    history=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(history.revision,'394103f912aaac8e2f69147519ab97c6c5107528');
    assert.equal(history.path,'.github/workflows/js.yml');
  }
  if(createHash('sha256').update(source).digest('hex')!==history.current.sha256)return source;
  assert.equal(Buffer.byteLength(source),history.current.bytes);
  const bytes=Buffer.from(source),edit=history.edit;
  assert.equal(edit.offsetUnit,'UTF-8 bytes');
  assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)
    &&edit.start>=0&&edit.end>=edit.start&&edit.end<=bytes.length);
  assert.equal(bytes.subarray(edit.start,edit.end).toString('utf8'),edit.inserted);
  const previous=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
  assert.equal(previous.toString('utf8'),history.source);
  verifyGitArtifactCertificate(history.certificate,previous,{revision:history.revision,path:history.path});
  return history.source;
}
