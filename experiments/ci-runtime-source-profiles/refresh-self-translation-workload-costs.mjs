import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkProjectData, readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads } from '../../js/scripts/test-groups.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const run = 45049378352;
const revision = 'f39cd80cf682b33d18a8f35a7a5af6ae39e69f98';
const mergeCommit = 'f1837361ae926828041a5b445e95e648bfc5d1d6';
const jobs = [137290888119, 137290888088, 137290887938, 137290888178, 137290888116, 137290887972, 137290887931, 137290888216];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root });
const [directory, mode] = process.argv.slice(2);
assert.ok(directory && [undefined, '--check', '--preview'].includes(mode));

const files = git('ls-tree', '-r', '--name-only', revision, '--', 'js/src').toString().trimEnd().split('\n').filter(file => file.endsWith('.js'));
const sources = new Map(files.map(file => [file, git('show', `${revision}:${file}`)]));
const previous = readProjectData(new URL('../../parity/fixtures/ci-39615628279-workload-costs.lino', import.meta.url));
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
  assert.ok(observedCount > 0 && (index === 4 ? observedCount < owners.size : observedCount === owners.size));
  logs.push({ job, shard: `${index + 1}/${jobs.length}`, bytes: log.length, sha256: digest(log), observedCount });
}

const packet = { schemaVersion: 1, model: 'observed CI workload scheduling costs 1', run, revision, mergeCommit,
  logs, entries: [...entries.values()].sort((left, right) => left.file.localeCompare(right.file)),
  boundary: 'Advisory time between recorded modules includes the time spent on overlapping imports. Unfinished modules retain older source-bound measurements or the scheduler estimate. Every module retains mandatory measurement under the same report deadline.' };
const output = new URL('../../parity/fixtures/ci-45049378352-self-translation-workload-costs.lino', import.meta.url);
const updatedOwners = partitionMeasuredWorkloads(files, jobs.length, { group, costs: packet, readSource });
const elapsed = owners => owners.map(shard => Math.round(shard.reduce((sum, file) => sum + (entries.get(file)?.milliseconds ?? 0), 0) / 1000));
console.log(`Refreshed ${observed.size} of ${files.length} module timings. Previous owners ${elapsed(initialOwners).join(', ')} seconds. Updated owners ${elapsed(updatedOwners).join(', ')} seconds.`);
if (mode === '--check') checkProjectData(output, packet);
else if (mode !== '--preview') writeProjectData(output, packet);
