import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';

import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

// Native CI repairs changed full-source naming witnesses on these paths.
export const nativeRepairNamingSources = Object.freeze([
  'experiments/binary-metadata-buffer/native-observations.mjs',
  'js/scripts/ascii-text-predicate-host-history.mjs',
  'js/scripts/declaration-dependency-native-interface.mjs',
  'js/scripts/generate-ascii-text-predicate-host-history.mjs',
  'js/scripts/generate-ascii-text-predicate-hosts.mjs',
  'js/scripts/generate-boolean-declaration-value-hosts.mjs',
  'js/scripts/generate-boolean-value-source-history.mjs',
  'js/scripts/generate-javascript-mutable-array-bindings.mjs',
  'js/scripts/generate-scoped-iteration-native-host-source-history.mjs',
  'js/scripts/record-rule-library.mjs',
  'js/scripts/scoped-iteration-native-host-source-history.mjs',
  'js/tests/translation-ascii-text-predicate-hosts.test.js',
  'js/tests/translation-required-field-native-host-succession.test.js',
  'js/tests/translation-scoped-iteration-native-source-history.test.js',
  'rust/src/formal_concept_definition.rs',
  'rust/src/self_translation/binding/deferred_scope.rs',
  'rust/src/self_translation/binding/dependency_rules.rs',
  'rust/src/self_translation/binding/owned_recursive_emission.rs',
  'rust/src/self_translation/binding/recursive_groups.rs',
  'rust/src/self_translation/scope.rs',
  'rust/src/shared_numeric_function_model.rs',
  'rust/src/translation/ascii_text_predicate_rules/part_2.rs',
  'rust/src/translation/declaration_dependency_rules/part_2.rs',
  'rust/src/translation/declaration_dependency_rules/part_3.rs',
  'rust/src/translation/declaration_dependency_rules/part_4.rs',
  'rust/src/translation/declaration_dependency_rules/part_5.rs',
  'rust/src/translation/emit_javascript/expressions.rs',
  'rust/src/translation/emit_rust/declarations.rs',
  'rust/src/translation/emit_rust/expressions.rs',
  'rust/src/translation/external_emission_rules/part_1.rs',
  'rust/src/translation/grammar_lexer_rules/part_4.rs',
  'rust/src/translation/javascript/declarations.rs',
  'rust/src/translation/javascript/infer/arrays.rs',
  'rust/src/translation/javascript_record_parameter_rules/part_3.rs',
  'rust/src/translation/reference_identity_rules/part_1.rs',
  'rust/src/translation/rust/expressions.rs',
  'rust/src/translation/rust_literal_match_rules/part_1.rs',
  'rust/src/typed_program_links/checked.rs',
  'rust/tests/unit/grammar_included_range_archive.rs',
  'rust/tests/unit/grammar_recursive_selection.rs',
  'rust/tests/unit/self_translation_recursive_declaration_groups.rs',
  'rust/tests/unit/self_translation_scope.rs',
  'rust/tests/unit/translation_arrays.rs',
  'rust/tests/unit/translation_javascript_record_parameters.rs',
  'rust/tests/unit/translation_owned_array_append.rs',
  'rust/tests/unit/translation_surface_reference_analysis.rs',
  'rust/tests/unit/typescript_erasure.rs',
  'js/tests/support/platform-test-deadline-history.js',
  'rust/tests/unit/ci-cd/workflow_release.rs',
  'js/scripts/dependency-inventory-append-history.mjs',
  'js/scripts/generate-dependency-inventory-order-history.mjs',
  'js/tests/issue-195-evidence-dependency-appends.test.js',
]);

// Historical host generators must preserve the exact files captured before lint-only edits.
export const nativeLintCaptureSources = Object.freeze([
  'rust/tests/unit/shared_numeric_function_links.rs',
  'rust/tests/unit/translation_ascii_text_predicates.rs',
  'rust/tests/unit/translation_javascript_array_bindings.rs',
]);

const archive = new URL('../../parity/self-translation/automated-translation-pause-source-history.lino', import.meta.url);
const archiveDigest = '22e5224c02862fae02cbaf04e284ae721648976b4dc0c9ae09cdb40f16656378';
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
let history;

export function restoreAutomatedTranslationPauseSource(source, file, revision) {
  const workflow = file.startsWith('.github/workflows/');
  if (!workflow && !nativeRepairNamingSources.includes(file) && !nativeLintCaptureSources.includes(file) &&
    !['js/scripts/current-ci-repair-source-history.mjs', 'js/scripts/rust-test-shards.mjs',
    'js/scripts/grammar-test-partition-history.mjs', 'js/scripts/generate-link-network-binary-rules.mjs', 'js/tests/cache-fresh-merge-dependencies.test.js',
    'js/scripts/own-codebase-baseline-naming-history.mjs', 'js/scripts/own-codebase-test-registration-history.mjs',
    'js/scripts/run-issue-195-consumer.mjs', 'js/scripts/test-groups.mjs',
    'js/scripts/scheduling-source-history.mjs', 'js/scripts/tooling-frozen-source-history.mjs',
    'js/scripts/formal-ai-partition-bundles.mjs', 'js/scripts/formal-ai-workload-partitions.mjs',
    'js/scripts/contributor-instruction-coverage.mjs', 'js/scripts/generate-contributor-instruction-coverage.mjs',
    'js/scripts/generate-raw-oracle-contract-partitions.mjs', 'js/scripts/raw-oracle-contract-partition-history.mjs',
    'js/scripts/issue-195-formal-ai-workloads.mjs', 'js/tests/grammar-embedded-region-rules.test.js',
    'js/tests/issue-195-evidence-contributor-instructions.test.js', 'js/tests/issue-195-evidence-formal-ai-partitions.test.js',
    'js/tests/issue-195-evidence-node-dependency.test.js', 'js/tests/issue-195-evidence-node-test-placement.test.js',
    'js/tests/issue-195-evidence-translation-partition-conservation.test.js',
    'js/tests/issue-195-formal-ai-partitions.test.js', 'js/tests/issue-195-naming.test.js', 'js/tests/link-network-binary.test.js',
    'js/tests/own-codebase-baseline-naming-history.test.js',
    'js/tests/test-groups.test.js', 'rust/src/formal_concept_definition.rs', 'rust/src/shared_numeric_function_model.rs',
    'parity/fixtures/grammar-selection-diagnostic-order.lino', 'rust/src/translation/javascript/declarations.rs',
    'rust/tests/unit/translation_surface_reference_analysis.rs'].includes(file)) {
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
