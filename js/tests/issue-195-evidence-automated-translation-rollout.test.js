import assert from 'node:assert/strict';
import test from 'node:test';

import { inspectAutomatedTranslationRollout } from '../scripts/check-automated-translation-rollout.mjs';

test('rollout inspection uses the current base and the latest eligible pull request attempt', async () => {
  const pull = (number) => ({
    number, html_url: `pull/${number}`, head: { sha: `head${number}` }, base: { ref: 'main', sha: 'previous' },
  });
  const run = (number, id, base, status = 'completed') => ({
    id, status, event: 'pull_request', head_sha: `head${number}`, html_url: `run/${id}`,
    run_number: id, run_attempt: 2, pull_requests: [{ number, base: { sha: base } }],
  });
  const query = async ([command, endpoint, paginate, slurp]) => {
    assert.ok(endpoint.startsWith('repos/example/project/'));
    const resource = endpoint.slice('repos/example/project/'.length);
    if (resource === 'git/ref/heads/main') {
      assert.deepEqual([command, paginate, slurp], ['api', '--allow-escape-sequences', undefined]);
      return { object: { sha: 'current' } };
    }
    assert.deepEqual([command, paginate, slurp], ['api', '--paginate', '--slurp']);
    if (resource.startsWith('pulls?')) return [[pull(1), pull(2)], [pull(3), pull(4)]];
    if (resource.includes('head_sha=head1')) return [
      { workflow_runs: [run(1, 8, 'current'), run(1, 12, 'previous')] },
      { workflow_runs: [run(1, 9, 'current'), { ...run(1, 16, 'current'), pull_requests: [] }] },
    ];
    if (resource.includes('head_sha=head2')) return [{ workflow_runs: [run(2, 14, 'previous')] }];
    if (resource.includes('head_sha=head3')) return [{ workflow_runs: [run(3, 15, 'current', 'queued')] }];
    if (resource.includes('head_sha=head4')) return [{ workflow_runs: [run(4, 17, 'current')] }];
    if (resource === 'actions/runs/9/attempts/2/jobs?per_page=100') return [{ jobs: [{
      name: 'Full Requirements Aggregate', conclusion: 'success',
      steps: [
        { name: 'Require the active checks while automated translation is paused', conclusion: 'success' },
        { name: 'Verify paused checkout head1 against current', conclusion: 'success' },
      ],
    }] }];
    if (resource === 'actions/runs/17/attempts/2/jobs?per_page=100') return [{ jobs: [{
      name: 'Full Requirements Aggregate', conclusion: 'success',
      steps: [
        { name: 'Require the active checks while automated translation is paused', conclusion: 'success' },
        { name: 'Verify paused checkout head4 against previous', conclusion: 'success' },
      ],
    }] }];
    assert.fail(`Unexpected GitHub request ${resource}`);
  };
  assert.deepEqual(await inspectAutomatedTranslationRollout('example/project', query), [
    { pull: 1, url: 'pull/1', head: 'head1', base: 'current', run: 'run/9', attempt: 2, aggregate: 'success', pauseStep: 'success', checkoutStep: 'success' },
    { pull: 2, url: 'pull/2', head: 'head2', base: 'current', problem: 'No CI pull_request run matches the current head and base.' },
    { pull: 3, url: 'pull/3', head: 'head3', base: 'current', run: 'run/15', attempt: 2, problem: 'Run is queued.' },
    { pull: 4, url: 'pull/4', head: 'head4', base: 'current', run: 'run/17', attempt: 2, aggregate: 'success', pauseStep: 'success', checkoutStep: null },
  ]);
});
