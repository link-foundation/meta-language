// The CI workflow structure: the Rust workflow runs only after the JavaScript
// workflow passes, the acceptance workflow's Rust stages run only after the
// matching JavaScript stages, and every job reports every failure in one run
// (cargo test --no-fail-fast, check steps that run unless the run was
// cancelled, and cargo fmt, clippy and doc in separate steps).
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { recordIssue195DirectiveObservation as observe } from './support/issue-195-observations.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const read = (file) => readFileSync(path.join(root, file), 'utf8');
const workflow = (name) => read(`.github/workflows/${name}`);
const jobs = (name) => new Map(parseWorkflow(workflow(name)).map((job) => [job.id, job]));

function needs(job) {
  const match = /^ {4}needs:\s*(.+)$/mu.exec(job.header);
  if (!match) return [];
  return match[1].replace(/^\[|\]$/gu, '').split(',').map((entry) => entry.trim()).filter(Boolean);
}

function condition(job) {
  return /^ {4}if:\s*(.+)$/mu.exec(job.header)?.[1] ?? '';
}

function triggers(text) {
  const block = /^on:\s*\n((?:(?: {2,}.*)?\n)*)/mu.exec(text)?.[1] ?? '';
  return [...block.matchAll(/^ {2}([A-Za-z_]+):/gmu)].map((match) => match[1]);
}

test('ci.yml calls the JavaScript workflow and then the Rust workflow, which needs it', () => {
  const text = workflow('ci.yml');
  assert.deepEqual(triggers(text).sort(), ['pull_request', 'push']);
  const ci = jobs('ci.yml');
  assert.match(ci.get('js').header, /^ {4}uses: \.\/\.github\/workflows\/js\.yml$/mu);
  assert.match(ci.get('rust').header, /^ {4}uses: \.\/\.github\/workflows\/rust\.yml$/mu);
  assert.ok(needs(ci.get('rust')).includes('js'), 'the rust job needs the js job');
  // A failed js job makes the rust job skipped: its condition must not override that with always() or !cancelled().
  assert.doesNotMatch(condition(ci.get('rust')), /always\(\)|cancelled\(\)/u);
  observe('I195-CI-RUST-AFTER-JAVASCRIPT', ['rustWorkflowNeedsJavaScript', 'skippedWhenJavaScriptFails'],
    'ci.yml runs the Rust workflow only after the JavaScript workflow, and skips it when JavaScript fails');
});

test('js.yml and rust.yml are called by ci.yml and have no push or pull_request triggers of their own', () => {
  for (const name of ['js.yml', 'rust.yml']) {
    const on = triggers(workflow(name));
    assert.ok(on.includes('workflow_call'), `${name} is a reusable workflow`);
    assert.ok(!on.includes('push'), `${name} has no push trigger`);
    assert.ok(!on.includes('pull_request'), `${name} has no pull_request trigger`);
    // github.workflow is the caller's name inside a called workflow; equal groups would deadlock.
    assert.doesNotMatch(workflow(name), /group:\s*\$\{\{\s*github\.workflow\s*\}\}/u, `${name} uses its own literal concurrency prefix`);
  }
  observe('I195-CI-RUST-AFTER-JAVASCRIPT', ['calledWorkflowsHaveNoOwnPushOrPullRequestTriggers'],
    'js.yml and rust.yml start only when ci.yml calls them');
});

test('every cargo test command in the workflows and the scripts they call uses --no-fail-fast', () => {
  const files = [
    '.github/workflows/ci.yml',
    '.github/workflows/js.yml',
    '.github/workflows/rust.yml',
    '.github/workflows/issue-195-acceptance.yml',
    'rust/scripts/simulate-fresh-merge.sh',
  ];
  let seen = 0;
  for (const file of files) {
    for (const [index, line] of read(file).split('\n').entries()) {
      if (/^\s*(?:#|echo\b)/u.test(line) || !/\bcargo\s+(?:\+\S+\s+)?test\b/u.test(line)) continue;
      seen += 1;
      assert.match(line, /\bcargo\s+(?:\+\S+\s+)?test\s+--no-fail-fast\b/u, `${file}:${index + 1} runs cargo test without --no-fail-fast`);
    }
  }
  // Scripts that spawn cargo with an argument list: every 'test' argument is followed by --no-fail-fast.
  for (const file of ['js/scripts/run-issue-195-evidence.mjs', 'js/scripts/run-rml-pr184-workloads.mjs', 'js/scripts/run-formal-ai-workloads.mjs']) {
    const text = read(file);
    for (const match of text.matchAll(/'cargo',\s*\[([^\]]*?)'test'(,\s*'[^']*')?/gu)) {
      seen += 1;
      assert.equal(match[2]?.replace(/^,\s*/u, ''), "'--no-fail-fast'", `${file} spawns cargo test without --no-fail-fast: ${match[0]}`);
    }
  }
  assert.ok(seen >= 6, `expected at least six cargo test commands, saw ${seen}`);
  observe('I195-CI-REPORT-EVERY-FAILURE', ['cargoTestNoFailFast'], 'every cargo test command runs with --no-fail-fast');
});

const FIRST_CHECK = /^(?:Check|Run|Reject|Build|Verify|Scan|Compare)\b/u;
const CHECK_JOBS = [
  ['rust.yml', ['lint', 'test', 'build']],
  ['js.yml', ['test']],
];

test('every step after the first check step of a job runs unless the run was cancelled', () => {
  for (const [name, ids] of CHECK_JOBS) {
    const parsed = jobs(name);
    for (const id of ids) {
      const job = parsed.get(id);
      assert.ok(job, `${name} has a ${id} job`);
      const first = job.steps.findIndex((step) => FIRST_CHECK.test(step.name));
      assert.ok(first >= 0, `${name} job ${id} has a check step`);
      const later = job.steps.slice(first + 1);
      assert.ok(later.length > 0, `${name} job ${id} has check steps after the first`);
      for (const step of later) {
        assert.ok(step.afterFailure, `${name} job ${id} step "${step.name}" does not run after an earlier failure (add if: \${{ !cancelled() }})`);
      }
    }
  }
  observe('I195-CI-REPORT-EVERY-FAILURE', ['checkStepsRunUnlessCancelled'], 'every check step runs unless the run was cancelled');
});

test('cargo fmt, cargo clippy and cargo doc are separate steps of the lint job', () => {
  const lint = jobs('rust.yml').get('lint');
  const tools = ['fmt', 'clippy', 'doc'];
  const steps = tools.map((tool) => {
    const matching = lint.steps.filter((step) => new RegExp(`\\bcargo\\s+${tool}\\b`, 'u').test(step.run));
    assert.equal(matching.length, 1, `exactly one lint step runs cargo ${tool}`);
    return matching[0];
  });
  assert.equal(new Set(steps).size, tools.length, 'cargo fmt, clippy and doc run in three different steps');
  for (const step of steps.slice(1)) assert.ok(step.afterFailure, `"${step.name}" runs after a formatting failure`);
  observe('I195-CI-REPORT-EVERY-FAILURE', ['lintStepsIndependent'], 'cargo fmt, clippy and doc run in separate steps');
});

test('the acceptance workflow runs each Rust stage only after the matching JavaScript stage passes', () => {
  const acceptance = jobs('issue-195-acceptance.yml');
  for (const [rust, javascript] of [['native-rust', 'native-javascript'], ['rust-suite', 'javascript-suite']]) {
    assert.ok(acceptance.has(javascript), `the acceptance workflow has a ${javascript} job`);
    assert.ok(acceptance.has(rust), `the acceptance workflow has a ${rust} job`);
    assert.ok(needs(acceptance.get(rust)).includes(javascript), `${rust} needs ${javascript}`);
    assert.match(condition(acceptance.get(rust)), new RegExp(`needs\\.${javascript}\\.result == 'success'`, 'u'), `${rust} runs only when ${javascript} succeeded`);
  }
  // No other job runs a JavaScript or Rust stage beside them, so the order cannot be bypassed.
  const native = acceptance.get('native-translations');
  assert.doesNotMatch(native.header, /target:\s*\[[^\]]*\b(?:javascript|rust)\b/u);
  assert.ok(!acceptance.has('evidence-stages'), 'the suite stages are separate jobs, not one matrix');
  const aggregate = needs(acceptance.get('acceptance'));
  for (const stage of ['native-javascript', 'native-rust', 'native-translations', 'javascript-suite', 'rust-suite', 'delivery']) {
    assert.ok(aggregate.includes(stage), `the aggregate needs ${stage}`);
  }
  observe('I195-CI-ACCEPTANCE-STAGE-ORDER', ['rustStagesNeedJavaScriptStages'], 'the Rust acceptance stages need the JavaScript stages');
});
