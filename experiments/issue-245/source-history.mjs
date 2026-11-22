import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { verifyGitArtifactCertificate } from '../../js/scripts/git-object-certificate.mjs';

const root = new URL('../../', import.meta.url);
const archive = new URL('./source-history.lino', import.meta.url);
const mainRevision = 'ec9f4efb0e9ef936114e09823981352f6db13700';
export const javascriptRuntimeSource = 'js/src/grammar-runtime.js';
export const platformDeadlineTestSource = 'js/tests/issue-195-evidence-platform-test-deadline.test.js';
export const runtimeSourceFiles = Object.freeze([
  'rust/src/grammar/feature_runtime/mod.rs',
  'rust/src/grammar/feature_runtime/operations.rs',
  javascriptRuntimeSource,
]);
const profiles = new Map([
  [runtimeSourceFiles[0], { revision: mainRevision,
    predecessor: '7b0c3936262249a43ce687a315737a59af51fed7c79ed616faddd35cd1aab221',
    current: '341e3a90ddd61d23d10966c68756c8756ae49edc1d1e5ec526fca8d1d38e9d66' }],
  [runtimeSourceFiles[1], { revision: mainRevision,
    predecessor: '638f88625853a5ead2bba6b6bba95f18acdfe5ad00d3b8b8e6fe8d605d434f0a',
    current: '9e8b3180caf5bc4d4cd6c7562af41682eb0eec321f3f5c2affebda39528f02c8' }],
  [javascriptRuntimeSource, { revision: mainRevision,
    predecessor: '680498a6118102d91f039d911a50197dad41778b9ee543be76a65a36e45223c6',
    current: 'adbd70d41959110356341de44370a94a9b866ab5a0de906ea41fac1b13c5c2c5' }],
  ['js/tests/issue-195-evidence-resources.test.js', { revision: '4edb0a624fe7068bc125bd72ceca3b3fe2d13201',
    predecessor: 'bd0048bcc1850c7e6f68db3ab01fb2468a0be639e513999f031976409cd024ac',
    current: '5cd085d696ba24466ad284cb7a22e04628d74f39be473d188410b13af96e48d0' }],
  [platformDeadlineTestSource, { revision: mainRevision,
    predecessor: '13b21ff57171819d06efed9a5484a00ad93bd827e018fdb10493ad7264cb0d49',
    current: '0110f454906fcc6c8bcf183269f6085011f6549013248850af1315562ef43e2d' }],
]);
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const text = bytes => {
  const source = bytes.toString('utf8');
  assert.ok(Buffer.from(source).equals(bytes));
  return source;
};
let pairs;

function verifySourceHistory(history) {
  assert.equal(history.schemaVersion, 1);
  assert.deepEqual(history.sources.map(row => row.file), [...profiles.keys()]);
  return new Map(history.sources.map(row => {
    const { file, predecessor, certificate, edit } = row;
    const { revision, predecessor: originalHash, current: currentHash } = profiles.get(file);
    const before = Buffer.from(predecessor);
    assert.equal(text(before), predecessor);
    assert.equal(digest(before), originalHash);
    verifyGitArtifactCertificate(certificate, before, { revision, path: file, limits: { maxBlobBytes: 196608 } });
    const { start, end, inserted } = edit;
    assert.ok(Number.isSafeInteger(start) && Number.isSafeInteger(end) && 0 <= start && start <= end && end <= before.length);
    const replacement = Buffer.from(inserted);
    assert.equal(text(replacement), inserted);
    const after = Buffer.concat([before.subarray(0, start), replacement, before.subarray(end)]);
    assert.ok(after.length <= 196608);
    assert.equal(digest(after), currentHash);
    const inverse = Buffer.concat([after.subarray(0, start), before.subarray(start, end), after.subarray(start + replacement.length)]);
    assert.ok(inverse.equals(before));
    return [file, Object.freeze({ predecessor, current: text(after) })];
  }));
}

export function readIssue245SourcePair(file) {
  assert.ok(profiles.has(file), `no reviewed successor for ${file}`);
  pairs ??= verifySourceHistory(readProjectData(archive));
  return pairs.get(file);
}

export function restoreIssue245Source(file, source) {
  const { predecessor, current } = readIssue245SourcePair(file);
  assert.ok(source === predecessor || source === current, `unreviewed complete source for ${file}`);
  return predecessor;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.deepEqual(process.argv.slice(2), ['--write']);
  const { captureGitArtifactCertificate } = await import('../../js/scripts/capture-git-artifact-certificate.mjs');
  const sources = [...profiles].map(([file, { revision }]) => {
    const { blob, certificate } = captureGitArtifactCertificate(revision, file);
    const current = readFileSync(new URL(file, root));
    let start = 0;
    let end = blob.length;
    let currentEnd = current.length;
    while (start < end && start < currentEnd && blob[start] === current[start]) start += 1;
    while ((blob[start] & 192) === 128 || (current[start] & 192) === 128) start -= 1;
    while (end > start && currentEnd > start && blob[end - 1] === current[currentEnd - 1]) {
      end -= 1;
      currentEnd -= 1;
    }
    while ((blob[end] & 192) === 128 || (current[currentEnd] & 192) === 128) {
      end += 1;
      currentEnd += 1;
    }
    return { file, predecessor: text(blob), certificate, edit: { start, end, inserted: text(current.subarray(start, currentEnd)) } };
  });
  const history = { schemaVersion: 1, sources };
  verifySourceHistory(history);
  writeProjectData(archive, history);
}
