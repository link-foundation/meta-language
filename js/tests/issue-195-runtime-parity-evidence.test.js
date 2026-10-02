// Issue #195 runtime-parity evidence: the checks behind each assertion of the
// parity and shared-concept requirements accept agreeing observations of the
// pinned fixtures and reject disagreeing ones, a different fixture revision,
// and observations that no longer show the requirement's property. The parity
// check (scripts/check-issue-195-runtime-parity.mjs) records the evidence
// against the Rust probe's output; this test records only the resource row
// for how the check streams and keeps that evidence.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  PARITY_ARTIFACT_FILES,
  PARITY_REQUIREMENTS,
  PARITY_SECTIONS,
  canonicalJson,
  mismatchedSections,
  ndjson,
  observationFromRecords,
  observationRecords,
  parityAssertions,
  parityDigests,
} from '../scripts/issue-195-parity-evidence.mjs';
import { runtimeObservation } from '../scripts/issue-195-runtime-observation.mjs';
import {
  ISSUE_195_FIXTURE_FILES,
  issue195FixtureDigest,
  recordIssue195DirectiveObservation as observe,
} from './support/issue-195-observations.js';

const manifest = JSON.parse(
  await readFile(new URL('../../parity/issue-195-requirements.json', import.meta.url)),
);
const pinnedDigest = issue195FixtureDigest(ISSUE_195_FIXTURE_FILES.fourLanguage);
const javascript = runtimeObservation();

function assertions(requirementId, runtime, observations, digest = pinnedDigest) {
  return parityAssertions({ requirementId, runtime, observations, pinnedDigest: digest });
}

test('issue 195 parity evidence covers every parity and shared-concept requirement', () => {
  const declared = manifest.atomicRequirements
    .filter(({ area }) => area === 'runtime-parity' || area === 'shared-concepts')
    .map(({ id }) => id)
    .sort();
  assert.deepEqual(Object.keys(PARITY_REQUIREMENTS).sort(), declared);
});

test('issue 195 parity evidence holds every declared assertion for agreeing observations', () => {
  assert.equal(javascript.fixtureDigest, pinnedDigest);
  const observations = { javascript, rust: structuredClone(javascript) };
  assert.deepEqual(mismatchedSections(observations.javascript, observations.rust), []);
  for (const requirement of manifest.atomicRequirements) {
    if (!PARITY_REQUIREMENTS[requirement.id]) continue;
    for (const cell of requirement.verifications) {
      const { passed, failed } = assertions(requirement.id, cell.runtime, observations);
      assert.deepEqual(failed, {}, `${cell.testId}`);
      assert.deepEqual(passed.sort(), [...cell.assertions].sort(), `${cell.testId}`);
    }
  }
});

test('issue 195 parity evidence detects each requirement mutation without a text change', () => {
  for (const [requirementId, { sections, mutate }] of Object.entries(PARITY_REQUIREMENTS)) {
    const rust = structuredClone(javascript);
    mutate(rust);
    const mismatches = mismatchedSections(javascript, rust);
    assert.ok(mismatches.length > 0, requirementId);
    assert.ok(mismatches.every((section) => sections.includes(section)), requirementId);
    // Disagreeing runtimes support none of the requirement's assertions.
    for (const runtime of ['javascript', 'rust']) {
      assert.deepEqual(assertions(requirementId, runtime, { javascript, rust }).passed, [], requirementId);
    }
  }
});

test('issue 195 parity evidence rejects another fixture revision', () => {
  const observations = { javascript, rust: structuredClone(javascript) };
  const { passed, failed } = assertions('I195-PARITY-CST', 'rust', observations, '0'.repeat(64));
  assert.ok(!passed.includes('sameFixtureRevision'));
  assert.ok(failed.sameFixtureRevision);
});

test('issue 195 parity evidence requires the requirement property on both runtimes', () => {
  const broken = structuredClone(javascript);
  broken.translations.pop();
  const observations = { javascript: broken, rust: structuredClone(broken) };
  assert.deepEqual(assertions('I195-PARITY-TRANSLATIONS', 'javascript', observations).passed, []);

  const collapsed = structuredClone(javascript);
  // Lean's proof vocabulary collapsed into Rocq's is no longer distinct.
  const rocqKinds = collapsed.semantics.find(({ language }) => language === 'Rocq').proofs
    .map(({ kind }) => kind);
  for (const proof of collapsed.semantics.find(({ language }) => language === 'Lean').proofs) {
    proof.kind = rocqKinds[0];
  }
  const same = { javascript: collapsed, rust: structuredClone(collapsed) };
  assert.deepEqual(assertions('I195-SHARED-CONCEPTS', 'rust', same).passed, []);
});

test('issue 195 parity evidence compares sections entry by entry exactly as their canonical JSON', () => {
  // The small sections keep this check fast; the PDF grammar trees alone are about 75 MB.
  const sections = ['fixtureDigest', 'semantics', 'transforms', 'translations'];
  const sameAsWhole = (left, right) => sections.filter((section) =>
    canonicalJson(left[section]) !== canonicalJson(right[section]));
  const longer = { ...javascript, semantics: [...javascript.semantics, javascript.semantics[0]] };
  const reordered = { ...javascript, transforms: [...javascript.transforms].reverse() };
  const scalar = { ...javascript, fixtureDigest: 'f'.repeat(64) };
  const listed = { ...javascript, translations: { 0: javascript.translations[0] } };
  const copied = { ...javascript, semantics: structuredClone(javascript.semantics) };
  for (const other of [javascript, copied, longer, reordered, scalar, listed]) {
    assert.deepEqual(mismatchedSections(javascript, other, sections), sameAsWhole(javascript, other));
  }
  assert.deepEqual(mismatchedSections(javascript, longer), ['semantics']);
  assert.deepEqual(mismatchedSections(javascript, scalar), ['fixtureDigest']);
  observe('I195-RESOURCE-PARITY-DIGESTS', ['entriesComparedOneAtATime'],
    'issue 195 parity evidence compares sections entry by entry exactly as their canonical JSON');
});

test('issue 195 parity evidence keeps entry digests and only the differing entries', () => {
  const rust = structuredClone(javascript);
  const same = parityDigests({ javascript, rust });
  const entries = PARITY_SECTIONS.reduce((count, section) =>
    count + (Array.isArray(javascript[section]) ? javascript[section].length : 1), 0);
  assert.equal(same.digests.length, entries);
  assert.ok(same.digests.every((record) => /^[0-9a-f]{64}$/u.test(record.javascript) && record.javascript === record.rust));
  assert.deepEqual(same.differences, []);

  rust.pdfGrammar[7].reconstruction += ' ';
  rust.semantics.push({ extra: true });
  const changed = parityDigests({ javascript, rust });
  assert.deepEqual(changed.differences.map(({ section, index }) => [section, index]), [
    ['pdfGrammar', 7], ['semantics', javascript.semantics.length],
  ]);
  assert.deepEqual(changed.differences[0].rust, rust.pdfGrammar[7]);
  assert.deepEqual(changed.differences[0].javascript, javascript.pdfGrammar[7]);
  assert.equal(changed.differences[1].javascript, null);
  assert.equal(changed.digests.find(({ section, index }) =>
    section === 'semantics' && index === javascript.semantics.length).javascript, null);
  // The kept evidence is a small fraction of the observations it stands for.
  const evidence = ndjson(changed.digests).length + ndjson(changed.differences).length;
  assert.ok(evidence * 10 < canonicalJson(javascript).length, `${evidence} bytes of evidence`);
  observe('I195-RESOURCE-PARITY-DIGESTS', ['fullEntriesOnlyWhereDifferent'],
    'issue 195 parity evidence keeps entry digests and only the differing entries');
});

test('issue 195 parity evidence rebuilds an observation from the probe NDJSON records', () => {
  const lines = ndjson([...observationRecords(javascript)]).trimEnd().split('\n');
  assert.equal(lines.length, parityDigests({ javascript }).digests.length);
  const rebuilt = observationFromRecords(lines.map((line) => JSON.parse(line)));
  assert.deepEqual(mismatchedSections(javascript, rebuilt), []);
  const empty = observationFromRecords([...observationRecords({ ...javascript, bindingRenames: [] })]);
  assert.deepEqual(empty.bindingRenames, []);
  assert.throws(() => observationFromRecords([{ section: 'trees', index: null, value: 1 }]), /unknown runtime-parity section trees/u);
  assert.throws(() => observationFromRecords([{ section: 'semantics', index: 1, value: {} }]), /semantics entry 1 is out of order/u);
  assert.throws(() => observationFromRecords([
    { section: 'schemaVersion', index: null, value: 1 }, { section: 'schemaVersion', index: null, value: 1 },
  ]), /schemaVersion is repeated/u);
});

test('issue 195 parity check streams the Rust probe to a file and keeps digests, not full trees', async () => {
  const check = await readFile(new URL('../scripts/check-issue-195-runtime-parity.mjs', import.meta.url), 'utf8');
  assert.doesNotMatch(check, /maxBuffer|execFileSync/u);
  assert.match(check, /'--example', 'issue_195_runtime_probe', '--', output/u);
  assert.match(check, /observationFromRecords\(records\)/u);
  assert.doesNotMatch(check, /'javascript\.json'|'rust\.json'|stableJson\(javascript\)|stableJson\(rust\)/u);
  assert.deepEqual(
    [PARITY_ARTIFACT_FILES.digests, PARITY_ARTIFACT_FILES.differences, PARITY_ARTIFACT_FILES.translations('rust')],
    ['digests.ndjson', 'differences.ndjson', 'rust-translations.json'],
  );
  const probe = await readFile(new URL('../../rust/examples/issue_195_runtime_probe.rs', import.meta.url), 'utf8');
  assert.match(probe, /BufWriter::new\(File::create\(path\)/u);
  assert.match(probe, /json!\(\{ "section": section, "index": index, "value": value \}\)/u);
  const runner = await readFile(new URL('../scripts/run-issue-195-evidence.mjs', import.meta.url), 'utf8');
  assert.match(runner, /PARITY_ARTIFACT_FILES\.translations\(runtime\)/u);
  observe('I195-RESOURCE-PARITY-DIGESTS', ['probeStreamsNdjson', 'noLargeOutputBuffer'],
    'issue 195 parity check streams the Rust probe to a file and keeps digests, not full trees');
});
