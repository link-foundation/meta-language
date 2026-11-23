import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';

import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

const archive = new URL('../../parity/self-translation/automated-translation-pause-source-history.lino', import.meta.url);
const archiveDigest = 'd4ab0a789ae5da4b36cf098aee7e9ef7bd354607a5397dc11b8a24bdf3c705e9';
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
let history;

export function restoreAutomatedTranslationPauseSource(source, file, revision) {
  const workflow = file.startsWith('.github/workflows/');
  if (!workflow && !['js/scripts/current-ci-repair-source-history.mjs', 'js/scripts/rust-test-shards.mjs',
    'js/scripts/grammar-test-partition-history.mjs', 'js/tests/cache-fresh-merge-dependencies.test.js',
    'js/scripts/own-codebase-baseline-naming-history.mjs', 'js/scripts/own-codebase-test-registration-history.mjs',
    'js/scripts/run-issue-195-consumer.mjs', 'js/scripts/test-groups.mjs',
    'js/scripts/scheduling-source-history.mjs', 'js/scripts/tooling-frozen-source-history.mjs',
    'js/scripts/formal-ai-partition-bundles.mjs', 'js/scripts/formal-ai-workload-partitions.mjs',
    'js/scripts/contributor-instruction-coverage.mjs', 'js/scripts/generate-contributor-instruction-coverage.mjs',
    'js/scripts/generate-raw-oracle-contract-partitions.mjs', 'js/scripts/raw-oracle-contract-partition-history.mjs',
    'js/scripts/issue-195-formal-ai-workloads.mjs', 'js/tests/grammar-embedded-region-rules.test.js',
    'js/tests/issue-195-evidence-contributor-instructions.test.js', 'js/tests/issue-195-evidence-node-dependency.test.js',
    'js/tests/issue-195-evidence-translation-partition-conservation.test.js',
    'js/tests/issue-195-formal-ai-partitions.test.js', 'js/tests/link-network-binary.test.js',
    'js/tests/test-groups.test.js'].includes(file)) {
    return source;
  }
  if (!history) {
    const bytes = readFileSync(archive);
    assert.equal(digest(bytes), archiveDigest);
    history = readBootstrapStructuredDataLino(bytes.toString('utf8'));
  }
  // Some older workflow readers identify the owner by digest alone. A match
  // still has to authenticate the predecessor under its actual Git path.
  const current = digest(source);
  const entry = history.sources.find((row) => row.current === current &&
    (revision === undefined || row.predecessor === revision) &&
    (row.file === file || (workflow && row.file.startsWith('.github/workflows/'))));
  if (!entry) return source;
  const original = Buffer.from(entry.original, 'utf8');
  verifyGitArtifactCertificate({
    ...entry.certificate,
    commitText: history.commits[entry.predecessor],
    trees: entry.certificate.trees.map((id) => history.trees[id]),
  }, original, { revision: entry.predecessor, path: entry.file });
  return entry.original;
}
