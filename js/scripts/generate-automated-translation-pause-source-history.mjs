import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

import { pausePredecessor, pauseSourceFiles } from './automated-translation-pause-source-history.mjs';
import { captureGitArtifactCertificate } from './capture-git-artifact-certificate.mjs';
import { checkProjectData, writeProjectData } from './project-data.mjs';

const root = new URL('../../', import.meta.url);
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const trees = {};
let commit;
const sources = pauseSourceFiles.map((file) => {
  const { blob, certificate } = captureGitArtifactCertificate(pausePredecessor, file);
  const current = digest(readFileSync(new URL(file, root)));
  assert.notEqual(certificate.blob.sha256, current, file);
  const { commitText, trees: proof, ...owner } = certificate;
  commit ??= commitText;
  assert.equal(commitText, commit);
  owner.trees = proof.map((tree) => {
    trees[tree.objectId] ??= tree;
    return tree.objectId;
  });
  return { file, current, original: blob.toString('utf8'), certificate: owner };
});
const history = { predecessor: pausePredecessor, commit, trees, sources };

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
