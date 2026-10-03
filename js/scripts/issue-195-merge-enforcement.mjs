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
  const changes = git(['status', '--porcelain', '--untracked-files=normal']);
  if (changes) {
    throw new Error(`evaluated checkout has uncommitted changes; commit them before recording evidence:\n${changes}`);
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

/**
 * Problems that let the aggregate job finish as skipped. GitHub reports a
 * skipped job as a successful required check, so the job must always run and
 * fail itself when the delivery candidates it evaluates are unavailable.
 */
export function aggregateSkipProblems(workflow) {
  if (typeof workflow !== 'string') return ['the acceptance workflow is unavailable'];
  const lines = workflow.split(/\r?\n/);
  const start = lines.findIndex((line) => line === `    name: ${FULL_REQUIREMENTS_CHECK}`);
  if (start < 1 || !/^  [\w-]+:$/.test(lines[start - 1])) {
    return [`the acceptance workflow has no job named ${FULL_REQUIREMENTS_CHECK}`];
  }
  const length = lines.slice(start).findIndex((line) => /^  \S/.test(line));
  const job = lines.slice(start, length < 0 ? undefined : start + length);
  const problems = [];
  if (!job.includes('    if: ${{ always() }}')) {
    problems.push(`${FULL_REQUIREMENTS_CHECK} must run with if: \${{ always() }}; otherwise it can be skipped and count as passing`);
  }
  const steps = job.findIndex((line) => line === '    steps:');
  const next = job.findIndex((line, index) => index > steps + 1 && line.startsWith('      - '));
  const guard = steps < 0 ? [] : job.slice(steps + 1, next < 0 ? job.length : next);
  if (!guard.includes("        if: ${{ needs.candidates.result != 'success' }}") ||
      !guard.some((line) => /^\s+exit 1$/.test(line))) {
    problems.push(`the first ${FULL_REQUIREMENTS_CHECK} step must fail when Delivery Candidates did not succeed; a skipped aggregate counts as passing`);
  }
  return problems;
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

const FAILED_CONCLUSIONS = Object.freeze(['failure', 'timed_out', 'action_required']);

/**
 * Evaluates the observed GitHub responses of the post-merge report; mock
 * responses never produce ledger records. The report runs on the default
 * branch after merge, so it never depends on, or waits for, the checks of the
 * pull request it would otherwise have to fail.
 */
export function evaluateMergeEnforcement(snapshot, { commit, manifest, acceptanceWorkflow }) {
  const errors = [];
  const notes = [];
  const branch = snapshot.repository?.default_branch;
  if (typeof branch !== 'string') errors.push('the repository response names no default branch');
  if (snapshot.branchHead !== commit) {
    notes.push(`the default branch is at ${snapshot.branchHead ?? 'an unknown commit'}, not the evaluated ${commit}; ` +
      'the report of the newer push supersedes this one');
  }

  // The effective rules (GET /repos/{owner}/{repo}/rules/branches/{branch})
  // and the ruleset's enforcement, target, rules and current-user bypass are
  // readable with the workflow token. The bypass-actor list is not; when
  // GitHub hides it, the report says so instead of asking for a credential.
  const enforcedRules = (snapshot.effectiveRules ?? []).filter((rule) => {
    const ruleset = (snapshot.rulesets ?? []).find(({ id }) => id === rule.ruleset_id);
    return typeof branch === 'string' && ruleset?.enforcement === 'active' &&
      ruleset.target === 'branch' && targetsDefaultBranch(ruleset, branch) &&
      ruleset.rules?.some((backing) => backing.type === rule.type &&
        isDeepStrictEqual(backing.parameters, rule.parameters));
  });
  const nonBypassable = (rule) => {
    const ruleset = snapshot.rulesets.find(({ id }) => id === rule.ruleset_id);
    return ruleset.current_user_can_bypass === 'never' &&
      (ruleset.bypass_actors === undefined || (Array.isArray(ruleset.bypass_actors) && ruleset.bypass_actors.length === 0));
  };
  const activeRuleTargetsDefaultBranch = enforcedRules.some(nonBypassable);
  if (!activeRuleTargetsDefaultBranch) errors.push('no active, non-bypassable rule targets the default branch');
  for (const ruleset of snapshot.rulesets ?? []) {
    if (ruleset.bypass_actors === undefined) {
      notes.push(`ruleset ${ruleset.id} does not show its bypass actors to the workflow token; ` +
        'the report relies on current_user_can_bypass');
    }
  }
  const requiredChecks = enforcedRules.flatMap((rule) => {
    const parameters = rule.parameters;
    return rule.type === 'required_status_checks' &&
      parameters?.strict_required_status_checks_policy === true &&
      parameters.do_not_enforce_on_create === false
      ? parameters.required_status_checks ?? [] : [];
  }).filter(({ context }) => context === FULL_REQUIREMENTS_CHECK);
  const skipProblems = aggregateSkipProblems(acceptanceWorkflow);
  errors.push(...skipProblems);
  const fullAggregateRequired = requiredChecks.length > 0 && skipProblems.length === 0;
  if (requiredChecks.length === 0) errors.push('Full Requirements Aggregate is not a strict required check');

  // The failing-check probe is any open pull request into the default branch
  // whose latest aggregate failed and whose merge GitHub reports as blocked.
  // Without such a pull request the assertion stays unobserved until one
  // exists; the report never makes a check fail to observe it.
  const blockedPullRequest = (snapshot.pullRequests ?? []).find((pull) => {
    if (pull.baseRefName !== branch || pull.mergeStateStatus !== 'BLOCKED') return false;
    const latest = (pull.checkRuns ?? [])
      .filter((entry) => entry.name === FULL_REQUIREMENTS_CHECK && entry.head_sha === pull.headRefOid)
      .sort((first, second) => second.id - first.id)[0];
    return latest?.status === 'completed' && FAILED_CONCLUSIONS.includes(latest.conclusion) &&
      requiredChecks.some(({ integration_id: application }) => !application || application === latest.app?.id);
  }) ?? null;
  const failingCheckBlocksMerge = fullAggregateRequired && blockedPullRequest !== null;
  if (!failingCheckBlocksMerge) {
    errors.push('no open pull request into the default branch has a failed required aggregate that blocks its merge');
  }

  const delivery = verifyDeliverySeparation(manifest, commit);
  if (!delivery.passed) errors.push('published delivery must remain a separate, fail-closed release-delivery checkpoint');
  return {
    checks: {
      activeRuleTargetsDefaultBranch, fullAggregateRequired, failingCheckBlocksMerge,
      publishedDeliverySeparatelyVerified: delivery.passed,
    },
    errors, notes, delivery, blockedPullRequest: blockedPullRequest?.number ?? null,
  };
}

/** Fetches live, read-only evidence with the workflow token; the raw responses stay in the report. */
export async function inspectMergeEnforcement({ repository, query = githubQuery, pullRequestLimit = 20 }) {
  const prefix = `repos/${repository}`;
  const metadata = await query(['api', prefix]);
  const branch = metadata.default_branch;
  const [branchResponse, effectiveRules, pulls] = await Promise.all([
    query(['api', `${prefix}/branches/${branch}`]),
    query(['api', `${prefix}/rules/branches/${branch}`, '--paginate']),
    query(['pr', 'list', '--repo', repository, '--state', 'open', '--base', branch,
      '--limit', String(pullRequestLimit), '--json', 'number,headRefOid,baseRefName,mergeStateStatus']),
  ]);
  const rulesets = await Promise.all([...new Set(effectiveRules.map(({ ruleset_id: id }) => id))]
    .map((id) => query(['api', `${prefix}/rulesets/${id}`])));
  const pullRequests = await Promise.all(pulls.map(async (pull) => {
    const pages = await query(['api',
      `${prefix}/commits/${pull.headRefOid}/check-runs?check_name=${encodeURIComponent(FULL_REQUIREMENTS_CHECK)}&filter=all`,
      '--paginate', '--slurp']);
    return { ...pull, checkRuns: pages.flatMap((page) => page.check_runs) };
  }));
  return {
    collectedAt: new Date().toISOString(), repository: metadata, branchHead: branchResponse.commit?.sha ?? null,
    effectiveRules, rulesets, pullRequests,
  };
}

export async function githubQuery(args, { environment = process.env, execute = execFileSync } = {}) {
  // setup-ocaml forces terminal colors for later steps. Forced settings can
  // override NO_COLOR, so remove both controls from this JSON subprocess.
  const queryEnvironment = { ...environment, NO_COLOR: '1', CLICOLOR: '0' };
  delete queryEnvironment.CLICOLOR_FORCE;
  delete queryEnvironment.GH_FORCE_TTY;
  return JSON.parse(execute('gh', args, {
    encoding: 'utf8', maxBuffer: 32 * 1024 * 1024,
    env: queryEnvironment,
  }));
}
