import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { captureGitArtifactCertificate } from './capture-git-artifact-certificate.mjs';
import { dependencyWorkflowHistoryPath, dependencyWorkflowPredecessors } from './dependency-workflow-source-history.mjs';
import { checkProjectData, writeProjectData } from './project-data.mjs';

const root = new URL('../../', import.meta.url);
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const check = process.argv.includes('--check');

const sources = Object.entries(dependencyWorkflowPredecessors).map(([file, revision]) => {
  const current = readFileSync(new URL(file, root));
  const { certificate, blob } = captureGitArtifactCertificate(revision, file);
  let offset = 0;
  let beforeEnd = blob.length;
  let afterEnd = current.length;
  while (offset < beforeEnd && offset < afterEnd && blob[offset] === current[offset]) offset += 1;
  while (offset > 0 && (current[offset] & 192) === 128) offset -= 1;
  while (beforeEnd > offset && afterEnd > offset && blob[beforeEnd - 1] === current[afterEnd - 1]) {
    beforeEnd -= 1;
    afterEnd -= 1;
  }
  while (afterEnd < current.length && (current[afterEnd] & 192) === 128) {
    beforeEnd += 1;
    afterEnd += 1;
  }
  const predecessor = blob.subarray(offset, beforeEnd).toString('utf8');
  assert.deepEqual(Buffer.concat([current.subarray(0, offset), Buffer.from(predecessor), current.subarray(afterEnd)]), blob);
  return { file, currentDigest: digest(current), currentBytes: current.length, certificate,
    edit: { offset, length: afterEnd - offset, predecessor } };
});
const output = new URL(dependencyWorkflowHistoryPath, root);
const history = { schemaVersion: 1, sources };
if (check) checkProjectData(output, history);
else writeProjectData(output, history);

const helper = new URL('./dependency-workflow-source-history.mjs', import.meta.url);
const source = readFileSync(helper, 'utf8');
const pin = /const archiveDigest = '(?:PENDING|[a-f0-9]{64})';/u;
assert.equal(source.split(pin).length, 2);
const updated = source.replace(pin, `const archiveDigest = '${digest(readFileSync(output))}';`);
if (check) assert.equal(source, updated);
else if (source !== updated) writeFileSync(helper, updated);
