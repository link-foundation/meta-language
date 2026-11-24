import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { writeProjectData } from '../../js/scripts/project-data.mjs';

const run = 45076523061;
const attempt = 2;
const revision = '432bc6adde4a01fcc1a2547a8f2b611cf26c1166';
const head = '93f789ffa42bf3e87d7f732b9db24363a591ea7e';
const root = new URL('../../', import.meta.url);
const directory = process.argv[2];
assert.equal(process.argv.length, 3, 'provide the directory with the retained complete CI logs');
const digest = value => createHash('sha256').update(value).digest('hex');
const { jobs } = JSON.parse(readFileSync(join(directory, run + '-jobs.json'), 'utf8'));
const reportJobs = jobs.flatMap(job => {
  const match = /^Self-Translation Report \((\d+)\/16\)$/u.exec(job.name);
  return match ? [{ number: Number(match[1]), job }] : [];
}).sort((left, right) => left.number - right.number);
assert.deepEqual(reportJobs.map(report => report.number), Array.from({ length: 16 }, (_, index) => index + 1));

const measurements = new Map();
const logs = reportJobs.map(({ number, job }) => {
  assert.equal(job.run_id, run);
  assert.equal(job.run_attempt, attempt);
  assert.equal(job.head_sha, head);
  assert.ok(job.conclusion === 'success' || job.conclusion === 'cancelled');
  const bytes = readFileSync(join(directory, 'report-current-' + job.id + '.log'));
  const source = bytes.toString('utf8');
  assert.ok(bytes.length <= 262144 && source.includes('Cleaning up orphan processes'), 'complete retained job log');
  assert.ok(source.includes(revision), 'the job checked out the reviewed merge commit');
  let start;
  for (const line of source.split('\n')) {
    const timestamp = Date.parse(line.slice(0, 28));
    if (start === undefined && line.includes('self-translation report: resolving js/src/')) start = timestamp;
    const completed = /self-translation report: recorded (js\/src\/[\w./-]+\.js) \(/u.exec(line);
    if (!completed) continue;
    assert.ok(Number.isFinite(start) && timestamp >= start && !measurements.has(completed[1]));
    measurements.set(completed[1], Math.max(1, timestamp - start));
    start = timestamp;
  }
  return { job: job.id, number, conclusion: job.conclusion, bytes: bytes.length, sha256: digest(bytes) };
});
assert.equal(measurements.size, 364);
const entries = [...measurements].sort(([left], [right]) => left.localeCompare(right)).map(([file, milliseconds]) => {
  const bytes = execFileSync('git', ['--no-replace-objects', 'show', revision + ':' + file],
    { cwd: root, maxBuffer: 2 * 1024 * 1024 });
  return { file, bytes: bytes.length, sha256: digest(bytes), milliseconds };
});
writeProjectData(new URL('parity/fixtures/ci-45076523061-report-workload-costs.lino', root), {
  schemaVersion: 1, run, attempt, head, revision, logs, entries,
  boundary: 'Advisory elapsed intervals ending in a recorded module in CI. Incomplete measurements are omitted. No requirement observation is inferred.',
});
