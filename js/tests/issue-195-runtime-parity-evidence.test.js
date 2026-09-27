// Issue #195 runtime-parity evidence: the checks behind each assertion of the
// parity and shared-concept requirements accept agreeing observations of the
// pinned fixtures and reject disagreeing ones, a different fixture revision,
// and observations that no longer show the requirement's property. The parity
// check (scripts/check-issue-195-runtime-parity.mjs) records the evidence
// against the Rust probe's output; this test records nothing.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  PARITY_REQUIREMENTS,
  mismatchedSections,
  parityAssertions,
} from '../scripts/issue-195-parity-evidence.mjs';
import { runtimeObservation } from '../scripts/issue-195-runtime-observation.mjs';
import {
  ISSUE_195_FIXTURE_FILES,
  issue195FixtureDigest,
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
