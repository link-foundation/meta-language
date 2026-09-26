// Shared writer of issue #195 execution records.
//
// The evidence runner (js/scripts/run-issue-195-evidence.mjs) sets
// ISSUE_195_OBSERVATION_FILE and ISSUE_195_COMMIT, runs the suites, and
// accepts a requirement cell only when every declared assertion has a passed
// record for every fixture. Tests call `recordIssue195Observations` after
// their assertions hold, so a failing test never produces a record.
import { createHash } from 'node:crypto';
import { appendFileSync, readFileSync } from 'node:fs';

const repositoryRoot = new URL('../../../', import.meta.url);

// The fixture files the requirement catalog points at, by kind of requirement.
export const ISSUE_195_FIXTURE_FILES = Object.freeze({
  fourLanguage: 'parity/fixtures/four-language-conformance.json',
  evidence: 'parity/fixtures/issue-195-evidence.json',
  grammarImporters: 'parity/fixtures/grammar-importers.json',
});

const digests = new Map();
const recorded = new Set();

// Mirrors `slug` in js/scripts/issue-195-requirements.mjs.
export function issue195Slug(value) {
  return value
    .normalize('NFKD')
    .toLowerCase()
    .replace(/\+/g, '-plus')
    .replace(/#/g, '-sharp')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '');
}

export function issue195FixtureDigest(fixtureFile) {
  if (!digests.has(fixtureFile)) {
    const bytes = readFileSync(new URL(fixtureFile, repositoryRoot));
    digests.set(fixtureFile, createHash('sha256').update(bytes).digest('hex'));
  }
  return digests.get(fixtureFile);
}

// Records that `assertions` passed for `fixtureId` under the verification
// `${requirementId}-javascript-${suffix}`. Repeated records of the same cell
// are written once: the runner rejects duplicates.
export function recordIssue195Observations({
  requirementId, suffix, fixtureId, fixtureFile, assertions, testName,
}) {
  const path = process.env.ISSUE_195_OBSERVATION_FILE;
  if (!path) return;
  const testId = `${requirementId}-javascript-${suffix}`.toLowerCase();
  const fixtureDigest = issue195FixtureDigest(fixtureFile);
  const lines = [];
  for (const assertionId of assertions) {
    const key = `${testId}\u0000${assertionId}\u0000${fixtureId}`;
    if (recorded.has(key)) continue;
    recorded.add(key);
    lines.push(JSON.stringify({
      testId, assertionId, fixtureId, fixtureDigest,
      runtime: 'javascript', commit: process.env.ISSUE_195_COMMIT,
      outcome: 'passed', testName,
    }));
  }
  // One append per call, so records of concurrently running suites never
  // interleave within a line.
  if (lines.length > 0) appendFileSync(path, `${lines.join('\n')}\n`);
}
