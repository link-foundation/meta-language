// Issue #195 gate self-test (pull request 196, "Path to a green PR and a
// release", section 0): every pull request check must be able to pass. The
// evaluator turns green on a fixture in which every pre-merge row passes, no
// pre-merge row needs its own aggregate to fail, and no pull request step
// depends on state that only exists after merge.
import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { buildIssue195Manifest, evaluateIssue195Acceptance } from '../scripts/issue-195-acceptance-lib.mjs';
import { evidenceStages } from '../scripts/issue-195-evidence-stages.mjs';
import { recordIssue195DirectiveObservation as observe } from './support/issue-195-observations.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const read = (relative) => readFileSync(path.join(root, relative), 'utf8');
const manifest = await buildIssue195Manifest(root);
const commit = 'c'.repeat(40);

/** A passing result for `cell`, with every field the evaluator requires. */
function passingResult(cell) {
  return {
    testId: cell.testId,
    outcome: 'passed',
    kind: cell.kind,
    positiveEvidence: true,
    command: 'gate self-test fixture',
    toolchainVersions: { node: process.version },
    grammarVersions: { corpus: 'pinned' },
    evidenceArtifacts: ['issue-195-results/execution-records.jsonl'],
    ...(cell.kind === 'fault-injection' ? { failureLogs: ['issue-195-results/logs/fault-injection.log'] } : {}),
    assertionsPassed: cell.assertions,
    executionRecords: cell.assertions.flatMap((assertionId) => cell.fixtureIds.map((fixtureId) => ({
      testId: cell.testId, assertionId, fixtureId, fixtureDigest: manifest.fixtureCatalog[fixtureId].sha256,
      runtime: cell.runtime, commit, outcome: 'passed', testName: 'gate self-test fixture',
    }))),
    fixtureDigests: Object.fromEntries(cell.fixtureIds.map((fixtureId) => [fixtureId, manifest.fixtureCatalog[fixtureId].sha256])),
  };
}

/**
 * The ledger as it is once every pre-merge row is implemented: rows still
 * waiting for an implementation name a fixture entry point.
 */
function implementedLedger() {
  const ledger = structuredClone(manifest);
  for (const entry of ledger.atomicRequirements) {
    for (const runtime of Object.keys(entry.implementationEntryPoints)) {
      entry.implementationEntryPoints[runtime] ??= ['gate self-test fixture'];
    }
  }
  return ledger;
}

const preMergeCells = (ledger) => ledger.atomicRequirements.flatMap((entry) =>
  entry.verifications.filter(({ checkpoint }) => checkpoint === 'pre-merge').map((cell) => ({ entry, cell })));
const document = (results) => ({
  schemaVersion: 1, issue: 195, commit, producer: 'gate-self-test', generatedAt: new Date(0).toISOString(), results,
});

test('the pre-merge aggregate is green when every pre-merge row passes', () => {
  const ledger = implementedLedger();
  const cells = preMergeCells(ledger);
  assert.ok(cells.length > 600, `${cells.length} pre-merge cells`);
  const report = evaluateIssue195Acceptance(ledger, [document(cells.map(({ cell }) => passingResult(cell)))], {
    checkpoint: 'pre-merge', commit,
  });
  assert.deepEqual(report.gateErrors, []);
  assert.deepEqual(report.requirements.filter(({ passed }) => !passed).map(({ id }) => id), []);
  assert.equal(report.passed, true);
  // The green fixture is not vacuous: one missing pre-merge result turns it red.
  const withoutOne = evaluateIssue195Acceptance(ledger, [document(cells.slice(1).map(({ cell }) => passingResult(cell)))], {
    checkpoint: 'pre-merge', commit,
  });
  assert.equal(withoutOne.passed, false);
  observe('I195-ACCEPTANCE-PR-CHECKS-PASSABLE', ['everyPreMergeRowPassingGivesGreenAggregate'],
    'the pre-merge aggregate is green when every pre-merge row passes');
});

test('no pre-merge row needs its own aggregate to fail or post-merge state to exist', () => {
  for (const { entry, cell } of preMergeCells(manifest)) {
    assert.notDeepEqual(entry.traceability.evidenceGroups, ['merge-enforcement'], entry.id);
    assert.equal(cell.assertions.includes('failingCheckBlocksMerge'), false, entry.id);
  }
  assert.equal(evidenceStages('pre-merge').some(({ name }) => name === 'merge-enforcement'), false);
  const moved = manifest.atomicRequirements.find(({ id }) => id === 'I195-ACCEPTANCE-REQUIRED-MERGE-CHECK');
  assert.deepEqual(moved.verifications.map(({ checkpoint }) => checkpoint), ['post-merge']);
  // Post-merge and release-delivery rows never count in the pull request aggregate.
  const report = evaluateIssue195Acceptance(manifest, [], { checkpoint: 'pre-merge', commit });
  const counted = new Set(report.requirements.map(({ id }) => id));
  for (const entry of manifest.atomicRequirements) {
    if (entry.verifications.every(({ checkpoint }) => checkpoint !== 'pre-merge')) {
      assert.equal(counted.has(entry.id), false, entry.id);
    }
  }
  observe('I195-ACCEPTANCE-PR-CHECKS-PASSABLE', ['noPreMergeRowNeedsItsOwnAggregateToFail'],
    'no pre-merge row needs its own aggregate to fail or post-merge state to exist');
});

/** The steps of `workflow`, each with the job it belongs to. */
function steps(workflow) {
  const result = [];
  let job = null;
  let current = null;
  for (const line of workflow.split(/\r?\n/u)) {
    const jobMatch = /^ {2}([\w-]+):$/u.exec(line);
    if (jobMatch) job = { id: jobMatch[1], header: [] };
    if (/^ {6}- /u.test(line)) {
      current = { job, lines: [line] };
      result.push(current);
    } else if (/^ {2}\S|^ {4}\S/u.test(line)) {
      current = null;
      if (job && !jobMatch) job.header.push(line);
    } else if (current) {
      current.lines.push(line);
    }
  }
  return result.map(({ job: owner, lines }) => ({ job: owner, text: lines.join('\n') }));
}

test('pull request checks use no post-merge state, and post-merge reports never block', () => {
  const workflows = readdirSync(path.join(root, '.github/workflows')).filter((name) => /\.ya?ml$/u.test(name));
  for (const name of workflows) {
    const text = read(`.github/workflows/${name}`);
    assert.doesNotMatch(text, /ISSUE_195_RULESET_TOKEN|ISSUE_195_PULL_REQUEST_HEAD/u, name);
    for (const { job, text: step } of steps(text)) {
      if (!/--online|--live\b|--checkpoint post-merge/u.test(step)) continue;
      const header = job.header.join('\n');
      const mainOnly = /github\.event_name != 'pull_request'/u.test(step) || /github\.event_name == 'push'/u.test(header);
      const nonBlocking = /continue-on-error: true/u.test(step) || /continue-on-error: true/u.test(header);
      assert.ok(mainOnly, `${name} ${job.id}: a post-merge step runs on pull requests:\n${step}`);
      assert.ok(nonBlocking, `${name} ${job.id}: a post-merge step can fail its workflow:\n${step}`);
    }
  }
  const acceptance = read('.github/workflows/issue-195-acceptance.yml');
  assert.match(acceptance, /check-dependencies\.mjs --delivery --offline/u);
  assert.match(acceptance, /\n {2}post-merge:\n {4}name: Post-merge Report\n {4}if: \$\{\{ github\.event_name == 'push' \}\}\n {4}continue-on-error: true\n/u);
  observe('I195-ACCEPTANCE-PR-CHECKS-PASSABLE', ['pullRequestChecksUseNoPostMergeState', 'postMergeReportsAreNonBlocking'],
    'pull request checks use no post-merge state, and post-merge reports never block');
});

test('a scheduled workflow on main refreshes the dependencies and opens its own pull request', () => {
  const refresh = read('.github/workflows/dependency-refresh.yml');
  assert.match(refresh, /\n {2}schedule:\n {4}- cron: '[^']+'\n/u);
  assert.doesNotMatch(refresh, /\n {2}pull_request/u);
  assert.match(refresh, /if: \$\{\{ github\.ref == 'refs\/heads\/main' \}\}/u);
  assert.match(refresh, /npm run dependencies:refresh/u);
  assert.match(refresh, /gh pr create --base main/u);
  observe('I195-ACCEPTANCE-PR-CHECKS-PASSABLE', ['dependencyRefreshScheduledOnMain'],
    'a scheduled workflow on main refreshes the dependencies and opens its own pull request');
});
