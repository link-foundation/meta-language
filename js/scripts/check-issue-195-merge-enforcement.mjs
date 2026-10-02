#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { buildIssue195Manifest } from './issue-195-acceptance-lib.mjs';
import {
  evaluateMergeEnforcement, githubQuery, inspectMergeEnforcement, verifyEvaluatedCheckout,
} from './issue-195-merge-enforcement.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const option = (name, fallback) => {
  const index = process.argv.indexOf(name);
  if (index < 0) return fallback;
  if (!process.argv[index + 1] || process.argv[index + 1].startsWith('--')) throw new Error(`${name} needs a value`);
  return process.argv[index + 1];
};
const commit = option('--commit', execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim());
const head = option('--head', process.env.ISSUE_195_PULL_REQUEST_HEAD ?? commit);
const runningWorkflowId = process.env.GITHUB_RUN_ID ?? null;
const output = path.resolve(option('--report', path.join(root, 'issue-195-results/artifacts/merge-enforcement.json')));
await mkdir(path.dirname(output), { recursive: true });
let report;
try {
  verifyEvaluatedCheckout(root, commit);
  // The workflow token reads the effective rules and the rulesets' strict
  // policy. It cannot read ruleset bypass actors; an optional token with
  // repository Administration read permission adds only those.
  const rulesetToken = process.env.ISSUE_195_RULESET_TOKEN || null;
  const snapshot = await inspectMergeEnforcement({
    repository: 'link-foundation/meta-language', pullRequest: 196,
    ...(rulesetToken ? { rulesetQuery: (args) => githubQuery(args, { token: rulesetToken }) } : {}),
  });
  snapshot.rulesetCredential = rulesetToken ? 'ISSUE_195_RULESET_TOKEN' : 'GH_TOKEN';
  const acceptanceWorkflow = await readFile(path.join(root, '.github/workflows/issue-195-acceptance.yml'), 'utf8');
  verifyEvaluatedCheckout(root, commit);
  report = {
    commit, head, snapshot,
    ...evaluateMergeEnforcement(snapshot, {
      head, commit, runningWorkflowId, acceptanceWorkflow, manifest: await buildIssue195Manifest(root),
    }),
  };
} catch (error) {
  report = { commit, head, checks: {}, errors: [error.message] };
}
await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
for (const [assertion, holds] of Object.entries(report.checks)) {
  console.log(`${assertion}: ${holds ? 'observed' : 'missing'}`);
}
for (const error of report.errors) console.error(error);
console.log(`Live merge enforcement report: ${output}`);
if (report.errors.length > 0) process.exitCode = 1;
