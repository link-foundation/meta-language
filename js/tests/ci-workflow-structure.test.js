// The CI workflow structure: the Rust workflow and the acceptance stages run
// only after the JavaScript workflow passes, the Rust acceptance stages run
// only after the matching JavaScript stages, and every job reports every failure in one run
// (cargo test --no-fail-fast, check steps that run unless the run was
// cancelled, and cargo fmt, clippy and doc in separate steps).
import { test } from 'node:test';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { lineCoverage, mergeLcov, renderLcov } from '../../scripts/merge-lcov.mjs';
import { partitionGroupFiles, partitionTests, testFiles, TEST_GROUPS } from '../scripts/test-groups.mjs';
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
  ['js.yml', ['package-checks', 'test']],
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
    assert.match(setup.text, /components:.*\brustfmt\b/u, `${id} installs the formatter used by generated-source freshness checks`);
  }
});

test('the JavaScript evidence matrix retains every group and file once with disjoint outputs under the original deadline', () => {
  const acceptance = jobs('ci.yml');
  const suite = acceptance.get('javascript-suite');
  const entries = [...suite.header.matchAll(/- group: ([a-z-]+)\n\s+shard: ([0-9]+\/[0-9]+)\n\s+partition: ([0-9]+-of-[0-9]+)/gu)]
    .map(([, group, shard, partition]) => ({ group, shard, partition }));
  const expected = Object.keys(TEST_GROUPS).flatMap(group => {
    const count = group === 'grammar-native' ? 3 : group === 'conformance' ? 2 : 1;
    return Array.from({ length: count }, (_, index) => ({ group, shard: `${index + 1}/${count}`, partition: `${index + 1}-of-${count}` }));
  });
  assert.deepEqual(entries, expected);
  const files = partitionTests(testFiles()).groups;
  const executed = entries.flatMap(({ group, shard }) => partitionGroupFiles(files[group], shard));
  assert.equal(new Set(executed).size, executed.length);
  assert.deepEqual(executed.sort(), Object.values(files).flat().sort());
  assert.match(suite.header, /timeout-minutes: 60\n/u);
  assert.match(suite.header, /fail-fast: false\n/u);
  assert.match(suite.header, /group: issue-195-stage-\$\{\{ github.ref \}\}-javascript-suite-\$\{\{ matrix.group \}\}-\$\{\{ matrix.partition \}\}/u);
  const execute = suite.steps.find(step => step.name === 'Produce the stage evidence');
  assert.match(execute.text, /JAVASCRIPT_GROUP: \$\{\{ matrix.group \}\}/u);
  assert.match(execute.text, /JAVASCRIPT_SHARD: \$\{\{ matrix.shard \}\}/u);
  assert.match(execute.run, /--javascript-group "\$JAVASCRIPT_GROUP"/u);
  assert.match(execute.run, /--javascript-shard "\$JAVASCRIPT_SHARD"/u);
  const upload = suite.steps.find(step => step.name === 'Upload the stage outputs');
  assert.ok(upload.afterFailure);
  assert.match(upload.text, /name: issue-195-stage-javascript-suite-\$\{\{ matrix.group \}\}-\$\{\{ matrix.partition \}\}-\$\{\{ github.sha \}\}/u);
  assert.match(upload.text, /issue-195-results\/work\/stage-\*/u);
  assert.ok(needs(acceptance.get('rust-suite')).includes('javascript-suite'));
  assert.match(condition(acceptance.get('rust-suite')), /needs.javascript-suite.result == 'success'/u);
  const download = acceptance.get('acceptance').steps.find(step => /pattern: issue-195-stage-/u.test(step.text));
  assert.match(download.text, /pattern: issue-195-stage-\*-\$\{\{ github.sha \}\}/u);
  assert.match(download.text, /merge-multiple: true/u);
});

test('the live RML head audit runs on main as a non-blocking post-merge report', () => {
  const report = jobs('js.yml').get('package-checks').steps.find((step) => /issue-195-rml-pr184\.mjs --online/u.test(step.run));
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

test('required JavaScript CI retains bounded memory regressions and actual target-tool conversion cycles', () => {
  const partition = partitionTests([
    'issue-195-evidence-memory.test.js', 'issue-195-parse-memory-budget.test.js', 'parser-memory.test.js',
  ]);
  assert.deepEqual(partition.unassigned, []);
  assert.deepEqual(partition.ambiguous, []);
  assert.deepEqual(partition.groups['grammar-scaling'], ['issue-195-parse-memory-budget.test.js', 'parser-memory.test.js']);
  assert.deepEqual(partition.groups.tooling, ['issue-195-evidence-memory.test.js']);
  const parsed = jobs('js.yml');
  const groupRun = parsed.get('tests').steps.find((step) => step.name === 'Run the test group');
  assert.ok(groupRun.afterFailure, 'test groups report failures even if the oracle build failed');
  const oracle = parsed.get('tests').steps.find((step) => /build-agda-layout-oracle\.mjs/u.test(step.run));
  assert.ok(oracle, 'the independent Agda scanner oracle is built in CI');
  assert.match(oracle.text, /contains\(fromJSON\('\["grammar", "grammar-native", "grammar-recovery"\]'\), matrix\.group\)/u);
  const evidence = jobs('ci.yml').get('javascript-suite');
  const rebuild = evidence.steps.findIndex((step) => /build-agda-layout-oracle\.mjs/u.test(step.run));
  const suite = evidence.steps.findIndex((step) => step.name === 'Produce the stage evidence');
  assert.ok(rebuild >= 0 && suite > rebuild, 'acceptance rebuilds the same independent oracle before its corpus tests');
  assert.ok(evidence.steps[suite].afterFailure, 'other acceptance tests still report failures if the oracle build fails');
  const conversion = parsed.get('package-checks').steps.find((step) => /run-grammar-interchange-tools\.mjs/u.test(step.run));
  assert.ok(conversion?.afterFailure, 'the real target compiler runs despite other package check failures');
  assert.doesNotMatch(conversion.text, /continue-on-error|github\.event_name != 'pull_request'/u);
  const report = parsed.get('package-checks').steps.find((step) => step.name === 'Upload the grammar conversion target-tool report');
  assert.ok(report?.afterFailure, 'the target-tool report is retained after check failures');
  assert.match(report.text, /grammar-interchange-tools-\$\{\{ github\.sha \}\}/u);
  const lark = parsed.get('package-checks').steps.find((step) => /run-grammar-interchange-lark\.mjs/u.test(step.run));
  assert.ok(lark?.afterFailure, 'the independent Python target still runs after other failures');
  assert.doesNotMatch(lark.text, /continue-on-error/u);
  const install = parsed.get('package-checks').steps.find((step) => /pip install/u.test(step.run));
  assert.match(install.run, /--no-cache-dir lark==1\.3\.1/u);
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

// Frozen from the 115 original package steps at 0203b814, before partitioning.
// Hashes include all non-comment fields, commands, conditions and artifact
// identities. Only the added matrix selector is removed for comparison.
const PACKAGE_GROUPS = ['native-imports', 'native-fixtures', 'grammar', 'contracts', 'generated'];
const ORIGINAL_PACKAGE_STEPS = [
  ["- uses: actions/checkout@v7", '6a4e22587a1c1ab9a1a80858c0b1d31d26580233f884253c583c43c5e5452297', 'all'],
  ["Simulate fresh merge with base branch", '7a5dda6e63b412064d418cdd82ae8a57f69b1f232aa50f2e72fe0a8f4461467e', 'all'],
  ["Setup Node.js", 'a00b8a26d76b43019bc515f8518aeebd94ffbe788e1850e8ce14f21568388401', 'all'],
  ["Install dependencies", 'dbb21dc1f95f1c3eecf831b6d7186dca393ec9d7b0fc22835962ad38f9beb966', 'all'],
  ["Provision the pinned external grammar check inputs", '68537213f39aa5039d3289d7d24030876cb4bd6c69ffd21c17fa0a3fc7d54c9f', 'all'],
  ["Check that every test file is in exactly one test group", 'd86ad8c3d885ff0b6451c5eae23a2691e78c300c842b9aeb6022e6e74be99c62', 'package'],
  ["Check the cache cleanup policy", '8c28eb925edad8809b0ce38fdf2a6ef8978bba5d5af956f7d03687cca547f39f', 'package'],
  ["Install and verify the development hook in this clone", 'df045c35575f3ca75c48aa8ff1afd1fa30999ea709b54d4620ad0f9f73e996bb', 'package'],
  ["Check Rust/JavaScript parity manifest", '8ef63613bc4de09766c029348580d8ee57ec213a479ca52ba7ef0157094993a8', 'package'],
  ["Check that every JavaScript export maps to a public Rust item", 'e2f916867e2b2303100a05ff65a284aba27e75adb4faaf0219b75e4d6ca2e734', 'package'],
  ["Check grammar-derived default CST expectations", '059013c3ad5f9c76fb0f856971a70bb80a84fca56b591a413a44d62026717bc6', 'grammar'],
  ["Check the official links-notation LiNo grammar cases", 'ad2222a08bea780fb758c5541a06ac7d8be8b1085521f9ce830955f4c75e14ea', 'grammar'],
  ["Check the pdf-lib PDF grammar cases", '8e984c531c9a6ce4a3db3231b58ab76bfdfbdc08d375b2f2603bca85beef0208', 'grammar'],
  ["Check the native grammars imported from their pinned sources", '94f6174e65623595d03c130545a2da2d9cb0ad399eb34b59e492d7d403336730', 'native-imports'],
  ["Check shared source prefix operand precedence fixtures", '3a7eb08547de0196632320695a716aa048d707557c3356662c2080c9367d03e5', 'grammar'],
  ["Check the pinned Ruby pattern fixtures against their independent oracle", 'e5f8db67e6a32ddc6225ef9118aa28cda7e70100e067fa0f82861a6078876181', 'grammar'],
  ["Check the pinned Ruby expression syntax fixtures against their independent oracle", '6cf0be5116ecd840ad81b8e5cefb7f6b9a0bbc852479149a7251fd5ea585aaf7', 'grammar'],
  ["Check the pinned Ruby heredoc call fixtures against their independent oracle", 'cb63b8946324d3fba5d4498c3d0ceda628213f88ad7757daf25cb33742dd4415', 'grammar'],
  ["Check the pinned Ruby method declaration fixtures against their independent oracle", 'd859a78917f80a13a938b842a080a15e2feba72d57d7e6aac4b7a8fe0d50b704', 'grammar'],
  ["Check the pinned Ruby literal CI fixtures against their independent oracle", '7be322734674537094e408cf872b7592ecfc49f18f2e164c073967b30f8c2202', 'grammar'],
  ["Check the native merged grammar fixtures against their oracles", '6318830c094b3b68ec9d2cbba8e099312f5fba2b06a8a896f05d75719b4723b8', 'native-fixtures'],
  ["Check the independent built-in grammar CST expectations", '5077c37db2c4b61992b86806b19117ba864a4ca480da7c77d840ddec9165d6b1', 'grammar'],
  ["Check the shared language catalog", 'bf377cb00efaf8946019c91b1948c3ede9fd65fff39a4524a25a4365a4164ffb', 'grammar'],
  ["Check the shared concept records", '93498bfc70f856a4510df6b94cfcf2ab296695e041786be7bb0239342f7908e7', 'grammar'],
  ["Check the native grammar concept reuse report", '1beb05c093080780c6d7754017f7db2f629e75eb1fca6c75df1216210e950a3a', 'grammar'],
  ["Check the native grammar merge quality evidence", 'bfd119c62449d3b7d3cfcbc0b34700f9092d45de9a72e8d371d3360380be1459', 'grammar'],
  ["Run grammar conversion cycles under the actual Peggy target tool", 'a8a5314a32ec08dec0287c6be02fc80900762d323608d89c787744c3cfbd4752', 'grammar'],
  ["Upload the grammar conversion target-tool report", '881abc338e6076f62da67b3ac8cb234d580b00e3e7ed76c1e6f160ff181088e7', 'grammar'],
  ["Install the pinned independent Lark grammar target tool", 'bc72966ab6234bfce464c2ffdaf0c3a712900a865bd9a7108306050dbb5412ab', 'grammar'],
  ["Run grammar conversion cycles under the actual Lark target tool", 'ccae56344c3e39fda3a4f30b2193c4a6bc106e6f1e887686a268edbf64d88536', 'grammar'],
  ["Upload the independent Lark conversion report", '107c2e83fd37637330c72fd246a935cd6630973041459e7ce70d630eb1ed042b', 'grammar'],
  ["Check the foundation models are neutral data", 'a0cc02d97df59e140b08a31b5d6a231de5983cc28a6a94e9356732712323d6b9', 'contracts'],
  ["Check issue 195 atomic requirement manifest", '8c415bcde8ebac858cdbb2dbbfdb98c97592e2814198aa273249ce04128dfdc0', 'contracts'],
  ["Check issue 195 source register against the ledger", 'd99124f24385272ec9fea89f6214f30ee50022c3db125527d818fa8eaaddc940', 'contracts'],
  ["Compare the issue 195 source register with the live discussion (post-merge report)", 'a0b46920deead512c9a34f9474b82a5d724ffa24b44fe6725cbb151742413d4e', 'contracts'],
  ["Check the downstream consumer matrix against the ledger and the tests", '298f5c36ea389207cc6370b72d8dad058d96ee5ef100625a5eb85bc00b4740f5', 'contracts'],
  ["Compare the pinned RML audit with the live pull request head (post-merge report)", 'defad42d6ab196b671582bbb5b61cbecaf9f660220cee59fafa9775140caac6e', 'contracts'],
  ["Check that every dependency, toolchain, action and image is inventoried at its current stable release or records why not", '743f6e960ad7411040bdfe1a29775fad67441a63b16926f5ecef16f6338b1a98', 'contracts'],
  ["Check readable English names against WordNet, the concept records and the source inventory", '64f82c67da7c2364d392a7a0104d8cec74d34cf1de142a1a7651b4c7c5b6e9ca', 'contracts'],
  ["Check associative data storage and necessary external JSON interfaces", 'f4d3240e8f655c72a10049e149b1928ffd9beaa8dcde98c0eb9817d977bdd98a', 'contracts'],
  ["Check that the documentation agrees with the requirement ledger", 'ebc412ddc6fabaac392252455e78e5506d9fe91ea75184247299736b7e95cbee', 'contracts'],
  ["Check the shared self-translation cases", '78220bdba2826fef3cd7752ff411dd71d72ac48c39d7a042a6ba220f1c71dbae', 'contracts'],
  ["Setup Rust formatting for generated frontend rules", '89a26c3870dd5e938703e0a25af2200e9ea81c969435752952c1ffb2e1696812', 'generated'],
  ["Check the generated Rust frontend rules against JavaScript", '0d36e6762aac4568957d559c7653e19483d5983f82b105177c4eed173d97969d', 'generated'],
  ["Check the generated global concept rules against JavaScript", '427c3f7cceef9385adc80d9630ee558660157561c282ef1493f92694562cfba3', 'generated'],
  ["Check the generated discovery integration rules", 'bf380380183cf89682722c79c77da05f6f4b5b95f2b19fbc705b03d53792c566', 'generated'],
  ["Check shared discovered concept graph fixtures", '88cec582b039d829e1a242f924cd6e2a36bd371321b48bd17a99c444b04865ad', 'generated'],
  ["Check generated concept view admission rules", '61b611c2998e097166d5b0610d2031e09d7eaede6268c9f00430df86926d09fa', 'generated'],
  ["Check shared ontology and taxonomy view fixtures", '0d86785636762415cef4443c9818c7ad46aeef48270cef99ee9b85ebe9525fd6', 'generated'],
  ["Check generated binary link network rules", 'de9adcdf5f4b415a91dbf9f1ae37eb77c27005637beec54fbd2f4722a8b8b0c7', 'generated'],
  ["Check shared binary link network fixtures", 'bdd95da9e1788e41ebecfac7cacdd360683de449bee2c1f79f1268c3af1ce53c', 'generated'],
  ["Check the generated Resource Description Framework rules", '909d6b2dc900debd3a53987792bf62f9ca5bf4c1f27b24e9ac5e60f8daf16b3d', 'generated'],
  ["Check shared Resource Description Framework graph fixtures", '881ebd25318e8a836e06ea82028ed8a523a2a48a7ab44797696a4750c5836b9b', 'generated'],
  ["Check generated resource triple syntax character rules", '13868a2e5fc4b0b8ff1c59f8acac9330345d66fc483917a2728a1e5f35de3d1b', 'generated'],
  ["Check generated resource triple syntax token rules", 'c79d0ed7c7565b3560adba146cd1da1f7cd32ba218113b45febf3a74b18bd7e7', 'generated'],
  ["Check shared resource triple syntax fixtures", '2379490a6f8fa9c5bb12e9b0f7b16c45dc06252dfb126c0e320cc0ba928dc26e', 'generated'],
  ["Check the generated grammar merge decisions against JavaScript", 'd9b443b23b573c8ed441ccb4f990399ecadf1c4c06b783eaf4f10adb6d8dd98f', 'generated'],
  ["Check the generated structural type rules against JavaScript", 'c1f1e4e21d3ae108f8b28504d4770c0feaf0ccc647d5bf8f4cb98ca3cdd0558c', 'generated'],
  ["Check the generated primitive value rules against JavaScript", '63e09201ebe7121a732098fb771c54a10a697e7fe918c728080c256feb2c3595', 'generated'],
  ["Check the shared Rust array runtime artifacts", '2048252c4902d4d84d2083ddcc326039c00e150ff25267fb10227323ffaf6eec', 'generated'],
  ["Check the generated TypeScript erasure rules against JavaScript", '4850b08b3ac35764c64bb083b15fd8ee83225fbcc1fd491afb78c38e870ed815', 'generated'],
  ["Check the generated typed program link rules against JavaScript", '0adf47f687b5ef9c012e255cc46555265f6aedd54a947d9d1db3ccc5b3761ba4', 'generated'],
  ["Check the generated Rust source indirection rules against JavaScript", '5644ba8811c1e26235a5c0a979bb308034fcf69711f4c9e135116ca1e05a7080', 'generated'],
  ["Check shared scoped Rust lifetime fixtures", '52116f7f1565bbb45784b50c0939c2e35a95cb9c379b45665d5c5844661d7a08', 'generated'],
  ["Check generated Rust array admission rules", '6a5f279c7c8d532c4b21af9518c63d6a04edd8e5d4d9a6607316ff7204436df0', 'generated'],
  ["Check generated Rust floating point predicate rules", '7c7c8667bafaf52cdc521552f5e7097c92099c6546baabaa8f41837e4dec32ba', 'generated'],
  ["Check shared Rust floating point predicate fixtures", '9d61ddc8802544fbd6bcf2ed5b36ec4c089bf2f0a51eb28ae13b1b6af861003d', 'generated'],
  ["Check shared Rust floating point operation fixtures", '403cd96a86af2ece1c169fc309f617689d37cc1b2cbd867e13340f734984593e', 'generated'],
  ["Check shared Rust floating point constant fixtures", 'fd9a7e7c3b1af2df61695e7d5a98be72dbc0b14588afb2ee74610f6b1a408ba1', 'generated'],
  ["Check generated Rust guarded function completion rules", '023ff2451988268ff75649d7d10d9ec28bc2c88800ce0a7f4e8673f4a4724314', 'generated'],
  ["Check shared Rust guarded function completion fixtures", 'b047d95cf4bf0fcd954e32f77e7a2d71c431ec6e74f4d3aab42f1b880d5be58d', 'generated'],
  ["Check shared Rust guarded completion panic fixtures", 'e0ebc776fcac7617e0f7a9af3592ec6b995c87e4d60cc54c8c8c5c7be8080283', 'generated'],
  ["Check generated read-only Rust scalar parameter rules", '2174e42702249c55675143e42278ea54015f29e44930d238cc5f8e487f7b6046', 'generated'],
  ["Check shared read-only Rust scalar parameter fixtures", '88d21f6453939675f82a0d2e2ce9c329cb69d4e1b19fe799d3fdde5a8efc3f6f', 'generated'],
  ["Check shared Rust array source fixtures", 'ef733f7a2d64d5c0373b4c41a421ccb3f18cc3a086c526d70e2eb450a68245e0', 'generated'],
  ["Check shared fresh Rust scalar vector result fixtures", '144024fb01c9758bea3752ad155db714428be4139ea9f6b5e1b8f38795c5822a', 'generated'],
  ["Check generated Rust string continuation rules", '7c36a68359b0df2c39999997bafb3e0eee0c55361b34be23bd76318a84cf2e30', 'generated'],
  ["Check shared Rust string continuation fixtures", 'c77626353ef6da9217fc8336cf131407f6752e62e1741fe9f5f97e702e323852', 'generated'],
  ["Check shared TypeScript documentation pattern fixtures", '1c6672496ee470466de5198be3813f79bcea78b01bf0d37eb99f526049d277be', 'generated'],
  ["Check generated formal function selection rules", '41e9db2c8c2b7678ad4fc2e5a321f97c2fb2ed329e18438340c8d44919d76467', 'generated'],
  ["Check shared formal function definition fixtures", 'f7f11b9d3532266e973d226a9acc9eee89c16864baf30f7b56bc70cbba1339b3', 'generated'],
  ["Check generated shared numeric function rules", '0a56465641ce64e1eaa71a22c091899daca66636fb7493510e430ed1abc60618', 'generated'],
  ["Check generated Rust source signature rules", '8c6f6a49781633dcbfd8ac8c823c7bc46bdc092626a936d49b2e5e0699493d57', 'generated'],
  ["Check shared Rust source signature fixtures", 'b845fbc896b64d507c7c0958320e73b222bc7c4f65e7121341427738ef6abba6', 'generated'],
  ["Check generated standard Rust readonly query rules", 'f32c4f3ac3cd579f1aa7ef42f43a7024064aa487d6c71e1b3dc488de59c48c08', 'generated'],
  ["Check standard Rust readonly query fixtures", '3766582f3a6a9c907e2d4da31227b68908c8496d29c77fb20b176da86f5be370', 'generated'],
  ["Check generated shared scalar loop rules", 'fcb199dc5ab3f16671aa60443e53cb6e4111f31b42942a382eabf5efe2c3b051', 'generated'],
  ["Check shared scalar loop source fixtures", '2cd1889348e86780d24801a3d79a30b4fadb6e64637a03f3ffe3034e98bc909d', 'generated'],
  ["Check owned declaration scalar loop source fixtures", '834f7e55d07dd2db43be69288c10a59cdbe63a91accb518480d80dbb192c7da6', 'generated'],
  ["Check generated primitive borrowed Rust header rules", '37e8211eef969bf57d80dd8a0ec3a1a74f6acbd3fd0f71d77a0952355ccdff09', 'generated'],
  ["Check primitive borrowed Rust header fixtures", '609f4077846d7ddf0adcc983ff85a761afeb26dfe5f83a0c55e8135f6f6264cb', 'generated'],
  ["Check shared numeric function definition fixtures", '3789f91999dd3ed2c8bc8dfd7c927c6c2d46f2786a142fda0a05aecae804abc9', 'generated'],
  ["Check strict public declaration consumers", 'e2cedade5e686eb364bc1a0d7dc5c11403d241fbd8263516f81bd4b1d9169b66', 'generated'],
  ["Authenticate reviewed generated-stage migration refreshes", '6c64037d1a235efadf2fcb59f3a24cfe5203e080b0595d58e0685a52e5ade0a7', 'generated'],
  ["Check the generated Rust source product rules against JavaScript", '07b2f60a9d1f0ec06362cf76ac887fcb6b161f5975af2f286fb5da5fb907814f', 'generated'],
  ["Check the generated immutable product emission rules against JavaScript", '112622ba84c2bb51ec300bbf51344e4dbff66348030cb175aa24ac7cd3aacace', 'generated'],
  ["Check the generated closed product source proofs against JavaScript", '6b6190438f3fcbe2c9f39bb3fe116cddfea859640713149a5d7dc8b32c3681f9', 'generated'],
  ["Check shared source-free typed program fixtures", 'e80f0c70702c2191fa18c49f7091099f9da9257cedad5c679afeab5aa21ade67', 'generated'],
  ["Check shared editable algebraic program fixtures", 'f74a45a955400a041289a10d2972e555c5b89a28b64bd0b149146d3a75e6b001', 'generated'],
  ["Check shared editable immutable product fixtures", '44522de88c52db5272181e4c2689fdacb9863782d813817b816021830313f589', 'generated'],
  ["Check shared product unit value fixtures", '777f84240b11f5c4efebe44240952a9e2652699b4c0727a600a61ca4eaa651df', 'generated'],
  ["Check the associative data tooling fixture", 'de9b9d3daa601e773b7f7efab10f3bcf1fbdede5164b9744ca5494022785baea', 'generated'],
  ["Check the recorded original structured data conversion revisions", '886d3b0182002c6708f8f9a7bdf748e9a9495299a60e01b2d249e070cf07dbff', 'generated'],
  ["Check shared formal definition and certificate fixtures", '64293f4f1cec9ec9c1fe3dc45306c347872285e4fb67eb7aa0d8f6f816553f33', 'generated'],
  ["Check generated formal relation identities", 'd04cb31ede3e4af1553d796fa81bf7b5cce15974477eb423724379e0eb1281f2', 'generated'],
  ["Check shared formal relation identity fixtures", 'c8b4429217c2c4629da8f67fc4594b7e8d36fbf3b8a288b504726b4e49e9b958', 'generated'],
  ["Check generated frontend rule compilation and style", 'cf05b1ee2e99c192428c0ee8682f96cfedf2e2ec104dd9e3ce126a736aced6f2', 'generated'],
  ["Check generated global concept rule compilation and style", '3015f53f809f5af8a779f252f83b4f07787ae150ebc9f8ad3c2b04bcffb1d9da', 'generated'],
  ["Retain exact generated sources and compiler feedback", '705c0043ad9380093be490b7ec05fc59a2b6a6a1c139925924f0543541a2388f', 'generated'],
  ["Compare the pull request description and the release reports with the requirement ledger (post-merge report)", 'cc0eb0d2657c500b9ca55c5b0530543470eb90b0b6b7be53468e52c073c1c5c0', 'contracts'],
  ["Verify npm package contents", '5ef9d560fc559e9c27ed86c31c47611cd84d159ecb32567e744ae62794be2b62', 'package'],
  ["Scan repository for secrets", '396152974e6bdb6f7111f05e062771c04b2d9fa2d56e2b670a8ce0a1eeec7f0a', 'package'],
  ["Remove the temporary external grammar inputs", '14956a39689b2e6ba4fe32b4268ec81eee06db295db57be0926a55a934f65efd', 'all'],
  ["Clean regenerable caches", 'f958ba58a92bd59affdcb579b84fd34e6eb61567c3f40709c773ed163a5b7d5b', 'all'],
];

function packageStepSignature(step) {
  const text = step.text.replace(/ && matrix\.group == '[a-z-]+'/gu, '')
    .split('\n').filter(line => line.trim() && !/^\s*#/u.test(line))
    .map(line => line.trimEnd()).join('\n');
  return createHash('sha256').update(text).digest('hex');
}

function checkPackageStepConservation(parsed) {
  const expected = new Map(ORIGINAL_PACKAGE_STEPS.map(([name, hash, owner]) => [name, { hash, owner }]));
  const found = new Map();
  const shared = new Set();
  for (const id of ['package-checks', 'test']) {
    for (const step of parsed.get(id).steps) {
      if (step.name === 'Require every JavaScript test group') continue;
      const baseline = expected.get(step.name);
      assert.ok(baseline, `unexpected package step: ${step.name}`);
      if (baseline.owner === 'all') {
        const key = `${id}:${step.name}`;
        assert.ok(!shared.has(key), `shared package step duplicated: ${key}`);
        shared.add(key);
      }
      const group = /matrix\.group == '([a-z-]+)'/u.exec(step.text)?.[1];
      const owner = id === 'test' ? 'package' : group ?? 'all';
      assert.equal(owner, baseline.owner === 'all' ? (id === 'test' ? 'package' : 'all') : baseline.owner,
        `package step owner changed: ${step.name}`);
      assert.equal(packageStepSignature(step), baseline.hash, `package step semantics changed: ${step.name}`);
      if (id === 'package-checks') {
        assert.equal((step.text.match(/matrix\.group/gu) ?? []).length, baseline.owner === 'all' ? 0 : 1,
          `package step has an ambiguous group selector: ${step.name}`);
        assert.ok(group === undefined || PACKAGE_GROUPS.includes(group));
      }
      const executions = id === 'package-checks' && !group ? PACKAGE_GROUPS.length : 1;
      found.set(step.name, (found.get(step.name) ?? 0) + executions);
    }
  }
  for (const [name, baseline] of expected) {
    if (baseline.owner === 'all') {
      for (const id of ['package-checks', 'test']) assert.ok(shared.has(`${id}:${name}`), `shared package setup/cleanup missing: ${id}:${name}`);
    }
    assert.equal(found.get(name), baseline.owner === 'all' ? PACKAGE_GROUPS.length + 1 : 1,
      `package step omitted or duplicated: ${name}`);
  }
}

test('package partitions conserve every original check, condition and artifact once under the ten-minute deadline', () => {
  const parsed = jobs('js.yml');
  const checks = parsed.get('package-checks');
  const groups = /^ {8}group: \[([^\]]+)\]/mu.exec(checks.header)?.[1].split(',').map(value => value.trim());
  assert.deepEqual(groups, PACKAGE_GROUPS);
  assert.match(checks.header, /fail-fast: false/u);
  assert.match(checks.header, /group: check-js-\$\{\{ github.ref \}\}-javascript-package-checks-\$\{\{ matrix.group \}\}/u);
  assert.deepEqual(needs(checks), ['tests', 'report-assertions']);
  assert.match(condition(checks), /!cancelled\(\)/u);
  for (const id of ['package-checks', 'test']) {
    assert.match(parsed.get(id).header, /timeout-minutes: 10\n/u);
    assert.ok(parsed.get(id).steps.at(-1).afterFailure);
    assert.match(parsed.get(id).steps.at(-1).run, /clean-caches\.mjs --event ci-teardown/u);
    assert.ok(parsed.get(id).steps.at(-2).afterFailure);
    assert.match(parsed.get(id).steps.at(-2).run, /external-grammar-cache\.mjs --clean-materialized/u);
  }
  checkPackageStepConservation(parsed);
  assert.deepEqual(needs(parsed.get('test')), ['tests', 'report-assertions', 'package-checks']);
  assert.deepEqual(needs(parsed.get('publish')), ['test']);
  assert.match(parsed.get('publish').header, /needs.test.result == 'success'/u);
});

test('frozen package conservation rejects omissions, duplicates, reassignment and command or condition edits', () => {
  const mutate = change => {
    const parsed = jobs('js.yml');
    change(parsed.get('package-checks').steps);
    assert.throws(() => checkPackageStepConservation(parsed), assert.AssertionError);
  };
  const index = steps => steps.findIndex(step => /check:native-imports/u.test(step.run));
  mutate(steps => steps.splice(index(steps), 1));
  mutate(steps => steps.push({ ...steps[index(steps)] }));
  mutate(steps => {
    const step = steps[index(steps)];
    step.text = step.text.replace("matrix.group == 'native-imports'", "matrix.group == 'contracts'");
  });
  mutate(steps => {
    const step = steps[index(steps)];
    step.text = step.text.replace('npm run check:native-imports', 'npm run check:builtin-cst');
  });
  mutate(steps => {
    const step = steps[index(steps)];
    step.text = step.text.replace('!cancelled()', 'success()');
  });
});

test('the final package gate requires all five partitions, all test groups and standalone report assertions', () => {
  const gate = jobs('js.yml').get('test').steps.find(step => step.name === 'Require every JavaScript test group');
  assert.ok(gate.afterFailure);
  const originalGate = gate.text
    .replace('          PACKAGE_CHECKS_RESULT: ${{ needs.package-checks.result }}\n', '')
    .replace(
      '          if [ "$PACKAGE_CHECKS_RESULT" != "success" ]; then\n' +
      '            echo "::error::JavaScript package check groups finished with \'$PACKAGE_CHECKS_RESULT\'; see the JavaScript Package Checks jobs."\n' +
      '            exit 1\n          fi\n', '');
  assert.equal(packageStepSignature({ text: originalGate }), 'dd833d50b883ad3316674a59f0638632dacaf701c06bb90ec93e9eae3b994e09',
    'the added partition guard preserves every original final-gate field');
  assert.match(gate.text, /PACKAGE_CHECKS_RESULT: \$\{\{ needs.package-checks.result \}\}/u);
  assert.match(gate.text, /TESTS_RESULT: \$\{\{ needs.tests.result \}\}/u);
  assert.match(gate.text, /REPORTS_RESULT: \$\{\{ needs.report-assertions.result \}\}/u);
  const execute = overrides => spawnSync('bash', ['-c', gate.run.replace(/^\|\s*\n/u, '')], {
    encoding: 'utf8', env: {
      ...process.env, PACKAGE_CHECKS_RESULT: 'success', TESTS_RESULT: 'success',
      REPORTS_RESULT: 'success', EXTERNAL_REPORTS: 'false', ...overrides,
    },
  });
  assert.equal(execute({}).status, 0);
  assert.equal(execute({ EXTERNAL_REPORTS: 'true', REPORTS_RESULT: 'skipped' }).status, 0);
  for (const result of ['failure', 'cancelled', 'skipped']) {
    for (const key of ['PACKAGE_CHECKS_RESULT', 'TESTS_RESULT', 'REPORTS_RESULT']) {
      assert.equal(execute({ [key]: result }).status, 1, `${key}=${result} must fail the package gate`);
    }
    assert.equal(execute({ PACKAGE_CHECKS_RESULT: result, EXTERNAL_REPORTS: 'true', REPORTS_RESULT: 'skipped' }).status, 1);
  }
});
