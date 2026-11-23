import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';

import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

const archive = new URL('../../parity/self-translation/automated-translation-pause-source-history.lino', import.meta.url);
const archiveDigest = 'efd31af216b6551338a21e155fa26c1d92211685eec4c3de7b7eebc92daf9515';
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
let history;

export function restoreAutomatedTranslationPauseSource(source, file, revision) {
  const workflow = file.startsWith('.github/workflows/');
  if (!workflow && !['js/scripts/current-ci-repair-source-history.mjs', 'js/scripts/rust-test-shards.mjs',
    'js/scripts/grammar-test-partition-history.mjs', 'js/tests/cache-fresh-merge-dependencies.test.js',
    'js/scripts/own-codebase-baseline-naming-history.mjs', 'js/scripts/run-issue-195-consumer.mjs',
    'js/scripts/scheduling-source-history.mjs', 'js/scripts/tooling-frozen-source-history.mjs',
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
