import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

export const workflowSourceHistoryFile = 'parity/fixtures/issue-227-workflow-source-history.lino';
export const workflowSourcePredecessors = Object.freeze({
  '.github/workflows/ci.yml': '865749f74f324a332e9201f85386517f8186f3c0',
  '.github/workflows/js.yml': 'd53fd67bb28cc4ae25b311ac58e83f1490775dc9',
  '.github/workflows/rust.yml': '478fb5873ddfd6ab7e4c4b6849654e2c2cef3fa8',
});
const historyDigest = '648aea8aab902ab5b5cea053aa1f63d4bfc868658e1e774dcc5f9d62515d0eb2';
let history;

export function workflowSourceIdentity(bytes) {
  return { bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex') };
}

/** Restore the historical test input for a specific, fully identified successor. */
export function restoreWorkflowSourceHistory(file, source) {
  if (!Object.hasOwn(workflowSourcePredecessors, file)) return source;
  if (!history) {
    const bytes = readFileSync(new URL('../../' + workflowSourceHistoryFile, import.meta.url));
    assert.equal(workflowSourceIdentity(bytes).sha256, historyDigest);
    const packet = readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(packet.schemaVersion, 1);
    assert.deepEqual(packet.sources.map(row => row.file), Object.keys(workflowSourcePredecessors));
    history = new Map(packet.sources.map(row => [row.file, row]));
  }
  const { successor, predecessor, certificate, edit } = history.get(file);
  const bytes = Buffer.from(source);
  const identity = workflowSourceIdentity(bytes);
  if (identity.sha256 !== successor.sha256) return source;
  assert.deepEqual(identity, successor);
  const { start, end, previousLines } = edit;
  assert.ok(Number.isSafeInteger(start) && Number.isSafeInteger(end));
  assert.ok(0 <= start && start <= end && end <= bytes.length);
  const restored = Buffer.concat([bytes.subarray(0, start), Buffer.from(previousLines), bytes.subarray(end)]);
  assert.deepEqual(workflowSourceIdentity(restored), predecessor);
  verifyGitArtifactCertificate(certificate, restored, { revision: workflowSourcePredecessors[file], path: file });
  return restored.toString('utf8');
}
