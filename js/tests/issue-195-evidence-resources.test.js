// Resource limits of the CI workflows and of local runs (docs/vision.md#resource-limits,
// comment 5956284229): a fast compile gate, test matrices with timeouts,
// cancellation of superseded runs, and the targeted-check, batching and
// cleanup rules agents and contributors follow.
import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import { test } from 'node:test';

import { boundedEnvironment } from '../../scripts/with-cache-cleanup.mjs';
import { TEST_GROUPS, partitionProblems, partitionTests, testFiles } from '../scripts/test-groups.mjs';
import { recordIssue195DirectiveObservation as observe } from './support/issue-195-observations.js';

const read = (relative) => restorePlatformTestDeadlineHistory(readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8'));
const WORKFLOWS = '.github/workflows';

/** The text of job `id` in `workflow`, from its key to the next job key. */
function job(workflow, id) {
  const start = workflow.indexOf(`\n  ${id}:\n`);
  assert.ok(start >= 0, `job ${id}`);
  const end = workflow.slice(start + 1).search(/\n {2}[\w-]+:\n/u);
  return workflow.slice(start, end < 0 ? undefined : start + 1 + end);
}

const needs = (text) => {
  const value = text.match(/\n {4}needs: ([^\n]+)/u)?.[1];
  if (!value) return [];
  return value.replace(/^\[|\]$/gu, '').split(',').map((name) => name.trim());
};

test('the Rust workflow checks all targets and features first and the compiling jobs need it', () => {
  const rust = read(`${WORKFLOWS}/rust.yml`);
  assert.match(job(rust, 'check'), /cargo check --locked --all-targets --all-features/u);
  assert.match(job(rust, 'check'), /timeout-minutes: 15/u);
  for (const id of ['test', 'msrv', 'coverage', 'fresh-merge']) {
    assert.ok(needs(job(rust, id)).includes('check'), `${id} needs the compile gate`);
  }
  // The build job needs the tests, which need the compile gate.
  assert.ok(needs(job(rust, 'build')).includes('test'));
  observe('I195-RESOURCE-CI-COMPILE-GATE', ['compileGateRunsAllTargetsAndFeatures', 'compilingJobsNeedTheCompileGate'],
    'the Rust workflow checks all targets and features first and the compiling jobs need it');
});

test('CI splits the Rust and JavaScript tests into matrix jobs with their own timeouts', () => {
  const rust = job(restoreNativeTestProfileHistory(read(`${WORKFLOWS}/rust.yml`)), 'test');
  assert.match(rust, /suite: \[grammar, inference, generative, translation, remaining\]/u);
  assert.match(rust, /cargo test --no-fail-fast --all-features --tests --verbose -- \$SUITE_FILTERS/u);
  // The remaining suite skips every filter another suite runs, so each test runs in one job.
  const filters = Object.fromEntries([...rust.matchAll(/- suite: (\w+)\n\s+filters: ([^\n]+)/gu)]
    .map(([, suite, line]) => [suite, line]));
  for (const prefix of ['grammar_', 'inference_', 'issue_195_generative', 'translation_', 'issue_195_', 'conformance']) {
    assert.ok(filters.remaining.includes(`--skip ${prefix}`), `remaining skips ${prefix}`);
  }
  const javascriptWorkflow = read(`${WORKFLOWS}/js.yml`);
  const acceptanceWorkflow = read(`${WORKFLOWS}/ci.yml`);
  const javascript = job(javascriptWorkflow, 'tests');
  const matrixGroups = javascript.match(/group: \[([^\]]*)\]/u)[1].split(', ');
  const reportGroups = ['report-artifacts'];
  assert.deepEqual(matrixGroups, Object.keys(TEST_GROUPS).filter((group) => !reportGroups.includes(group)));
  assert.deepEqual([...matrixGroups, ...reportGroups].sort(), Object.keys(TEST_GROUPS).sort());
  assert.equal(new Set([...matrixGroups, ...reportGroups]).size, matrixGroups.length + reportGroups.length);
  assert.match(javascript, /scripts\/test-groups\.mjs --run "\$TEST_GROUP"/u);
  for (const [workflow, id, producer, condition] of [
    [javascriptWorkflow, 'report-assertions', 'reports', /\n {4}if: \$\{\{ !cancelled\(\) && !inputs\.external_reports \}\}/u],
    [acceptanceWorkflow, 'self-translation-report-validation', 'self-translation-report', /\n {4}if: \$\{\{ !cancelled\(\) && github\.event_name != 'release' \}\}/u],
  ]) {
    const reports = job(workflow, id);
    assert.deepEqual(needs(reports), [producer], `${id} waits for the complete report shards`);
    assert.match(reports, condition);
    assert.match(reports, /\n {4}timeout-minutes: \d+\n/u);
    assert.doesNotMatch(reports, /continue-on-error: true/u);
    assert.doesNotMatch(reports, /generate-self-translation-report\.mjs/u);
    assert.match(reports, /pattern: self-translation-report-\$\{\{ github\.sha \}\}-\*/u);
    assert.match(reports, /ISSUE_195_SELF_TRANSLATION_REPORT_DIRECTORY: \$\{\{ github\.workspace \}\}\/issue-195-artifacts\/self-translation-report/u);
    assert.equal([...reports.matchAll(/scripts\/test-groups\.mjs --run report-artifacts/gu)].length, 1);
    assert.equal([...workflow.matchAll(/scripts\/test-groups\.mjs --run report-artifacts/gu)].length, 1);
    const download = reports.indexOf('actions/download-artifact@');
    assert.ok(download >= 0, `${id} downloads the generated reports`);
    assert.ok(download < reports.indexOf('scripts/test-groups.mjs --run report-artifacts'));
    assert.match(job(workflow, producer), /shard: \[1, 2, 3, 4, 5, 6, 7, 8\]/u);
  }
  // A standalone run validates its own reports; reusable CI supplies them to
  // its dedicated required job, so neither path repeats the report module work.
  assert.match(job(javascriptWorkflow, 'reports'), /\n {4}if: \$\{\{ !inputs\.external_reports \}\}/u);
  assert.match(job(acceptanceWorkflow, 'js'), /external_reports: true/u);
  const packageGate = job(javascriptWorkflow, 'test');
  assert.deepEqual(needs(packageGate), ['tests', 'report-assertions', 'package-checks', 'translation-test-feedback']);
  assert.ok(packageGate.includes('TRANSLATION_FEEDBACK_RESULT: ${{ needs.translation-test-feedback.result }}'));
  assert.ok(packageGate.includes('if [ "$TRANSLATION_FEEDBACK_RESULT" != "success" ]; then'));
  const packageChecks = job(javascriptWorkflow, 'package-checks');
  assert.deepEqual(needs(packageChecks), ['tests', 'report-assertions']);
  assert.match(packageChecks, /timeout-minutes: 10\n/u);
  assert.match(packageChecks, /fail-fast: false\n/u);
  assert.match(packageGate, /PACKAGE_CHECKS_RESULT: \$\{\{ needs\.package-checks\.result \}\}/u);
  assert.match(packageGate, /if \[ "\$PACKAGE_CHECKS_RESULT" != "success" \]; then[\s\S]*?exit 1/u);
  assert.match(packageGate, /REPORTS_RESULT: \$\{\{ needs\.report-assertions\.result \}\}/u);
  assert.match(packageGate, /EXTERNAL_REPORTS: \$\{\{ inputs\.external_reports \}\}/u);
  assert.match(packageGate, /if \[ "\$TESTS_RESULT" != "success" \]; then[\s\S]*?exit 1/u);
  assert.match(packageGate, /if \[ "\$EXTERNAL_REPORTS" != "true" \] && \[ "\$REPORTS_RESULT" != "success" \]; then[\s\S]*?exit 1/u);
  for (const id of ['rust', 'candidates', 'runtime-parity', 'acceptance']) {
    const prerequisites = needs(job(acceptanceWorkflow, id));
    assert.ok(prerequisites.includes('js'), `${id} needs the normal JavaScript groups`);
    assert.ok(prerequisites.includes('self-translation-report-validation'), `${id} needs the dedicated report assertions`);
  }
  const aggregate = job(acceptanceWorkflow, 'acceptance');
  assert.match(aggregate, /\n {4}if: \$\{\{ always\(\) \}\}/u);
  assert.match(aggregate, /REPORTS_RESULT: \$\{\{ needs\.self-translation-report-validation\.result \}\}/u);
  assert.match(aggregate, /\[ "\$REPORTS_RESULT" = failure \] \|\| \[ "\$REPORTS_RESULT" = cancelled \]/u);
  assert.deepEqual(partitionProblems(partitionTests(testFiles())), []);
  assert.match(javascript, /\n {4}timeout-minutes: \d+\n/u);
  assert.match(rust, /timeout-minutes: \$\{\{ matrix\.os == 'windows-latest' && \(matrix\.logical_suite \|\| matrix\.suite\) == 'grammar' && 30 \|\| 15 \}\}/u);
  observe('I195-RESOURCE-CI-TEST-MATRIX',
    ['rustTestsSplitBySuite', 'javascriptTestsSplitByGroup', 'everyTestFileInExactlyOneGroup', 'matrixJobsHaveTimeouts'],
    'CI splits the Rust and JavaScript tests into matrix jobs with their own timeouts');
});

test('every workflow cancels the runs a newer push supersedes', () => {
  const workflows = readdirSync(new URL(`../../${WORKFLOWS}`, import.meta.url)).filter((name) => /\.ya?ml$/u.test(name));
  assert.ok(workflows.length >= 3);
  for (const name of workflows) {
    const actual = read(`${WORKFLOWS}/${name}`);
    const text = ['ci.yml', 'js.yml', 'rust.yml'].includes(name) ? restoreVerificationConcurrencyHistory(actual, name) : actual;
    // A workflow ci.yml calls sees the caller's github.workflow, so it uses its own literal prefix instead.
    const prefix = /^ {2}workflow_call:/mu.test(text) ? name.replace(/\.ya?ml$/u, '') : '${{ github.workflow }}';
    assert.ok(text.includes(`\nconcurrency:\n  group: ${prefix}-`), `${name} groups its runs under ${prefix}-`);
    assert.match(text, /^concurrency:\n {2}group: [^\n]*github\.ref[^\n]*\n {2}cancel-in-progress: true$/mu, name);
  }
  observe('I195-RESOURCE-CI-CONCURRENCY', ['everyWorkflowHasConcurrencyGroup', 'supersededRunsCancelled'],
    'every workflow cancels the runs a newer push supersedes');
});

test('AGENTS.md and CONTRIBUTING.md keep local runs targeted, bounded and cleaned up', () => {
  const agents = read('AGENTS.md');
  const section = agents.slice(agents.indexOf('## Local checks and resources'));
  const contributing = read('CONTRIBUTING.md');
  assert.match(section, /Only targeted JavaScript tests/u);
  assert.match(section, /node --test tests\/<area>\*\.test\.js/u);
  for (const text of [section, contributing]) {
    assert.match(text, /rustfmt --edition 2024 --check/u);
    assert.match(text, /--config skip_children=true/u);
    assert.match(text, /Never compile Rust locally/u);
    assert.match(text.replace(/\s+/gu, ' '), /(?:let CI run native|runs native) compilation, tests and runtime parity/u);
    assert.match(text, /with-cache-cleanup\.mjs --event test --/u);
  }
  assert.match(contributing, /CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 CARGO_INCREMENTAL=0/u);
  const env = boundedEnvironment({ META_LANGUAGE_JOBS: '2' });
  assert.deepEqual([env.CARGO_BUILD_JOBS, env.RUST_TEST_THREADS, env.CARGO_INCREMENTAL], ['2', '2', '0']);
  const rust = read(`${WORKFLOWS}/rust.yml`);
  assert.match(job(rust, 'check'), /with-cache-cleanup\.mjs --event build -- cargo check --locked --all-targets --all-features/u);
  assert.match(job(rust, 'test'), /with-cache-cleanup\.mjs --event test -- cargo test --no-fail-fast/u);
  for (const forbidden of ['npm run acceptance:issue-195', 'run-issue-195-evidence.mjs', 'Lean, Rocq and Rust',
    'clean-consumer installs', 'workload clones', 'cargo llvm-cov', 'whole grammar registry']) {
    assert.ok(section.replace(/\s+/gu, ' ').includes(forbidden), `AGENTS.md forbids ${forbidden} locally`);
  }
  assert.match(agents.replace(/\s+/gu, ' '), /Bulk work: commit steps, review diffs, push once per batch after previous CI finishes and its errors are addressed/u);
  assert.match(section, /node scripts\/clean-caches\.mjs/u);
  assert.match(section.replace(/\s+/gu, ' '), /\[CI workflow\]\(\.github\/workflows\/ci\.yml\) runs native compilation, tests and runtime parity, full suites/u);
  observe('I195-RESOURCE-AGENT-RULES', [
    'targetedChecksDocumented',
    'boundedParallelismDocumented',
    'forbiddenLocalRunsListed',
    'batchingAndCleanupDocumented',
    'fullVerificationDelegatedToCi',
  ], 'AGENTS.md and CONTRIBUTING.md keep local runs targeted, bounded and cleaned up');
});

test('the cleanup wrapper is the documented way to run long local commands and bounds them', () => {
  assert.match(read('AGENTS.md'), /Wrap long checks with `node scripts\/with-cache-cleanup\.mjs --event test -- <command>`/u);
  assert.match(read('docs/cache-cleanup.md'), /`node scripts\/with-cache-cleanup\.mjs --event <event> -- <command>` runs the command and cleans afterwards, also when it fails/u);
  assert.match(read('AGENTS.md'), /After batches run `node scripts\/clean-caches\.mjs`/u);
  const env = boundedEnvironment({ PATH: '/bin', META_LANGUAGE_JOBS: '2' });
  assert.deepEqual(
    [env.CARGO_BUILD_JOBS, env.RUST_TEST_THREADS, env.CARGO_INCREMENTAL],
    ['2', '2', '0'],
  );
  assert.equal(boundedEnvironment({ CARGO_BUILD_JOBS: '1' }).CARGO_BUILD_JOBS, '1');
  observe('I195-RESOURCE-CACHE-CLEANUP-WRAPPER',
    ['wrapperDocumentedForLocalRuns', 'cleanupBetweenBatchesDocumented', 'wrapperBoundsParallelism'],
    'the cleanup wrapper is the documented way to run long local commands and bounds them');
});

import { restoreVerificationConcurrencyHistory } from './support/verification-concurrency-history.js';

import { restoreNativeTestProfileHistory } from './support/native-test-profile-history.js';

import {restorePlatformTestDeadlineHistory} from './support/platform-test-deadline-history.js';
