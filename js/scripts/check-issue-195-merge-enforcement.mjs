#!/usr/bin/env node
// The post-merge merge-enforcement report (I195-ACCEPTANCE-REQUIRED-MERGE-CHECK).
// It runs on main after merge with the workflow token and never as a pull
// request check: the rules it inspects are live state outside the pull request.
import { execFileSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { buildIssue195Manifest } from './issue-195-acceptance-lib.mjs';
import {
  evaluateMergeEnforcement, inspectMergeEnforcement, verifyEvaluatedCheckout,
} from './issue-195-merge-enforcement.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const option = (name, fallback) => {
  const index = process.argv.indexOf(name);
  if (index < 0) return fallback;
  if (!process.argv[index + 1] || process.argv[index + 1].startsWith('--')) throw new Error(`${name} needs a value`);
  return process.argv[index + 1];
};
const commit = option('--commit', execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim());
const output = path.resolve(option('--report', path.join(root, 'issue-195-results/artifacts/merge-enforcement.json')));
await mkdir(path.dirname(output), { recursive: true });
let report;
try {
  verifyEvaluatedCheckout(root, commit);
  const snapshot = await inspectMergeEnforcement({ repository: 'link-foundation/meta-language' });
  const acceptanceWorkflow = await readFile(path.join(root, '.github/workflows/ci.yml'), 'utf8');
  verifyEvaluatedCheckout(root, commit);
  report = {
    commit, snapshot,
    ...evaluateMergeEnforcement(snapshot, {
      commit, acceptanceWorkflow, manifest: await buildIssue195Manifest(root),
    }),
  };
} catch (error) {
  report = { commit, checks: {}, errors: [error.message], notes: [] };
}
await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
for (const [assertion, holds] of Object.entries(report.checks)) {
  console.log(`${assertion}: ${holds ? 'observed' : 'missing'}`);
}
for (const note of report.notes) console.log(`note: ${note}`);
for (const error of report.errors) console.error(error);
console.log(`Post-merge merge enforcement report: ${output}`);
if (report.errors.length > 0) process.exitCode = 1;
