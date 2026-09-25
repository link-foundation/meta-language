// Runs the two default CST evidence tests with an observation file and reports
// how many positive CST cells of the pre-merge plan their records complete.
import { execFileSync } from 'node:child_process';
import { readFile, rm } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { buildIssue195Manifest } from '../scripts/issue-195-acceptance-lib.mjs';
import { buildEvidencePlan, observedEvidenceForCell } from '../scripts/issue-195-evidence-plan.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const observations = '/tmp/cst-positive-observations.jsonl';
await rm(observations, { force: true });
const env = { ...process.env, ISSUE_195_OBSERVATION_FILE: observations, ISSUE_195_COMMIT: 'experiment' };
execFileSync('node', ['--test', 'tests/default-cst-expectations.test.js'], { cwd: path.join(root, 'js'), env, stdio: 'ignore' });
execFileSync('cargo', ['test', '--all-features', '--test', 'unit', 'every_rust_inventory'], { cwd: path.join(root, 'rust'), env, stdio: 'ignore' });
const records = (await readFile(observations, 'utf8')).split('\n').filter(Boolean).map((line) => JSON.parse(line));
const manifest = await buildIssue195Manifest(root);
const cells = buildEvidencePlan(manifest, 'pre-merge')
  .flatMap(({ group, cells }) => cells.map(({ cell }) => ({ group, cell })))
  .filter(({ cell }) => /^i195-cst-.*-positive$/u.test(cell.testId));
const incomplete = cells.filter(({ cell }) => !observedEvidenceForCell(cell, records).complete);
console.log(`${records.length} records; ${cells.length - incomplete.length}/${cells.length} positive CST cells complete`);
for (const { group, cell } of incomplete.slice(0, 5)) console.log('incomplete', group, cell.testId, cell.fixtureIds);
