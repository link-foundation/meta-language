import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';

export const dependencyWorkflowHistoryPath = 'parity/self-translation/dependency-workflow-source-history.lino';
export const dependencyWorkflowPredecessors = Object.freeze({
  '.github/workflows/ci.yml': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  '.github/workflows/js.yml': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  '.github/workflows/rust.yml': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  '.github/workflows/dependency-refresh.yml': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/ci-workflow-structure.test.js': '269c48e497280025271ade3765f808b2f372d1b6',
  'js/tests/support/generated-package-owner-history.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/test-groups.test.js': '8bc0ef9725388c7927acf1854870bf46f5e365be',
  'rust/Cargo.toml': '753e163487ccbb59e22844d402b9938464064c41',
});
const root = new URL('../../', import.meta.url);
const archiveDigest = 'dfd1878093b7f885eeac0f467592e7a02d7813862c7f896bb1ab23ed971c76b0';
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
let rows;

/** Supply exact predecessor bytes to already registered historical assertions. */
export function restoreDependencyWorkflowSource(source, file) {
  const revision = dependencyWorkflowPredecessors[file];
  if (!revision) return source;
  if (!rows) {
    const archive = readFileSync(new URL(dependencyWorkflowHistoryPath, root));
    assert.equal(digest(archive), archiveDigest);
    const history = readBootstrapStructuredDataLino(archive.toString('utf8'));
    assert.equal(history.schemaVersion, 1);
    assert.deepEqual(history.sources.map(({ file }) => file), Object.keys(dependencyWorkflowPredecessors));
    rows = new Map(history.sources.map((row) => [row.file, row]));
  }
  const row = rows.get(file);
  const bytes = Buffer.from(source);
  if (digest(bytes) !== row.currentDigest) return source;
  assert.equal(bytes.length, row.currentBytes);
  const { offset, length, predecessor } = row.edit;
  assert.ok(Number.isSafeInteger(offset) && offset >= 0 && Number.isSafeInteger(length)
    && length >= 0 && offset + length <= bytes.length);
  const previous = Buffer.concat([bytes.subarray(0, offset), Buffer.from(predecessor), bytes.subarray(offset + length)]);
  return verifyGitArtifactCertificate(row.certificate, previous, { revision, path: file }).toString('utf8');
}
