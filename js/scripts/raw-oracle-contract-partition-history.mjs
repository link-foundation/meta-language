// Authenticate complete source views; unrelated changes stay observable.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const archiveHash='97bc907646b4f88823015ce1c5775a2ca3a4a3d7b6f4da798a96bc2265e14f60';
let history;

const reportScheduleHistory = new Map([
  ['6274838f6f901b71b1f0540b40b741640a8c23abde25b7afde91368bee4186bf', {
    file: '.github/workflows/js.yml', bytes: 88666,
    sha256: 'ef8805928effb164e3ba8497214b4168ad2d03497d28c9d356dbf57471b1a63e',
  }],
  ['3fa0faf82dc34b1b0fff35c671570f0dc21dd7b124bc1d51d72dde5af695f39b', {
    file: '.github/workflows/ci.yml', bytes: 92242,
    sha256: 'cb5b398a05d3c1cf724907efe4f998d8a65c5dd9a5a13dcf64e2b927abfabc2b',
  }],
]);

/** Admit only the complete reviewed report matrix change for historical tests. */
export function restoreReportScheduleHistory(source) {
  const previous = reportScheduleHistory.get(createHash('sha256').update(source).digest('hex'));
  if (!previous) return source;
  const original = source
    .replaceAll('${{ matrix.shard }}/16', '${{ matrix.shard }}/8')
    .replace('shard: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]',
      'shard: [1, 2, 3, 4, 5, 6, 7, 8]');
  assert.equal(Buffer.byteLength(original), previous.bytes, previous.file);
  assert.equal(createHash('sha256').update(original).digest('hex'), previous.sha256, previous.file);
  return original;
}

/** Restore complete certified predecessors for the original raw CI guards. */
export function restoreRawOracleContractPartitionHistory(source,path) {
  assert.equal(typeof source,'string');assert.equal(typeof path,'string');
  const workflow=['.github/workflows/js.yml','.github/workflows/ci.yml','.github/workflows/rust.yml'].includes(path);
  if(!workflow&&path!=='experiments/original-link-complete-table-projection/raw-native-observations.test.mjs')return source;
  if (workflow) source = restoreReportScheduleHistory(source);
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
