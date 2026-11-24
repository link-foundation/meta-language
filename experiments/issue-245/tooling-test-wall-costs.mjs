import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads, partitionTests } from '../../js/scripts/test-groups.mjs';
import { measureSequentialTestFiles } from './javascript-test-wall-costs.mjs';

const [directory, option, ...extra] = process.argv.slice(2);
assert.ok(directory && extra.length === 0 && (option === undefined || option === '--write'));
const root = new URL('../../', import.meta.url);
const baseline = readProjectData(new URL('parity/fixtures/ci-39615628279-workload-costs.lino', root));
const snapshots = [
  { run: 45250356617, head: '4ecca483361d5f47ad38698fe0d24653f6ef641f',
    revision: '3224c9a1d9e26d7eebe31e7cbfd563555bd69b9b',
    jobIds: [137958366983, 137958366483, 137958366592, 137958365906] },
  { run: 45296878458, head: '8561a770850ea0b9cfa218c23ccc6958dae2a16a',
    revision: 'c96947107d97819e90af6406279a8f76212f3bc8',
    jobIds: [138119600350, 138119600477, 138119599865, 138119600190] },
];

const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2_000_000 });
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const logs = [];
const entries = new Map();
let files, precedingAssignment;

for (const { run, head, revision, jobIds } of snapshots) {
  assert.equal(git('rev-parse', revision + '^2').toString().trim(), head);
  assert.equal(git('rev-parse', revision + '^{tree}').toString(), git('rev-parse', head + '^{tree}').toString());
  const jobs = JSON.parse(readFileSync(join(directory, `ci-${run}-jobs.json`))).flatMap(page => page.jobs);
  files = partitionTests(git('ls-tree', '--name-only', `${revision}:js/tests`).toString().trim().split('\n')
    .filter(file => file.endsWith('.test.js'))).groups.tooling.map(file => 'js/tests/' + file);
  const sources = new Map(files.map(file => [file, git('show', `${revision}:${file}`)]));
  precedingAssignment = Array.from({ length: 4 }, (_, index) => files.filter((_, offset) => offset % 4 === index));

  for (const [index, id] of jobIds.entries()) {
    const number = index + 1;
    const job = jobs.find(row => row.id === id);
    assert.ok(job);
    assert.equal(job.name, `JavaScript / JavaScript Tests (tooling, ${number}/4)`);
    assert.equal(job.run_id, run);
    assert.equal(job.run_attempt, 2);
    assert.equal(job.head_sha, head);
    assert.ok(['success', 'failure', 'cancelled'].includes(job.conclusion));
    const bytes = readFileSync(join(directory, `ci-${run}-job-${id}.log`));
    const log = bytes.toString('utf8');
    assert.ok(bytes.length < 2_000_000 && log.includes(revision) && log.includes('Cleaning up orphan processes'));
    assert.ok(log.includes(`TEST_GROUP_SHARD: ${number}/4`));
    const failures = Number(/Z # fail (\d+)\r?$/mu.exec(log)?.[1]);
    assert.ok(Number.isSafeInteger(failures));
    assert.equal(failures > 0, job.conclusion === 'failure');
    const assigned = precedingAssignment[index];
    const { samples, testMilliseconds } = measureSequentialTestFiles(log, assigned, sources);
    const setupMilliseconds = Date.parse(job.steps.find(step => step.name === 'Run the test group').started_at)
      - Date.parse(job.started_at);
    assert.ok(setupMilliseconds > 0 && testMilliseconds > 0);
    logs.push({ run, head, revision, id, number, conclusion: job.conclusion, bytes: bytes.length, sha256: hash(bytes),
      setupMilliseconds, testMilliseconds, failedTests: failures, measuredFiles: samples.size, scheduledFiles: assigned.length });
    for (const [file, milliseconds] of samples) {
      const source = sources.get(file);
      const sha256 = hash(source);
      const older = entries.get(file);
      entries.set(file, { file, bytes: source.length, sha256,
        milliseconds: older?.sha256 === sha256 ? Math.max(milliseconds, older.milliseconds) : milliseconds });
    }
  }
}

const currentEntries = files.flatMap(file => entries.has(file) ? [entries.get(file)] : []);
const measuredLoads = shards => shards.map(shard => shard.reduce((total, file) => total + (entries.get(file)?.milliseconds ?? 0), 0));
const balanced = partitionMeasuredWorkloads(files, 4, { costs: { ...baseline, entries: currentEntries } });
console.log(JSON.stringify({ logs, measured: currentEntries.length, total: files.length,
  unmeasured: files.filter(file => !entries.has(file)), previous: measuredLoads(precedingAssignment), balanced: measuredLoads(balanced) }, null, 2));

if (option === '--write') {
  assert.ok(currentEntries.length >= files.length * 0.85);
  const { run, revision } = snapshots.at(-1);
  writeProjectData(new URL(`parity/fixtures/ci-${run}-tooling-test-wall-costs.lino`, root), {
    schemaVersion: 1, run, revision, logs, entries: currentEntries,
    boundary: 'Advisory sequential JavaScript tooling file costs from complete retained CI test streams. Each source digest uses the largest measured time for its exact bytes. Unidentified and changed files retain the existing size-based estimate. Samples choose owners and never establish test completion.',
  });
}
