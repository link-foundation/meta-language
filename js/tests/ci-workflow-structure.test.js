// The CI workflow structure: the Rust workflow and the acceptance stages run
// only after the JavaScript workflow passes, the Rust acceptance stages run
// only after the matching JavaScript stages, and every job reports every failure in one run
// (cargo test --no-fail-fast, check steps that run unless the run was
// cancelled, and cargo fmt, clippy and doc in separate steps).
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { lineCoverage, mergeLcov, renderLcov } from '../../scripts/merge-lcov.mjs';
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
  assert.deepEqual(triggers(text).sort(), ['pull_request', 'push', 'release']);
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

test('Rust test jobs install the JavaScript dependencies used by the shared reports', () => {
  const rust = jobs('rust.yml');
  for (const id of ['test', 'coverage', 'fresh-merge']) {
    const job = rust.get(id);
    const setup = job.steps.findIndex((step) => /actions\/setup-node@/u.test(step.text));
    assert.ok(setup >= 0, `${id} sets up Node.js`);
    const install = job.steps.findIndex((step) => /\bnpm ci\b/u.test(step.run));
    if (id === 'fresh-merge') {
      const script = read('rust/scripts/simulate-fresh-merge.sh');
      assert.match(script, /npm ci --ignore-scripts/u);
      assert.ok(script.indexOf('npm ci --ignore-scripts') < script.indexOf('cargo fmt --all'), 'the merged tree installs before checks');
    } else {
      assert.ok(install > setup, `${id} installs JavaScript dependencies after Node.js setup`);
      assert.match(job.steps[install].text, /working-directory: js/u);
      const run = job.steps.findIndex((step) => /cargo (?:test|llvm-cov)/u.test(step.run));
      assert.ok(run > install, `${id} installs before executing Rust tests`);
    }
  }
});

test('both self-translation acceptance suites install Clippy for the generated Rust', () => {
  const acceptance = jobs('ci.yml');
  for (const id of ['javascript-suite', 'rust-suite']) {
    const setup = acceptance.get(id).steps.find((step) => /dtolnay\/rust-toolchain@/u.test(step.text));
    assert.ok(setup, `${id} sets up Rust`);
    assert.match(setup.text, /components:.*\bclippy\b/u, `${id} installs clippy-driver`);
  }
});

test('the live RML head audit runs on main as a non-blocking post-merge report', () => {
  const report = jobs('js.yml').get('test').steps.find((step) => /issue-195-rml-pr184\.mjs --online/u.test(step.run));
  assert.ok(report, 'the live head comparison remains executable');
  assert.match(report.text, /github\.ref == 'refs\/heads\/main'/u);
  assert.match(report.text, /github\.event_name != 'pull_request'/u);
  assert.match(report.text, /!cancelled\(\)/u);
  assert.match(report.text, /continue-on-error: true/u);
  assert.match(report.text, /working-directory: js/u);
  assert.match(report.text, /GITHUB_TOKEN:/u);
});

test('fresh merge keeps build artifacts out of its cache and optimizes the unchanged full test suite', () => {
  const job = jobs('rust.yml').get('fresh-merge');
  const cache = job.steps.find((step) => /actions\/cache@/u.test(step.text));
  assert.ok(cache, 'fresh merge caches its registry dependencies');
  assert.match(cache.text, /~\/\.cargo\/registry/u);
  assert.match(cache.text, /~\/\.cargo\/git/u);
  assert.doesNotMatch(cache.text, /(?:^|\n)\s*(?:rust\/)?target(?:\/|\s|$)/u, 'large build artifacts do not consume the cache-save deadline');
  assert.match(job.header, /timeout-minutes: 20/u);
  const script = read('rust/scripts/simulate-fresh-merge.sh');
  assert.match(script, /with-cache-cleanup\.mjs --event test -- cargo test --no-fail-fast --all-features/u);
  assert.match(script, /profile\.test\.package\.meta-language\.opt-level=3/u);
  assert.match(script, /profile\.test\.package\.meta-language\.debug-assertions=true/u);
  assert.match(script, /profile\.test\.package\.meta-language\.overflow-checks=true/u);
});

test('both acceptance suites verify the complete published self-translation reports', () => {
  const acceptance = jobs('ci.yml');
  assert.ok(needs(acceptance.get('javascript-suite')).includes('self-translation-report'));
  for (const id of ['javascript-suite', 'rust-suite']) {
    const steps = acceptance.get(id).steps;
    const download = steps.findIndex((step) => step.name === 'Download the complete self-translation reports');
    const execute = steps.findIndex((step) => step.name === 'Produce the stage evidence');
    assert.ok(download >= 0 && execute > download, `${id} downloads all reports before observing evidence`);
    assert.match(steps[download].text, /pattern: self-translation-report-\$\{\{ github.sha \}\}-\*/u);
    assert.match(steps[download].text, /path: issue-195-artifacts\/self-translation-report/u);
    assert.doesNotMatch(steps[download].text, /merge-multiple: true/u, 'each shard retains its JSON and Markdown pair');
    assert.match(steps[execute].text, /ISSUE_195_SELF_TRANSLATION_REPORT_DIRECTORY:/u);
  }
});

test('the acceptance workflow runs each Rust stage only after the matching JavaScript stage passes', () => {
  const acceptance = jobs('ci.yml');
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

/** The jobs `id` needs, directly or through the jobs it needs. */
function ancestors(parsed, id, seen = new Set()) {
  for (const parent of needs(parsed.get(id))) {
    if (!seen.has(parent)) {
      seen.add(parent);
      ancestors(parsed, parent, seen);
    }
  }
  return seen;
}

const ACCEPTANCE_JOBS = [
  'candidates', 'consumers', 'rml-workloads', 'formal-ai-workloads', 'runtime-parity', 'native-javascript', 'native-rust',
  'native-translations', 'javascript-suite', 'rust-suite', 'delivery', 'post-merge', 'acceptance',
];

test('the acceptance stages are ci.yml jobs that start only after every JavaScript job passed', () => {
  const ci = jobs('ci.yml');
  for (const id of ACCEPTANCE_JOBS) {
    assert.ok(ci.has(id), `ci.yml has the acceptance job ${id}`);
    assert.ok(ancestors(ci, id).has('js'), `${id} needs the js job`);
    // always() or !cancelled() would run the job after a failed js job; such a job must test the js result itself.
    const own = condition(ci.get(id));
    if (/always\(\)|cancelled\(\)/u.test(own) && id !== 'acceptance') {
      const gated = /needs\.js\.result == 'success'/u.test(own) ||
        needs(ci.get(id)).some((parent) => new RegExp(`needs\\.${parent}\\.result == 'success'`, 'u').test(own) && parent !== 'js');
      assert.ok(gated, `${id} runs after a failed js job: ${own}`);
    }
  }
  // Only a release, whose js.yml runs by its own trigger, starts the candidates without the js job.
  assert.match(condition(ci.get('candidates')),
    /needs\.js\.result == 'success' \|\| \(github\.event_name == 'release' && needs\.js\.result == 'skipped'\)/u);
  assert.match(condition(ci.get('js')), /github\.event_name != 'release'/u);
  assert.ok(!triggers(workflow('ci.yml')).includes('workflow_run'));
  const workflows = readdirSync(path.join(root, '.github/workflows'));
  assert.ok(!workflows.includes('issue-195-acceptance.yml'), 'no separate acceptance workflow starts beside the JavaScript jobs');
  observe('I195-CI-ACCEPTANCE-AFTER-JAVASCRIPT', ['acceptanceJobsNeedJavaScript', 'noSeparateAcceptanceWorkflow'],
    'the acceptance stages are ci.yml jobs that start only after every JavaScript job passed');
});

test('a failed JavaScript job is one aggregate gate error naming the failed jobs', () => {
  const aggregate = jobs('ci.yml').get('acceptance');
  assert.match(aggregate.header, /^ {4}if: \$\{\{ always\(\) \}\}$/mu);
  const [guard] = aggregate.steps;
  assert.equal(guard.name, 'Require delivery candidates');
  assert.match(guard.run, /JAVASCRIPT_RESULT" = failure/u);
  assert.match(guard.run, /actions\/runs\/\$GITHUB_RUN_ID\/attempts\/\$GITHUB_RUN_ATTEMPT\/jobs/u);
  assert.match(guard.run, /startswith\("JavaScript \/ "\)/u);
  // One ::error:: per branch and one exit: the skipped stages add no error of their own.
  assert.equal(guard.run.match(/::error::/gu).length, 2);
  assert.match(guard.run, /^\s+else$/mu);
  assert.equal(guard.run.match(/exit 1/gu).length, 1);
  assert.match(aggregate.header, /actions: read/u);
  observe('I195-CI-SKIPPED-STAGE-ONE-GATE-ERROR', ['failedJavaScriptIsOneGateError'],
    'a failed JavaScript job is one aggregate gate error naming the failed jobs');
});

test('the Rust coverage shards run the test suites and their merged report counts the hits of every shard', () => {
  const rust = jobs('rust.yml');
  const shards = rust.get('coverage');
  const report = rust.get('coverage-report');
  assert.match(shards.header, /suite: \[grammar, inference, generative, translation, remaining\]/u);
  assert.deepEqual(needs(report), ['coverage']);
  assert.match(report.steps.map((step) => step.run ?? '').join('\n'), /merge-lcov\.mjs --output rust\/lcov\.info --fail-under-lines 84\.30/u);
  const shard = (lines, hit) => `TN:\nSF:/src/a.rs\nFN:1,f\nFNDA:${hit},f\nFNF:1\nFNH:${hit > 0 ? 1 : 0}\nBRDA:2,0,0,${hit > 0 ? hit : '-'}\n${lines}\nend_of_record\n`;
  const files = mergeLcov([shard('DA:1,0\nDA:2,3\nDA:3,0', 0), shard('DA:1,2\nDA:2,0\nDA:3,0', 2)]);
  assert.deepEqual(lineCoverage(files), { covered: 2, total: 3, percent: 200 / 3 });
  assert.equal(renderLcov(files), 'TN:\nSF:/src/a.rs\nFN:1,f\nFNDA:2,f\nFNF:1\nFNH:1\nBRDA:2,0,0,2\nBRF:1\nBRH:1\nDA:1,2\nDA:2,3\nDA:3,0\nLF:3\nLH:2\nend_of_record\n');
  assert.deepEqual(lineCoverage(mergeLcov([renderLcov(files)])), lineCoverage(files));
});
