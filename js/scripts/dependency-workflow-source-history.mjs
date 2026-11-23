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
  'experiments/grammar-context-source-metadata/generate.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/original-link-complete-table-projection/generate-raw-planning-source-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/original-link-complete-table-projection/raw-native-observations.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-original-number-observations.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-evidence-translation-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-scheduling-repair-archive.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/grammar-outcome-network/native-transport/runner/README.md': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/grammar-outcome-network/native-transport/runner/plan.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/grammar-outcome-network/native-transport/runner/run.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/grammar-outcome-network/native-transport/runner/runner.test.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/current-ci-repair-source-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/dependency-inventory.mjs': '81107a5945651f17c962e4bb81fb560f96c65ccd',
  'js/scripts/dependency-inventory-append-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/evidence-translation-source-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/generate-boolean-parameter-tuples.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/generate-dependency-inventory-append-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/generate-raw-oracle-contract-partitions.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/generate-self-translation-report.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/scripts/grammar-context-metadata-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/grammar-selection-capture-profile.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/own-codebase-baseline-naming-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/own-codebase-test-registration-history.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/scripts/scheduling-source-history.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/self-translation-report-worker-host.mjs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/scripts/test-groups.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/src/translation/types.js': '93c65ca1ada6c5001a56acc4430500e55062027a',
  'js/tests/cache-fresh-merge-dependencies.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/ci-workflow-structure.test.js': '269c48e497280025271ade3765f808b2f372d1b6',
  'js/tests/issue-195-evidence-ci-node-runtime.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-downstream-formal-ai-workloads.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-conformance-history.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-dependency-appends.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-node-dependency.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-node-test-placement.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-producer.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-raw-oracle-partitions.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-raw-planning-deadline.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-tooling-partitions.test.js': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/tests/issue-195-evidence-translation-feedback.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-translation-partition-conservation.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-naming.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-self-translation-report.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-sources.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/lino-compatibility-matrix.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/own-codebase-baseline-naming-history.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/package-release.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/project-data-fixture-links-storage.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/support/generated-package-owner-history.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/support/platform-test-deadline-history.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/support/windows-linker-lookup-history.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/test-groups.test.js': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/tests/translation-boolean-parameter-tuples.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/translation-frontend-rule-styles.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/translation-grammar-context-metadata.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/translation-raw-source-archive-workers.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/translation-scheduling-source-history.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/translation-typescript-workflow.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'parity/language-grammar-inventory.lino': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'parity/fixtures/builtin-cst-expected.lino': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'parity/fixtures/default-cst-expected.lino': 'bb6722e623fabc6a8be8ce6c0c5a4679602f1711',
  'parity/fixtures/lino-compatibility-matrix.lino': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'parity/fixtures/lino-grammar-cases.lino': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'parity/fixtures/native-default-cst-expected.lino': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'parity/self-translation/frontend-rules-decorators.lino': '8f2f6c483ec32a3ffc17afec694463965dc7551d',
  'parity/self-translation/owned-outcome-number-kind-rules-decorators.lino': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'rust/Cargo.toml': '753e163487ccbb59e22844d402b9938464064c41',
  'rust/Cargo.lock': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'rust/oracles/native-source-oracles/build.rs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'rust/src/lib.rs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'experiments/grammar-outcome-network/native-transport/diagnostic-order/crate-root-source-view.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'experiments/grammar-outcome-network/native-transport/diagnostic-order/generate-crate-root-source.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-number-kinds.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'experiments/grammar-outcome-network/native-transport/inverse/number-kind-rules.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'experiments/grammar-outcome-network/native-transport/inverse/number-profile.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'experiments/grammar-outcome-network/native-transport/runner/compiler-profile.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'experiments/grammar-outcome-network/native-transport/runner/compiler-profile.test.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/scripts/agda-oracle-profiles.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/scripts/raw-oracle-contract-partition-history.mjs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/tests/grammar-agda-queue-safe-oracle.test.js': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/tests/grammar-current-context-source-views.test.js': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/tests/translation-owned-number-kinds.test.js': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'js/tests/translation-owned-original-numbers.test.js': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'rust/src/translation/owned_outcome_number_kind_rules/part_1.rs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
  'rust/tests/unit/grammar_mapping_capture_source.rs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'rust/tests/unit/lino_compatibility_matrix.rs': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'rust/tests/unit/grammar_outcome_number_kind_rules.rs': '41055e5cbe206b5f04ef1f1f1658773bcaa50ce1',
});
export const dependencyWorkflowHistoricalPredecessors = Object.freeze({
  'js/tests/ci-workflow-structure.test.js': 'ec9f4efb0e9ef936114e09823981352f6db13700',
  'js/tests/issue-195-evidence-translation-feedback.test.js': '3f72318d4d63e26323f86b50dcf389d5a531e4c4',
  'js/tests/support/platform-test-deadline-history.js': '3f72318d4d63e26323f86b50dcf389d5a531e4c4',
  'js/tests/test-groups.test.js': '8bc0ef9725388c7927acf1854870bf46f5e365be',
  'parity/fixtures/native-default-cst-expected.lino': 'bb6722e623fabc6a8be8ce6c0c5a4679602f1711',
});
const root = new URL('../../', import.meta.url);
const archiveDigest = '190835bc2082a3062fb050d3224bf83ea4af8ac900d0bcab8404d4d4f5e6f38c';
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
let archive;
const verifiedPredecessors = new WeakMap();

/** Supply exact predecessor bytes to already registered historical assertions. */
export function restoreDependencyWorkflowSource(source, file, revision = dependencyWorkflowPredecessors[file]) {
  if (!revision || (revision !== dependencyWorkflowPredecessors[file]
    && revision !== dependencyWorkflowHistoricalPredecessors[file])) return source;
  if (!archive) {
    const bytes = readFileSync(new URL(dependencyWorkflowHistoryPath, root));
    assert.equal(digest(bytes), archiveDigest);
    const history = readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(history.schemaVersion, 2);
    const sources = [...Object.entries(dependencyWorkflowPredecessors), ...Object.entries(dependencyWorkflowHistoricalPredecessors)];
    assert.deepEqual(history.sources.map(({ file, revision }) => [file, revision]), sources);
    const commits = new Map(history.commits);
    const trees = new Map(history.trees.map((tree) => [tree.objectId, tree]));
    assert.equal(commits.size, history.commits.length);
    assert.equal(trees.size, history.trees.length);
    archive = { commits, trees, sources: new Map(history.sources.map((row) => [`${row.revision}:${row.file}`, row])) };
  }
  const row = archive.sources.get(`${revision}:${file}`);
  const bytes = Buffer.from(source);
  if (digest(bytes) !== row.currentDigest) return source;
  assert.equal(bytes.length, row.currentBytes);
  if (verifiedPredecessors.has(row)) return verifiedPredecessors.get(row);
  const { offset, length, predecessor } = row.edit;
  assert.ok(Number.isSafeInteger(offset) && offset >= 0 && Number.isSafeInteger(length)
    && length >= 0 && offset + length <= bytes.length);
  const previous = Buffer.concat([bytes.subarray(0, offset), Buffer.from(predecessor), bytes.subarray(offset + length)]);
  const certificate = {
    schemaVersion: 1,
    objectFormat: 'sha1',
    revision,
    path: file,
    commitText: archive.commits.get(revision),
    trees: row.certificate.ancestry.map((objectId) => archive.trees.get(objectId)),
    blob: row.certificate.blob,
  };
  const verified = verifyGitArtifactCertificate(certificate, previous, { revision, path: file }).toString('utf8');
  verifiedPredecessors.set(row, verified);
  return verified;
}
