import { execFileSync } from 'node:child_process';
import { isDeepStrictEqual } from 'node:util';

import { evaluateIssue195Acceptance } from './issue-195-acceptance-lib.mjs';

export const FULL_REQUIREMENTS_CHECK = 'Full Requirements Aggregate';
const RELEASE_REQUIREMENTS = Object.freeze([
  'I195-DELIVERY-NPM-PUBLISHED',
  'I195-DELIVERY-CRATE-PUBLISHED',
  'I195-DELIVERY-RML-PUBLISHED',
  'I195-DOWNSTREAM-FORMAL-AI-PUBLISHED',
]);

/** Evidence may only describe the clean revision that actually executes it. */
export function verifyEvaluatedCheckout(root, commit) {
  const git = (args) => execFileSync('git', args, { cwd: root, encoding: 'utf8' }).trim();
  if (git(['rev-parse', 'HEAD']) !== commit) throw new Error('evaluated commit differs from the checked-out commit');
  if (git(['status', '--porcelain', '--untracked-files=normal'])) {
    throw new Error('evaluated checkout has uncommitted changes; commit them before recording evidence');
  }
}

function targetsDefaultBranch(ruleset, branch) {
  const names = ruleset.conditions?.ref_name;
  // The effective-rules endpoint resolves glob patterns. Also require an
  // explicit default-branch target in the backing rule, rather than guessing
  // whether a pattern includes the branch.
  const targets = ['~DEFAULT_BRANCH', '~ALL', `refs/heads/${branch}`];
  return names?.include?.some((name) => targets.includes(name)) === true &&
    Array.isArray(names.exclude) && names.exclude.length === 0;
}

function verifyDeliverySeparation(manifest, commit) {
  const preMerge = evaluateIssue195Acceptance(manifest, [], { checkpoint: 'pre-merge', commit });
  const release = evaluateIssue195Acceptance(manifest, [], { checkpoint: 'release-delivery', commit });
  const releaseRequirements = release.requirements.map(({ id }) => id).sort();
  const passed = JSON.stringify(releaseRequirements) === JSON.stringify([...RELEASE_REQUIREMENTS].sort()) &&
    !preMerge.requirements.some(({ id }) => RELEASE_REQUIREMENTS.includes(id)) &&
    RELEASE_REQUIREMENTS.every((id) => {
      const row = manifest.atomicRequirements.find((entry) => entry.id === id);
      return row?.verifications.length > 0 && row.verifications.every((cell) =>
        cell.checkpoint === 'release-delivery' && cell.assertions.length > 0);
    }) && release.passed === false && release.requirements.every((row) =>
      !row.passed && row.cells.every((cell) => cell.reasons.includes('required result is missing')));
  return { passed, releaseRequirements, releaseEvidencePassed: release.passed };
}

/** Evaluate observed GitHub responses; mock responses never produce ledger records. */
export function evaluateMergeEnforcement(snapshot, { head, commit, manifest, runningWorkflowId = null }) {
  const errors = [];
  const branch = snapshot.repository?.default_branch;
  const pull = snapshot.pullRequest;
  const merge = snapshot.mergeState;
  const candidate = snapshot.candidate;
  const current = typeof head === 'string' && /^[a-f0-9]{40}$/.test(head) &&
    pull?.state === 'open' && pull.head?.sha === head && merge?.headRefOid === head &&
    pull.base?.ref === branch && pull.base?.sha === merge.baseRefOid &&
    candidate?.sha === pull.merge_commit_sha &&
    [head, candidate?.sha].includes(commit) &&
    candidate?.parents?.some(({ sha }) => sha === head) &&
    candidate?.parents?.some(({ sha }) => sha === pull.base.sha);
  if (!current) errors.push('PR head, base and merge candidate must match the evaluated revision');

  const activeRules = (snapshot.effectiveRules ?? []).filter((rule) => {
    const ruleset = (snapshot.rulesets ?? []).find(({ id }) => id === rule.ruleset_id);
    return typeof branch === 'string' && ruleset?.enforcement === 'active' &&
      ruleset.target === 'branch' && Array.isArray(ruleset.bypass_actors) &&
      ruleset.current_user_can_bypass === 'never' &&
      ruleset.bypass_actors.length === 0 && targetsDefaultBranch(ruleset, branch) &&
      ruleset.rules?.some((backing) => backing.type === rule.type &&
        isDeepStrictEqual(backing.parameters, rule.parameters));
  });
  const activeRuleTargetsDefaultBranch = activeRules.length > 0;
  if (!activeRuleTargetsDefaultBranch) errors.push('no active, non-bypassable rule targets the default branch');
  const requiredChecks = activeRules.flatMap((rule) => {
    const parameters = rule.parameters;
    return rule.type === 'required_status_checks' &&
      parameters?.strict_required_status_checks_policy === true &&
      parameters.do_not_enforce_on_create === false
      ? parameters.required_status_checks ?? [] : [];
  }).filter(({ context }) => context === FULL_REQUIREMENTS_CHECK);
  const fullAggregateRequired = requiredChecks.length > 0;
  if (!fullAggregateRequired) errors.push('Full Requirements Aggregate is not a strict required check');

  const matchesCandidate = (workflow) => workflow?.event === 'pull_request' && workflow.head_sha === head &&
    workflow.path === '.github/workflows/issue-195-acceptance.yml' &&
    workflow.pull_requests?.some((entry) => entry.number === pull?.number &&
      entry.head?.sha === head && entry.base?.sha === pull?.base?.sha && entry.base?.ref === branch);
  const checks = (snapshot.checkRuns ?? []).filter((entry) =>
    entry.name === FULL_REQUIREMENTS_CHECK && [head, candidate?.sha].includes(entry.head_sha))
    .sort((first, second) => second.id - first.id);
  // A running evidence producer cannot use itself as a failed-check probe.
  // On a rerun it can inspect a completed failure of the same head/base pair.
  // Other pending reruns still invalidate the probe, and a changed candidate
  // needs a new completed failure before this assertion can be recorded.
  const producingCheck = checks.find((entry) => {
    const workflow = (snapshot.workflowRuns ?? []).find(({ checkId }) => checkId === entry.id);
    return entry.status !== 'completed' && runningWorkflowId !== null &&
      String(workflow?.id) === String(runningWorkflowId) && matchesCandidate(workflow);
  });
  const check = checks.find((entry) => entry !== producingCheck);
  // GitHub attaches pull-request check runs to the head SHA even though
  // checkout evaluates the synthetic merge. The workflow's authenticated
  // head/base pair must therefore match both parents of the current candidate.
  const workflow = (snapshot.workflowRuns ?? []).find(({ checkId }) => checkId === check?.id);
  const failingCheckBlocksMerge = current && fullAggregateRequired &&
    matchesCandidate(workflow) &&
    check?.status === 'completed' && ['failure', 'timed_out', 'action_required'].includes(check.conclusion) &&
    requiredChecks.some(({ integration_id: application }) =>
      !application || application === check.app?.id) && merge.mergeStateStatus === 'BLOCKED';
  if (!failingCheckBlocksMerge) errors.push('no completed failing required check blocks the current merge candidate');

  const delivery = verifyDeliverySeparation(manifest, commit);
  if (!delivery.passed) errors.push('published delivery must remain a separate, fail-closed release-delivery checkpoint');
  return {
    checks: {
      activeRuleTargetsDefaultBranch, fullAggregateRequired, failingCheckBlocksMerge,
      publishedDeliverySeparatelyVerified: delivery.passed,
    },
    errors, delivery, candidate: candidate?.sha ?? null, check: check ?? null,
    producingCheck: producingCheck ?? null,
  };
}

/** Fetch live, read-only evidence. Preserve raw responses for independent review. */
export async function inspectMergeEnforcement({ repository, pullRequest, query = githubQuery }) {
  const prefix = `repos/${repository}`;
  const [metadata, pull, mergeState] = await Promise.all([
    query(['api', prefix]),
    query(['api', `${prefix}/pulls/${pullRequest}`]),
    query(['pr', 'view', String(pullRequest), '--repo', repository, '--json',
      'headRefOid,baseRefOid,mergeStateStatus']),
  ]);
  const effectiveRules = await query(['api', `${prefix}/rules/branches/${metadata.default_branch}`, '--paginate']);
  const [candidate, checkPages, ...rulesets] = await Promise.all([
    query(['api', `${prefix}/commits/${pull.merge_commit_sha}`]),
    query(['api', `${prefix}/commits/${pull.head.sha}/check-runs?filter=all`, '--paginate', '--slurp']),
    ...[...new Set(effectiveRules.map(({ ruleset_id: id }) => id))].map((id) =>
      query(['api', `${prefix}/rulesets/${id}`])),
  ]);
  const checkRuns = checkPages.flatMap((page) => page.check_runs);
  const workflowRuns = await Promise.all(checkRuns.filter(({ name }) => name === FULL_REQUIREMENTS_CHECK)
    .map(async (check) => {
      const runId = check.details_url?.match(/\/actions\/runs\/(\d+)\/job\/\d+$/)?.[1];
      if (!runId) return { checkId: check.id };
      return { ...await query(['api', `${prefix}/actions/runs/${runId}`]), checkId: check.id };
    }));
  // A second query detects a head/base change while collecting the responses.
  const finalMergeState = await query(['pr', 'view', String(pullRequest), '--repo', repository,
    '--json', 'headRefOid,baseRefOid,mergeStateStatus']);
  if (mergeState.headRefOid !== finalMergeState.headRefOid ||
      mergeState.baseRefOid !== finalMergeState.baseRefOid) {
    throw new Error('PR revision changed during live rule inspection; collect fresh evidence');
  }
  return {
    collectedAt: new Date().toISOString(), repository: metadata, pullRequest: pull,
    mergeState: finalMergeState, candidate, effectiveRules, rulesets,
    checkRuns, workflowRuns,
  };
}

async function githubQuery(args) {
  return JSON.parse(execFileSync('gh', args, { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 }));
}
