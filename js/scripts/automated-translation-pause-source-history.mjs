import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';

import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

const archive = new URL('../../parity/self-translation/automated-translation-pause-source-history.lino', import.meta.url);
const archiveDigest = '7801a673c7a55343b9f0264cf2ddd2229ea0e23b685852c8c916220f32d69820';
const revision = 'ec9f4efb0e9ef936114e09823981352f6db13700';
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
let history;

export function restoreAutomatedTranslationPauseSource(source, file) {
  const workflow = file.startsWith('.github/workflows/');
  if (!workflow && !['js/scripts/current-ci-repair-source-history.mjs', 'js/scripts/rust-test-shards.mjs',
    'js/scripts/run-issue-195-consumer.mjs'].includes(file)) {
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
    (row.file === file || (workflow && row.file.startsWith('.github/workflows/'))));
  if (!entry) return source;
  const original = Buffer.from(entry.original, 'utf8');
  verifyGitArtifactCertificate({
    ...entry.certificate,
    commitText: history.commitText,
    trees: entry.certificate.trees.map((id) => history.trees[id]),
  }, original, { revision, path: entry.file });
  return entry.original;
}
