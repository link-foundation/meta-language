import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

import { captureGitArtifactCertificate } from './capture-git-artifact-certificate.mjs';
import { checkProjectData, writeProjectData } from './project-data.mjs';

const root = new URL('../../', import.meta.url);
const revision = 'ec9f4efb0e9ef936114e09823981352f6db13700';
const files = [
  '.github/workflows/ci.yml',
  '.github/workflows/js.yml',
  '.github/workflows/rust.yml',
  'js/scripts/contributor-instruction-coverage.mjs',
  'js/scripts/current-ci-repair-source-history.mjs',
  'js/scripts/formal-ai-partition-bundles.mjs',
  'js/scripts/formal-ai-workload-partitions.mjs',
  'js/scripts/generate-contributor-instruction-coverage.mjs',
  'js/scripts/generate-raw-oracle-contract-partitions.mjs',
  'js/scripts/grammar-test-partition-history.mjs',
  'js/scripts/issue-195-formal-ai-workloads.mjs',
  'js/scripts/own-codebase-baseline-naming-history.mjs',
  'js/scripts/own-codebase-test-registration-history.mjs',
  'js/scripts/raw-oracle-contract-partition-history.mjs',
  'js/scripts/run-issue-195-consumer.mjs',
  'js/scripts/rust-test-shards.mjs',
  'js/scripts/test-groups.mjs',
  'js/tests/cache-fresh-merge-dependencies.test.js',
  'js/tests/grammar-embedded-region-rules.test.js',
  'js/tests/issue-195-evidence-contributor-instructions.test.js',
  'js/tests/issue-195-evidence-formal-ai-partitions.test.js',
  'js/tests/issue-195-evidence-node-dependency.test.js',
  'js/tests/issue-195-evidence-node-test-placement.test.js',
  'js/tests/issue-195-evidence-translation-partition-conservation.test.js',
  'js/tests/issue-195-formal-ai-partitions.test.js',
  'js/tests/issue-195-naming.test.js',
  'js/tests/link-network-binary.test.js',
  'js/tests/own-codebase-baseline-naming-history.test.js',
  'js/tests/test-groups.test.js',
].map((file) => ({ file, revision }));
files.push({ file: 'js/tests/test-groups.test.js', revision: '231c6610fc93225fccb1682ab2943729e0c0e52e' });
files.push({ file: 'js/scripts/scheduling-source-history.mjs', revision: '15745ffcd322c48bdd537ccb48a5f7c9deb96917' });
files.push({ file: 'js/scripts/tooling-frozen-source-history.mjs', revision: '269c48e497280025271ade3765f808b2f372d1b6' });
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const trees = {};
const commits = {};
const sources = files.map(({ file, revision: predecessor }) => {
  const { blob, certificate } = captureGitArtifactCertificate(predecessor, file);
  const current = digest(readFileSync(new URL(file, root)));
  assert.notEqual(certificate.blob.sha256, current, file);
  const { commitText: commit, trees: proof, ...owner } = certificate;
  commits[predecessor] ??= commit;
  assert.equal(commit, commits[predecessor]);
  owner.trees = proof.map((tree) => {
    trees[tree.objectId] ??= tree;
    return tree.objectId;
  });
  return { file, current, predecessor, original: blob.toString('utf8'), certificate: owner };
});
const history = { commits, trees, sources };

const archive = new URL('parity/self-translation/automated-translation-pause-source-history.lino', root);
const verifier = new URL('automated-translation-pause-source-history.mjs', import.meta.url);
const check = process.argv.includes('--check');
assert.ok(process.argv.slice(2).every((argument) => argument === '--check'));
if (check) checkProjectData(archive, history);
else writeProjectData(archive, history);

const original = readFileSync(verifier, 'utf8');
const updated = original.replace(/const archiveDigest = '(?:PENDING|[0-9a-f]{64})';/u,
  "const archiveDigest = '" + digest(readFileSync(archive)) + "';");
if (check) assert.equal(original, updated);
else writeFileSync(verifier, updated);
