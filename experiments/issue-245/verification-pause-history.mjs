// Historical checks retain their complete Git-authenticated workflow inputs.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from '../../js/scripts/git-object-certificate.mjs';

const root = new URL('../../', import.meta.url);
const archive = new URL('./verification-pause-history.lino', import.meta.url);
const revision = 'eb11fcf94d1a4332ba336e4b2281f084a1a4e3a9';
const archiveDigest = '3fa6491ba1f035b793ae8bf611c542e2c6c999a1a915e9d781aeec8099eca491';
const paths = [
  '.github/workflows/ci.yml',
  '.github/workflows/js.yml',
  '.github/workflows/rust.yml',
  'js/scripts/dependency-inventory-append-history.mjs',
  'js/scripts/dependency-inventory.mjs',
  'js/scripts/issue-195-formal-ai-workloads.mjs',
  'js/tests/issue-195-evidence-node-dependency.test.js',
  'js/tests/test-groups.test.js',
  'rust/tests/unit/ci-cd/workflow_release.rs',
];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
let history;

export function restoreVerificationPauseSource(file, source) {
  if (file !== undefined && !paths.includes(file)) return source;
  if (!history) {
    const bytes = readFileSync(archive);
    assert.equal(digest(bytes), archiveDigest);
    history = readBootstrapStructuredDataLino(bytes.toString('utf8'));
  }
  const current = digest(source);
  const entry = history.sources.find(row => current === row.current &&
    (row.file === file || (file === undefined && row.file.startsWith('.github/workflows/'))));
  if (!entry) return source;
  const original = Buffer.from(entry.original, 'utf8');
  verifyGitArtifactCertificate({
    ...entry.certificate,
    commitText: history.commit,
    trees: entry.certificate.trees.map(id => history.trees[id]),
  }, original, { revision, path: entry.file, limits: { maxBlobBytes: 262144 } });
  return entry.original;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const check = process.argv[2] === '--check';
  assert.deepEqual(process.argv.slice(2), [check ? '--check' : '--write']);
  const { captureGitArtifactCertificate } = await import('../../js/scripts/capture-git-artifact-certificate.mjs');
  const { checkProjectData, writeProjectData } = await import('../../js/scripts/project-data.mjs');
  const { writeFileSync } = await import('node:fs');
  const trees = {};
  let commit;
  const sources = paths.map(file => {
    const { blob, certificate } = captureGitArtifactCertificate(revision, file);
    const current = digest(readFileSync(new URL(file, root)));
    assert.notEqual(current, certificate.blob.sha256, file);
    const { commitText, trees: proof, ...owner } = certificate;
    commit ??= commitText;
    assert.equal(commitText, commit);
    owner.trees = proof.map(tree => {
      trees[tree.objectId] ??= tree;
      return tree.objectId;
    });
    return { file, current, original: blob.toString('utf8'), certificate: owner };
  });
  const value = { commit, trees, sources };
  if (check) checkProjectData(archive, value);
  else writeProjectData(archive, value);
  const updated = digest(readFileSync(archive));
  if (check) assert.equal(archiveDigest, updated);
  else writeFileSync(new URL(import.meta.url), readFileSync(new URL(import.meta.url), 'utf8')
    .replace(/const archiveDigest = '[^']+';/u, `const archiveDigest = '${updated}';`));
}
