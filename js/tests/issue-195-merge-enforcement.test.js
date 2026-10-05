import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import { evaluateMergeEnforcement, githubQuery, inspectMergeEnforcement, verifyEvaluatedCheckout } from '../scripts/issue-195-merge-enforcement.mjs';

const manifest = JSON.parse(await readFile(
  new URL('../../parity/issue-195-requirements.json', import.meta.url), 'utf8',
));
const acceptanceWorkflow = await readFile(
  new URL('../../.github/workflows/ci.yml', import.meta.url), 'utf8',
);
const head = 'a'.repeat(40);
const merged = 'c'.repeat(40);
const options = { commit: merged, manifest, acceptanceWorkflow };

function snapshot() {
  const parameters = {
    strict_required_status_checks_policy: true,
    do_not_enforce_on_create: false,
    required_status_checks: [{ context: 'Full Requirements Aggregate' }],
  };
  return {
    repository: { full_name: 'link-foundation/meta-language', default_branch: 'main' },
    branchHead: merged,
    effectiveRules: [{ type: 'required_status_checks', parameters, ruleset_id: 42 }],
    rulesets: [{
      id: 42, target: 'branch', enforcement: 'active', bypass_actors: [],
      current_user_can_bypass: 'never',
      conditions: { ref_name: { include: ['~DEFAULT_BRANCH'], exclude: [] } },
      rules: [{ type: 'required_status_checks', parameters }],
    }],
    pullRequests: [{
      number: 200, headRefOid: head, baseRefName: 'main', mergeStateStatus: 'BLOCKED',
      checkRuns: [{
        id: 7, name: 'Full Requirements Aggregate', head_sha: head,
        status: 'completed', conclusion: 'failure',
        app: { id: 15368, slug: 'github-actions' },
      }],
    }],
  };
}

test('the post-merge report needs an active strict rule and an open pull request blocked by a failed aggregate', () => {
  const result = evaluateMergeEnforcement(snapshot(), options);
  assert.deepEqual(result.errors, []);
  assert.deepEqual(result.notes, []);
  assert.equal(Object.values(result.checks).every(Boolean), true);
  assert.equal(result.blockedPullRequest, 200);
  assert.equal(result.delivery.releaseRequirements.length, 4);
  assert.equal(result.delivery.releaseEvidencePassed, false);
});

test('a workflow name without an effective rule cannot establish merge enforcement', () => {
  const live = snapshot();
  live.effectiveRules = [];
  const result = evaluateMergeEnforcement(live, options);
  assert.equal(result.checks.activeRuleTargetsDefaultBranch, false);
  assert.equal(result.checks.fullAggregateRequired, false);
  assert.equal(result.checks.failingCheckBlocksMerge, false);
});

for (const [name, mutate] of [
  ['disabled rule', (live) => { live.rulesets[0].enforcement = 'evaluate'; }],
  ['different branch', (live) => { live.rulesets[0].conditions.ref_name.include = ['refs/heads/develop']; }],
  ['excluded default branch', (live) => { live.rulesets[0].conditions.ref_name.exclude = ['~DEFAULT_BRANCH']; }],
  ['optional aggregate', (live) => { live.effectiveRules[0].parameters.required_status_checks = []; }],
  ['non-strict checks', (live) => { live.effectiveRules[0].parameters.strict_required_status_checks_policy = false; }],
  ['unenforced creation', (live) => { live.effectiveRules[0].parameters.do_not_enforce_on_create = true; }],
  ['wrong application', (live) => { live.effectiveRules[0].parameters.required_status_checks[0].integration_id = 123; }],
]) {
  test(`merge enforcement rejects ${name}`, () => {
    const live = snapshot();
    mutate(live);
    const result = evaluateMergeEnforcement(live, options);
    assert.equal(result.checks.failingCheckBlocksMerge, false);
    assert.notEqual(result.errors.length, 0);
  });
}

// A bypassable rule is its own missing evidence: the strict required check
// and the blocked pull request are still observed.
for (const [name, mutate] of [
  ['bypass actor', (live) => { live.rulesets[0].bypass_actors = [{ actor_id: 1, bypass_mode: 'always' }]; }],
  ['current user bypass', (live) => { live.rulesets[0].current_user_can_bypass = 'always'; }],
]) {
  test(`merge enforcement rejects ${name} as a non-bypassable rule only`, () => {
    const live = snapshot();
    mutate(live);
    const result = evaluateMergeEnforcement(live, options);
    assert.equal(result.checks.activeRuleTargetsDefaultBranch, false);
    assert.equal(result.checks.fullAggregateRequired, true);
    assert.equal(result.checks.failingCheckBlocksMerge, true);
    assert.equal(result.checks.publishedDeliverySeparatelyVerified, true);
    assert.match(result.errors.join('\n'), /non-bypassable/);
  });
}

// The report uses the workflow token only. A ruleset that hides its bypass
// actors from it is reported as a note, never as a request for a credential.
test('hidden bypass actors are a note of the workflow-token report, not a missing credential', () => {
  const live = snapshot();
  delete live.rulesets[0].bypass_actors;
  const result = evaluateMergeEnforcement(live, options);
  assert.deepEqual(result.errors, []);
  assert.equal(Object.values(result.checks).every(Boolean), true);
  assert.match(result.notes.join('\n'), /ruleset 42 does not show its bypass actors.*current_user_can_bypass/);
  assert.doesNotMatch(JSON.stringify(result), /RULESET_TOKEN|Administration/);
});

for (const [name, mutate] of [
  ['no open pull request', (live) => { live.pullRequests = []; }],
  ['a pull request into another branch', (live) => { live.pullRequests[0].baseRefName = 'develop'; }],
  ['an old check SHA', (live) => { live.pullRequests[0].checkRuns[0].head_sha = 'd'.repeat(40); }],
  ['a passing check', (live) => { live.pullRequests[0].checkRuns[0].conclusion = 'success'; }],
  ['a pending check', (live) => { live.pullRequests[0].checkRuns[0].status = 'in_progress'; }],
  ['a permitted merge', (live) => { live.pullRequests[0].mergeStateStatus = 'CLEAN'; }],
  ['a draft pull request', (live) => { live.pullRequests[0].mergeStateStatus = 'DRAFT'; }],
  ['a newer pending rerun', (live) => {
    live.pullRequests[0].checkRuns.push({ ...live.pullRequests[0].checkRuns[0], id: 8, status: 'in_progress', conclusion: null });
  }],
]) {
  test(`the failed-check probe rejects ${name}`, () => {
    const live = snapshot();
    mutate(live);
    const result = evaluateMergeEnforcement(live, options);
    assert.equal(result.checks.failingCheckBlocksMerge, false);
    assert.equal(result.blockedPullRequest, null);
    assert.match(result.errors.join('\n'), /no open pull request/);
  });
}

test('the probe uses any blocked open pull request, never the pull request that is being evaluated', () => {
  const live = snapshot();
  live.pullRequests.unshift({ number: 199, headRefOid: 'e'.repeat(40), baseRefName: 'main', mergeStateStatus: 'CLEAN', checkRuns: [] });
  const result = evaluateMergeEnforcement(live, options);
  assert.equal(result.checks.failingCheckBlocksMerge, true);
  assert.equal(result.blockedPullRequest, 200);
});

test('a newer push to the default branch is a note, not a failure', () => {
  const live = snapshot();
  live.branchHead = 'f'.repeat(40);
  const result = evaluateMergeEnforcement(live, options);
  assert.deepEqual(result.errors, []);
  assert.match(result.notes.join('\n'), /newer push supersedes/);
});

test('release evidence cannot be replaced by pre-merge evidence', () => {
  const weakened = structuredClone(manifest);
  const row = weakened.atomicRequirements.find(({ id }) => id === 'I195-DELIVERY-NPM-PUBLISHED');
  row.verifications[0].checkpoint = 'pre-merge';
  const result = evaluateMergeEnforcement(snapshot(), { ...options, manifest: weakened });
  assert.equal(result.checks.publishedDeliverySeparatelyVerified, false);
  assert.match(result.errors.join('\n'), /release-delivery/);
});

test('a missing live response fails closed', () => {
  const result = evaluateMergeEnforcement({}, options);
  assert.equal(result.checks.activeRuleTargetsDefaultBranch, false);
  assert.equal(result.checks.failingCheckBlocksMerge, false);
});

// GitHub reports a skipped job as a successful required check, so a skipped
// aggregate would let the pull request merge without any evidence.
for (const [name, from, to] of [
  ['a conditional aggregate', 'if: ${{ always() }}', "if: ${{ !cancelled() && needs.candidates.result == 'success' }}"],
  ['an aggregate without a job condition', '    if: ${{ always() }}\n', ''],
  ['an aggregate that ignores missing candidates', "if: ${{ needs.candidates.result != 'success' }}", 'if: ${{ false }}'],
  ['an aggregate whose guard does not fail', 'exit 1', 'exit 0'],
  ['a renamed aggregate', 'name: Full Requirements Aggregate', 'name: Requirements Aggregate'],
]) {
  test(`merge enforcement rejects ${name}`, () => {
    assert.ok(acceptanceWorkflow.includes(from), `workflow contains ${JSON.stringify(from)}`);
    const result = evaluateMergeEnforcement(snapshot(), { ...options, acceptanceWorkflow: acceptanceWorkflow.replace(from, to) });
    assert.equal(result.checks.fullAggregateRequired, false);
    assert.equal(result.checks.failingCheckBlocksMerge, false);
    assert.match(result.errors.join('\n'), /skipped|no job named/);
  });
}

test('a missing workflow cannot establish a required aggregate', () => {
  const result = evaluateMergeEnforcement(snapshot(), { ...options, acceptanceWorkflow: undefined });
  assert.equal(result.checks.fullAggregateRequired, false);
});

test('live inspection reads the default branch, its rules and the open pull requests with one token', async () => {
  const live = snapshot();
  const calls = [];
  const prefix = 'repos/link-foundation/meta-language';
  const query = async (args) => {
    calls.push(args);
    if (args[0] === 'pr') {
      assert.deepEqual(args.slice(0, 8), ['pr', 'list', '--repo', 'link-foundation/meta-language', '--state', 'open', '--base', 'main']);
      return live.pullRequests.map(({ checkRuns, ...pull }) => pull);
    }
    const endpoint = args[1];
    if (endpoint === prefix) return live.repository;
    if (endpoint === `${prefix}/branches/main`) return { commit: { sha: merged } };
    if (endpoint === `${prefix}/rules/branches/main`) return live.effectiveRules;
    if (endpoint === `${prefix}/rulesets/42`) return live.rulesets[0];
    if (endpoint === `${prefix}/commits/${head}/check-runs?check_name=Full%20Requirements%20Aggregate&filter=all`) {
      assert.ok(args.includes('--paginate'));
      assert.ok(args.includes('--slurp'));
      return [{ check_runs: live.pullRequests[0].checkRuns }, { check_runs: [] }];
    }
    throw new Error(`unexpected GitHub request: ${endpoint}`);
  };
  const result = await inspectMergeEnforcement({ repository: 'link-foundation/meta-language', query });
  assert.equal(result.branchHead, merged);
  assert.equal(result.pullRequests[0].checkRuns.length, 1);
  assert.equal(calls.filter(([command]) => command === 'pr').length, 1);
  assert.deepEqual(evaluateMergeEnforcement(result, options).errors, []);
});

test('the merge-enforcement tooling and workflows need no ruleset credential', async () => {
  for (const file of [
    '../scripts/issue-195-merge-enforcement.mjs', '../scripts/check-issue-195-merge-enforcement.mjs',
    '../../.github/workflows/ci.yml',
  ]) {
    assert.doesNotMatch(await readFile(new URL(file, import.meta.url), 'utf8'), /ISSUE_195_RULESET_TOKEN/, file);
  }
});

for (const forcedColors of [{ CLICOLOR_FORCE: '1' }, { GH_FORCE_TTY: '1' }, { CLICOLOR_FORCE: '1', GH_FORCE_TTY: '1' }]) {
  test(`GitHub JSON subprocess disables forced colors: ${Object.keys(forcedColors).join(', ')}`, async () => {
    const directory = await mkdtemp(path.join(os.tmpdir(), 'issue-195-github-colors-'));
    const executable = path.join(directory, 'github-fixture.cjs');
    const environment = { ...process.env, GH_TOKEN: 'fixture-authentication' };
    delete environment.NO_COLOR;
    delete environment.CLICOLOR_FORCE;
    delete environment.GH_FORCE_TTY;
    Object.assign(environment, forcedColors);
    const originalEnvironment = { ...environment };
    try {
      // Exercise a real subprocess using the documented GitHub CLI color
      // controls. The same forced settings are inherited from setup-ocaml.
      await writeFile(executable, `
        if (process.env.GH_TOKEN !== 'fixture-authentication') process.exit(2);
        // Forced terminal settings can take precedence over NO_COLOR.
        const colored = process.env.CLICOLOR_FORCE === '1' || process.env.GH_FORCE_TTY === '1';
        const body = JSON.stringify({ default_branch: 'main' });
        process.stdout.write(colored ? '\\x1b[1;37m' + body + '\\x1b[0m' : body);
      `);
      const result = await githubQuery(['api', 'repos/link-foundation/meta-language'], {
        environment,
        execute: (command, args, settings) => {
          assert.equal(command, 'gh');
          assert.deepEqual(args, ['api', 'repos/link-foundation/meta-language']);
          return execFileSync(process.execPath, [executable, ...args], settings);
        },
      });
      assert.deepEqual(result, { default_branch: 'main' });
      assert.deepEqual(environment, originalEnvironment);
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  });
}

test('invalid GitHub JSON remains an error', async () => {
  await assert.rejects(githubQuery(['api', 'repos/link-foundation/meta-language'], {
    execute: () => 'not JSON',
  }), SyntaxError);
});

test('evidence rejects an arbitrary commit, a changed tracked file and an untracked executable', async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'issue-195-evaluated-checkout-'));
  const git = (args) => execFileSync('git', args, { cwd: directory, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
  try {
    git(['init']);
    git(['config', 'user.email', 'test@example.invalid']);
    git(['config', 'user.name', 'Evidence test']);
    await writeFile(path.join(directory, 'source.js'), 'export const value = 1;\n');
    git(['add', 'source.js']);
    git(['commit', '-m', 'fixture']);
    const revision = git(['rev-parse', 'HEAD']);
    assert.doesNotThrow(() => verifyEvaluatedCheckout(directory, revision));
    assert.throws(() => verifyEvaluatedCheckout(directory, 'd'.repeat(40)), /checked-out commit/);
    await writeFile(path.join(directory, 'source.js'), 'export const value = 2;\n');
    assert.throws(() => verifyEvaluatedCheckout(directory, revision), /uncommitted changes[\s\S]*source\.js/);
    git(['restore', 'source.js']);
    await writeFile(path.join(directory, 'extra.test.js'), 'throw new Error("not in the commit");\n');
    assert.throws(() => verifyEvaluatedCheckout(directory, revision), /uncommitted changes[\s\S]*extra\.test\.js/);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test('the evidence runner rejects a mislabeled revision before planning or producing artifacts', () => {
  assert.throws(() => execFileSync(process.execPath, [
    fileURLToPath(new URL('../scripts/run-issue-195-evidence.mjs', import.meta.url)),
    '--commit', '0'.repeat(40), '--checkpoint', 'invalid',
  ], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }), (error) => {
    assert.match(error.stderr, /evaluated commit differs from the checked-out commit/);
    return true;
  });
});

test('installed OCaml tools leave the evaluated checkout clean without hiding source changes', async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'issue-195-ocaml-checkout-'));
  const git = (args) => execFileSync('git', args, { cwd: directory, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
  try {
    git(['init']);
    git(['config', 'user.email', 'test@example.invalid']);
    git(['config', 'user.name', 'Evidence test']);
    await writeFile(path.join(directory, '.gitignore'), await readFile(
      new URL('../../.gitignore', import.meta.url), 'utf8',
    ));
    await writeFile(path.join(directory, 'source.js'), 'export const value = 1;\n');
    git(['add', '.gitignore', 'source.js']);
    git(['commit', '-m', 'fixture']);
    const revision = git(['rev-parse', 'HEAD']);
    // setup-ocaml creates its local switch in this directory before the
    // evidence producer runs. Its installed binaries are regenerable tools.
    await mkdir(path.join(directory, '_opam', 'bin'), { recursive: true });
    await writeFile(path.join(directory, '_opam', 'bin', 'rocq'), 'installed compiler\n');
    assert.doesNotThrow(() => verifyEvaluatedCheckout(directory, revision));
    await writeFile(path.join(directory, 'source.js'), 'export const value = 2;\n');
    assert.throws(() => verifyEvaluatedCheckout(directory, revision), /uncommitted changes/);
    git(['restore', 'source.js']);
    await writeFile(path.join(directory, 'extra.test.js'), 'uncommitted test\n');
    assert.throws(() => verifyEvaluatedCheckout(directory, revision), /uncommitted changes/);
    await rm(path.join(directory, 'extra.test.js'));
    // Git still reports edits to tracked files even inside an ignored folder.
    git(['add', '--force', '_opam/bin/rocq']);
    git(['commit', '-m', 'tracked fixture']);
    const trackedRevision = git(['rev-parse', 'HEAD']);
    await writeFile(path.join(directory, '_opam', 'bin', 'rocq'), 'changed tracked file\n');
    assert.throws(() => verifyEvaluatedCheckout(directory, trackedRevision), /uncommitted changes/);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});
