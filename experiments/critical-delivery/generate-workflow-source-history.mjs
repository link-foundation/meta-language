import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { captureGitArtifactCertificate } from '../../js/scripts/capture-git-artifact-certificate.mjs';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { workflowSourceHistoryFile, workflowSourceIdentity, workflowSourcePredecessors } from '../../js/scripts/workflow-source-history.mjs';

const root = new URL('../../', import.meta.url);
const sources = Object.entries(workflowSourcePredecessors).map(([file, revision]) => {
  const { blob: original, certificate } = captureGitArtifactCertificate(revision, file);
  const current = readFileSync(new URL(file, root));
  const before = original.toString('utf8').split('\n');
  const after = current.toString('utf8').split('\n');
  let head = 0;
  while (head < before.length && head < after.length && before[head] === after[head]) head++;
  let tail = 0;
  while (tail < before.length - head && tail < after.length - head
    && before.at(-tail - 1) === after.at(-tail - 1)) tail++;
  const start = head === 0 ? 0 : Buffer.byteLength(after.slice(0, head).join('\n')) + 1;
  const end = tail === 0 ? current.length : current.length - Buffer.byteLength(after.slice(-tail).join('\n'));
  const previousEnd = tail === 0 ? original.length : original.length - Buffer.byteLength(before.slice(-tail).join('\n'));
  const previousLines = original.subarray(start, previousEnd).toString('utf8');
  assert.deepEqual(Buffer.concat([current.subarray(0, start), Buffer.from(previousLines), current.subarray(end)]), original);
  return { file, successor: workflowSourceIdentity(current), predecessor: workflowSourceIdentity(original),
    edit: { start, end, previousLines }, certificate };
});
const output = new URL(workflowSourceHistoryFile, root);
writeProjectData(output, { schemaVersion: 1, sources });
const helper = new URL('js/scripts/workflow-source-history.mjs', root);
const input = readFileSync(helper, 'utf8');
const pattern = /^const historyDigest = '(?:PENDING|[0-9a-f]{64})';$/mu;
assert.equal(input.split(pattern).length, 2);
const { sha256 } = workflowSourceIdentity(readFileSync(output));
writeFileSync(helper, input.replace(pattern, `const historyDigest = '${sha256}';`));
