import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';

import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

export const ciIntegrationHistoryPath = 'parity/self-translation/ci-integration-source-history.lino';
export const ciIntegrationParents = Object.freeze({
  dependencies: 'a74478502d3d068c11b474fd7f0917f2103d80d9',
  translationPause: '7c0a19857db0a1646f0a96fb5ea5e718d8ad6865',
});
// These existing inventories identify complete source definitions changed by the merge.
export const ciIntegrationNamingSources = Object.freeze({
  'js/scripts/formal-ai-partition-bundles.mjs': 'B70 physical field source view names',
  'js/scripts/formal-ai-workload-partitions.mjs': 'B70 physical field source view names',
  'js/scripts/generate-ascii-text-predicate-host-history.mjs': 'B70 physical field source view names',
  'js/scripts/generate-ascii-text-predicate-hosts.mjs': 'B70 physical field source view names',
  'js/scripts/generate-boolean-value-source-history.mjs': 'B70 physical field source view names',
  'js/scripts/generate-javascript-mutable-array-bindings.mjs': 'B70 physical field source view names',
  'js/scripts/generate-modern-javascript-array-parity-fixtures.mjs': 'B70 physical field source view names',
  'js/scripts/generate-program-string-host-registration-history.mjs': 'B70 physical field source view names',
  'rust/tests/unit/grammar_included_range_archive.rs': 'B69 included range archive names',
  'rust/tests/unit/grammar_recursive_selection.rs': 'B63 actual source names',
  'rust/tests/unit/self_translation_scope.rs': 'B70 physical field source view names',
  'rust/tests/unit/shared_numeric_function_links.rs': 'B70 physical field source view names',
  'rust/tests/unit/translation_arrays.rs': 'B70 physical field source view names',
  'rust/tests/unit/translation_ascii_text_predicates.rs': 'B70 physical field source view names',
  'rust/tests/unit/translation_javascript_array_bindings.rs': 'B70 physical field source view names',
  'rust/tests/unit/translation_owned_array_append.rs': 'B70 physical field source view names',
  'rust/tests/unit/typescript_erasure.rs': 'B70 physical field source view names',
});
const root = new URL('../../', import.meta.url);
const archiveDigest = '9db5d3335dad184c2f62f5ceadf2a18b1208c4ee1fd1203c09dfeafc5458b97d';
const digest = source => createHash('sha256').update(source).digest('hex');
let archive;

/** Authenticate a complete input before exposing either review's source view. */
export function restoreCiIntegrationSource(source, file, parent) {
  if (!archive) {
    const bytes = readFileSync(new URL(ciIntegrationHistoryPath, root));
    assert.equal(digest(bytes), archiveDigest);
    const history = readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.deepEqual(history.parents, ciIntegrationParents);
    archive = {
      ...history,
      sources: new Map(history.sources.map(row => [`${row.parent}:${row.current}`, row])),
      verified: new Map(),
    };
    assert.equal(archive.sources.size, history.sources.length);
  }
  const key = `${parent}:${digest(source)}`;
  const row = archive.sources.get(key);
  if (!row || (row.file !== file && !(file.startsWith('.github/workflows/') && row.file.startsWith('.github/workflows/')))) {
    return source;
  }
  if (!archive.verified.has(key)) {
    assert.equal(row.bytes, Buffer.byteLength(source));
    const revision = ciIntegrationParents[parent];
    const certificate = {
      ...row.certificate,
      commitText: archive.commits[revision],
      trees: row.certificate.trees.map(id => archive.trees[id]),
    };
    const original = verifyGitArtifactCertificate(certificate, Buffer.from(row.original), { revision, path: row.file });
    archive.verified.set(key, original.toString('utf8'));
  }
  return archive.verified.get(key);
}
