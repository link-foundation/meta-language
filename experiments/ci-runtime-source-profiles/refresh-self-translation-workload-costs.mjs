import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkProjectData, readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads } from '../../js/scripts/test-groups.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const captures = {
  45049378352: {
    revision: 'f39cd80cf682b33d18a8f35a7a5af6ae39e69f98',
    mergeCommit: 'f1837361ae926828041a5b445e95e648bfc5d1d6',
    jobs: [137290888119, 137290888088, 137290887938, 137290888178, 137290888116, 137290887972, 137290887931, 137290888216],
    unfinished: [137290888116],
    previousRun: 39615628279,
  },
  45055131879: {
    revision: '81e9e733677877f2440018a09d65d3d7f5f1f113',
    mergeCommit: 'f7c86d18040a42f241347d4115f68d23423c33a3',
    jobs: [137308556165, 137308556172, 137308556310, 137308556270, 137308556147, 137308556118, 137308556276, 137308556219],
    unfinished: [137308556276, 137308556219],
    previousRun: 45049378352,
  },
  45060692627: {
    revision: 'f1c7cb555e37b9ddf09c10f772b0adebf4f6e587',
    mergeCommit: 'f956894159f88162f30bb4473a73edf7ea6d7283',
    jobs: [137325711520, 137325711617, 137325711503, 137325711666, 137325711685, 137325711656, 137325711693, 137325711694],
    unfinished: [137325711694],
    previousRun: 45055131879,
    nextCount: 12,
  },
};
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root });
const [directory, mode, runOption = 45049378352] = process.argv.slice(2);
const run = Number(runOption);
assert.ok(directory && Object.hasOwn(captures, run) && [undefined, '--check', '--preview', '--write'].includes(mode));
const { revision, mergeCommit, jobs, unfinished, previousRun, nextCount = jobs.length } = captures[run];
const costPath = costRun => new URL(`../../parity/fixtures/ci-${costRun}-${costRun === 39615628279 ? '' : 'self-translation-'}workload-costs.lino`, import.meta.url);

const files = git('ls-tree', '-r', '--name-only', revision, '--', 'js/src').toString().trimEnd().split('\n').filter(file => file.endsWith('.js'));
const sources = new Map(files.map(file => [file, git('show', `${revision}:${file}`)]));
const previous = readProjectData(costPath(previousRun));
const group = 'self-translation-report';
const readSource = file => sources.get(file);
const initialOwners = partitionMeasuredWorkloads(files, jobs.length, { group, costs: previous, readSource });
const entries = new Map(previous.entries.filter(row => sources.has(row.file)).map(row => [row.file, row]));
const observed = new Set();
const logs = [];

for (const [index, job] of jobs.entries()) {
  const log = readFileSync(join(directory, `ci-${run}-job-${job}.log`));
  const content = log.toString('utf8');
  assert.ok(content.includes(mergeCommit), `exact pre-merge checkout in job ${job}`);
  const owners = new Set(initialOwners[index]);
  let previousEnd;
  let observedCount = 0;
  for (const line of content.split('\n')) {
    const sample = /^(\S+Z) self-translation report: (resolving|recorded) (js\/src\/[^\s]+\.js)(?: \(\d+ ms\))?$/u.exec(line);
    if (!sample) continue;
    const [, time, phase, file] = sample;
    const timestamp = Date.parse(time);
    assert.ok(Number.isFinite(timestamp) && owners.has(file));
    previousEnd ??= timestamp;
    if (phase !== 'recorded') continue;
    assert.ok(timestamp >= previousEnd && !observed.has(file));
    observed.add(file);
    const source = sources.get(file);
    entries.set(file, { file, bytes: source.length, sha256: digest(source), milliseconds: Math.max(1, timestamp - previousEnd) });
    previousEnd = timestamp;
    observedCount++;
  }
  assert.ok(observedCount > 0 && (unfinished.includes(job) ? observedCount < owners.size : observedCount === owners.size));
  logs.push({ job, shard: `${index + 1}/${jobs.length}`, bytes: log.length, sha256: digest(log), observedCount });
}

const packet = { schemaVersion: 1, model: 'observed CI workload scheduling costs 1', run, revision, mergeCommit,
  logs, entries: [...entries.values()].sort((left, right) => left.file.localeCompare(right.file)),
  boundary: 'Advisory time between recorded modules includes the time spent on overlapping imports. Unfinished modules retain older source-bound measurements or the scheduler estimate. Every module retains mandatory measurement under the same report deadline.' };
const output = costPath(run);
const updatedGroup = nextCount === jobs.length ? group : `${group}-${nextCount}`;
const updatedOwners = partitionMeasuredWorkloads(files, nextCount, { group: updatedGroup, costs: packet, readSource });
const elapsed = owners => owners.map(shard => Math.round(shard.reduce((sum, file) => sum + (entries.get(file)?.milliseconds ?? 0), 0) / 1000));
console.log(`Refreshed ${observed.size} of ${files.length} module timings. Previous owners ${elapsed(initialOwners).join(', ')} seconds. Updated owners ${elapsed(updatedOwners).join(', ')} seconds.`);
if (mode === '--check') checkProjectData(output, packet);
else if (mode !== '--preview') writeProjectData(output, packet);
