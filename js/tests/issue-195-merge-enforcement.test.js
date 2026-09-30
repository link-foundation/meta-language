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
const head = 'a'.repeat(40);
const base = 'b'.repeat(40);
const candidate = 'c'.repeat(40);
const options = { head, commit: candidate, manifest };

function snapshot() {
  const parameters = {
    strict_required_status_checks_policy: true,
    do_not_enforce_on_create: false,
    required_status_checks: [{ context: 'Full Requirements Aggregate' }],
  };
  return {
    repository: { full_name: 'link-foundation/meta-language', default_branch: 'main' },
    pullRequest: {
      number: 196, state: 'open', head: { sha: head },
      base: { sha: base, ref: 'main' }, merge_commit_sha: candidate,
    },
    mergeState: { headRefOid: head, baseRefOid: base, mergeStateStatus: 'BLOCKED' },
    candidate: { sha: candidate, parents: [{ sha: base }, { sha: head }] },
    effectiveRules: [{ type: 'required_status_checks', parameters, ruleset_id: 42 }],
    rulesets: [{
      id: 42, target: 'branch', enforcement: 'active', bypass_actors: [],
      current_user_can_bypass: 'never',
      conditions: { ref_name: { include: ['~DEFAULT_BRANCH'], exclude: [] } },
      rules: [{ type: 'required_status_checks', parameters }],
    }],
    checkRuns: [{
      id: 7, name: 'Full Requirements Aggregate', head_sha: head,
      status: 'completed', conclusion: 'failure',
      app: { id: 15368, slug: 'github-actions' },
    }],
    workflowRuns: [{
      checkId: 7, event: 'pull_request', head_sha: head,
      path: '.github/workflows/issue-195-acceptance.yml',
      pull_requests: [{ number: 196, head: { sha: head }, base: { sha: base, ref: 'main' } }],
    }],
  };
}

test('live rule evaluation needs an active strict rule and a failed current merge candidate', () => {
  const result = evaluateMergeEnforcement(snapshot(), options);
  assert.deepEqual(result.errors, []);
  assert.equal(Object.values(result.checks).every(Boolean), true);
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
  ['bypass actor', (live) => { live.rulesets[0].bypass_actors = [{ actor_id: 1, bypass_mode: 'always' }]; }],
  ['current user bypass', (live) => { live.rulesets[0].current_user_can_bypass = 'always'; }],
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

for (const [name, mutate] of [
  ['stale PR head', (live) => { live.pullRequest.head.sha = 'd'.repeat(40); }],
  ['stale merge-state query', (live) => { live.mergeState.headRefOid = 'd'.repeat(40); }],
  ['stale base', (live) => { live.mergeState.baseRefOid = 'd'.repeat(40); }],
  ['different candidate', (live) => { live.pullRequest.merge_commit_sha = 'd'.repeat(40); }],
  ['unrelated candidate parents', (live) => { live.candidate.parents = [{ sha: base }]; }],
  ['old check SHA', (live) => { live.checkRuns[0].head_sha = 'd'.repeat(40); }],
  ['old workflow base', (live) => { live.workflowRuns[0].pull_requests[0].base.sha = 'd'.repeat(40); }],
  ['unrelated workflow', (live) => { live.workflowRuns[0].path = '.github/workflows/other.yml'; }],
  ['passing check', (live) => { live.checkRuns[0].conclusion = 'success'; }],
  ['pending check', (live) => { live.checkRuns[0].status = 'in_progress'; }],
  ['merge permitted', (live) => { live.mergeState.mergeStateStatus = 'CLEAN'; }],
  ['newer pending rerun', (live) => {
    live.checkRuns.push({ ...live.checkRuns[0], id: 8, status: 'in_progress', conclusion: null });
  }],
]) {
  test(`a failed-check probe rejects ${name}`, () => {
    const live = snapshot();
    mutate(live);
    const result = evaluateMergeEnforcement(live, options);
    assert.equal(result.checks.failingCheckBlocksMerge, false);
    assert.notEqual(result.errors.length, 0);
  });
}

test('release evidence cannot be replaced by pre-merge evidence', () => {
  const weakened = structuredClone(manifest);
  const row = weakened.atomicRequirements.find(({ id }) => id === 'I195-DELIVERY-NPM-PUBLISHED');
  row.verifications[0].checkpoint = 'pre-merge';
  const result = evaluateMergeEnforcement(snapshot(), { ...options, manifest: weakened });
  assert.equal(result.checks.publishedDeliverySeparatelyVerified, false);
  assert.match(result.errors.join('\n'), /release-delivery/);
});

test('a producing rerun uses a completed failure of the same candidate without trusting its own pending result', () => {
  const live = snapshot();
  live.checkRuns.push({ ...live.checkRuns[0], id: 8, status: 'in_progress', conclusion: null });
  live.workflowRuns.push({ ...live.workflowRuns[0], checkId: 8, id: 97 });
  const result = evaluateMergeEnforcement(live, { ...options, runningWorkflowId: '97' });
  assert.equal(result.checks.failingCheckBlocksMerge, true);
  assert.equal(result.check.id, 7);
  assert.equal(result.producingCheck.id, 8);
  assert.equal(evaluateMergeEnforcement(live, { ...options, runningWorkflowId: '98' })
    .checks.failingCheckBlocksMerge, false);
  live.workflowRuns[1].pull_requests[0].base.sha = 'd'.repeat(40);
  assert.equal(evaluateMergeEnforcement(live, { ...options, runningWorkflowId: '97' })
    .checks.failingCheckBlocksMerge, false);
});

test('the first producing run cannot replace a missing failed-check probe', () => {
  const live = snapshot();
  live.checkRuns[0].status = 'in_progress';
  live.checkRuns[0].conclusion = null;
  live.workflowRuns[0].id = 97;
  const result = evaluateMergeEnforcement(live, { ...options, runningWorkflowId: '97' });
  assert.equal(result.checks.failingCheckBlocksMerge, false);
});

test('a missing live response fails closed', () => {
  const result = evaluateMergeEnforcement({}, options);
  assert.equal(result.checks.activeRuleTargetsDefaultBranch, false);
  assert.equal(result.checks.failingCheckBlocksMerge, false);
});

test('a rule response without permission to inspect bypass actors fails closed', () => {
  const live = snapshot();
  delete live.rulesets[0].bypass_actors;
  const result = evaluateMergeEnforcement(live, options);
  assert.equal(result.checks.activeRuleTargetsDefaultBranch, false);
  assert.equal(result.checks.fullAggregateRequired, false);
});

test('live inspection retrieves all check attempts and verifies the revision again', async () => {
  const live = snapshot();
  live.checkRuns[0].details_url = 'https://github.com/link-foundation/meta-language/actions/runs/97/job/123';
  const calls = [];
  const prefix = 'repos/link-foundation/meta-language';
  const query = async (args) => {
    calls.push(args);
    if (args[0] === 'pr') return live.mergeState;
    const endpoint = args[1];
    if (endpoint === prefix) return live.repository;
    if (endpoint === `${prefix}/pulls/196`) return live.pullRequest;
    if (endpoint === `${prefix}/rules/branches/main`) return live.effectiveRules;
    if (endpoint === `${prefix}/rulesets/42`) return live.rulesets[0];
    if (endpoint === `${prefix}/commits/${candidate}`) return live.candidate;
    if (endpoint === `${prefix}/commits/${head}/check-runs?filter=all`) {
      assert.ok(args.includes('--paginate'));
      assert.ok(args.includes('--slurp'));
      return [{ check_runs: [live.checkRuns[0]] }, { check_runs: [] }];
    }
    if (endpoint === `${prefix}/actions/runs/97`) return live.workflowRuns[0];
    throw new Error(`unexpected GitHub request: ${endpoint}`);
  };
  const result = await inspectMergeEnforcement({ repository: 'link-foundation/meta-language', pullRequest: 196, query });
  assert.equal(result.checkRuns.length, 1);
  assert.equal(result.workflowRuns[0].checkId, 7);
  assert.equal(calls.filter(([command]) => command === 'pr').length, 2);
  assert.deepEqual(evaluateMergeEnforcement(result, options).errors, []);
  let revisionQueries = 0;
  await assert.rejects(inspectMergeEnforcement({
    repository: 'link-foundation/meta-language', pullRequest: 196,
    query: async (args) => args[0] === 'pr' && ++revisionQueries === 2
      ? { ...live.mergeState, baseRefOid: 'd'.repeat(40) } : query(args),
  }), /revision changed/);
});

for (const forcedColors of [{ CLICOLOR_FORCE: '1' }, { GH_FORCE_TTY: '1' }, { CLICOLOR_FORCE: '1', GH_FORCE_TTY: '1' }]) {
  test(`GitHub JSON subprocess disables forced colors: ${Object.keys(forcedColors).join(', ')}`, async () => {
    const directory = await mkdtemp(path.join(os.tmpdir(), 'issue-195-github-colors-'));
    const executable = path.join(directory, 'github-fixture.cjs');
    const environment = { ...process.env, ...forcedColors, GH_TOKEN: 'fixture-authentication' };
    delete environment.NO_COLOR;
    const originalEnvironment = { ...environment };
    try {
      // Exercise a real subprocess using the documented GitHub CLI color
      // controls. The same forced settings are inherited from setup-ocaml.
      await writeFile(executable, `
        if (process.env.GH_TOKEN !== 'fixture-authentication') process.exit(2);
        const colored = !process.env.NO_COLOR &&
          (process.env.CLICOLOR_FORCE === '1' || process.env.GH_FORCE_TTY === '1');
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
