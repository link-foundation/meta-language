import assert from 'node:assert/strict';
import test from 'node:test';

import { inspectAutomatedTranslationRollout } from '../scripts/check-automated-translation-rollout.mjs';

test('rollout inspection uses the current base and the latest eligible pull request attempt', async () => {
  const pull = (number) => ({
    number, html_url: `pull/${number}`, head: { sha: `head${number}`, ref: `branch${number}`, repo: { id: number < 5 ? 1 : 2 } },
    base: { ref: number === 4 ? 'maintenance/release-1' : 'main', sha: 'previous', repo: { id: 1 } },
  });
  const run = (number, id, base, status = 'completed') => ({
    id, status, event: 'pull_request', head_sha: `head${number}`, html_url: `run/${id}`,
    run_number: id, run_attempt: 2, pull_requests: [{ number, base: { sha: base } }],
  });
  const forkRun = (number, id, repository = 2) => ({
    ...run(number, id, 'current'), pull_requests: [], head_repository: { id: repository }, head_branch: `branch${number}`,
  });
  const query = async ([command, endpoint, paginate, slurp]) => {
    assert.ok(endpoint.startsWith('repos/example/project/'));
    const resource = endpoint.slice('repos/example/project/'.length);
    if (resource.startsWith('git/ref/heads/')) {
      assert.deepEqual([command, paginate, slurp], ['api', undefined, undefined]);
      const heads = { 'git/ref/heads/main': 'current', 'git/ref/heads/maintenance/release-1': 'release' };
      assert.ok(heads[resource]);
      return { object: { sha: heads[resource] } };
    }
    assert.deepEqual([command, paginate, slurp], ['api', '--paginate', '--slurp']);
    if (resource.startsWith('pulls?')) return [[pull(1), pull(2)], [pull(3), pull(4), pull(5), pull(6)]];
    // PR #249 had a queued run missing from GitHub's filtered endpoints.
    if (resource.includes('head_sha=')) assert.fail('The head filter can hide the latest run');
    if (resource === 'actions/workflows/ci.yml/runs?per_page=100') return [
      { workflow_runs: [
        forkRun(5, 21, 3), forkRun(6, 19), forkRun(5, 18), run(4, 17, 'release'), forkRun(1, 16, 1),
        { ...run(1, 20, 'current'), event: 'workflow_dispatch' },
        { ...run(2, 22, 'current'), head_sha: 'stale' },
        run(3, 15, 'current', 'queued'), run(2, 14, 'previous'), run(1, 12, 'previous'),
      ] },
      { workflow_runs: [run(3, 11, 'current'), run(1, 9, 'current'), run(1, 8, 'current')] },
    ];
    if (resource === 'actions/runs/9/attempts/2/jobs?per_page=100') return [{ jobs: [{
      name: 'Full Requirements Aggregate', conclusion: 'success',
      steps: [
        { name: 'Require the active checks while automated translation is paused', conclusion: 'success' },
        { name: 'Verify Linux-only workflow scheduling', conclusion: 'success' },
        { name: 'Verify paused checkout for PR 1 at head1 against current', conclusion: 'success' },
      ],
    }] }];
    if (resource === 'actions/runs/17/attempts/2/jobs?per_page=100') return [{ jobs: [{
      name: 'Full Requirements Aggregate', conclusion: 'success',
      steps: [
        { name: 'Require the active checks while automated translation is paused', conclusion: 'success' },
        { name: 'Verify Linux-only workflow scheduling', conclusion: 'success' },
        { name: 'Verify paused checkout for PR 4 at head4 against previous', conclusion: 'success' },
      ],
    }] }];
    if (resource === 'actions/runs/18/attempts/2/jobs?per_page=100' || resource === 'actions/runs/19/attempts/2/jobs?per_page=100') {
      const head = resource.includes('/18/') ? 'head5' : 'head6';
      return [{ jobs: [{ name: 'Full Requirements Aggregate', conclusion: 'success', steps: [
        { name: 'Require the active checks while automated translation is paused', conclusion: 'success' },
        { name: 'Verify Linux-only workflow scheduling', conclusion: 'failure' },
        { name: `Verify paused checkout for PR 5 at ${head} against current`, conclusion: 'success' },
      ] }] }];
    }
    assert.fail(`Unexpected GitHub request ${resource}`);
  };
  assert.deepEqual(await inspectAutomatedTranslationRollout('example/project', query), [
    { pull: 1, url: 'pull/1', head: 'head1', base: 'current', run: 'run/9', attempt: 2, aggregate: 'success', pauseStep: 'success', platformStep: 'success', checkoutStep: 'success' },
    { pull: 2, url: 'pull/2', head: 'head2', base: 'current', problem: 'No CI pull_request run matches the current head and base.' },
    { pull: 3, url: 'pull/3', head: 'head3', base: 'current', run: 'run/15', attempt: 2, problem: 'Run is queued.' },
    { pull: 4, url: 'pull/4', head: 'head4', base: 'release', run: 'run/17', attempt: 2, aggregate: 'success', pauseStep: 'success', platformStep: 'success', checkoutStep: null },
    { pull: 5, url: 'pull/5', head: 'head5', base: 'current', run: 'run/18', attempt: 2, aggregate: 'success', pauseStep: 'success', platformStep: 'failure', checkoutStep: 'success' },
    { pull: 6, url: 'pull/6', head: 'head6', base: 'current', run: 'run/19', attempt: 2, aggregate: 'success', pauseStep: 'success', platformStep: 'failure', checkoutStep: null },
  ]);
});
