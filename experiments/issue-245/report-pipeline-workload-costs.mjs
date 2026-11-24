import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { writeProjectData } from '../../js/scripts/project-data.mjs';

const attempt = 2;
const shardCount = 16;
const root = new URL('../../', import.meta.url);
const [directory, runOption = '45104457483', ...extra] = process.argv.slice(2);
assert.ok(directory && extra.length === 0, 'provide the retained CI log directory and optional run ID');
const runs = {
  45104457483: {
    revision: 'be84faf03e8cf02b3d135f9f5f7c8bf8406da239',
    head: '591e98639e99c569f063214c3ae8a756185f4add',
    cancelledShard: 16,
    expectedUnfinished: ['js/src/typed-program-links.js', 'js/src/typescript-erasure-rules.js'],
  },
  45215384144: {
    revision: 'e6fd033e357df12c76a2714a463a769964307d00',
    head: 'f92830dae2cad6f4cd9cb679bd09ab471635a4a6',
    cancelledShard: 5,
    expectedUnfinished: ['js/src/typed-program-expression-projection.js'],
  },
  45296878458: {
    revision: 'c96947107d97819e90af6406279a8f76212f3bc8',
    head: '8561a770850ea0b9cfa218c23ccc6958dae2a16a',
    cancelledShard: null,
    expectedUnfinished: [],
  },
};
const run = Number(runOption);
assert.ok(Object.hasOwn(runs, run), 'run ID requires a reviewed merge checkout and report outcome');
const { revision, head, cancelledShard, expectedUnfinished } = runs[run];
const digest = value => createHash('sha256').update(value).digest('hex');
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2 * 1024 * 1024 });
assert.equal(git('rev-parse', revision + '^2').toString().trim(), head);
const sources = git('ls-tree', '-r', '--name-only', revision, 'js/src').toString().split('\n').filter(file => file.endsWith('.js')).sort();

const jobs = JSON.parse(readFileSync(join(directory, 'ci-' + run + '-jobs.json'), 'utf8')).flatMap(page => page.jobs);
const reportJobs = jobs.flatMap(job => {
  const match = /^Self-Translation Report \((\d+)\/16\)$/u.exec(job.name);
  return match ? [{ number: Number(match[1]), job }] : [];
}).sort((left, right) => left.number - right.number);
assert.deepEqual(reportJobs.map(report => report.number), Array.from({ length: shardCount }, (_, index) => index + 1));

const started = new Set();
const entries = [];
const logs = reportJobs.map(({ number, job }) => {
  assert.equal(job.run_id, run);
  assert.equal(job.run_attempt, attempt);
  assert.equal(job.head_sha, head);
  assert.equal(job.conclusion, number === cancelledShard ? 'cancelled' : 'success');
  const bytes = readFileSync(join(directory, 'ci-' + run + '-job-' + job.id + '.log'));
  const source = bytes.toString('utf8');
  assert.ok(bytes.length <= 262144 && source.includes('Cleaning up orphan processes'), 'complete retained job log');
  assert.ok(source.includes('ref: ' + revision) && source.includes('HEAD is now at ' + revision.slice(0, 7) + ' Merge ' + head), 'exact CI merge checkout');

  const pending = new Map();
  const completed = new Map();
  for (const [, time, phase, file, elapsed] of source.matchAll(/^([\dTZ.:\-]+) self-translation report: (resolving|recorded) (js\/src\/[\w./-]+\.js)(?: \((\d+) ms\))?\r?$/gmu)) {
    const timestamp = Date.parse(time);
    assert.ok(Number.isFinite(timestamp));
    if (phase === 'resolving') {
      assert.ok(pending.size < 2 && !started.has(file), file);
      started.add(file);
      pending.set(file, { timestamp, shared: 0 });
      continue;
    }
    assert.equal(file, pending.keys().next().value);
    const { shared } = pending.get(file);
    pending.delete(file);
    const next = pending.values().next().value;
    const overlap = next ? (timestamp - next.timestamp) / 2 : 0;
    assert.ok(overlap >= 0 && elapsed !== undefined);
    if (next) next.shared += overlap;
    completed.set(file, Math.max(1, Math.round(Number(elapsed) - shared - overlap)));
  }

  const unfinished = [...pending.keys()];
  assert.deepEqual(unfinished, number === cancelledShard ? expectedUnfinished : []);
  for (const [file, milliseconds] of completed) {
    const sourceBytes = git('show', revision + ':' + file);
    entries.push({ file, bytes: sourceBytes.length, sha256: digest(sourceBytes), milliseconds });
  }
  return { job: job.id, number, conclusion: job.conclusion, bytes: bytes.length, sha256: digest(bytes), unfinished };
});
assert.deepEqual([...started].sort(), sources);
assert.equal(entries.length, sources.length - expectedUnfinished.length);
entries.sort((left, right) => left.file.localeCompare(right.file));
writeProjectData(new URL(`parity/fixtures/ci-${run}-report-workload-costs.lino`, root), {
  schemaVersion: 1, run, attempt, head, revision, logs, entries,
  boundary: 'Advisory module elapsed durations with shared import and measurement overlap divided evenly. Unfinished modules supply no timing sample or requirement observation.',
});
