// Replay bounded archived CI logs into the four-shard advisory tooling costs.
// Usage: node experiments/ci-workload-costs/capture-tooling-wall-schedule.mjs ci-logs [output]
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads, partitionTests } from '../../js/scripts/test-groups.mjs';
import { testFileWallCosts } from './compare-translation-wall-costs.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const [directory, output = join(root, 'parity/fixtures/ci-45186623729-tooling-wall-costs.lino')] = process.argv.slice(2);
assert.ok(directory, 'pass the directory containing the retained tooling job logs');
const previous = readProjectData(join(root, 'parity/fixtures/ci-45112870920-tooling-costs.lino'));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2 * 1024 * 1024 });
const captures = [
  { run: 45133354161, prefix: '80d8cef69', revision: '80d8cef693b328e54ad31d74b3c4b688f6c00f06',
    merge: '0f33a52c609f09f5227b244353d012e683c321a1', jobs: [null, 137553726880, null, null] },
  { run: 45159625352, prefix: '03a81ad9e', revision: '03a81ad9e04abf583eabca7f13f8d6bb6947b521',
    merge: '27b65da66f8e82f226686e6108e7321fdf66d4c8', jobs: [137641360003, 137641359142, 137641358796, 137641359095] },
  { run: 45186623729, prefix: '897b1f6b', revision: '897b1f6b2ff7df5e094ca5d420b9c6e0057ef030',
    merge: '432b7c03ff50bb0c2462973e30a7daf5767f3dad', jobs: [null, 137733805955, null, null] },
];
const filesAt = revision => partitionTests(git('ls-tree', '-r', '--name-only', revision, 'js/tests').toString()
  .split('\n').filter(file => /^js\/tests\/[^/]+\.test\.js$/u.test(file)).map(file => file.slice('js/tests/'.length)))
  .groups.tooling.map(file => `js/tests/${file}`);
const { run, revision } = captures.at(-1);
const entries = filesAt(revision).map(file => {
  const bytes = git('show', `${revision}:${file}`);
  return { file, bytes: bytes.length, sha256: digest(bytes), milliseconds: 0 };
});
const byFile = new Map(entries.map(entry => [entry.file, entry]));
const logs = [];
for (const { run, prefix, revision, merge, jobs } of captures) {
  const sources = new Map();
  const readSource = file => {
    if (!sources.has(file)) sources.set(file, git('show', `${revision}:${file}`));
    return sources.get(file);
  };
  const shards = partitionMeasuredWorkloads(filesAt(revision), 4, { readSource, costs: previous });
  for (const [index, job] of jobs.entries()) {
    if (job === null) continue;
    const partition = index + 1;
    const path = join(directory, `${prefix}-tooling-${partition}-${job}.log`);
    assert.ok(statSync(path).size <= 1024 * 1024, `job ${job} log exceeds the experiment's byte limit`);
    const bytes = readFileSync(path);
    const source = bytes.toString('utf8');
    assert.ok(source.includes(`js.yml@refs/pull/249/merge (${merge})`), `merge input in ${path}`);
    assert.ok(source.includes(`TEST_GROUP_SHARD: ${partition}/4`), `partition in ${path}`);
    assert.ok(source.includes('TAP version 13'), `test output in ${path}`);
    const completed = /Z # fail 0\s*$/mu.test(source);
    assert.equal(completed, job !== 137733805955, `expected completion for ${job}`);
    const timings = testFileWallCosts(shards[index].map(file => file.slice(3)), bytes,
      url => readSource(relative(root, fileURLToPath(url))).toString('utf8'));
    for (const [file, { elapsed, reported }] of timings) {
      const entry = byFile.get(`js/${file}`);
      if (entry?.sha256 !== digest(readSource(`js/${file}`))) continue;
      entry.milliseconds = Math.max(entry.milliseconds, Math.ceil(Math.max(elapsed, reported)));
    }
    logs.push({ run, revision, merge, job, partition, completed, bytes: bytes.length, sha256: digest(bytes) });
  }
}
for (const prior of previous.entries) {
  const entry = byFile.get(prior.file);
  if (entry?.sha256 === prior.sha256) entry.milliseconds = Math.max(entry.milliseconds, prior.milliseconds);
}
const observed = entries.filter(entry => entry.milliseconds > 0);
assert.ok(observed.length >= entries.length * 0.9, 'most exact source files have attributed durations');
writeProjectData(output, { schemaVersion: 1, model: previous.model, run, revision, logs, entries: observed,
  boundary: 'Advisory maxima of attributed elapsed output intervals, reported TAP durations and exact earlier source-bound costs. The deadline job contributes its printed prefix. Changed or unmeasured files use the existing byte-size estimate. Every tooling file and all four partitions remain mandatory.' });
console.log(`Captured advisory wall costs for ${observed.length} of ${entries.length} tooling files from ${logs.length} bounded job logs.`);
