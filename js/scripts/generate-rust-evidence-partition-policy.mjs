import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { readProjectData, writeProjectData } from './project-data.mjs';

export function rustEvidencePartitionPolicy() {
  return {
    shards: 8,
    unit: 'top-level native test module',
    // Keep the original per-process observation journal and duplicate gate.
    // These source modules mention a common requirement and stay together.
    sharedScopes: [
      ['issue_195_grammar_feature_union', 'issue_195_grammar_feature_union_interchange'],
      ['issue_195_grammar_shared_concepts', 'issue_195_merge_quality_evidence'],
      ['export_parity_corpus', 'javascript_export_parity'],
    ],
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/rust-test-physical-shards.lino', import.meta.url);
  const policy = readProjectData(file);
  const evidence = rustEvidencePartitionPolicy();
  if (process.argv.includes('--check')) {
    if (!isDeepStrictEqual(policy.evidence, evidence)) throw new Error('Rust evidence partition scheduling is stale');
  } else {
    if (policy.evidence !== undefined && !isDeepStrictEqual(policy.evidence, evidence)) {
      throw new Error('the existing Rust evidence partition policy requires an explicit authoring update');
    }
    writeProjectData(file, { ...policy, evidence });
  }
  console.log('Rust evidence partitions: existing test policies retained with bounded native observation scopes');
}
