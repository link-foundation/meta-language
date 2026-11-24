import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { verifyGitArtifactCertificate } from '../../js/scripts/git-object-certificate.mjs';

const root = new URL('../../', import.meta.url);
const archive = new URL('./source-history.lino', import.meta.url);
const mainRevision = 'ec9f4efb0e9ef936114e09823981352f6db13700';
const grammarSearchRevision = '4edb0a624fe7068bc125bd72ceca3b3fe2d13201';
export const javascriptRuntimeSource = 'js/src/grammar-runtime.js';
export const platformDeadlineTestSource = 'js/tests/issue-195-evidence-platform-test-deadline.test.js';
export const larkEmitterSource = 'rust/src/grammar/emit/lark.rs';
export const javascriptTestGroupsSource = 'js/scripts/test-groups.mjs';
export const javascriptWorkflowSource = '.github/workflows/js.yml';
export const nodeDependencyTestSource = 'js/tests/issue-195-evidence-node-dependency.test.js';
export const typedProgramLinksValidationSource = 'js/src/typed-program-links-validation.js';
export const asciiPredicateAssertionSource = 'js/tests/translation-ascii-text-predicates.test.js';
export const nativeNumberObservationSource = 'parity/fixtures/original-native-number-observations.lino';
export const booleanProjectionCheckerSource = 'js/src/translation/check.js';
export const runtimeSourceFiles = Object.freeze([
  'rust/src/grammar/feature_runtime/mod.rs',
  'rust/src/grammar/feature_runtime/operations.rs',
  javascriptRuntimeSource,
]);
const schedulingRevision = '0deae514483822ffd2a42a4fa0f4c88744d0745d';
const precedingCheckRevision = '591e98639e99c569f063214c3ae8a756185f4add';
const translationTimingRevision = 'b9c1be99f8b5bbac85cf29014c5d14ff6d7bea2d';
const grammarTimingRevision = '4ecca483361d5f47ad38698fe0d24653f6ef641f';
const toolingTimingRevision = '8561a770850ea0b9cfa218c23ccc6958dae2a16a';
const profiles = [
  [runtimeSourceFiles[0], { revision: mainRevision,
    predecessor: '7b0c3936262249a43ce687a315737a59af51fed7c79ed616faddd35cd1aab221',
    current: '341e3a90ddd61d23d10966c68756c8756ae49edc1d1e5ec526fca8d1d38e9d66' }],
  [runtimeSourceFiles[1], { revision: mainRevision,
    predecessor: '638f88625853a5ead2bba6b6bba95f18acdfe5ad00d3b8b8e6fe8d605d434f0a',
    current: '9e8b3180caf5bc4d4cd6c7562af41682eb0eec321f3f5c2affebda39528f02c8' }],
  [javascriptRuntimeSource, { revision: mainRevision,
    predecessor: '680498a6118102d91f039d911a50197dad41778b9ee543be76a65a36e45223c6',
    current: 'adbd70d41959110356341de44370a94a9b866ab5a0de906ea41fac1b13c5c2c5',
    successorRevision: grammarSearchRevision }],
  ['js/tests/issue-195-evidence-resources.test.js', { revision: '4edb0a624fe7068bc125bd72ceca3b3fe2d13201',
    predecessor: 'bd0048bcc1850c7e6f68db3ab01fb2468a0be639e513999f031976409cd024ac',
    current: '5cd085d696ba24466ad284cb7a22e04628d74f39be473d188410b13af96e48d0' }],
  [platformDeadlineTestSource, { revision: mainRevision,
    predecessor: '13b21ff57171819d06efed9a5484a00ad93bd827e018fdb10493ad7264cb0d49',
    current: '0110f454906fcc6c8bcf183269f6085011f6549013248850af1315562ef43e2d' }],
  [larkEmitterSource, { revision: mainRevision,
    predecessor: 'ecb991ffa3586243e69f6e94aeae3913ec6a6e93698d1959e224bce5d397273b',
    current: 'd7759822bcf37a897ddb1da22dd5c2250753a7346a7c87b156e0227922a3a207' }],
  [javascriptTestGroupsSource, { revision: '5b2a0f71350c96520c1cda8efe004e644aa53158',
    predecessor: '4dce7b79635607f76fe6b0216775bac1293df47ae5f27580be2015a1d7897678',
    current: '027b53e855b0992612c91e32f2d3f75acba994a4d8994f0cda1acb556890b3d9',
    successorRevision: schedulingRevision }],
  [nodeDependencyTestSource, { revision: mainRevision,
    predecessor: '0f2f9cfdd17ad49c46b62f805738f7e43b35ee1d1e44a11db04102f75f1f731b',
    current: '0299e111c4080f248603a32ecaa53427fff053f47e50270dccb8cb7e52e0af5e' }],
  [typedProgramLinksValidationSource, { revision: mainRevision,
    predecessor: '6c2ea9bcb6266ef98af1b86708561dd8fd991e002f46c22dbfa047d90de39051',
    current: '0a3166246358793e5212c7f351f5e1d2254b27d699aceb15fc8ab71324e3e757' }],
  [asciiPredicateAssertionSource, { revision: mainRevision,
    predecessor: 'fbfe8f91bc813cd58ca68aab83a418c97824af6420f5683b0c72f2d98b012242',
    current: '25aa26956e6db42b260269a4b30ac1681325eb3d9496a77c18fd54cb0fd68ebc' }],
  [nativeNumberObservationSource, { revision: mainRevision,
    predecessor: '75d3ad2a41c3ca7ffb615b6f6c78e94c67191bb60b3f0c010dec4bb61b171150',
    current: 'b467cdfb67daa832a262e9ddb861adb270e027fe8076dad513bce95a421955e0' }],
  [javascriptTestGroupsSource, { revision: schedulingRevision,
    predecessor: '027b53e855b0992612c91e32f2d3f75acba994a4d8994f0cda1acb556890b3d9',
    current: '842f0097fc91ef7a52bf565e59fa640672a09c3ea72380ddf3b205dc09629bc0',
    successorRevision: translationTimingRevision }],
  [javascriptWorkflowSource, { revision: schedulingRevision,
    predecessor: '6274838f6f901b71b1f0540b40b741640a8c23abde25b7afde91368bee4186bf',
    current: 'f92347173c03d0ca6998379b4d5c13b2d264079455e8fe1f7746423889285bcf',
    successorRevision: precedingCheckRevision }],
  [javascriptWorkflowSource, { revision: precedingCheckRevision,
    predecessor: 'f92347173c03d0ca6998379b4d5c13b2d264079455e8fe1f7746423889285bcf',
    current: '8df6b6086e83ba7204e441008f191f9506911299eeff441e0a5492244b74f4f2' }],
  [booleanProjectionCheckerSource, { revision: '3fe3a9f9decfa6431b5b0718729a5d123b5f236b',
    predecessor: 'b07b8ffb95d6bf4274d8bf5a16c43b2673d1869a08b1173a3fc65bcf1c14daf2',
    current: 'c0b461b416fd1ab812cb1a8670b6911a3ee905015872e689959826e932f40709',
    successorRevision: 'da38bf7a9a3cadcd3d768cb6f87a1bbf9e02e257' }],
  [javascriptTestGroupsSource, { revision: translationTimingRevision,
    predecessor: '842f0097fc91ef7a52bf565e59fa640672a09c3ea72380ddf3b205dc09629bc0',
    current: '40805d77e1a4aa59749ab2df63640c9fe7062ee5246e6bdfe13e45a4a88680dd',
    successorRevision: grammarTimingRevision }],
  [javascriptRuntimeSource, { revision: grammarSearchRevision,
    predecessor: 'adbd70d41959110356341de44370a94a9b866ab5a0de906ea41fac1b13c5c2c5',
    current: '7db56777697ee79ad3ee7d303ce65434297a82decd77388e8a950cea00e9a4c5' }],
  [javascriptTestGroupsSource, { revision: grammarTimingRevision,
    predecessor: '40805d77e1a4aa59749ab2df63640c9fe7062ee5246e6bdfe13e45a4a88680dd',
    current: '21b9f6b4ccd7c35468b0e63e1bffd770be5fa1e2f7e1b7998bea9fa40449e38d',
    successorRevision: toolingTimingRevision }],
  [javascriptTestGroupsSource, { revision: toolingTimingRevision,
    predecessor: '21b9f6b4ccd7c35468b0e63e1bffd770be5fa1e2f7e1b7998bea9fa40449e38d',
    current: 'c0fb616bde11397df01a4a2b3d0324eaa565fca3c5af1aa8fc6b045ec2622c53' }],
];
const reviewedFiles = new Set(profiles.map(([file]) => file));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const text = bytes => {
  const source = bytes.toString('utf8');
  assert.ok(Buffer.from(source).equals(bytes));
  return source;
};
let pairs;

function verifySourceHistory(history) {
  assert.equal(history.schemaVersion, 1);
  assert.deepEqual(history.sources.map(row => row.file), profiles.map(([file]) => file));
  const verified = new Map();
  for (const [index, row] of history.sources.entries()) {
    const { file, predecessor, certificate, edit } = row;
    const { revision, predecessor: originalHash, current: currentHash } = profiles[index][1];
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
    const earlier = verified.get(file);
    assert.ok(!earlier || earlier.current === predecessor);
    const current = text(after);
    verified.set(file, Object.freeze({
      predecessor: earlier?.predecessor ?? predecessor,
      current,
      versions: Object.freeze(earlier ? [...earlier.versions, current] : [predecessor, current]),
    }));
  }
  return verified;
}

export function readIssue245SourcePair(file) {
  assert.ok(reviewedFiles.has(file), `no reviewed successor for ${file}`);
  pairs ??= verifySourceHistory(readProjectData(archive));
  return pairs.get(file);
}

export function restoreIssue245Source(file, source) {
  const { predecessor, versions } = readIssue245SourcePair(file);
  assert.ok(versions.includes(source), `unreviewed complete source for ${file}`);
  return predecessor;
}

export function restoreIssue245SourceSuccessor(file, source) {
  if (!reviewedFiles.has(file)) return source;
  const { predecessor, versions } = readIssue245SourcePair(file);
  return versions.includes(source) ? predecessor : source;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.deepEqual(process.argv.slice(2), ['--write']);
  const { captureGitArtifactCertificate } = await import('../../js/scripts/capture-git-artifact-certificate.mjs');
  const sources = profiles.map(([file, { revision, successorRevision }]) => {
    const { blob, certificate } = captureGitArtifactCertificate(revision, file);
    const current = successorRevision
      ? captureGitArtifactCertificate(successorRevision, file).blob
      : readFileSync(new URL(file, root));
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
