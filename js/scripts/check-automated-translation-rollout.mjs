#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { mkdirSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { FULL_REQUIREMENTS_CHECK, githubQuery, verifyEvaluatedCheckout } from './issue-195-merge-enforcement.mjs';
import { writeProjectData } from './project-data.mjs';

const pauseStep = 'Require the active checks while automated translation is paused';
const platformStep = 'Verify Linux-only workflow scheduling';

export async function inspectAutomatedTranslationRollout(repository, query = githubQuery) {
  const api = `repos/${repository}`;
  const pages = (endpoint) => query(['api', `${api}/${endpoint}`, '--paginate', '--slurp']);
  const pulls = (await pages('pulls?state=open&per_page=100')).flat();
  if (pulls.length === 0) return [];
  const baseCommits = new Map();
  for (const ref of new Set(pulls.map(pull => pull.base.ref))) {
    const branch = ref.split('/').map(encodeURIComponent).join('/');
    const reference = await query(['api', `${api}/git/ref/heads/${branch}`]);
    baseCommits.set(ref, reference.object.sha);
  }
  // GitHub's head and event filters can omit runs that appear in the full list.
  const runs = (await pages('actions/workflows/ci.yml/runs?per_page=100'))
    .flatMap(page => page.workflow_runs)
    .filter(run => run.event === 'pull_request')
    .sort((left, right) => right.run_number - left.run_number);
  const observations = [];
  for (const pull of pulls) {
    const head = pull.head.sha;
    const base = baseCommits.get(pull.base.ref);
    const run = runs.find(candidate => {
      if (candidate.head_sha !== head) return false;
      if (candidate.pull_requests?.length) {
        return candidate.pull_requests.some(associated => associated.number === pull.number && associated.base.sha === base);
      }
      // GitHub can omit the association for a fork. The workflow's checkout
      // step must still confirm the exact pull request, head and base below.
      const source = pull.head.repo;
      return source?.id !== undefined && source.id !== pull.base.repo?.id &&
        candidate.head_repository?.id === source.id && candidate.head_branch === pull.head.ref;
    });
    const observation = { pull: pull.number, url: pull.html_url, head, base };
    if (!run) {
      observations.push({ ...observation, problem: 'No CI pull_request run matches the current head and base.' });
      continue;
    }
    Object.assign(observation, { run: run.html_url, attempt: run.run_attempt });
    if (run.status !== 'completed') {
      observations.push({ ...observation, problem: `Run is ${run.status}.` });
      continue;
    }
    const jobs = (await pages(`actions/runs/${run.id}/attempts/${run.run_attempt}/jobs?per_page=100`))
      .flatMap(page => page.jobs);
    const aggregate = jobs.find(job => job.name === FULL_REQUIREMENTS_CHECK);
    const pause = aggregate?.steps?.find(step => step.name === pauseStep);
    const platform = aggregate?.steps?.find(step => step.name === platformStep);
    const checkout = aggregate?.steps?.find(step => step.name === `Verify paused checkout for PR ${pull.number} at ${head} against ${base}`);
    observations.push({
      ...observation,
      aggregate: aggregate?.conclusion ?? null,
      pauseStep: pause?.conclusion ?? null,
      platformStep: platform?.conclusion ?? null,
      checkoutStep: checkout?.conclusion ?? null,
    });
  }
  return observations;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const [output] = process.argv.slice(2);
  if (!output || process.argv.length !== 3) throw new Error('provide the output report path');
  const root = fileURLToPath(new URL('../../', import.meta.url));
  const commit = process.env.GITHUB_SHA || execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
  const repository = process.env.GITHUB_REPOSITORY || 'link-foundation/meta-language';
  let observations = [];
  let error;
  try {
    verifyEvaluatedCheckout(root, commit);
    observations = await inspectAutomatedTranslationRollout(repository);
    verifyEvaluatedCheckout(root, commit);
  } catch (cause) {
    error = cause.message;
  }
  const everyOpenPullRequestRunsPausedWorkflow = error === undefined && observations.every(row =>
    row.aggregate === 'success' && row.pauseStep === 'success' && row.platformStep === 'success' && row.checkoutStep === 'success');
  mkdirSync(path.dirname(output), { recursive: true });
  writeProjectData(output, { commit, observations, everyOpenPullRequestRunsPausedWorkflow, ...(error && { error }) }, { format: 'lino' });
  if (error) console.error(error);
  for (const row of observations) {
    console.log(`#${row.pull} ${row.head} ${row.run ?? row.url} ${row.problem ?? `aggregate ${row.aggregate}, pause step ${row.pauseStep}, platform step ${row.platformStep}, checkout step ${row.checkoutStep}`}`);
  }
  if (!everyOpenPullRequestRunsPausedWorkflow) process.exitCode = 1;
}
