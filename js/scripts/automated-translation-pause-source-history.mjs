import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';

import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

export const pausePredecessor = 'c41404a2199e5057c23a39f3645c1ecc516f1d47';
export const pauseSourceFiles = Object.freeze([
  '.github/workflows/ci.yml',
  '.github/workflows/js.yml',
  '.github/workflows/rust.yml',
  'js/scripts/current-ci-repair-source-history.mjs',
  'js/scripts/dependency-inventory.mjs',
  'js/scripts/formal-ai-partition-bundles.mjs',
  'js/scripts/formal-ai-workload-partitions.mjs',
  'js/scripts/issue-195-formal-ai-workloads.mjs',
  'js/scripts/run-issue-195-consumer.mjs',
  'js/scripts/rust-test-shards.mjs',
  'js/scripts/test-groups.mjs',
  'js/tests/cache-fresh-merge-dependencies.test.js',
  'js/tests/grammar-embedded-region-rules.test.js',
  'js/tests/issue-195-evidence-formal-ai-partitions.test.js',
  'js/tests/issue-195-evidence-node-dependency.test.js',
  'js/tests/issue-195-evidence-rust-shards.test.js',
  'js/tests/issue-195-formal-ai-partitions.test.js',
  'js/tests/link-network-binary.test.js',
  'js/tests/test-groups.test.js',
  'rust/tests/unit/ci-cd/workflow_release.rs',
]);

const archive = new URL('../../parity/self-translation/automated-translation-pause-source-history.lino', import.meta.url);
const archiveDigest = '11a209758613a04a15cefab7fcc35f2806d6c32eab7c213db5d54b89146111b9';
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
let history;

/** Resolve historical controls only when their full input matches a certified adaptation. */
export function restoreAutomatedTranslationPauseSource(source, file) {
  if (!pauseSourceFiles.includes(file)) return source;
  if (!history) {
    const bytes = readFileSync(archive);
    assert.equal(digest(bytes), archiveDigest);
    history = readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(history.predecessor, pausePredecessor);
    assert.deepEqual(history.sources.map((row) => row.file), pauseSourceFiles);
  }
  // Several legacy workflow helpers use one workflow name for all three inputs.
  // Authenticate the exact hashed source under the actual path in its Git proof.
  const current = digest(source);
  const workflow = file.startsWith('.github/workflows/');
  const entry = history.sources.find((row) => row.current === current &&
    (row.file === file || (workflow && row.file.startsWith('.github/workflows/'))));
  if (!entry) return source;
  const original = Buffer.from(entry.original, 'utf8');
  verifyGitArtifactCertificate({
    ...entry.certificate,
    commitText: history.commit,
    trees: entry.certificate.trees.map((id) => history.trees[id]),
  }, original, { revision: pausePredecessor, path: entry.file });
  return entry.original;
}
