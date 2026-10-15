// The CI workflow structure: the Rust workflow and the acceptance stages run
// only after the JavaScript workflow passes, the Rust acceptance stages run
// only after the matching JavaScript stages, and every job reports every failure in one run
// (cargo test --no-fail-fast, check steps that run unless the run was
// cancelled, and cargo fmt, clippy and doc in separate steps).
import { test } from 'node:test';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
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

function selectNativeTestInputs(paths, { base = 'base', valid = true } = {}) {
  const step = jobs('ci.yml').get('changes').steps.find(step => step.id === 'paths' || step.name === 'Detect whether the Rust workflow is needed');
  assert.ok(step);
  const stub = `git() {
    case "$1" in
      cat-file) [ "$TEST_NATIVE_BASE_VALID" = true ] ;;
      diff) printf '%s\\n' "$TEST_NATIVE_CHANGED_PATHS" ;;
      *) return 99 ;;
    esac
  }\n`;
  // GitHub Actions provides a regular output file. Linux cannot reopen Node's
  // captured subprocess pipe through /dev/stdout on every runner.
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-native-input-'));
  const output = path.join(directory, 'github-output');
  try {
    const result = spawnSync('bash', ['-e', '-o', 'pipefail', '-c', stub + step.run.replace(/^\|\s*\n/u, '')], {
      encoding: 'utf8', env: { ...process.env, BASE_SHA: base, GITHUB_OUTPUT: output,
        TEST_NATIVE_BASE_VALID: String(valid), TEST_NATIVE_CHANGED_PATHS: paths.join('\n') },
    });
    assert.equal(result.status, 0, result.stderr);
    const text = readFileSync(output, 'utf8');
    assert.match(text, /^rust=(?:true|false)\nnative_test_inputs_changed=(?:true|false)\n$/u,
      'the actual selector writes each Boolean output exactly once');
    return Object.fromEntries([...text.matchAll(/^(rust|native_test_inputs_changed)=(true|false)$/gmu)].map(([, key, value]) => [key, value === 'true']));
  } finally {
    rmSync(directory, {recursive: true, force: true});
  }
}

function nativeInputCondition(job) {
  const lines = job.header.split('\n'), index = lines.findIndex(line => /^ {4}if:/u.test(line));
  assert.ok(index >= 0, job.id);
  let expression = lines[index].replace(/^ {4}if:\s*/u, '');
  if (expression === '|') expression = lines.slice(index + 1).filter((line, offset, rest) =>
    !rest.slice(0, offset + 1).some(previous => /^ {4}\S/u.test(previous))).join('\n').trim();
  return expression.replace(/^\$\{\{\s*|\s*\}\}$/gu, '');
}

function evaluateNativeInputCondition(job, context) {
  let expression = nativeInputCondition(job);
  if (!/\b(?:always|cancelled|failure|success)\(/u.test(expression)
    && (context.cancelled || needs(job).some(name => context.results[name] !== 'success'))) return false;
  expression = expression.replaceAll('!cancelled()', String(!context.cancelled))
    .replaceAll('github.event_name', JSON.stringify(context.event))
    .replaceAll('inputs.native_test_inputs_changed', String(context.input ?? false))
    .replaceAll('needs.changes.outputs.rust', JSON.stringify(String(context.selection.rust)));
  for (const [name, value] of Object.entries(context.outputs)) expression = expression.replaceAll(`needs.detect-changes.outputs.${name}`, JSON.stringify(String(value)));
  for (const [name, value] of Object.entries(context.results)) expression = expression.replaceAll(`needs.${name}.result`, JSON.stringify(value));
  assert.doesNotMatch(expression, /needs\.|inputs\.|github\./u, expression);
  return Function(`return (${expression});`)();
}

function nativeInputJobChain(paths, { event = 'pull_request', javascript = 'success', input, base, valid } = {}) {
  const selection = selectNativeTestInputs(paths, { base, valid });
  const detector = read('rust/scripts/detect-code-changes.rs');
  const excluded = [.../let excluded_folders = \[([\s\S]*?)\];/u.exec(detector)[1].matchAll(/"([^"]+)"/gu)].map(match => match[1]);
  const pattern = new RegExp(/fn code_change_pattern\(\)[\s\S]*?r"([^"]+)"/u.exec(detector)[1], 'u');
  const included = paths.filter(file => !file.endsWith('.md') && !excluded.some(folder =>
    file.startsWith(folder) || file.replace(/^(?:rust|js)\//u, '').startsWith(folder)));
  const context = { event, input: input ?? selection.native_test_inputs_changed, selection, outputs: {
    'rs-changed': included.some(file => file.endsWith('.rs')),
    'toml-changed': included.some(file => file.endsWith('.toml') || file.endsWith('Cargo.lock')),
    'workflow-changed': included.some(file => file.startsWith('.github/workflows/')),
    'any-code-changed': included.some(file => pattern.test(file)),
  }, results: { changes: 'success', js: javascript, 'self-translation-report-validation': 'success', 'detect-changes': 'success' } };
  const outer = evaluateNativeInputCondition(jobs('ci.yml').get('rust'), context), results = {};
  for (const name of ['changelog', 'cargo-lock', 'check', 'test']) {
    results[name] = outer && evaluateNativeInputCondition(jobs('rust.yml').get(name), context);
    context.results[name] = results[name] ? 'success' : 'skipped';
  }
  return { selection, outputs: context.outputs, outer, ...results };
}

test('registered native experiments and Links data select the complete prerequisite chain without changing ordinary exclusions', () => {
  for (const file of ['experiments/original-link-identity/policy.mjs', 'experiments/original-link-identity/data/policy.lino',
    'experiments/rust-text-value-projection/native-observations.test.mjs', 'experiments/source-native-observations.mjs',
    'parity/fixtures/link-network-binary.lino', 'rust/src/data/link-network-binary-schema.lino']) {
    const actual = nativeInputJobChain([file]);
    assert.deepEqual(actual.selection, { rust: true, native_test_inputs_changed: true }, file);
    assert.deepEqual(actual.outputs, { 'rs-changed': false, 'toml-changed': false, 'workflow-changed': false, 'any-code-changed': false }, file);
    assert.equal(actual.changelog, false, file);for (const job of ['outer', 'cargo-lock', 'check', 'test']) assert.equal(actual[job], true, `${file}: ${job}`);
    assert.equal(nativeInputJobChain([file], { javascript: 'failure' }).outer, false, file);
    assert.equal(nativeInputJobChain([file], { input: false }).test, false, 'the new input closes the actual old gate-chain gap');
  }
  for (const file of ['experiments/repro.mjs', 'experiments/repro.lino', 'js/experiments/repro.mjs', 'docs/notes.md',
    'experiments/original-link-identity/README.md', 'parity/README.md', 'js/src/plain.js']) {
    const actual = nativeInputJobChain([file]);assert.equal(actual.selection.native_test_inputs_changed, false, file);assert.equal(actual.test, false, file);
  }
  const rust = nativeInputJobChain(['rust/src/link_network.rs']);assert.equal(rust.selection.native_test_inputs_changed, false);assert.equal(rust.changelog, true);assert.equal(rust.test, true);
  for (const options of [{ base: '' }, { valid: false }]) {
    const unknown = nativeInputJobChain([], options);assert.deepEqual(unknown.selection, { rust: true, native_test_inputs_changed: true });assert.equal(unknown.test, true);
  }
  assert.equal(nativeInputJobChain(['docs/notes.md'], { event: 'push' }).test, true);
  assert.equal(nativeInputJobChain(['experiments/original-link-identity/data/policy.lino'], { event: 'push' }).test, true);
  assert.equal(nativeInputJobChain(['experiments/repro.mjs'], { event: 'push' }).outer, false);
});

test('the optional native-input workflow interface preserves defaults, JavaScript gating and all original prerequisites', () => {
  const ci = jobs('ci.yml'), rust = jobs('rust.yml');
  assert.match(workflow('rust.yml'), /^ {2}workflow_call:\n {4}inputs:\n {6}native_test_inputs_changed:\n {8}description: [^\n]+\n {8}type: boolean\n {8}required: false\n {8}default: false$/mu);
  assert.match(ci.get('changes').header, /native_test_inputs_changed: \$\{\{ steps\.paths\.outputs\.native_test_inputs_changed \}\}/u);
  assert.match(ci.get('rust').header, /native_test_inputs_changed: \$\{\{ needs\.changes\.outputs\.native_test_inputs_changed == 'true' \}\}/u);
  assert.ok(needs(ci.get('rust')).includes('js'));assert.doesNotMatch(nativeInputCondition(ci.get('rust')), /always\(|cancelled\(/u);
  const changed = [...rust].filter(([, job]) => job.header.includes('inputs.native_test_inputs_changed')).map(([name]) => name).sort();
  assert.deepEqual(changed, ['cargo-lock', 'check', 'test']);
  assert.deepEqual(needs(rust.get('check')), ['detect-changes', 'cargo-lock']);assert.deepEqual(needs(rust.get('test')), ['detect-changes', 'changelog', 'cargo-lock', 'check']);
  for (const [name, fields] of [['cargo-lock', ['rs-changed', 'toml-changed', 'workflow-changed', 'any-code-changed']], ['check', ['any-code-changed', 'rs-changed', 'toml-changed', 'workflow-changed']], ['test', ['any-code-changed', 'rs-changed', 'toml-changed', 'workflow-changed']]]) {
    const expression = nativeInputCondition(rust.get(name));assert.match(expression, /!cancelled\(\)/u);assert.match(expression, /github\.event_name == 'push'/u);assert.match(expression, /github\.event_name == 'workflow_dispatch'/u);
    for (const field of fields) assert.ok(expression.includes(`needs.detect-changes.outputs.${field} == 'true'`));
    const context = { event: 'pull_request', selection: { rust: true }, outputs: Object.fromEntries(fields.map(field => [field, false])), results: { 'detect-changes': 'success', 'cargo-lock': 'success', check: 'success', changelog: 'skipped' } };
    assert.equal(evaluateNativeInputCondition(rust.get(name), context), false, 'default false preserves old selection');
    assert.equal(evaluateNativeInputCondition(rust.get(name), { ...context, input: true }), true);
    if (name !== 'cargo-lock') assert.equal(evaluateNativeInputCondition(rust.get(name), { ...context, input: true, results: { ...context.results, 'cargo-lock': 'failure' } }), false);
    if (name === 'test') for (const failed of ['check', 'changelog']) assert.equal(evaluateNativeInputCondition(rust.get(name), { ...context, input: true, results: { ...context.results, [failed]: 'failure' } }), false);
  }
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
  for (const id of ['javascript-suite-independent', 'javascript-suite', 'rust-suite']) {
    const setup = acceptance.get(id).steps.find((step) => /dtolnay\/rust-toolchain@/u.test(step.text));
    assert.ok(setup, `${id} sets up Rust`);
    assert.match(setup.text, /components:.*\bclippy\b/u, `${id} installs clippy-driver`);
    assert.match(setup.text, /components:.*\brustfmt\b/u, `${id} installs the formatter used by generated-source freshness checks`);
  }
});

test('the JavaScript evidence matrix retains every group and file once with disjoint outputs under the original deadline', () => {
  const acceptance = jobs('ci.yml');
  const suites = ['javascript-suite-independent', 'javascript-suite'].map(id => acceptance.get(id));
  const entries = suites.flatMap(suite => [...suite.header.matchAll(/- group: ([a-z-]+)\n\s+shard: ([0-9]+\/[0-9]+)\n\s+partition: ([0-9]+-of-[0-9]+)/gu)]
    .map(([, group, shard, partition]) => ({ group, shard, partition })));
  const expected = Object.keys(TEST_GROUPS).flatMap(group => {
    const count = group === 'grammar-native' ? 3 : group === 'conformance' ? 2 : 1;
    return Array.from({ length: count }, (_, index) => ({ group, shard: `${index + 1}/${count}`, partition: `${index + 1}-of-${count}` }));
  });
  const byScope = (left, right) => `${left.group}/${left.shard}`.localeCompare(`${right.group}/${right.shard}`);
  assert.deepEqual([...entries].sort(byScope), [...expected].sort(byScope));
  const files = partitionTests(testFiles()).groups;
  const executed = entries.flatMap(({ group, shard }) => partitionGroupFiles(files[group], shard));
  assert.equal(new Set(executed).size, executed.length);
  assert.deepEqual(executed.sort(), Object.values(files).flat().sort());
  for (const suite of suites) {
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
    assert.ok(needs(acceptance.get('rust-suite')).includes(suite.id));
    assert.ok(condition(acceptance.get('rust-suite')).includes(`needs.${suite.id}.result == 'success'`));
    assert.ok(needs(acceptance.get('acceptance')).includes(suite.id));
  }
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
  for (const id of ['javascript-suite-independent', 'javascript-suite']) {
    assert.ok(needs(acceptance.get(id)).includes('self-translation-report'));
  }
  for (const id of ['javascript-suite-independent', 'javascript-suite', 'rust-suite']) {
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
  for (const id of ['javascript-suite-independent', 'javascript-suite']) {
    const evidence = jobs('ci.yml').get(id);
    const rebuild = evidence.steps.findIndex((step) => /build-agda-layout-oracle\.mjs/u.test(step.run));
    const suite = evidence.steps.findIndex((step) => step.name === 'Produce the stage evidence');
    assert.ok(rebuild >= 0 && suite > rebuild, 'acceptance rebuilds the same independent oracle before its corpus tests');
    assert.ok(evidence.steps[suite].afterFailure, 'other acceptance tests still report failures if the oracle build fails');
  }
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
  for (const [rust, javascript] of [['native-rust', 'native-javascript'], ['rust-suite', 'javascript-suite'], ['rust-suite', 'javascript-suite-independent']]) {
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
  for (const stage of ['native-javascript', 'native-rust', 'native-translations', 'javascript-suite-independent', 'javascript-suite', 'rust-suite', 'delivery']) {
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
  'native-translations', 'javascript-suite-independent', 'javascript-suite', 'rust-suite', 'delivery', 'post-merge', 'acceptance',
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
const PACKAGE_GROUPS = ['native-imports', 'native-fixtures', 'grammar', 'contracts', 'generated', 'source-models'];
const ADDITIONAL_SOURCE_PACKAGE_CELLS = [
  { group: 'source-model-tables', generated_slice: 'first' },
  { group: 'binary-candidates', generated_slice: 'first' },
];
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

// New generator gates have their own frozen records; the original baseline
// remains unchanged. Addition hashes also include the exact matrix selector.
const ADDITIONAL_PACKAGE_STEPS = [
  ["Check generated Rust borrowed call interface rules", "node scripts/generate-rust-borrowed-call-interface-rules.mjs --check", '1f46a207a6fb3432e56ac6d785eb03f12289040b2ad41472ddf967acbfebf7a4', 'generated'],
  ["Check shared Rust borrowed call interface fixtures", "node scripts/generate-rust-borrowed-call-interface-fixtures.mjs --check", '64891f2eda98607ed1e13697b4a5627bb0dec2f72806f696cb5a876baf024f00', 'generated'],
  ["Check generated Rust primitive owner rules", "node scripts/generate-rust-primitive-owner-rules.mjs --check", 'c2846fc406532cf47d014908653cd848fcc246850362c60a9911acf7cd5168e8', 'generated'],
  ["Check shared Rust primitive owner fixtures", "node scripts/generate-rust-primitive-owner-fixtures.mjs --check", '7ede0060b0523316a65bc9ae7476e4b1fb16f8e417939d26c2a90e68cf0fe6a6', 'generated'],
  ["Check generated JavaScript scoped binding emission rules", "node scripts/generate-javascript-scoped-binding-emission-rules.mjs --check", '7f0c0306f141f7e8446d6a604c3d6b91c1197f787184e020670178b0a4889571', 'generated'],
  ["Check shared JavaScript scoped binding emission fixtures", "node scripts/generate-javascript-scoped-binding-emission-fixtures.mjs --check", '995a45b9c2a45647b4a877e7b0a6f2814271611f04a77ce6aaf69bacf2663f2d', 'generated'],
];

// Empty-option gates are a separate additive freeze; earlier records stay fixed.
const EMPTY_OPTION_PACKAGE_STEPS = [
  ["Check generated Rust empty option rules", "node scripts/generate-rust-empty-option-rules.mjs --check", '5be3baca04a57c9e4b2a09659627b3210c4daa64aad47694881a3c6f104139a1', 'generated'],
  ["Check shared Rust empty option fixtures", "node scripts/generate-rust-empty-option-fixtures.mjs --check", '9807ef1905742ff2f8007ecdb76455b8f6d0517a134d1b243cb421b60d848762', 'generated'],
];

// Type-namespace gates have their own additive freeze; all earlier rows remain.
const TYPE_NAMESPACE_PACKAGE_STEPS = [
  ["Check generated Rust type namespace rules", "node scripts/generate-rust-type-namespace-rules.mjs --check", 'f120089b51fd5288836e249a56302588790551560b20c3087df4db3b6655f4d6', 'generated'],
  ["Check shared Rust type namespace fixtures", "node scripts/generate-rust-type-namespace-fixtures.mjs --check", '5f3554013a2366dfb14e6e6d63013aaa183d7b011ae8db0969ef4977b19053b0', 'generated'],
];

// Bulk twenty additions are frozen separately from every earlier gate.
const PURE_UNIT_PACKAGE_STEPS = [
  ["Check generated Rust pure Unit call rules", "node scripts/generate-rust-pure-unit-call-rules.mjs --check", 'eca96f54e18ab3e37d09cb936049f3c93160fd9195d5f2577e2e5aadb7b141fc', 'generated'],
  ["Check shared Rust private pure Unit call fixtures", "node scripts/generate-rust-pure-unit-call-fixtures.mjs --check", '6ffefb18bfe4de23bb4d67093cbac30a7ec4a344bbe9044c1da34b2d835c5561', 'generated'],
  ["Check shared Rust public pure Unit call fixtures", "node scripts/generate-rust-public-pure-unit-call-fixtures.mjs --check", '725cf8d6c033b6a3ddd8257614a286c4756abe4fecdff78a25cdb65d8ecbafed', 'generated'],
];
const SOURCE_SPAN_PACKAGE_STEPS = [
  ["Check generated source span rules", "node scripts/generate-source-span-rules.mjs --check", 'c3f8959bdaa5024e679a16434036f6eb1d518d6ca0f72ded2101772c38b8fa22', 'generated'],
  ["Check shared source span offset fixtures", "node scripts/generate-source-span-fixtures.mjs --check", 'd7556f6f378fc8316bf0c3ebc80ab87af09d5bcd05f56aa5392e7dbd44370e33', 'generated'],
];
const INTEGRAL_METADATA_PACKAGE_STEPS = [
  ['Check generated integral program metadata rules', 'node scripts/generate-program-numeric-metadata-rules.mjs --check', '68f2e0a33ba07c5f208f0bc67dcadaa722fb64d10e5e30c99415f32b53f6ef3e', 'generated'],
  ['Check shared integral program metadata fixtures', 'node ../experiments/generate-program-numeric-metadata-fixture.mjs --check', 'fd284d751def0418595364f1441265a9281ec97dfbac702f5359b38ea39e96bd', 'generated'],
  ['Check shared owned occurrence position fixtures', 'node ../experiments/generate-owned-occurrence-position-fixture.mjs --check', '835f45ba351fd3fc0fef0c68aa24cafc0da6aa48b5a3b7509c1def89ab09b168', 'generated'],
];
const INTEGRAL_METADATA_EXPERIMENT_PACKAGE_STEPS = [
  ['Run integral program metadata checks', 'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/program-numeric-metadata.test.mjs', '1eff414d8e3fa073988ff08d0e9ba2bf6b80010e1c59a78fd452e2c2d79de54c', 'generated'],
  ['Run owned occurrence position checks', 'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/owned-occurrence-position.test.mjs', 'd179c728750a9e894baef1886d13d39853f93f45989aab41087af6059683ee98', 'generated'],
];
const OWNED_TEXT_GENERATOR_PACKAGE_STEPS = [
  ["Check generated Rust owned local text state rules", "node ../experiments/generate-rust-owned-local-state-rules.mjs --check", '9f63b3bacdbe0a6bb5a421ff26c75bc5e806e0628f31040b7aaabdc00ea536b8', 'generated'],
  ["Check shared Rust owned local text state fixtures", "node ../experiments/generate-rust-owned-local-state-fixture.mjs --check", '9705af30ae034ec26327def2f595654344283cd01044b25ff65a0ad5b57f2024', 'generated'],
  ["Check shared Rust owned local source operation fixtures", "node ../experiments/generate-rust-owned-local-source-operation-fixture.mjs --check", 'e10eb6d30a3db1d756bc191e17e1d04e5e0812ab6f719e19ca3c898d83770d5f', 'generated'],
  ["Check shared Rust owned local container fixtures", "node ../experiments/generate-rust-owned-local-container-fixture.mjs --check", '24ac4f4ba1655f7fc9c08264f98bfa7997516c5c872c1a7667635d435d0869f3', 'generated'],
];
const OWNED_TEXT_EXPERIMENT_PACKAGE_STEPS = [
  ["Run Rust owned local text state prototype checks", "node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-owned-local-state-prototype.test.mjs", '9efb2471f3674e8a0aab546eb8ecc27422439044bd97b5a7087be640219fe0cb', 'generated'],
  ["Run Rust owned local text public profile checks", "node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-owned-local-state-public.test.mjs", '6ae0bb72e081015ff45fa7c7ad3aebdc9fff92e8afa2fc1dd4e6aff8aba927fa', 'generated'],
  ["Run Rust owned local text unit checks", "node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-owned-local-text-units.test.mjs", '8ecf1dadd7f187c39e8483dbf5b5f7e3005eea9202dce2a172b3ad8bcedd66bb', 'generated'],
  ["Run Rust owned local container checks", "node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-owned-local-container.test.mjs", '0010868feb0f784edacd150e76dec71add2119de452037dc7788b9c8183728d5', 'generated'],
];

// Const-predicate and byte gates add exact obligations without changing earlier records.
const CONST_AND_BYTE_GENERATOR_PACKAGE_STEPS = [
  [
    "Check generated closed Rust const predicate rules",
    "node ../experiments/generate-rust-const-function-rules.mjs --check",
    "9496f2280a290cab8bfde47cda20d0dfa426c1ba2edf84c0ab531d63665f4378",
    "generated"
  ],
  [
    "Check actual styled Rust const predicate fixtures",
    "node ../experiments/generate-rust-const-function-fixture.mjs --check",
    "51371b5cc85d2a8df9364cd0206b00a9df0ad53646fd5999c85b8d6c2e05c85c",
    "generated"
  ],
  [
    "Check generated source text byte rules",
    "node scripts/generate-source-text-byte-rules.mjs --check",
    "47f08f82382a4fc64326166292e0f32e7c1150bf483b5278f16d5d9dc772579f",
    "generated"
  ],
  [
    "Check shared source text byte fixtures",
    "node scripts/generate-source-text-byte-fixtures.mjs --check",
    "75cda79cebdd7fff01bc4352311916289b7c1c0dc9107395bac48e246e0c0f6e",
    "generated"
  ]
];
const CONST_AND_BYTE_EXPERIMENT_PACKAGE_STEPS = [
  [
    "Run closed Rust const predicate prototype checks",
    "node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-const-function-prototype.test.mjs",
    "d4444582ae1106fcd4bce1e49ccf1ba1a185c3acf9fb978605eefb4ecd1a5df3",
    "generated"
  ],
  [
    "Check portable source text byte kernels",
    "node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/source-text-byte-kernel.test.mjs ../experiments/complete-binary-formatter-probe.test.mjs",
    "2082ceaa9c585324089ee5e3174f5cc155e1b1d397dda9c2659feddda7b11b30",
    "generated"
  ]
];

// Complete runtime-shell gates retain every earlier mandatory gate.
const RUNTIME_SHELL_PACKAGE_STEPS = [
  [
    "Check generated whole Rust runtime shell rules",
    "node scripts/generate-rust-runtime-shell-rules.mjs --check",
    "60b3663cb9606f09a83db980710b9409204088635821dc2c9e97c7627bc28e5c",
    "generated"
  ],
  [
    "Check complete Rust runtime shell fixtures",
    "node scripts/generate-rust-runtime-shell-fixtures.mjs --check",
    "38ebdad6ce944c9eeacccce072c633fbefa5fa97f2a763730ee07210c1b1c160",
    "generated"
  ]
];

// Numeric boundary generation retains every earlier mandatory gate.
const RUST_NUMBER_PACKAGE_STEPS = [
  [
    "Check generated Rust numeric boundary rules",
    "node scripts/generate-rust-number-rules.mjs --check",
    "a9e1ebac8bf540cf204eb1e2669980da924c664e71d73dbccc309f62ba105cb7",
    "generated"
  ]
];

const JAVASCRIPT_LINE_TERMINATOR_PACKAGE_STEPS = [
  [
    "Check generated JavaScript line terminator rules",
    "node scripts/generate-javascript-line-terminator-rules.mjs --check",
    "3f1c47883f675612b6286db36236cf4af8aba614c0ada9db0f0eafde92cd7906",
    "generated"
  ]
];

const GENERATED_JAVASCRIPT_UNIT_PACKAGE_STEPS = [
  [
    "Check generated JavaScript Unit module experiment rules",
    "node ../experiments/generated-javascript-unit/generate-rules.mjs --check",
    "d87a6c2f050ec7b711baf691ba78705b85da2ded1572ddb16ee5a3d096135fc5",
    "generated"
  ],
  [
    "Check complete generated JavaScript Unit module experiment fixtures",
    "node ../experiments/generated-javascript-unit/generate-fixtures.mjs --check",
    "5d5a780433038f4bce1cc0ffed08311960e3dbda4786e9f9eae2dd2ab486318c",
    "generated"
  ]
];

const GENERATED_JAVASCRIPT_UNIT_EXPERIMENT_STEPS = [
  [
    "Run generated JavaScript Unit module experiment",
    "node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/generated-javascript-unit/model.test.mjs",
    "c9b231ccebcf35f57faf1c91f72f7a7fd7e93090f719a53461b47b2d189ba6d9",
    "generated"
  ]
];

const OWNED_TUPLE_CALL_GENERATOR_PACKAGE_STEPS = [
  [
    'Check generated owned tuple initialization call rules',
    'node ../experiments/generate-rust-owned-tuple-call-rules.mjs --check',
    '63bb53648d2af06ed6689d216974f983f82840e6271ead38f77e83f96f421f93',
    'generated'
  ],
  [
    'Check owned tuple initialization call fixtures',
    'node ../experiments/generate-owned-tuple-call-fixture.mjs --check',
    '1d9a99322c68b6fe094fd6f7708db4752aaaa9b90cf8786a132ece9c146d319e',
    'generated'
  ],
  [
    'Check actual public owned tuple call native archives',
    'node ../experiments/generate-owned-tuple-call-native-fixture.mjs --check',
    '3971b503fcfa263040fdf68901bc19fd895075c9bf68bf4358778aeda1339410',
    'generated'
  ]
];

const OWNED_TUPLE_CALL_EXPERIMENT_PACKAGE_STEPS = [
  [
    'Run owned tuple initialization call checks',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/owned-tuple-call.test.mjs',
    '2ed9f5b7851a920edd29e9816a42a757baedc840e1739b59a4b60e1f86b19e19',
    'generated'
  ]
];

const RUST_LEXICAL_AND_OWNED_MODULE_GENERATOR_STEPS = [
  ['Check generated Rust cooked string escape rules',
    'node scripts/generate-rust-cooked-string-rules.mjs --check',
    '52f1ba5e4cc7587870bd1bfa1e9d3cd18435a9b790f2454d388f32269e63cd5f', 'generated'],
  ['Check shared Rust cooked string escape fixtures',
    'node scripts/build-rust-cooked-string-escape-fixtures.mjs --check',
    '345c8bc5486e854fde308e6ebb92b67bb4a7abafbc4e5b75450877ddfa2dfc44', 'generated'],
  ['Check generated Rust Unicode scalar escape rules',
    'node scripts/generate-rust-unicode-escape-rules.mjs --check',
    'bf9496b12449d314d40d8b1944394a2926d6550a9372d1f0f6156ec7f65c590c', 'generated'],
  ['Check shared Rust Unicode scalar escape fixtures',
    'node scripts/build-rust-unicode-escape-fixtures.mjs --check',
    '5eac32a1287035c4f6045ff7b1f46c30ab1e45635ed511de3a81301ed2ba5372', 'generated'],
  ['Check generated Rust source whitespace and boundary rules',
    'node scripts/generate-rust-source-whitespace-rules.mjs --check',
    '39e7b178926e8ccfa9723de5aacae66b0664a8699f4a27b8ab4e02e20e231cda', 'generated'],
  ['Check generated owned Rust module experiment rules',
    'node ../experiments/generated-owned-rust/generate-rules.mjs --check',
    'dd990ba959a39dc3562855c15eb2c7f4a0984536308a4f4f701eae75408a31de', 'generated'],
  ['Check complete generated owned Rust module experiment fixtures',
    'node ../experiments/generated-owned-rust/generate-fixtures.mjs --check',
    'cb69f420d26ffea65c21d8bc8f8695fb38e94e69fde2d09127451b462c818fa3', 'generated']
];

const GENERATED_OWNED_RUST_MODULE_EXPERIMENT_STEPS = [
  ['Run generated owned Rust module experiment',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/generated-owned-rust/model.test.mjs',
    '456ff45cb502154ebcf1bf135d3db1da3589f4848f6e521ed7a033b1c5298b01', 'generated']
];

const RUST_RAW_STRING_AND_LITERAL_EDIT_GENERATOR_STEPS = [
  ['Check generated Rust raw string literal rules',
    'node scripts/generate-rust-raw-string-rules.mjs --check',
    '917c63982ebf45374d00fb00b87b2b9adc95fc8352b3542b11900b675f8b53a0', 'generated'],
  ['Check generated owned Rust literal value edit experiment rules',
    'node ../experiments/generated-owned-rust/generate-value-edit-rules.mjs --check',
    '751a40835f8f2fe205adfe2ed930555657d34447ec6cd349d4c4e60f3c084eb6', 'generated'],
  ['Check complete generated owned Rust literal value edit fixtures',
    'node ../experiments/generated-owned-rust/generate-value-edit-fixtures.mjs --check',
    '852dad5049a7edee04cbbff150ff1921af745e8a63e552a80e3154af1ee426fe', 'generated']
];

const OWNED_RUST_LITERAL_EDIT_EXPERIMENT_STEPS = [
  ['Run generated owned Rust literal value edit experiment',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/generated-owned-rust/value-edits.test.mjs',
    '9c3e7655ac025a7a7aed1aa872aa80c6c54698df0fe322d6bd6ea868f1388548', 'generated']
];

const RUST_TEXT_VALUE_PROJECTION_GENERATOR_STEPS = [
  ['Check generated Rust text value projection experiment rules',
    'node ../experiments/rust-text-value-projection/generate-rules.mjs --check',
    '53c030d86af799a2b1fb8042cfec0f7cb1c79375332117076f72e48c38e6c289', 'generated'],
  ['Check complete Rust text value projection experiment fixtures',
    'node ../experiments/rust-text-value-projection/generate-fixtures.mjs --check',
    'f8b6fb66b072243d84471cc9d25ad549412e8ee3f5cd8f9d602bf22c2b97204e', 'generated']
];

const RUST_TEXT_VALUE_PROJECTION_EXPERIMENT_STEPS = [
  ['Run Rust text value projection experiment',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-text-value-projection/model.test.mjs',
    '97a38ea5f8dac31f5778cfb203911e46560028a4671c1b90b641e0a98692a6f4', 'generated']
];

const RUST_RAW_BINDING_IDENTIFIER_GENERATOR_STEPS = [
  ['Check generated Rust raw binding identifier experiment rules and fixtures',
    'node ../experiments/rust-raw-binding-identifiers/generate-rules.mjs --check',
    '5502b13100d0d342741905624167acb3cc9de64228a9c795c0c9bed9df342a80', 'source-models']
];

const RUST_RAW_BINDING_IDENTIFIER_EXPERIMENT_STEPS = [
  ['Run Rust raw binding identifier experiment',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-raw-binding-identifiers/model.test.mjs',
    '7c986492eab66bb0347d115d513ec11af833e3acb42822aa6a73998c6b3edcbe', 'source-models']
];

const RUST_TEXT_CERTIFICATE_GENERATOR_STEPS = [
  ['Check generated complete Rust text certificate kernel',
    'node ../experiments/rust-text-certificate/generate.mjs --check',
    '94403fbdacab7b68d43a94f3c7e12456604703b4037e039e110eedcc387fd399', 'source-models']
];

const RUST_TEXT_CERTIFICATE_EXPERIMENT_STEPS = [
  ['Run portable Rust text certificate feasibility checks',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-text-certificate/feasibility.mjs',
    'cf534d4a66964b2b953d8ba5725270b745426887b3bc469fb9a2aec745ebdaf0', 'source-models'],
  ['Run complete portable Rust text certificate kernel',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/rust-text-certificate/kernel.test.mjs',
    'c6d370c7e06647c6ce0d2cf06c5cb9866084b5e0e426cfa66552cea87b67a481', 'source-models']
];

const OWNED_SOURCE_ADAPTER_PACKAGE_STEPS = [
  ['Check owned Rust text certificate overlay receipts',
    'node ../experiments/rust-text-certificate-adapter/generate.mjs --check',
    '02d73621214b349c2589547bcb283a2f2d32334756a1ecd84b2559d49ef52182', 'source-models'],
  ['Check owned Rust raw binding certificate rules',
    'node ../experiments/rust-raw-binding-certificate-adapter/generate-rules.mjs --check',
    '13a97ec8398b85d38b0e97e166a1569da66eaf71b34b7af5aa9e9fdcf053fbb8', 'source-models'],
  ['Check owned Rust raw binding overlay receipts',
    'node ../experiments/rust-raw-binding-certificate-adapter/generate-overlay.mjs --check',
    '6eab228a6f69af59e97ad1bb7c8f129c87ba0106b0e23ed9f8271c321ca5b591', 'source-models'],
  ['Run owned Rust text and raw binding certificate adapters',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ' +
    '../experiments/rust-text-certificate-adapter/overlay-builder.test.mjs ' +
    '../experiments/rust-text-certificate-adapter/overlay.test.mjs ' +
    '../experiments/rust-text-certificate-adapter/model.test.mjs ' +
    '../experiments/rust-text-certificate-adapter/raw-identity.test.mjs ' +
    '../experiments/rust-raw-binding-certificate-adapter/model.test.mjs',
    '88b05f1085040735263e8ae8592ab9f25630de62735435f370749437b545ac5b', 'source-models'],
  ['Check complete Rust text certificate library packaging',
    'node ../experiments/rust-text-certificate-library/generate.mjs --check',
    'f3a126c1750b14af4d696d47b9b8e38c2db4c8f1bd99da0d19f37c3d31826adf', 'source-models'],
  ['Run complete Rust source oracle contracts',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ' +
    '../experiments/source-native-observations.test.mjs ' +
    '../experiments/rust-text-certificate-library/native-observations.test.mjs',
    'e259a15270916aeaa4a42b29020abb8b7288b74f29058b6afba9b0fdbe07d583', 'source-models'],
  ['Check exact original link identity kernel',
    'node ../experiments/original-link-identity/generate.mjs --check',
    'f0f905b7863523c494c2ee2b77ccb22dd95127b1e3adb5757ded3b7c681d32e5', 'source-models'],
  ['Run exact original link identity checks',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/original-link-identity/identity.test.mjs',
    '897a38592267f3399b503775f93027045bc882fa339c1e69c71592b5210d0875', 'source-models'],
  ['Run exact original link identity oracle contracts',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/original-link-identity/native-observations.test.mjs',
    'b6e2ec0b38634255126b994944962796f0419f66c7a1848d91d2df8187d54539', 'source-models'],
  ['Check fresh Number array collector rules',
    'node ../experiments/fresh-number-array-collectors/generate.mjs --check',
    '225cb45b8afec904e0d5425f721a3c06b61699a4426225d6ceffd92b10ab3f62', 'source-models'],
  ['Check complete fresh Number array collector fixtures',
    'node ../experiments/fresh-number-array-collectors/generate-fixtures.mjs --check',
    'f053f84d4f2ba2f46321262c265e6734d66281cdae4913676d0176409b96969c', 'source-models'],
  ['Run fresh Number array collector certificates',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/fresh-number-array-collectors/model.test.mjs',
    '216da9daa16788533b6cbb9ddfd8f2dc4ff2a9457fc01e247f0a6d3fa1bb9bfe', 'source-models'],
  ['Check original link role projection kernel',
    'node ../experiments/original-link-role-projection/generate.mjs --check',
    'ac7fb19d1870dcae640cb6753b81901dec8e6b92913cad8a041a9432754f2c21', 'source-models'],
  ['Run original link certificate role projection',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/original-link-role-projection/projection.test.mjs',
    'fadcdf19bc24ef39bced296e54ac578f7e9ab9d5424016147152ea1d8764b7ab', 'source-models'],
];

const ORIGINAL_LINK_PROJECTION_PACKAGE_STEPS = [
  ['Check original link symbol projection kernel',
    'node ../experiments/original-link-symbol-projection/generate.mjs --check',
    '5843066c61fe0eca8380f88997ff9152b5e9f3b677b6f7c6ebe699b0a667f381', 'source-models'],
  ['Run original link symbol projection checks',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/original-link-symbol-projection/projection.test.mjs',
    'deba4c68eb88722927d29a68270523a759b94ad5659a54dce370e89aaf945d4f', 'source-models'],
  ['Check original link table projection kernel',
    'node ../experiments/original-link-table-projection/generate.mjs --check',
    'a1e7ed7050764669f5ac6d0202e8cc8f533318fcb6412a6899d83f97e78f4cfb', 'source-models'],
  ['Run original link table projection checks',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/original-link-table-projection/projection.test.mjs',
    '91b5611fcaabd12768c1c2238263ee06ae46d7d0bd5aded7154e4d9133efa1c6', 'source-models'],
];

const ORIGINAL_LINK_SYMBOL_ORACLE_PACKAGE_STEPS = [
  ['Run original link symbol native oracle contracts',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test ../experiments/original-link-symbol-projection/native-observations.test.mjs',
    '698b65e2a0045c584b8e7c6305069b501a57048e6cf53df6da91120f83b107e5', 'source-models'],
];

const COMPLETE_TABLE_PACKAGE_STEPS = [
  ['Check complete original link table projection kernel',
    'node ../experiments/original-link-complete-table-projection/generate.mjs --check',
    '30c11ef000a16337cc347216d950c3ee36eceb2e136f7a84a471c18b22002b78', 'source-model-tables'],
  ['Run complete original link table projection checks',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test --test-concurrency=1 ../experiments/original-link-complete-table-projection/projection.test.mjs',
    'cf502913f15b101b3155cf8433a453464e22ccba5470a3bb64676dab3407aea4', 'source-model-tables'],
  ['Run complete original link table native oracle contracts',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test --test-concurrency=1 ../experiments/original-link-complete-table-projection/native-observations.test.mjs',
    '2b4c55ed3f961fed7ffc7f2d568e790572c9b9033f393fe1d4fb5eb1f8fea5f2', 'source-model-tables'],
];

const BINARY_METADATA_PACKAGE_STEPS = [
  ['Setup Rust formatting for binary metadata candidate', '',
    '68918d6dd7d379b6e5970cbd22d5621c0ed91b6cd790a2d606474c648aa5dbb6', 'binary-candidates'],
  ['Check generated binary metadata buffer candidate',
    'node scripts/generate-link-network-binary-rules.mjs --metadata-buffer-candidate --check',
    '8aa8a2a1cf6ea4f7e729b56a2c969843c05d620e2d1227b2628c81592c65928f', 'binary-candidates'],
  ['Run binary metadata buffer candidate checks',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test --test-concurrency=1 ../experiments/binary-metadata-buffer/model.test.mjs',
    '71e441a0e138430dad15b29d987d193a74a68e4276430fda79f8af50d44c6676', 'binary-candidates'],
  ['Run binary metadata buffer candidate style checks',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test --test-concurrency=1 ../experiments/binary-metadata-buffer/styles.test.mjs',
    '7b17cfc69ffefb4a66b0be6b7489e7e0c5752fcc730659b0982d3e1b6767b0c1', 'binary-candidates'],
  ['Run binary metadata buffer native oracle contracts',
    'node ../scripts/with-cache-cleanup.mjs --event test -- node --test --test-concurrency=1 ../experiments/binary-metadata-buffer/native-observations.test.mjs',
    '504f362f65879d332962e1fcbc260e9dc583fc58464058d8bbaabcf67e49163f', 'binary-candidates'],
];

// Exact physical owners of the unchanged generated checks. Shared Rust setup is
// per runner; both compiler feedback producers and their upload remain together.
const GENERATED_PACKAGE_SLICES = ['first', 'second'];
const GENERATED_PACKAGE_SHARED_SETUP = 'Setup Rust formatting for generated frontend rules';
const GENERATED_PACKAGE_SLICE_STEPS = {
  first: [
    'Check generated owned Rust literal value edit experiment rules',
    'Check complete generated owned Rust literal value edit fixtures',
    'Run generated owned Rust literal value edit experiment',
  ],
  second: [
    'Check the generated Rust frontend rules against JavaScript',
    'Check the generated global concept rules against JavaScript',
    'Check the generated discovery integration rules',
    'Check shared discovered concept graph fixtures',
    'Check generated concept view admission rules',
    'Check shared ontology and taxonomy view fixtures',
    'Check generated binary link network rules',
    'Check shared binary link network fixtures',
    'Check the generated Resource Description Framework rules',
    'Check shared Resource Description Framework graph fixtures',
    'Check generated resource triple syntax character rules',
    'Check generated resource triple syntax token rules',
    'Check shared resource triple syntax fixtures',
    'Check the generated grammar merge decisions against JavaScript',
    'Check the generated structural type rules against JavaScript',
    'Check the generated primitive value rules against JavaScript',
    'Check the shared Rust array runtime artifacts',
    'Check the generated TypeScript erasure rules against JavaScript',
    'Check the generated typed program link rules against JavaScript',
    'Check the generated Rust source indirection rules against JavaScript',
    'Check shared scoped Rust lifetime fixtures',
    'Check generated Rust array admission rules',
    'Check generated Rust floating point predicate rules',
    'Check shared Rust floating point predicate fixtures',
    'Check shared Rust floating point operation fixtures',
    'Check shared Rust floating point constant fixtures',
    'Check generated Rust guarded function completion rules',
    'Check shared Rust guarded function completion fixtures',
    'Check shared Rust guarded completion panic fixtures',
    'Check generated read-only Rust scalar parameter rules',
    'Check shared read-only Rust scalar parameter fixtures',
    'Check shared Rust array source fixtures',
    'Check shared fresh Rust scalar vector result fixtures',
    'Check generated Rust string continuation rules',
    'Check shared Rust string continuation fixtures',
    'Check generated Rust cooked string escape rules',
    'Check shared Rust cooked string escape fixtures',
    'Check generated Rust Unicode scalar escape rules',
    'Check shared Rust Unicode scalar escape fixtures',
    'Check generated Rust source whitespace and boundary rules',
    'Check generated Rust raw string literal rules',
    'Check shared TypeScript documentation pattern fixtures',
    'Check generated formal function selection rules',
    'Check shared formal function definition fixtures',
    'Check generated shared numeric function rules',
    'Check generated Rust source signature rules',
    'Check shared Rust source signature fixtures',
    'Check generated Rust borrowed call interface rules',
    'Check shared Rust borrowed call interface fixtures',
    'Check generated Rust primitive owner rules',
    'Check shared Rust primitive owner fixtures',
    'Check generated Rust type namespace rules',
    'Check shared Rust type namespace fixtures',
    'Check generated Rust pure Unit call rules',
    'Check shared Rust private pure Unit call fixtures',
    'Check shared Rust public pure Unit call fixtures',
    'Check generated source span rules',
    'Check shared source span offset fixtures',
    'Check generated Rust numeric boundary rules',
    'Check generated JavaScript line terminator rules',
    'Check generated JavaScript Unit module experiment rules',
    'Check complete generated JavaScript Unit module experiment fixtures',
    'Run generated JavaScript Unit module experiment',
    'Check generated owned Rust module experiment rules',
    'Check complete generated owned Rust module experiment fixtures',
    'Run generated owned Rust module experiment',
    'Check generated Rust text value projection experiment rules',
    'Check complete Rust text value projection experiment fixtures',
    'Run Rust text value projection experiment',
    'Check generated whole Rust runtime shell rules',
    'Check complete Rust runtime shell fixtures',
    'Check generated source text byte rules',
    'Check shared source text byte fixtures',
    'Check portable source text byte kernels',
    'Check generated integral program metadata rules',
    'Check shared integral program metadata fixtures',
    'Run integral program metadata checks',
    'Check shared owned occurrence position fixtures',
    'Run owned occurrence position checks',
    'Check generated closed Rust const predicate rules',
    'Check actual styled Rust const predicate fixtures',
    'Run closed Rust const predicate prototype checks',
    'Check generated Rust owned local text state rules',
    'Check generated owned tuple initialization call rules',
    'Check owned tuple initialization call fixtures',
    'Check actual public owned tuple call native archives',
    'Run owned tuple initialization call checks',
    'Check shared Rust owned local text state fixtures',
    'Check shared Rust owned local source operation fixtures',
    'Check shared Rust owned local container fixtures',
    'Run Rust owned local text state prototype checks',
    'Run Rust owned local text public profile checks',
    'Run Rust owned local text unit checks',
    'Run Rust owned local container checks',
    'Check generated Rust empty option rules',
    'Check shared Rust empty option fixtures',
    'Check generated JavaScript scoped binding emission rules',
    'Check shared JavaScript scoped binding emission fixtures',
    'Check generated standard Rust readonly query rules',
    'Check standard Rust readonly query fixtures',
    'Check generated shared scalar loop rules',
    'Check shared scalar loop source fixtures',
    'Check owned declaration scalar loop source fixtures',
    'Check generated primitive borrowed Rust header rules',
    'Check primitive borrowed Rust header fixtures',
    'Check shared numeric function definition fixtures',
    'Check strict public declaration consumers',
    'Authenticate reviewed generated-stage migration refreshes',
    'Check the generated Rust source product rules against JavaScript',
    'Check the generated immutable product emission rules against JavaScript',
    'Check the generated closed product source proofs against JavaScript',
    'Check shared source-free typed program fixtures',
    'Check shared editable algebraic program fixtures',
    'Check shared editable immutable product fixtures',
    'Check shared product unit value fixtures',
    'Check the associative data tooling fixture',
    'Check the recorded original structured data conversion revisions',
    'Check shared formal definition and certificate fixtures',
    'Check generated formal relation identities',
    'Check shared formal relation identity fixtures',
    'Check generated frontend rule compilation and style',
    'Check generated global concept rule compilation and style',
    'Retain exact generated sources and compiler feedback',
  ],
};

function normalizeGeneratedPackageSlice(step) {
  const owners = GENERATED_PACKAGE_SLICES.filter(slice => GENERATED_PACKAGE_SLICE_STEPS[slice].includes(step.name));
  assert.ok(owners.length <= 1, `generated check has multiple physical owners: ${step.name}`);
  if (!owners.length) {
    assert.ok(!step.text.includes('matrix.generated_slice'), `unregistered slice selector: ${step.name}`);
    return step.text;
  }
  const appendix = ` && matrix.generated_slice == '${owners[0]}'`;
  const expected = `        if: \${{ !cancelled() && matrix.group == 'generated'${appendix} }}`;
  const predicates = step.text.split('\n').filter(line => /^\s+if:/u.test(line));
  assert.deepEqual(predicates, [expected], `generated slice must only append its exact admission: ${step.name}`);
  assert.equal(step.text.split('matrix.generated_slice').length, 2, `ambiguous physical owner: ${step.name}`);
  return step.text.replace(appendix, '');
}

function packageCheckCells(checks) {
  assert.equal([...checks.header.matchAll(/^ {6}matrix:$/gmu)].length, 1);
  const matrix = /^ {6}matrix:\n([\s\S]*?)^ {4}steps:\n$/mu.exec(checks.header)?.[1];
  const expected = `        group: [${PACKAGE_GROUPS.join(', ')}]\n` +
    '        generated_slice: [first]\n' +
    '        include:\n' +
    '          - group: generated\n' +
    '            generated_slice: second\n' +
    '          - group: source-model-tables\n' +
    '            generated_slice: first\n' +
    '          - group: binary-candidates\n' +
    '            generated_slice: first\n';
  assert.equal(matrix, expected, 'the seven published cells retain their order before exactly two new source-model cells');
  const groups = /^ {8}group: \[([^\]]+)\]$/mu.exec(matrix)[1].split(', ');
  const slice = /^ {8}generated_slice: \[([^\]]+)\]$/mu.exec(matrix)[1];
  const included = [...matrix.matchAll(/^ {10}- group: ([a-z-]+)\n {12}generated_slice: ([a-z]+)$/gmu)]
    .map(([, group, generated_slice]) => ({ group, generated_slice }));
  return [...groups.map(group => ({ group, generated_slice: slice })), ...included];
}

function checkGeneratedPackageSlices(parsed) {
  const checks = parsed.get('package-checks');
  const cells = packageCheckCells(checks);
  assert.equal(cells.length, 9);
  assert.equal(new Set(cells.map(cell => `${cell.group}:${cell.generated_slice}`)).size, 9);
  const display = /^ {4}name: (.+)$/mu.exec(checks.header)?.[1];
  const secondLabel = "${{ matrix.generated_slice == 'second' && ' / second' || '' }}";
  assert.equal(display, 'JavaScript Package Checks (${{ matrix.group }})' + secondLabel);
  const displayNames = cells.map(cell => display.replace('${{ matrix.group }}', cell.group)
    .replace(secondLabel, cell.generated_slice === 'second' ? ' / second' : ''));
  assert.deepEqual(displayNames.slice(0, 6), PACKAGE_GROUPS.map(group => `JavaScript Package Checks (${group})`),
    'all six published check context names remain unchanged');
  assert.equal(displayNames[6], 'JavaScript Package Checks (generated) / second');
  assert.deepEqual(cells.slice(7), ADDITIONAL_SOURCE_PACKAGE_CELLS);
  assert.equal(new Set(displayNames).size, 9);
  const concurrencyGroups = [...checks.header.matchAll(/^ {6}group: (.+)$/gmu)].map(match => match[1]);
  assert.equal(concurrencyGroups.length, 1);
  const concurrency = concurrencyGroups[0];
  assert.equal(concurrency, 'check-js-${{ github.ref }}-javascript-package-checks-${{ matrix.group }}-${{ matrix.generated_slice }}',
    'both generated runners require independent concurrency identities');
  const physicalNames = cells.map(cell => concurrency
    .replace('${{ matrix.group }}', cell.group).replace('${{ matrix.generated_slice }}', cell.generated_slice));
  assert.equal(new Set(physicalNames).size, 9, 'a slice must never cancel a sibling runner');
  assert.match(checks.header, /^ {6}cancel-in-progress: true$/mu);
  assert.match(checks.header, /^ {6}fail-fast: false$/mu);
  for (const id of ['package-checks', 'test']) {
    assert.deepEqual([...parsed.get(id).header.matchAll(/^ {4}timeout-minutes: (.+)$/gmu)].map(match => match[1]), ['10']);
    assert.equal(condition(parsed.get(id)), '${{ !cancelled() }}');
  }
  const registered = GENERATED_PACKAGE_SLICES.flatMap(slice => GENERATED_PACKAGE_SLICE_STEPS[slice]);
  assert.equal(registered.length, 126);
  assert.equal(new Set(registered).size, registered.length);
  for (const step of checks.steps) normalizeGeneratedPackageSlice(step);
  const setup = checks.steps.filter(step => step.name === GENERATED_PACKAGE_SHARED_SETUP);
  assert.equal(setup.length, 1, 'formatting setup remains available in both physical generated runners');
  assert.equal(/^\s+if: (.+)$/mu.exec(setup[0].text)?.[1], "${{ !cancelled() && matrix.group == 'generated' }}");
  for (const slice of GENERATED_PACKAGE_SLICES) {
    const actual = checks.steps.filter(step => step.text.includes(`matrix.generated_slice == '${slice}'`));
    assert.deepEqual(actual.map(step => step.name), GENERATED_PACKAGE_SLICE_STEPS[slice],
      'every generated check retains one owner and its original producer/consumer order');
    assert.ok(checks.steps.indexOf(setup[0]) < checks.steps.indexOf(actual[0]));
  }
  const feedback = [
    'Check generated frontend rule compilation and style',
    'Check generated global concept rule compilation and style',
    'Retain exact generated sources and compiler feedback',
  ];
  assert.deepEqual(GENERATED_PACKAGE_SLICE_STEPS.second.slice(-3), feedback,
    'both feedback producers and their single upload execute in the same runner');
  assert.ok(needs(parsed.get('test')).includes('package-checks'));
  const gate = parsed.get('test').steps.filter(step => step.name === 'Require every JavaScript test group');
  assert.equal(gate.length, 1);
  assert.match(gate[0].text, /PACKAGE_CHECKS_RESULT: \$\{\{ needs.package-checks.result \}\}/u);
  assert.match(gate[0].text, /^\s+if: \$\{\{ !cancelled\(\) \}\}$/mu);
  assert.ok(gate[0].run.includes('if [ "$PACKAGE_CHECKS_RESULT" != "success" ]; then'));
  return cells;
}

function packageStepSignature(step, includeGroup = false) {
  const normalized = normalizeGeneratedPackageSlice(step);
  const fields = includeGroup ? normalized : normalized.replace(/ && matrix\.group == '[a-z-]+'/gu, '');
  const text = fields
    .split('\n').filter(line => line.trim() && !/^\s*#/u.test(line))
    .map(line => line.trimEnd()).join('\n');
  return createHash('sha256').update(text).digest('hex');
}

function checkRustTextValueNativeStep(parsed) {
  const steps = parsed.get('test').steps.filter(step => step.name === 'Run complete Rust text value source observations');
  assert.equal(steps.length, 1, 'the complete-source native oracle must execute once in its selected matrix cell');
  assert.equal(packageStepSignature(steps[0], true), '752006807d9fed41319980bd6552d651fb61f841a3952d959a5e595da7cb3fd5',
    'the native source oracle retains its exact condition, command, directory and explicit native switch');
  const artifacts = parsed.get('test').steps.filter(step => step.name === 'Retain complete Rust text value source observations');
  assert.equal(artifacts.length, 1, 'the exact native observation receipt must be retained once');
  assert.equal(packageStepSignature(artifacts[0], true), 'cb344920a23537b6e42f01adee93c496cfca25d73d77d49c18e5635620962117',
    'the native receipt retains its condition, action, commit identity, Links path and missing-file failure');
}

test('the complete Rust text source oracle is mandatory after JavaScript and its source changes select Rust', () => {
  checkRustTextValueNativeStep(jobs('rust.yml'));
  const parsed = jobs('ci.yml');
  assert.ok(needs(parsed.get('rust')).includes('js'));
  const classifier = /rust_paths='([^']+)'/u.exec(workflow('ci.yml'))?.[1];
  assert.ok(classifier);
  const addition = '|experiments/rust-text-value-projection/';
  assert.equal(classifier.split(addition).length, 2);
  assert.equal(classifier.replace(addition, '').replace('|experiments/rust-raw-binding-identifiers/', '')
    .replace('|experiments/rust-text-certificate/', '')
    .replace('|experiments/rust-raw-binding-certificate-adapter/', '')
    .replace('|experiments/rust-text-certificate-adapter/', '')
    .replace('|experiments/rust-text-certificate-library/', '')
    .replace('|experiments/original-link-identity/', '')
    .replace('|experiments/original-link-role-projection/', '')
    .replace('|experiments/original-link-symbol-projection/', '')
    .replace('|experiments/original-link-table-projection/', '')
    .replace('|experiments/original-link-complete-table-projection/', '')
    .replace('|experiments/binary-metadata-buffer/', '')
    .replace('|js/scripts/generate-link-network-binary-rules\\.mjs$', '')
    .replace('|js/scripts/binary-metadata-candidate-styles\\.mjs$', '')
    .replace('|js/scripts/source-text-byte-library-style\\.mjs$', '')
    .replace('|js/scripts/product-rule-style-corrections\\.mjs$', '')
    .replace('|experiments/fresh-number-array-collectors/', '')
    .replace('|js/scripts/rust-library-packaging\\.mjs$', '')
    .replace('|experiments/source-native-observations\\.mjs$', ''),
    '^(rust/|parity/|docs/|scripts/|\\.github/workflows/(ci|rust)\\.yml$|\\.secretlintrc\\.json$)',
    'the native source selection preserves every existing Rust workflow trigger');
  const matches = new RegExp(classifier, 'u');
  for (const file of ['model.mjs', 'source.mjs', 'data/fixtures.lino', 'native-observations.mjs', 'native-observations.test.mjs']) {
    assert.ok(matches.test(`experiments/rust-text-value-projection/${file}`), file);
  }
});

test('the complete Rust text source oracle rejects omission, duplication and native gate mutations', () => {
  const mutate = change => {
    const parsed = jobs('rust.yml');
    const steps = parsed.get('test').steps;
    const index = steps.findIndex(step => step.name === 'Run complete Rust text value source observations');
    assert.ok(index >= 0);
    change(steps, index);
    assert.throws(() => checkRustTextValueNativeStep(parsed), assert.AssertionError);
  };
  mutate((steps, index) => steps.splice(index, 1));
  mutate((steps, index) => steps.push({ ...steps[index] }));
  for (const [before, after] of [
    ['!cancelled()', 'success()'],
    ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
    ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
    ["CI: 'true'", "CI: 'false'"],
    ["RUST_TEXT_VALUE_PROJECTION_NATIVE: '1'", "RUST_TEXT_VALUE_PROJECTION_NATIVE: '0'"],
    ['--event test --', '--event build --'],
    ['node --test experiments/', 'node --test --test-name-pattern optional experiments/'],
    ['working-directory: ${{ github.workspace }}', 'working-directory: rust'],
    ['working-directory: ${{ github.workspace }}', 'continue-on-error: true\n        working-directory: ${{ github.workspace }}'],
  ]) {
    mutate((steps, index) => {
      assert.ok(steps[index].text.includes(before));
      steps[index].text = steps[index].text.replace(before, after);
    });
  }
});

test('the complete Rust text source receipt rejects artifact omission and retention mutations', () => {
  const mutate = change => {
    const parsed = jobs('rust.yml');
    const steps = parsed.get('test').steps;
    const index = steps.findIndex(step => step.name === 'Retain complete Rust text value source observations');
    assert.ok(index >= 0);
    change(steps, index);
    assert.throws(() => checkRustTextValueNativeStep(parsed), assert.AssertionError);
  };
  mutate((steps, index) => steps.splice(index, 1));
  mutate((steps, index) => steps.push({ ...steps[index] }));
  for (const [before, after] of [
    ['!cancelled()', 'success()'],
    ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
    ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
    ['uses: actions/upload-artifact@v7', 'uses: actions/upload-artifact@v6'],
    ['${{ github.sha }}', '${{ github.ref }}'],
    ['native-observations.lino', 'native-observations.json'],
    ['if-no-files-found: error', 'if-no-files-found: ignore'],
    ['retention-days: 7', 'retention-days: 0'],
  ]) {
    mutate((steps, index) => {
      assert.ok(steps[index].text.includes(before));
      steps[index].text = steps[index].text.replace(before, after);
    });
  }
});

const RUST_RAW_BINDING_NATIVE_STEPS = [
  ['Run complete Rust raw binding source observations', '7e88baee564180b46d5605767bc25d484fdc51b056d030deeb48e8f7ebd5f62a'],
  ['Retain complete Rust raw binding source observations', '6676c44963694145d41e72076e2919d3c4c59089e8cf2c42d5f5f29d3bce6ff8'],
];

function checkRustRawBindingNativeSteps(parsed) {
  for (const [name, hash] of RUST_RAW_BINDING_NATIVE_STEPS) {
    const steps = parsed.get('test').steps.filter(step => step.name === name);
    assert.equal(steps.length, 1, `the raw-binding source observation step must execute once: ${name}`);
    assert.equal(packageStepSignature(steps[0], true), hash, `raw-binding native step changed: ${name}`);
  }
}

test('Rust raw-binding source observations retain mandatory compilation, receipt upload and source triggers', () => {
  checkRustRawBindingNativeSteps(jobs('rust.yml'));
  const classifier = /rust_paths='([^']+)'/u.exec(workflow('ci.yml'))?.[1];
  for (const addition of ['|experiments/rust-raw-binding-identifiers/', '|experiments/source-native-observations\\.mjs$']) {
    assert.equal(classifier.split(addition).length, 2, 'each native source oracle path is selected exactly once');
  }
  const matches = new RegExp(classifier, 'u');
  for (const file of ['model.mjs', 'syntax.mjs', 'data/fixtures.lino', 'native-observations.mjs', 'native-observations.test.mjs']) {
    assert.ok(matches.test(`experiments/rust-raw-binding-identifiers/${file}`), file);
  }
  assert.ok(matches.test('experiments/source-native-observations.mjs'));
  assert.ok(!matches.test('experiments/source-native-observations.mjs.extra'));
});

test('Rust raw-binding native observation gates refuse omissions, duplication and compiler or artifact weakening', () => {
  for (const [name] of RUST_RAW_BINDING_NATIVE_STEPS) {
    const mutate = change => {
      const parsed = jobs('rust.yml');
      const steps = parsed.get('test').steps;
      const index = steps.findIndex(step => step.name === name);
      assert.ok(index >= 0);
      change(steps, index);
      assert.throws(() => checkRustRawBindingNativeSteps(parsed), assert.AssertionError);
    };
    mutate((steps, index) => steps.splice(index, 1));
    mutate((steps, index) => steps.push({ ...steps[index] }));
    const edits = [
      ['!cancelled()', 'success()'],
      ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
      ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
      ...(name.startsWith('Run ') ? [
        ["CI: 'true'", "CI: 'false'"],
        ["RUST_RAW_BINDING_IDENTIFIER_NATIVE: '1'", "RUST_RAW_BINDING_IDENTIFIER_NATIVE: '0'"],
        ['--event test --', '--event build --'],
        ['node --test experiments/', 'node --test --test-name-pattern optional experiments/'],
        ['working-directory: ${{ github.workspace }}', 'working-directory: rust'],
        ['working-directory: ${{ github.workspace }}', 'continue-on-error: true\n        working-directory: ${{ github.workspace }}'],
      ] : [
        ['uses: actions/upload-artifact@v7', 'uses: actions/upload-artifact@v6'],
        ['${{ github.sha }}', '${{ github.ref }}'],
        ['native-observations.lino', 'native-observations.json'],
        ['if-no-files-found: error', 'if-no-files-found: ignore'],
        ['retention-days: 7', 'retention-days: 0'],
      ]),
    ];
    for (const [before, after] of edits) {
      mutate((steps, index) => {
        assert.ok(steps[index].text.includes(before));
        steps[index].text = steps[index].text.replace(before, after);
      });
    }
  }
});

const RUST_TEXT_CERTIFICATE_NATIVE_STEPS = [
  ['Run complete translated Rust text certificate observations', 'c7609c51a80704d8dbcdf49bb5e790ab5f0672f984ded50f20c30adebf883fcb'],
  ['Retain complete translated Rust text certificate observations', '0f6abe1826a6e6c8e5f56974cf6c1d8464afefeb2b8c7dc03b29c4d09792f7bc'],
];

function checkRustTextCertificateNativeSteps(parsed) {
  for (const [name, hash] of RUST_TEXT_CERTIFICATE_NATIVE_STEPS) {
    const steps = parsed.get('test').steps.filter(step => step.name === name);
    assert.equal(steps.length, 1, `the text-certificate source observation step must execute once: ${name}`);
    assert.equal(packageStepSignature(steps[0], true), hash, `text-certificate native step changed: ${name}`);
  }
}

test('Rust text-certificate source observations retain mandatory compilation, receipt upload and source triggers', () => {
  checkRustTextCertificateNativeSteps(jobs('rust.yml'));
  const classifier = /rust_paths='([^']+)'/u.exec(workflow('ci.yml'))?.[1];
  for (const addition of ['|experiments/rust-text-certificate/']) {
    assert.equal(classifier.split(addition).length, 2, 'each native source oracle path is selected exactly once');
  }
  const matches = new RegExp(classifier, 'u');
  for (const file of ['bridge.mjs', 'rules.mjs', 'data/translation.lino', 'native-observations.mjs', 'native-observations.test.mjs']) {
    assert.ok(matches.test(`experiments/rust-text-certificate/${file}`), file);
  }
  assert.ok(matches.test('experiments/source-native-observations.mjs'));
  assert.ok(!matches.test('experiments/source-native-observations.mjs.extra'));
});

test('Rust text-certificate native observation gates refuse omissions, duplication and compiler or artifact weakening', () => {
  for (const [name] of RUST_TEXT_CERTIFICATE_NATIVE_STEPS) {
    const mutate = change => {
      const parsed = jobs('rust.yml');
      const steps = parsed.get('test').steps;
      const index = steps.findIndex(step => step.name === name);
      assert.ok(index >= 0);
      change(steps, index);
      assert.throws(() => checkRustTextCertificateNativeSteps(parsed), assert.AssertionError);
    };
    mutate((steps, index) => steps.splice(index, 1));
    mutate((steps, index) => steps.push({ ...steps[index] }));
    const edits = [
      ['!cancelled()', 'success()'],
      ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
      ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
      ...(name.startsWith('Run ') ? [
        ["CI: 'true'", "CI: 'false'"],
        ["RUST_TEXT_CERTIFICATE_NATIVE: '1'", "RUST_TEXT_CERTIFICATE_NATIVE: '0'"],
        ['--event test --', '--event build --'],
        ['node --test experiments/', 'node --test --test-name-pattern optional experiments/'],
        ['working-directory: ${{ github.workspace }}', 'working-directory: rust'],
        ['working-directory: ${{ github.workspace }}', 'continue-on-error: true\n        working-directory: ${{ github.workspace }}'],
      ] : [
        ['uses: actions/upload-artifact@v7', 'uses: actions/upload-artifact@v6'],
        ['${{ github.sha }}', '${{ github.ref }}'],
        ['native-observations.lino', 'native-observations.json'],
        ['if-no-files-found: error', 'if-no-files-found: ignore'],
        ['retention-days: 7', 'retention-days: 0'],
      ]),
    ];
    for (const [before, after] of edits) {
      mutate((steps, index) => {
        assert.ok(steps[index].text.includes(before));
        steps[index].text = steps[index].text.replace(before, after);
      });
    }
  }
});

const RUST_RAW_BINDING_RULE_NATIVE_STEPS = [
  ['Run complete Rust raw binding rule observations', '60fb15bf3398a6a186d26f0ec2c6adf716c0b5e035370a55333be6b4a066ef87'],
  ['Retain complete Rust raw binding rule observations', 'b697ffbc4631897dac90caca1df58104d8f00c9ddc26d1f52e6efcb0c2ef0430'],
];

function checkRustRawBindingRuleNativeSteps(parsed) {
  for (const [name, hash] of RUST_RAW_BINDING_RULE_NATIVE_STEPS) {
    const steps = parsed.get('test').steps.filter(step => step.name === name);
    assert.equal(steps.length, 1, `the raw-binding rule source observation step must execute once: ${name}`);
    assert.equal(packageStepSignature(steps[0], true), hash, `raw-binding rule native step changed: ${name}`);
  }
}

test('Rust raw-binding rule source observations retain mandatory compilation, receipt upload and source triggers', () => {
  checkRustRawBindingRuleNativeSteps(jobs('rust.yml'));
  const classifier = /rust_paths='([^']+)'/u.exec(workflow('ci.yml'))?.[1];
  for (const addition of ['|experiments/rust-raw-binding-certificate-adapter/']) {
    assert.equal(classifier.split(addition).length, 2, 'each native source oracle path is selected exactly once');
  }
  const matches = new RegExp(classifier, 'u');
  for (const file of ['native-observations.mjs', 'native-observations.test.mjs']) {
    assert.ok(matches.test(`experiments/rust-raw-binding-certificate-adapter/${file}`), file);
  }
  assert.ok(matches.test('experiments/source-native-observations.mjs'));
  assert.ok(!matches.test('experiments/source-native-observations.mjs.extra'));
});

test('Rust raw-binding rule native observation gates refuse omissions, duplication and compiler or artifact weakening', () => {
  for (const [name] of RUST_RAW_BINDING_RULE_NATIVE_STEPS) {
    const mutate = change => {
      const parsed = jobs('rust.yml');
      const steps = parsed.get('test').steps;
      const index = steps.findIndex(step => step.name === name);
      assert.ok(index >= 0);
      change(steps, index);
      assert.throws(() => checkRustRawBindingRuleNativeSteps(parsed), assert.AssertionError);
    };
    mutate((steps, index) => steps.splice(index, 1));
    mutate((steps, index) => steps.push({ ...steps[index] }));
    const edits = [
      ['!cancelled()', 'success()'],
      ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
      ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
      ...(name.startsWith('Run ') ? [
        ["CI: 'true'", "CI: 'false'"],
        ["RUST_RAW_BINDING_RULE_NATIVE: '1'", "RUST_RAW_BINDING_RULE_NATIVE: '0'"],
        ['--event test --', '--event build --'],
        ['node --test experiments/', 'node --test --test-name-pattern optional experiments/'],
        ['working-directory: ${{ github.workspace }}', 'working-directory: rust'],
        ['working-directory: ${{ github.workspace }}', 'continue-on-error: true\n        working-directory: ${{ github.workspace }}'],
      ] : [
        ['uses: actions/upload-artifact@v7', 'uses: actions/upload-artifact@v6'],
        ['${{ github.sha }}', '${{ github.ref }}'],
        ['native-observations.lino', 'native-observations.json'],
        ['if-no-files-found: error', 'if-no-files-found: ignore'],
        ['retention-days: 7', 'retention-days: 0'],
      ]),
    ];
    for (const [before, after] of edits) {
      mutate((steps, index) => {
        assert.ok(steps[index].text.includes(before));
        steps[index].text = steps[index].text.replace(before, after);
      });
    }
  }
});

const RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE_STEPS = [
  ['Run complete Rust text certificate library observations', '3cc6f191da33b2065b98915a0622858d60fc9b2a7ac2a699acbb8f16c1c3df8a'],
  ['Retain complete Rust text certificate library observations', '773b4482ac2646ce711c0d5db13ac83fc151a6c339fb7815c0ba38e60bce9a17'],
];

function checkRustTextCertificateLibraryNativeSteps(parsed) {
  for (const [name, hash] of RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE_STEPS) {
    const steps = parsed.get('test').steps.filter(step => step.name === name);
    assert.equal(steps.length, 1, `the complete library observation step executes once: ${name}`);
    assert.equal(packageStepSignature(steps[0], true), hash, `complete library native step changed: ${name}`);
  }
}

test('the complete generated Rust library retains mandatory native observations and source selection', () => {
  checkRustTextCertificateLibraryNativeSteps(jobs('rust.yml'));
  assert.ok(needs(jobs('ci.yml').get('rust')).includes('js'));
  const classifier = /rust_paths='([^']+)'/u.exec(workflow('ci.yml'))?.[1];
  const matches = new RegExp(classifier, 'u');
  for (const file of ['data/library.lino', 'data/decorators.lino', 'native-observations.mjs', 'native-observations.test.mjs']) {
    assert.ok(matches.test(`experiments/rust-text-certificate-library/${file}`), file);
  }
});

test('complete generated Rust library observations refuse compiler or receipt gate weakening', () => {
  for (const [name] of RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE_STEPS) {
    const mutate = change => {
      const parsed = jobs('rust.yml'), steps = parsed.get('test').steps;
      const index = steps.findIndex(step => step.name === name);
      assert.ok(index >= 0);
      change(steps, index);
      assert.throws(() => checkRustTextCertificateLibraryNativeSteps(parsed), assert.AssertionError);
    };
    mutate((steps, index) => steps.splice(index, 1));
    mutate((steps, index) => steps.push({ ...steps[index] }));
    for (const [from, to] of [
      ['!cancelled()', 'success()'],
      ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
      ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
      ...(name.startsWith('Run ') ? [
        ["CI: 'true'", "CI: 'false'"],
        ["RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE: '1'", "RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE: '0'"],
        ['--event test --', '--event build --'],
        ['node --test experiments/', 'node --test --test-name-pattern optional experiments/'],
        ['working-directory: ${{ github.workspace }}', 'working-directory: rust'],
        ['working-directory: ${{ github.workspace }}', 'continue-on-error: true\n        working-directory: ${{ github.workspace }}'],
      ] : [
        ['uses: actions/upload-artifact@v7', 'uses: actions/upload-artifact@v6'],
        ['${{ github.sha }}', '${{ github.ref }}'],
        ['native-observations.lino', 'native-observations.json'],
        ['if-no-files-found: error', 'if-no-files-found: ignore'],
        ['retention-days: 7', 'retention-days: 0'],
      ]),
    ]) mutate((steps, index) => {
      assert.ok(steps[index].text.includes(from));
      steps[index].text = steps[index].text.replace(from, to);
    });
  }
});

const ORIGINAL_LINK_IDENTITY_NATIVE_STEPS = [
  ['Run complete original link identity observations', 'aee6d2b321dcd0e99cfe2a4ff4d172d1e2b18aa4e23ba0f076ffba32fe615134'],
  ['Retain complete original link identity observations', '1f5a23112645595696663d701691ed2761397745490d7ef7e104e313760a1d76'],
];

function checkOriginalLinkIdentityNativeSteps(parsed) {
  for (const [name, hash] of ORIGINAL_LINK_IDENTITY_NATIVE_STEPS) {
    const steps = parsed.get('test').steps.filter(step => step.name === name);
    assert.equal(steps.length, 1, `the complete identity observation step executes once: ${name}`);
    assert.equal(packageStepSignature(steps[0], true), hash, `complete identity native step changed: ${name}`);
  }
}

test('exact original link identity observations require JavaScript, native execution and complete source selection', () => {
  checkOriginalLinkIdentityNativeSteps(jobs('rust.yml'));
  assert.ok(needs(jobs('ci.yml').get('rust')).includes('js'));
  const classifier = /rust_paths='([^']+)'/u.exec(workflow('ci.yml'))?.[1];
  assert.equal(classifier.split('|experiments/original-link-identity/').length, 2);
  const matches = new RegExp(classifier, 'u');
  for (const file of ['policy.mjs', 'rules.mjs', 'generate.mjs', 'data/policy.lino', 'data/native-definitions.lino',
    'identity.test.mjs', 'native-observations.mjs', 'native-observations.test.mjs']) {
    assert.ok(matches.test(`experiments/original-link-identity/${file}`), file);
  }
});

test('exact original link identity native gates reject execution and receipt weakening', () => {
  for (const [name] of ORIGINAL_LINK_IDENTITY_NATIVE_STEPS) {
    const mutate = change => {
      const parsed = jobs('rust.yml'), steps = parsed.get('test').steps;
      const index = steps.findIndex(step => step.name === name);
      assert.ok(index >= 0);
      change(steps, index);
      assert.throws(() => checkOriginalLinkIdentityNativeSteps(parsed), assert.AssertionError);
    };
    mutate((steps, index) => steps.splice(index, 1));
    mutate((steps, index) => steps.push({ ...steps[index] }));
    for (const [from, to] of [
      ['!cancelled()', 'success()'],
      ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
      ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
      ...(name.startsWith('Run ') ? [
        ["CI: 'true'", "CI: 'false'"],
        ["ORIGINAL_LINK_IDENTITY_NATIVE: '1'", "ORIGINAL_LINK_IDENTITY_NATIVE: '0'"],
        ['--event test --', '--event build --'],
        ['node --test experiments/', 'node --test --test-name-pattern optional experiments/'],
        ['working-directory: ${{ github.workspace }}', 'working-directory: rust'],
        ['working-directory: ${{ github.workspace }}', 'continue-on-error: true\n        working-directory: ${{ github.workspace }}'],
      ] : [
        ['uses: actions/upload-artifact@v7', 'uses: actions/upload-artifact@v6'],
        ['${{ github.sha }}', '${{ github.ref }}'],
        ['native-observations.lino', 'native-observations.json'],
        ['if-no-files-found: error', 'if-no-files-found: ignore'],
        ['retention-days: 7', 'retention-days: 0'],
      ]),
    ]) mutate((steps, index) => {
      assert.ok(steps[index].text.includes(from));
      steps[index].text = steps[index].text.replace(from, to);
    });
  }
});

const ORIGINAL_LINK_SYMBOL_NATIVE_STEPS = [
  ['Run complete original link symbol observations', 'db01f6b5227e91fbb3418482ef26703301cdc2629b0c184e19f2e66c543e4eeb'],
  ['Retain complete original link symbol observations', 'b74c66243d9d077ba5d84277a4e37a7948eaec3ea08adfdaca024ba2f938e80f'],
];

function checkOriginalLinkSymbolNativeSteps(parsed) {
  for (const [name, hash] of ORIGINAL_LINK_SYMBOL_NATIVE_STEPS) {
    const steps = parsed.get('test').steps.filter(step => step.name === name);
    assert.equal(steps.length, 1, `the complete symbol observation step executes once: ${name}`);
    assert.equal(packageStepSignature(steps[0], true), hash, `complete symbol native step changed: ${name}`);
  }
}

test('complete original symbol observations retain injected JavaScript contracts and mandatory native execution after JavaScript', () => {
  checkPackageStepConservation(jobs('js.yml'));checkOriginalLinkSymbolNativeSteps(jobs('rust.yml'));
  assert.ok(needs(jobs('ci.yml').get('rust')).includes('js'));
  assert.match(read('experiments/original-link-symbol-projection/native-observations.test.mjs'), /if\(nativeEnabled\)test\(/u);
  for (const file of ['native-observations.mjs', 'native-observations.test.mjs']) {
    const source = `experiments/original-link-symbol-projection/${file}`, chain = nativeInputJobChain([source]);
    assert.deepEqual(chain.selection, { rust: true, native_test_inputs_changed: true });assert.equal(chain.test, true);
    assert.equal(nativeInputJobChain([source], { javascript: 'failure' }).outer, false);
  }
});

test('complete original symbol observation gates reject native execution and receipt weakening', () => {
  for (const [name] of ORIGINAL_LINK_SYMBOL_NATIVE_STEPS) {
    const mutate = change => {
      const parsed = jobs('rust.yml'), steps = parsed.get('test').steps, index = steps.findIndex(step => step.name === name);assert.ok(index >= 0);
      change(steps, index);assert.throws(() => checkOriginalLinkSymbolNativeSteps(parsed), assert.AssertionError);
    };
    mutate((steps, index) => steps.splice(index, 1));mutate((steps, index) => steps.push({ ...steps[index] }));
    for (const [from, to] of [
      ['!cancelled()', 'success()'],
      ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
      ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
      ...(name.startsWith('Run ') ? [
        ["CI: 'true'", "CI: 'false'"], ["ORIGINAL_LINK_SYMBOL_NATIVE: '1'", "ORIGINAL_LINK_SYMBOL_NATIVE: '0'"],
        ['--event test --', '--event build --'], ['node --test experiments/', 'node --test --test-name-pattern optional experiments/'],
        ['working-directory: ${{ github.workspace }}', 'working-directory: rust'],
        ['working-directory: ${{ github.workspace }}', 'continue-on-error: true\n        working-directory: ${{ github.workspace }}'],
      ] : [
        ['uses: actions/upload-artifact@v7', 'uses: actions/upload-artifact@v6'], ['${{ github.sha }}', '${{ github.ref }}'],
        ['native-observations.lino', 'native-observations.json'], ['if-no-files-found: error', 'if-no-files-found: ignore'], ['retention-days: 7', 'retention-days: 0'],
      ]),
    ]) mutate((steps, index) => {assert.ok(steps[index].text.includes(from));steps[index].text = steps[index].text.replace(from, to);});
  }
  const parsed = jobs('js.yml'), step = parsed.get('package-checks').steps.find(step => step.name === ORIGINAL_LINK_SYMBOL_ORACLE_PACKAGE_STEPS[0][0]);
  assert.ok(step);step.text = step.text.replace("ORIGINAL_LINK_SYMBOL_NATIVE: '0'", "ORIGINAL_LINK_SYMBOL_NATIVE: '1'");
  assert.throws(() => checkPackageStepConservation(parsed), assert.AssertionError);
});

function checkOriginalLinkProjectionPaths(classifier) {
  const matches = new RegExp(classifier, 'u');
  for (const name of ['symbol', 'table']) {
    const prefix = `experiments/original-link-${name}-projection/`;
    assert.equal(classifier.split(`|${prefix}`).length, 2, `${prefix} is registered exactly once`);
    for (const file of ['generate.mjs', 'projection.test.mjs', 'policy.mjs', 'rules.mjs', 'transport.mjs', 'data/policy.lino', 'data/native-definitions.lino']) assert.ok(matches.test(prefix + file), prefix + file);
    assert.equal(matches.test(prefix.replace(/\/$/u, '-other/projection.test.mjs')), false);
  }
}

test('original linked symbol and table projection gates retain complete paths and reach native prerequisites', () => {
  checkPackageStepConservation(jobs('js.yml'));
  const classifier = /rust_paths='([^']+)'/u.exec(workflow('ci.yml'))?.[1];checkOriginalLinkProjectionPaths(classifier);
  for (const name of ['symbol', 'table']) {
    const prefix = `experiments/original-link-${name}-projection/`;
    for (const file of ['generate.mjs', 'projection.test.mjs']) assert.ok(read(prefix + file).length > 0);
    for (const file of ['projection.test.mjs', 'data/policy.lino']) {
      const chain = nativeInputJobChain([prefix + file]);
      assert.deepEqual(chain.selection, { rust: true, native_test_inputs_changed: true });
      for (const job of ['outer', 'cargo-lock', 'check', 'test']) assert.equal(chain[job], true, prefix + file + ': ' + job);
      assert.equal(chain.changelog, false);
      assert.equal(nativeInputJobChain([prefix + file], { javascript: 'failure' }).outer, false);
    }
    assert.equal(nativeInputJobChain([prefix + 'README.md']).selection.native_test_inputs_changed, false);
    for (const forged of [classifier.replace(`|${prefix}`, ''), classifier.replace(`|${prefix}`, `|${prefix}|${prefix}`), classifier.replace(`|${prefix}`, `|${prefix}other/`)]) assert.throws(() => checkOriginalLinkProjectionPaths(forged), assert.AssertionError);
  }
});

const BINARY_METADATA_NATIVE_STEPS = [
  ['Run complete binary metadata buffer candidate observations', '1ce517ff5e2b6cffb6240bd0b75c28f7164a9c81321d8af6cca9ad8c885e7fbf'],
  ['Retain complete binary metadata buffer candidate observations', '099e5b6d8e1f8657c02ed895c60335cb694599318a0a788d8c04a0fcae1e252f'],
];

const COMPLETE_TABLE_NATIVE_STEPS = [
  ['Run complete original link table observations', 'dfff389c0b95019abc112515449504b3e643e4870497e15a2c37cad5bb62c3de'],
  ['Retain complete original link table observations', '01f4bc2789f501b76a7ed435cd51dff0db73831f074e6645e8072615f2be8df5'],
];

function checkBinaryMetadataNativeSteps(parsed) {
  for (const [name, hash] of BINARY_METADATA_NATIVE_STEPS) {
    const steps = parsed.get('test').steps.filter(step => step.name === name);
    assert.equal(steps.length, 1, `the binary candidate native observation step executes once: ${name}`);
    assert.equal(packageStepSignature(steps[0], true), hash, `binary candidate native step changed: ${name}`);
  }
}

function checkCompleteTableNativeSteps(parsed) {
  for (const [name, hash] of COMPLETE_TABLE_NATIVE_STEPS) {
    const steps = parsed.get('test').steps.filter(step => step.name === name);
    assert.equal(steps.length, 1, `the complete table native observation step executes once: ${name}`);
    assert.equal(packageStepSignature(steps[0], true), hash, `complete table native step changed: ${name}`);
  }
}

test('new source-model cells append to the original seven cells without moving any existing checks', () => {
  const parsed = jobs('js.yml'), cells = checkGeneratedPackageSlices(parsed);
  assert.deepEqual(cells.slice(7), ADDITIONAL_SOURCE_PACKAGE_CELLS);
  checkPackageStepConservation(parsed);
  const checks = parsed.get('package-checks').steps;
  for (const table of [COMPLETE_TABLE_PACKAGE_STEPS, BINARY_METADATA_PACKAGE_STEPS]) {
    const owner = table[0][3];
    assert.deepEqual(checks.filter(step => step.text.includes(`matrix.group == '${owner}'`)).map(step => step.name),
      table.map(([name]) => name), 'new generators, injected contracts and formatting prerequisites retain their producer order');
  }
  assert.ok(checks.findIndex(step => step.name === BINARY_METADATA_PACKAGE_STEPS[0][0]) <
    checks.findIndex(step => step.name === BINARY_METADATA_PACKAGE_STEPS[1][0]));
});

test('new source-model cells reject missing, repeated, reassigned and weakened checks', () => {
  for (const [name, , , owner] of [...COMPLETE_TABLE_PACKAGE_STEPS, ...BINARY_METADATA_PACKAGE_STEPS]) {
    const mutate = change => {
      const parsed = jobs('js.yml'), steps = parsed.get('package-checks').steps;
      const index = steps.findIndex(step => step.name === name);
      assert.ok(index >= 0);
      change(steps, index);
      assert.throws(() => checkPackageStepConservation(parsed), assert.AssertionError, name);
    };
    mutate((steps, index) => steps.splice(index, 1));
    mutate((steps, index) => steps.push({ ...steps[index] }));
    for (const [before, after] of [
      ['!cancelled()', 'success()'],
      [`matrix.group == '${owner}'`, "matrix.group == 'source-models'"],
      [name, name + '\n        continue-on-error: true'],
      ...(name.startsWith('Run ') ? [
        ['--event test --', '--event build --'],
        ['--test-concurrency=1', '--test-concurrency=2'],
        ["TRANSLATION_CHECK_RUST: '0'", "TRANSLATION_CHECK_RUST: '1'"],
        ...(name.includes('binary metadata') ? [["BINARY_METADATA_BUFFER_NATIVE: '0'", "BINARY_METADATA_BUFFER_NATIVE: '1'"]] : []),
        ...(name === 'Run complete original link table native oracle contracts' ? [
          ["CI: 'true'", "CI: 'false'"],
          ["ORIGINAL_LINK_COMPLETE_TABLE_NATIVE: '0'", "ORIGINAL_LINK_COMPLETE_TABLE_NATIVE: '1'"],
        ] : []),
      ] : name.startsWith('Check ') ? [['--check', '--javascript-only']] : [['components: rustfmt, clippy', 'components: clippy']]),
    ]) mutate((steps, index) => {
      assert.ok(steps[index].text.includes(before), `${name}: ${before}`);
      steps[index].text = steps[index].text.replace(before, after);
    });
  }
  for (const cell of ADDITIONAL_SOURCE_PACKAGE_CELLS) {
    const row = `          - group: ${cell.group}\n            generated_slice: first\n`;
    for (const replacement of ['', row + row, row.replace('first', 'second'), row.replace(cell.group, 'source-models')]) {
      const parsed = jobs('js.yml'), checks = parsed.get('package-checks');
      assert.ok(checks.header.includes(row));
      checks.header = checks.header.replace(row, replacement);
      assert.throws(() => checkGeneratedPackageSlices(parsed), assert.AssertionError);
    }
  }
});

test('new native source paths preserve the exact old selector and reach every native prerequisite after JavaScript', () => {
  const step = jobs('ci.yml').get('changes').steps.find(step => step.name === 'Detect whether the Rust workflow is needed');
  assert.ok(step);
  const additions = ['experiments/original-link-complete-table-projection/', 'experiments/binary-metadata-buffer/',
    'js/scripts/generate-link-network-binary-rules\\.mjs$', 'js/scripts/binary-metadata-candidate-styles\\.mjs$',
    'js/scripts/source-text-byte-library-style\\.mjs$', 'js/scripts/product-rule-style-corrections\\.mjs$'];
  let original = step.run;
  const classifier = /rust_paths='([^']+)'/u.exec(step.run)[1], matches = new RegExp(classifier, 'u');
  for (const addition of additions) {
    assert.equal(classifier.split('|' + addition).length, 2, `${addition} is registered exactly once`);
    original = original.replace('|' + addition, '');
  }
  const inner = '|^js/scripts/(generate-link-network-binary-rules|binary-metadata-candidate-styles|source-text-byte-library-style|product-rule-style-corrections)\\.mjs$';
  assert.equal(original.split(inner).length, 2);
  original = original.replace(inner, '');
  assert.equal(createHash('sha256').update(original).digest('hex'), 'd66cc6c3e7004b34a9914cefb9548150933b57939f700112f7704493906ed633',
    'removing only the six registered additions restores every old selector byte');
  const paths = [
    'experiments/original-link-complete-table-projection/data/native-definitions.lino',
    'experiments/original-link-complete-table-projection/projection.test.mjs',
    'experiments/original-link-complete-table-projection/native-observations.mjs',
    'experiments/original-link-complete-table-projection/native-observations.test.mjs',
    'experiments/binary-metadata-buffer/native-observations.test.mjs',
    'experiments/binary-metadata-buffer/native/part_0.rs',
    ...additions.slice(2).map(file => file.replace('\\.', '.').replace(/\$$/u, '')),
  ];
  for (const file of paths) {
    assert.ok(matches.test(file), file);
    const chain = nativeInputJobChain([file]);
    assert.deepEqual(chain.selection, { rust: true, native_test_inputs_changed: true }, file);
    for (const job of ['outer', 'cargo-lock', 'check', 'test']) assert.equal(chain[job], true, `${file}: ${job}`);
    assert.equal(nativeInputJobChain([file], { javascript: 'failure' }).outer, false, file);
  }
  for (const file of ['experiments/original-link-complete-table-projection/README.md',
    'experiments/binary-metadata-buffer/README.md', 'experiments/original-link-complete-table-projection-other/model.mjs',
    'experiments/binary-metadata-buffer-other/model.mjs', 'js/scripts/binary-metadata-candidate-styles.mjs.extra']) {
    assert.equal(nativeInputJobChain([file]).selection.native_test_inputs_changed, false, file);
  }
});

test('complete table and binary native observations require real native switches and mandatory exact-commit receipts', () => {
  const families = [
    [BINARY_METADATA_NATIVE_STEPS, checkBinaryMetadataNativeSteps, 'BINARY_METADATA_BUFFER_NATIVE', 'binary-metadata-buffer'],
    [COMPLETE_TABLE_NATIVE_STEPS, checkCompleteTableNativeSteps, 'ORIGINAL_LINK_COMPLETE_TABLE_NATIVE', 'original-link-complete-table-projection'],
  ];
  for (const [table, check, nativeSwitch, directory] of families) {
    check(jobs('rust.yml'));
    for (const file of ['native-observations.mjs', 'native-observations.test.mjs']) assert.ok(read(`experiments/${directory}/${file}`).length > 0);
    assert.ok(needs(jobs('ci.yml').get('rust')).includes('js'));
    for (const [name] of table) {
      const mutate = change => {
        const parsed = jobs('rust.yml'), steps = parsed.get('test').steps;
        const index = steps.findIndex(step => step.name === name);
        assert.ok(index >= 0);
        change(steps, index);
        assert.throws(() => check(parsed), assert.AssertionError);
      };
      mutate((steps, index) => steps.splice(index, 1));
      mutate((steps, index) => steps.push({ ...steps[index] }));
      for (const [before, after] of [
        ['!cancelled()', 'success()'],
        ["matrix.suite == 'translation'", "matrix.suite == 'remaining'"],
        ["matrix.os == 'ubuntu-latest'", "matrix.os == 'windows-latest'"],
        [name, name + '\n        continue-on-error: true'],
        ...(name.startsWith('Run ') ? [
          ["CI: 'true'", "CI: 'false'"], [`${nativeSwitch}: '1'`, `${nativeSwitch}: '0'`],
          ['--event test --', '--event build --'], ['--test-concurrency=1', '--test-concurrency=2'],
          ['working-directory: ${{ github.workspace }}', 'working-directory: rust'],
        ] : [
          ['uses: actions/upload-artifact@v7', 'uses: actions/upload-artifact@v6'], ['${{ github.sha }}', '${{ github.ref }}'],
          ['native-observations.lino', 'native-observations.json'], ['if-no-files-found: error', 'if-no-files-found: ignore'],
          ['retention-days: 7', 'retention-days: 0'],
        ]),
      ]) mutate((steps, index) => {
        assert.ok(steps[index].text.includes(before));
        steps[index].text = steps[index].text.replace(before, after);
      });
    }
  }
});

function checkPackageStepConservation(parsed) {
  const cells = checkGeneratedPackageSlices(parsed);
  const additions = [...ADDITIONAL_PACKAGE_STEPS, ...EMPTY_OPTION_PACKAGE_STEPS, ...TYPE_NAMESPACE_PACKAGE_STEPS,
    ...PURE_UNIT_PACKAGE_STEPS, ...SOURCE_SPAN_PACKAGE_STEPS, ...OWNED_TEXT_GENERATOR_PACKAGE_STEPS,
    ...OWNED_TEXT_EXPERIMENT_PACKAGE_STEPS, ...INTEGRAL_METADATA_PACKAGE_STEPS, ...INTEGRAL_METADATA_EXPERIMENT_PACKAGE_STEPS,
    ...CONST_AND_BYTE_GENERATOR_PACKAGE_STEPS, ...CONST_AND_BYTE_EXPERIMENT_PACKAGE_STEPS, ...RUNTIME_SHELL_PACKAGE_STEPS, ...RUST_NUMBER_PACKAGE_STEPS,
    ...JAVASCRIPT_LINE_TERMINATOR_PACKAGE_STEPS, ...GENERATED_JAVASCRIPT_UNIT_PACKAGE_STEPS,
    ...GENERATED_JAVASCRIPT_UNIT_EXPERIMENT_STEPS,
    ...OWNED_TUPLE_CALL_GENERATOR_PACKAGE_STEPS, ...OWNED_TUPLE_CALL_EXPERIMENT_PACKAGE_STEPS,
    ...RUST_LEXICAL_AND_OWNED_MODULE_GENERATOR_STEPS, ...GENERATED_OWNED_RUST_MODULE_EXPERIMENT_STEPS,
    ...RUST_RAW_STRING_AND_LITERAL_EDIT_GENERATOR_STEPS, ...OWNED_RUST_LITERAL_EDIT_EXPERIMENT_STEPS,
    ...RUST_TEXT_VALUE_PROJECTION_GENERATOR_STEPS, ...RUST_TEXT_VALUE_PROJECTION_EXPERIMENT_STEPS,
    ...RUST_RAW_BINDING_IDENTIFIER_GENERATOR_STEPS, ...RUST_RAW_BINDING_IDENTIFIER_EXPERIMENT_STEPS,
    ...RUST_TEXT_CERTIFICATE_GENERATOR_STEPS, ...RUST_TEXT_CERTIFICATE_EXPERIMENT_STEPS,
    ...OWNED_SOURCE_ADAPTER_PACKAGE_STEPS, ...ORIGINAL_LINK_PROJECTION_PACKAGE_STEPS, ...ORIGINAL_LINK_SYMBOL_ORACLE_PACKAGE_STEPS,
    ...COMPLETE_TABLE_PACKAGE_STEPS, ...BINARY_METADATA_PACKAGE_STEPS];
  const expected = new Map([
    ...ORIGINAL_PACKAGE_STEPS.map(([name, hash, owner]) => [name, { hash, owner }]),
    ...additions.map(([name, command, hash, owner]) => [name, { hash, owner, command, addition: true }]),
  ]);
  assert.equal(expected.size, ORIGINAL_PACKAGE_STEPS.length + additions.length,
    'additive gates cannot shadow an original step or another addition');
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
      if (baseline.addition) assert.equal(step.run.trim(), baseline.command, `additive gate command changed: ${step.name}`);
      assert.equal(packageStepSignature(step, baseline.addition), baseline.hash, `package step semantics changed: ${step.name}`);
      if (id === 'package-checks') {
        assert.equal((step.text.match(/matrix\.group/gu) ?? []).length, baseline.owner === 'all' ? 0 : 1,
          `package step has an ambiguous group selector: ${step.name}`);
        assert.ok(group === undefined || PACKAGE_GROUPS.includes(group) ||
          ADDITIONAL_SOURCE_PACKAGE_CELLS.some(cell => cell.group === group));
      }
      const slice = /matrix\.generated_slice == '([a-z]+)'/u.exec(step.text)?.[1];
      const executions = id === 'package-checks'
        ? cells.filter(cell => (!group || cell.group === group) && (!slice || cell.generated_slice === slice)).length : 1;
      found.set(step.name, (found.get(step.name) ?? 0) + executions);
    }
  }
  for (const [name, baseline] of expected) {
    if (baseline.owner === 'all') {
      for (const id of ['package-checks', 'test']) assert.ok(shared.has(`${id}:${name}`), `shared package setup/cleanup missing: ${id}:${name}`);
    }
    const executions = baseline.owner === 'all' ? cells.length + 1 : name === GENERATED_PACKAGE_SHARED_SETUP ? 2 : 1;
    assert.equal(found.get(name), executions,
      `package step omitted or duplicated: ${name}`);
  }
}

test('package partitions conserve every original check, condition and artifact once under the ten-minute deadline', () => {
  const parsed = jobs('js.yml');
  const checks = parsed.get('package-checks');
  const groups = /^ {8}group: \[([^\]]+)\]/mu.exec(checks.header)?.[1].split(',').map(value => value.trim());
  assert.deepEqual(groups.slice(0, 5), ['native-imports', 'native-fixtures', 'grammar', 'contracts', 'generated'],
    'all five published partitions remain required in their original order');
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
  assert.deepEqual(needs(parsed.get('test')), ['tests', 'report-assertions', 'package-checks', 'translation-test-feedback']);
  assert.deepEqual(needs(parsed.get('publish')), ['test']);
  assert.match(parsed.get('publish').header, /needs.test.result == 'success'/u);
});

test('generated package slices preserve six logical groups and execute every check exactly once across seven cells', () => {
  const parsed = jobs('js.yml');
  const cells = checkGeneratedPackageSlices(parsed);
  assert.deepEqual(cells.slice(0, 7), [
    ...PACKAGE_GROUPS.map(group => ({ group, generated_slice: 'first' })),
    { group: 'generated', generated_slice: 'second' },
  ]);
  assert.deepEqual(GENERATED_PACKAGE_SLICE_STEPS.first, [
    'Check generated owned Rust literal value edit experiment rules',
    'Check complete generated owned Rust literal value edit fixtures',
    'Run generated owned Rust literal value edit experiment',
  ]);
  assert.equal(GENERATED_PACKAGE_SLICE_STEPS.second.length, 123);
  const checks = parsed.get('package-checks').steps;
  for (const slice of GENERATED_PACKAGE_SLICES) {
    for (const name of GENERATED_PACKAGE_SLICE_STEPS[slice]) {
      const step = checks.find(step => step.name === name);
      assert.ok(step.afterFailure, `${name} must report its own failure after an earlier failed check`);
      assert.equal(cells.filter(cell => cell.group === 'generated' && cell.generated_slice === slice).length, 1);
      assert.equal(normalizeGeneratedPackageSlice(step).includes('matrix.generated_slice'), false);
    }
  }
  checkPackageStepConservation(parsed);
});

test('generated slice conservation refuses every omitted, duplicated, reassigned or altered check', () => {
  for (const slice of GENERATED_PACKAGE_SLICES) {
    for (const name of GENERATED_PACKAGE_SLICE_STEPS[slice]) {
      const mutate = change => {
        const parsed = jobs('js.yml'), steps = parsed.get('package-checks').steps;
        const index = steps.findIndex(step => step.name === name);
        assert.ok(index >= 0);
        change(steps, index);
        assert.throws(() => checkPackageStepConservation(parsed), assert.AssertionError, name);
      };
      mutate((steps, index) => steps.splice(index, 1));
      mutate((steps, index) => steps.push({ ...steps[index] }));
      for (const [before, after] of [
        [` && matrix.generated_slice == '${slice}'`, ''],
        [`matrix.generated_slice == '${slice}'`, `matrix.generated_slice == '${slice === 'first' ? 'second' : 'first'}'`],
        [`matrix.generated_slice == '${slice}'`, `(matrix.generated_slice == '${slice}' || true)`],
        ['!cancelled()', 'success()'],
        ["matrix.group == 'generated'", "matrix.group == 'contracts'"],
      ]) mutate((steps, index) => {
        assert.ok(steps[index].text.includes(before));
        steps[index].text = steps[index].text.replace(before, after);
      });
      mutate((steps, index) => {
        const key = steps[index].run ? 'run:' : 'uses:';
        assert.ok(steps[index].text.includes(key));
        steps[index].text = steps[index].text.replace(key, `${key} optional-replacement `);
      });
    }
  }
});

test('physical generated cells reject exclusions, extra cells, shared concurrency and longer deadlines', () => {
  const mutate = change => {
    const parsed = jobs('js.yml');
    change(parsed);
    assert.throws(() => checkGeneratedPackageSlices(parsed), assert.AssertionError);
  };
  for (const [before, after] of [
    ['        generated_slice: [first]\n', '        generated_slice: [first, second]\n'],
    ['          - group: generated\n            generated_slice: second\n', ''],
    ['            generated_slice: second\n', '            generated_slice: first\n'],
    ['            generated_slice: second\n', '            generated_slice: second\n          - group: generated\n            generated_slice: second\n'],
    ['        include:\n', '        exclude:\n          - group: generated\n            generated_slice: first\n        include:\n'],
    ['            generated_slice: second\n', '            generated_slice: second\n\n        exclude:\n          - group: generated\n            generated_slice: second\n'],
    ['generated, source-models]', 'source-models]'],
    ['-${{ matrix.generated_slice }}', ''],
    ['      cancel-in-progress: true', '      group: shared-generated-runner\n      cancel-in-progress: true'],
    ["${{ matrix.generated_slice == 'second' && ' / second' || '' }}", ''],
    ['      fail-fast: false', '      fail-fast: true'],
  ]) mutate(parsed => {
    const job = parsed.get('package-checks');
    assert.ok(job.header.includes(before));
    job.header = job.header.replace(before, after);
  });
  for (const id of ['package-checks', 'test']) {
    for (const minutes of ['9', '11', '20']) mutate(parsed => {
      parsed.get(id).header = parsed.get(id).header.replace('timeout-minutes: 10', `timeout-minutes: ${minutes}`);
    });
    mutate(parsed => {
      parsed.get(id).header = parsed.get(id).header.replace('if: ${{ !cancelled() }}', 'if: ${{ success() }}');
    });
  }
  mutate(parsed => {
    const steps = parsed.get('package-checks').steps;
    const index = steps.findIndex(step => step.name === GENERATED_PACKAGE_SHARED_SETUP);
    steps.splice(index, 1);
  });
});

test('the unchanged final package gate rejects a failed, cancelled or skipped result from either physical generated cell', () => {
  const parsed = jobs('js.yml'), cells = checkGeneratedPackageSlices(parsed);
  const gate = parsed.get('test').steps.find(step => step.name === 'Require every JavaScript test group');
  const execute = results => spawnSync('bash', ['-c', gate.run.replace(/^\|\s*\n/u, '')], {
    encoding: 'utf8', env: {
      ...process.env, TESTS_RESULT: 'success', REPORTS_RESULT: 'success', TRANSLATION_FEEDBACK_RESULT: 'success',
      EXTERNAL_REPORTS: 'false', PACKAGE_CHECKS_RESULT: results.every(result => result === 'success') ? 'success' : 'failure',
    },
  });
  assert.equal(execute(cells.map(() => 'success')).status, 0);
  for (const [index, cell] of cells.entries()) {
    for (const result of ['failure', 'cancelled', 'skipped']) {
      const results = cells.map(() => 'success');
      results[index] = result;
      assert.equal(execute(results).status, 1, `${cell.group}:${cell.generated_slice}=${result}`);
    }
  }
  for (const change of [
    job => { job.header = job.header.replace('package-checks, ', ''); },
    job => { job.steps = job.steps.filter(step => step.name !== gate.name); },
    job => {
      const step = job.steps.find(step => step.name === gate.name);
      step.text = step.text.replace('needs.package-checks.result', 'needs.tests.result');
    },
  ]) {
    const changed = jobs('js.yml');
    change(changed.get('test'));
    assert.throws(() => checkGeneratedPackageSlices(changed), assert.AssertionError);
  }
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

test('owned source adapters retain all bounded checks and refuse gate weakening', () => {
  checkPackageStepConservation(jobs('js.yml'));
  for (const [name] of [...OWNED_SOURCE_ADAPTER_PACKAGE_STEPS, ...ORIGINAL_LINK_PROJECTION_PACKAGE_STEPS, ...ORIGINAL_LINK_SYMBOL_ORACLE_PACKAGE_STEPS]) {
    const mutate = change => {
      const parsed = jobs('js.yml'), steps = parsed.get('package-checks').steps;
      const index = steps.findIndex(step => step.name === name);
      assert.ok(index >= 0);
      change(steps, index);
      assert.throws(() => checkPackageStepConservation(parsed), assert.AssertionError);
    };
    mutate((steps, index) => steps.splice(index, 1));
    mutate((steps, index) => steps.push({ ...steps[index] }));
    for (const [from, to] of [
      ['!cancelled()', 'success()'],
      ["matrix.group == 'source-models'", "matrix.group == 'generated'"],
      ['working-directory: js', 'working-directory: rust'],
      ['working-directory: js', 'continue-on-error: true\n        working-directory: js'],
      ...(name.startsWith('Run ') ? [
        ["TRANSLATION_CHECK_RUST: '0'", "TRANSLATION_CHECK_RUST: '1'"],
        ["SELF_TRANSLATION_CHECK_RUST: '0'", "SELF_TRANSLATION_CHECK_RUST: '1'"],
        ['--event test --', '--event build --'],
        ['node --test ', 'node --test --test-name-pattern optional '],
      ] : [['--check', '--write']]),
    ]) mutate((steps, index) => {
      assert.ok(steps[index].text.includes(from));
      steps[index].text = steps[index].text.replace(from, to);
    });
  }
  const classifier = /rust_paths='([^']+)'/u.exec(workflow('ci.yml'))?.[1];
  const matches = new RegExp(classifier, 'u');
  for (const path of [
    'experiments/rust-text-certificate-adapter/',
    'experiments/rust-text-certificate-library/',
    'experiments/original-link-role-projection/',
    'experiments/fresh-number-array-collectors/',
    'js/scripts/rust-library-packaging\\.mjs$',
  ]) assert.equal(classifier.split('|' + path).length, 2, 'the source path is selected once');
  for (const file of [
    'experiments/rust-text-certificate-adapter/model.mjs',
    'experiments/rust-text-certificate-adapter/data/identity-transport.lino',
    'experiments/rust-text-certificate-library/data/library.lino',
    'experiments/original-link-role-projection/transport.mjs',
    'experiments/original-link-role-projection/data/native-definitions.lino',
    'experiments/fresh-number-array-collectors/model.mjs',
    'experiments/fresh-number-array-collectors/data/rule-translation.lino',
    'js/scripts/rust-library-packaging.mjs',
  ]) assert.ok(matches.test(file), file);
  assert.ok(!matches.test('js/scripts/rust-library-packaging.mjs.extra'));
});

test('the final package gate requires all five partitions, all test groups and standalone report assertions', () => {
  const gate = jobs('js.yml').get('test').steps.find(step => step.name === 'Require every JavaScript test group');
  assert.ok(gate.afterFailure);
  const originalGate = gate.text
    .replace('          PACKAGE_CHECKS_RESULT: ${{ needs.package-checks.result }}\n', '')
    .replace('          TRANSLATION_FEEDBACK_RESULT: ${{ needs.translation-test-feedback.result }}\n', '')
    .replace(
      '          if [ "$TRANSLATION_FEEDBACK_RESULT" != "success" ]; then\n' +
      '            echo "::error::Translation feedback merge finished with \'$TRANSLATION_FEEDBACK_RESULT\'; see the Translation Test Feedback job."\n' +
      '            exit 1\n          fi\n', '')
    .replace(
      '          if [ "$PACKAGE_CHECKS_RESULT" != "success" ]; then\n' +
      '            echo "::error::JavaScript package check groups finished with \'$PACKAGE_CHECKS_RESULT\'; see the JavaScript Package Checks jobs."\n' +
      '            exit 1\n          fi\n', '');
  assert.equal(packageStepSignature({ text: originalGate }), 'dd833d50b883ad3316674a59f0638632dacaf701c06bb90ec93e9eae3b994e09',
    'the added partition guard preserves every original final-gate field');
  assert.match(gate.text, /PACKAGE_CHECKS_RESULT: \$\{\{ needs.package-checks.result \}\}/u);
  assert.match(gate.text, /TRANSLATION_FEEDBACK_RESULT: \$\{\{ needs.translation-test-feedback.result \}\}/u);
  assert.match(gate.text, /TESTS_RESULT: \$\{\{ needs.tests.result \}\}/u);
  assert.match(gate.text, /REPORTS_RESULT: \$\{\{ needs.report-assertions.result \}\}/u);
  const execute = overrides => spawnSync('bash', ['-c', gate.run.replace(/^\|\s*\n/u, '')], {
    encoding: 'utf8', env: {
      ...process.env, PACKAGE_CHECKS_RESULT: 'success', TESTS_RESULT: 'success', TRANSLATION_FEEDBACK_RESULT: 'success',
      REPORTS_RESULT: 'success', EXTERNAL_REPORTS: 'false', ...overrides,
    },
  });
  assert.equal(execute({}).status, 0);
  assert.equal(execute({ EXTERNAL_REPORTS: 'true', REPORTS_RESULT: 'skipped' }).status, 0);
  for (const result of ['failure', 'cancelled', 'skipped']) {
    for (const key of ['PACKAGE_CHECKS_RESULT', 'TESTS_RESULT', 'REPORTS_RESULT', 'TRANSLATION_FEEDBACK_RESULT']) {
      assert.equal(execute({ [key]: result }).status, 1, `${key}=${result} must fail the package gate`);
    }
    assert.equal(execute({ PACKAGE_CHECKS_RESULT: result, EXTERNAL_REPORTS: 'true', REPORTS_RESULT: 'skipped' }).status, 1);
    assert.equal(execute({ TRANSLATION_FEEDBACK_RESULT: result, EXTERNAL_REPORTS: 'true', REPORTS_RESULT: 'skipped' }).status, 1);
  }
});

test('every frozen generator addition rejects omission, duplication, condition, command, directory and owner edits', () => {
  const mutate = (name, change) => {
    const parsed = jobs('js.yml');
    const steps = parsed.get('package-checks').steps;
    const index = steps.findIndex(step => step.name === name);
    assert.ok(index >= 0, `required additive gate missing: ${name}`);
    change(steps, index);
    assert.throws(() => checkPackageStepConservation(parsed), assert.AssertionError, name);
  };
  for (const [name, , , owner] of [...ADDITIONAL_PACKAGE_STEPS, ...EMPTY_OPTION_PACKAGE_STEPS, ...TYPE_NAMESPACE_PACKAGE_STEPS,
    ...PURE_UNIT_PACKAGE_STEPS, ...SOURCE_SPAN_PACKAGE_STEPS, ...OWNED_TEXT_GENERATOR_PACKAGE_STEPS, ...INTEGRAL_METADATA_PACKAGE_STEPS, ...CONST_AND_BYTE_GENERATOR_PACKAGE_STEPS, ...RUNTIME_SHELL_PACKAGE_STEPS, ...RUST_NUMBER_PACKAGE_STEPS,
    ...JAVASCRIPT_LINE_TERMINATOR_PACKAGE_STEPS, ...GENERATED_JAVASCRIPT_UNIT_PACKAGE_STEPS,
    ...OWNED_TUPLE_CALL_GENERATOR_PACKAGE_STEPS, ...RUST_LEXICAL_AND_OWNED_MODULE_GENERATOR_STEPS,
    ...RUST_RAW_STRING_AND_LITERAL_EDIT_GENERATOR_STEPS, ...RUST_TEXT_VALUE_PROJECTION_GENERATOR_STEPS,
    ...RUST_RAW_BINDING_IDENTIFIER_GENERATOR_STEPS, ...RUST_TEXT_CERTIFICATE_GENERATOR_STEPS]) {
    mutate(name, (steps, index) => steps.splice(index, 1));
    mutate(name, (steps, index) => steps.push({ ...steps[index] }));
    for (const [before, after] of [
      ['!cancelled()', 'success()'],
      ['--check', '--javascript-only'],
      ['working-directory: js', 'working-directory: .'],
      [`matrix.group == '${owner}'`, "matrix.group == 'contracts'"],
      ['working-directory: js', 'continue-on-error: true\n        working-directory: js'],
    ]) {
      mutate(name, (steps, index) => {
        steps[index].text = steps[index].text.replace(before, after);
      });
    }
  }
});

test('every frozen text experiment addition retains its mandatory execution, wrapper, directory and owner', () => {
  for (const [name, , , owner] of [...OWNED_TEXT_EXPERIMENT_PACKAGE_STEPS, ...INTEGRAL_METADATA_EXPERIMENT_PACKAGE_STEPS, ...CONST_AND_BYTE_EXPERIMENT_PACKAGE_STEPS,
    ...GENERATED_JAVASCRIPT_UNIT_EXPERIMENT_STEPS, ...OWNED_TUPLE_CALL_EXPERIMENT_PACKAGE_STEPS,
    ...GENERATED_OWNED_RUST_MODULE_EXPERIMENT_STEPS, ...OWNED_RUST_LITERAL_EDIT_EXPERIMENT_STEPS,
    ...RUST_TEXT_VALUE_PROJECTION_EXPERIMENT_STEPS, ...RUST_RAW_BINDING_IDENTIFIER_EXPERIMENT_STEPS,
    ...RUST_TEXT_CERTIFICATE_EXPERIMENT_STEPS]) {
    const mutate = change => {
      const parsed = jobs('js.yml');
      const steps = parsed.get('package-checks').steps;
      const index = steps.findIndex(step => step.name === name);
      assert.ok(index >= 0, `required owned text experiment missing: ${name}`);
      change(steps, index);
      assert.throws(() => checkPackageStepConservation(parsed), assert.AssertionError, name);
    };
    mutate((steps, index) => steps.splice(index, 1));
    mutate((steps, index) => steps.push({ ...steps[index] }));
    for (const [before, after] of [
      ['!cancelled()', 'success()'],
      ['--event test --', '--event build --'],
      ['node --test ../experiments/', 'node --test --test-name-pattern optional ../experiments/'],
      ['working-directory: js', 'working-directory: .'],
      [`matrix.group == '${owner}'`, "matrix.group == 'contracts'"],
      ['working-directory: js', 'continue-on-error: true\n        working-directory: js'],
    ]) {
      mutate((steps, index) => {
        assert.ok(steps[index].text.includes(before), `the mutation changes the actual experiment step: ${before}`);
        steps[index].text = steps[index].text.replace(before, after);
      });
    }
  }
});
