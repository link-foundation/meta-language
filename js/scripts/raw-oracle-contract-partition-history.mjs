// Authenticate complete source views; unrelated changes stay observable.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const archiveHash='97bc907646b4f88823015ce1c5775a2ca3a4a3d7b6f4da798a96bc2265e14f60';
let history;

/** Restore complete certified predecessors for the original raw CI guards. */
export function restoreRawOracleContractPartitionHistory(source,path) {
  assert.equal(typeof source,'string');assert.equal(typeof path,'string');
  const workflow=['.github/workflows/js.yml','.github/workflows/ci.yml','.github/workflows/rust.yml'].includes(path);
  if(!workflow&&path!=='experiments/original-link-complete-table-projection/raw-native-observations.test.mjs')return source;
  if(!history){
    const bytes=readFileSync(new URL('../../parity/self-translation/raw-oracle-contract-partitions.lino',import.meta.url));
    assert.equal(createHash('sha256').update(bytes).digest('hex'),archiveHash);
    history=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(history.revision,'3a8cbfb2d87366702f7bc7a50182892d4ee1188a');
  }
  const row=history.sources.find(row=>row.path===(workflow?'.github/workflows/js.yml':path));assert.ok(row);
  if(createHash('sha256').update(source).digest('hex')!==row.current.sha256)return source;
  assert.equal(Buffer.byteLength(source),row.current.bytes);
  let previous=source;
  for(const edit of [...row.edits].reverse()){
    assert.equal(previous.split(edit.after).length,2,'one exact complete source adaptation');
    previous=previous.replace(edit.after,()=>edit.before);
  }
  assert.equal(previous,row.source);
  verifyGitArtifactCertificate(row.certificate,Buffer.from(previous),{revision:history.revision,path:row.path});
  return previous;
}
