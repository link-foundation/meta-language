// Replay the bounded archived CI logs into the advisory eight-shard cost fixture.
// Usage: node experiments/ci-workload-costs/capture-translation-schedule.mjs ci-logs [output]
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads, partitionTests, testFiles } from '../../js/scripts/test-groups.mjs';
import { testFileWallCosts } from './compare-translation-wall-costs.mjs';

const [directory, output = fileURLToPath(new URL('../../parity/fixtures/ci-45186623729-translation-wall-costs.lino', import.meta.url))] = process.argv.slice(2);
assert.ok(directory, 'pass the directory containing the retained translation job logs');
const root = fileURLToPath(new URL('../../', import.meta.url));
const previousCosts = readProjectData(join(root, 'parity/fixtures/ci-45080642295-translation-costs.lino'));
const files = partitionTests(testFiles()).groups.translation.map(file => `js/tests/${file}`);
const previous = partitionMeasuredWorkloads(files, 8, { costs: previousCosts });
const rebalanced = previous.map(shard => [...shard]);
for (const [file, destination] of [['js/tests/self-translation.test.js', 5], ['js/tests/translation-owned-original-numbers.test.js', 7]]) {
  const position = rebalanced[7].indexOf(file);
  assert.ok(position >= 0, `expected prior eighth-shard owner for ${file}`);
  rebalanced[7].splice(position, 1);
  rebalanced[destination - 1].push(file);
}

const captures = [
  { run: 45133354161, prefix: '80d8cef69', revision: '80d8cef693b328e54ad31d74b3c4b688f6c00f06',
    merge: '0f33a52c609f09f5227b244353d012e683c321a1', shards: previous,
    jobs: [137553726849, 137553726760, 137553727011, 137553727235, 137553727306, 137553726649, 137553726817, 137553727247] },
  { run: 45159625352, prefix: '03a81ad9e', revision: '03a81ad9e04abf583eabca7f13f8d6bb6947b521',
    merge: '27b65da66f8e82f226686e6108e7321fdf66d4c8', shards: previous,
    jobs: [137641358943, 137641360659, 137641359491, 137641358498, 137641358949, 137641358514, 137641358649, 137641360015] },
  { run: 45186623729, prefix: '897b1f6b', revision: '897b1f6b2ff7df5e094ca5d420b9c6e0057ef030',
    merge: '432b7c03ff50bb0c2462973e30a7daf5767f3dad', shards: rebalanced,
    jobs: [null, 137733805085, 137733804834, 137733805489, 137733805323, null, 137733805446, 137733805453] },
];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const tests = new Set(files);
const entries = files.map(file => {
  const bytes = readFileSync(join(root, file));
  return { file, bytes: bytes.length, sha256: digest(bytes), milliseconds: 0 };
});
const byFile = new Map(entries.map(entry => [entry.file, entry]));
const logs = [];
for (const { run, prefix, revision, merge, shards, jobs } of captures) {
  const changed = execFileSync('git', ['diff', '--name-only', revision, '--', 'js/tests', 'experiments'], { cwd: root, encoding: 'utf8' });
  assert.ok(!changed.trim().split('\n').some(file => tests.has(file) || file.endsWith('.test.mjs')), `translation sources differ from ${revision}`);
  for (const [index, job] of jobs.entries()) {
    if (job === null) continue;
    const shard = index + 1;
    const path = join(directory, `${prefix}-translation-${shard}-${job}.log`);
    assert.ok(statSync(path).size <= 1024 * 1024, `job ${job} log exceeds the experiment's byte limit`);
    const bytes = readFileSync(path);
    const text = bytes.toString('utf8');
    assert.ok(text.includes(`js.yml@refs/pull/249/merge (${merge})`), `merge input in ${path}`);
    assert.ok(text.includes(`TEST_GROUP_SHARD: ${shard}/8`), `partition in ${path}`);
    assert.ok(text.includes('TAP version 13'), `test output in ${path}`);
    const completed = /Z # fail 0\s*$/mu.test(text);
    assert.equal(completed, job !== 137641360015 && job !== 137733805489, `expected completion for ${job}`);
    const times = testFileWallCosts(shards[index].map(file => file.slice(3)), bytes);
    for (const [file, { elapsed, reported }] of times) {
      const entry = byFile.get(`js/${file}`);
      entry.milliseconds = Math.max(entry.milliseconds, Math.ceil(Math.max(elapsed, reported)));
    }
    logs.push({ run, revision, merge, job, shard, completed, bytes: bytes.length, sha256: digest(bytes) });
  }
}
const observed = entries.filter(entry => entry.milliseconds > 0).length;
assert.equal(observed, entries.length, 'every translation file has an attributed observation');
for (const prior of previousCosts.entries) {
  const entry = byFile.get(prior.file);
  if (entry?.sha256 === prior.sha256) entry.milliseconds = Math.max(entry.milliseconds, prior.milliseconds);
}
const { run, revision } = captures.at(-1);
writeProjectData(output, { schemaVersion: 1, model: previousCosts.model, run, revision, logs,
  entries: entries.filter(entry => entry.milliseconds > 0),
  boundary: 'Advisory maxima of attributed elapsed output intervals, reported TAP durations and exact earlier source-bound costs. The source-matched complete earlier matrix supplies files beyond both timed-out prefixes. The data only selects eight file owners. Every test and compiler feedback partition remains mandatory.' });
console.log(`Captured advisory wall costs for ${observed} of ${entries.length} translation files from ${logs.length} bounded job logs.`);
