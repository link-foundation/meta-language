import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {restoreBooleanValueGuardHistory} from '../scripts/boolean-value-source-history.mjs';

test('the exact current raw guard reconstructs its whole certified predecessor and retains unknown edits',()=>{
  const file='js/tests/support/raw-contract-partition-history.js';
  const source=readFileSync(new URL('../../'+file,import.meta.url),'utf8');
  const archive=readProjectData(new URL('../../parity/naming/boolean-value-source-history.lino',import.meta.url));
  const row=archive.sources.find(value=>value.file===file);assert.ok(row);
  const previous=restoreBooleanValueGuardHistory(source,file);
  assert.equal(previous,row.sourceText);assert.notEqual(previous,source);
  verifyGitArtifactCertificate(row.certificate,Buffer.from(previous),{
    revision:'76e27c44c5854a2cb547aea5cef06ea80adf729c',path:file});
  assert.equal(restoreBooleanValueGuardHistory(previous,file),previous);
  assert.equal(restoreBooleanValueGuardHistory(source+'\n# unrelated',file),source+'\n# unrelated');
  assert.equal(restoreBooleanValueGuardHistory(source.replace('restoreToolingPartitionHistory(source)',
    'restoreToolingPartitionHistory(changed)'),file),source.replace('restoreToolingPartitionHistory(source)',
    'restoreToolingPartitionHistory(changed)'));
  assert.equal(restoreBooleanValueGuardHistory(source,'unrelated.js'),source);
  assert.throws(()=>restoreBooleanValueGuardHistory(null,file));
});
