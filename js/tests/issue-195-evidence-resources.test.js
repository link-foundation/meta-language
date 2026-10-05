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

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const WORKFLOWS = '.github/workflows';

/** The text of job `id` in `workflow`, from its key to the next job key. */
function job(workflow, id) {
  const start = workflow.indexOf(`\n  ${id}:\n`);
  assert.ok(start >= 0, `job ${id}`);
  const end = workflow.slice(start + 1).search(/\n {2}[\w-]+:\n/u);
  return workflow.slice(start, end < 0 ? undefined : start + 1 + end);
}

const needs = (text) => text.match(/\n {4}needs: \[([^\]]*)\]/u)?.[1].split(',').map((name) => name.trim()) ?? [];

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
  const rust = job(read(`${WORKFLOWS}/rust.yml`), 'test');
  assert.match(rust, /suite: \[grammar, inference, generative, translation, remaining\]/u);
  assert.match(rust, /cargo test --no-fail-fast --all-features --tests --verbose -- \$SUITE_FILTERS/u);
  // The remaining suite skips every filter another suite runs, so each test runs in one job.
  const filters = Object.fromEntries([...rust.matchAll(/- suite: (\w+)\n\s+filters: ([^\n]+)/gu)]
    .map(([, suite, line]) => [suite, line]));
  for (const prefix of ['grammar_', 'inference_', 'issue_195_generative', 'translation_', 'issue_195_', 'conformance']) {
    assert.ok(filters.remaining.includes(`--skip ${prefix}`), `remaining skips ${prefix}`);
  }
  const javascript = job(read(`${WORKFLOWS}/js.yml`), 'tests');
  assert.deepEqual(javascript.match(/group: \[([^\]]*)\]/u)[1].split(', '), Object.keys(TEST_GROUPS));
  assert.match(javascript, /scripts\/test-groups\.mjs --run "\$TEST_GROUP"/u);
  assert.deepEqual(partitionProblems(partitionTests(testFiles())), []);
  for (const text of [rust, javascript]) assert.match(text, /\n {4}timeout-minutes: \d+\n/u);
  observe('I195-RESOURCE-CI-TEST-MATRIX',
    ['rustTestsSplitBySuite', 'javascriptTestsSplitByGroup', 'everyTestFileInExactlyOneGroup', 'matrixJobsHaveTimeouts'],
    'CI splits the Rust and JavaScript tests into matrix jobs with their own timeouts');
});

test('every workflow cancels the runs a newer push supersedes', () => {
  const workflows = readdirSync(new URL(`../../${WORKFLOWS}`, import.meta.url)).filter((name) => /\.ya?ml$/u.test(name));
  assert.ok(workflows.length >= 3);
  for (const name of workflows) {
    const text = read(`${WORKFLOWS}/${name}`);
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
  assert.match(section, /Run only the checks that cover the change/u);
  assert.match(section, /node --test tests\/<area>\*\.test\.js/u);
  assert.match(section, /cargo test --test unit <filter>/u);
  for (const text of [section, read('CONTRIBUTING.md')]) {
    assert.match(text, /CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 CARGO_INCREMENTAL=0/u);
    assert.match(text, /with-cache-cleanup\.mjs --event test --/u);
  }
  for (const forbidden of ['npm run acceptance:issue-195', 'run-issue-195-evidence.mjs', 'Lean, Rocq and Rust',
    'clean-consumer installs', 'workload clones', 'cargo llvm-cov', 'whole grammar registry']) {
    assert.ok(section.replace(/\s+/gu, ' ').includes(forbidden), `AGENTS.md forbids ${forbidden} locally`);
  }
  assert.match(section, /Work in batches: commit each step, re-read the diff and push once per batch/u);
  assert.match(section, /node scripts\/clean-caches\.mjs/u);
  assert.match(section, /`CI` workflow\s+\(`\.github\/workflows\/ci\.yml`\)[^;]*\s+runs them on every push/u);
  observe('I195-RESOURCE-AGENT-RULES', [
    'targetedChecksDocumented',
    'boundedParallelismDocumented',
    'forbiddenLocalRunsListed',
    'batchingAndCleanupDocumented',
    'fullVerificationDelegatedToCi',
  ], 'AGENTS.md and CONTRIBUTING.md keep local runs targeted, bounded and cleaned up');
});

test('the cleanup wrapper is the documented way to run long local commands and bounds them', () => {
  assert.match(read('AGENTS.md'), /Wrap long runs in `node scripts\/with-cache-cleanup\.mjs --event test -- <command>`/u);
  assert.match(read('docs/cache-cleanup.md'), /`node scripts\/with-cache-cleanup\.mjs --event <event> -- <command>` runs the command and cleans afterwards, also when it fails/u);
  assert.match(read('AGENTS.md'), /After a batch, run `node scripts\/clean-caches\.mjs`/u);
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
