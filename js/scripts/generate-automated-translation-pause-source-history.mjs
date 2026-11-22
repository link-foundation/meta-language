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
  'js/scripts/current-ci-repair-source-history.mjs',
  'js/scripts/run-issue-195-consumer.mjs',
  'js/scripts/rust-test-shards.mjs',
];
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const trees = {};
let commitText;
const sources = files.map((file) => {
  const { blob, certificate } = captureGitArtifactCertificate(revision, file);
  const current = digest(readFileSync(new URL(file, root)));
  assert.notEqual(certificate.blob.sha256, current, file);
  const { commitText: commit, trees: proof, ...owner } = certificate;
  commitText ??= commit;
  assert.equal(commit, commitText);
  owner.trees = proof.map((tree) => {
    trees[tree.objectId] ??= tree;
    return tree.objectId;
  });
  return { file, current, original: blob.toString('utf8'), certificate: owner };
});
const history = { commitText, trees, sources };

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
