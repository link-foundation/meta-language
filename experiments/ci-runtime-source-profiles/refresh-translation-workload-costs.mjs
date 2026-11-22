import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkProjectData, readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads, partitionTests } from '../../js/scripts/test-groups.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const revision = 'f39cd80cf682b33d18a8f35a7a5af6ae39e69f98';
const mergeCommit = 'f1837361ae926828041a5b445e95e648bfc5d1d6';
const run = 45049378352;
const jobIds = [137290889651, 137290888843, 137290889098, 137290888904, 137290889358, 137290888814];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root });
const [directory, mode] = process.argv.slice(2);
assert.ok(directory && [undefined, '--check', '--preview'].includes(mode));
const sourcePaths = git('ls-tree', '--name-only', `${revision}:js/tests`).toString().trimEnd().split('\n');
const { translation } = partitionTests(sourcePaths.filter(file => file.endsWith('.test.js'))).groups;
const sources = new Map(translation.map(file => [`js/tests/${file}`, git('show', `${revision}:js/tests/${file}`)]));
const previous = readProjectData(new URL('../../parity/fixtures/ci-45042220137-translation-workload-costs.lino', import.meta.url));
const files = [...sources.keys()];
const initialOwners = partitionMeasuredWorkloads(files, jobIds.length, {
  group: 'translation', costs: previous, readSource: file => sources.get(file),
});
const entries = new Map(previous.entries.map(row => [row.file, row]));
const logs = [];
const elapsed = owners => owners.map(shard => Math.round(shard.reduce((sum, file) => sum + (entries.get(file)?.milliseconds ?? 0), 0) / 1000));

for (const [index, job] of jobIds.entries()) {
  const log = readFileSync(join(directory, `ci-${run}-job-${job}.log`));
  const content = log.toString('utf8');
  assert.ok(content.includes(mergeCommit), `exact pre-merge checkout in job ${job}`);
  const owners = initialOwners[index];
  const titles = new Map();
  for (const file of owners) {
    for (const match of sources.get(file).toString().matchAll(/\b(?:test|it)\(\s*(['"`])([^\n]*?)\1\s*,/gu)) {
      const name = match[2];
      titles.set(name, titles.has(name) && titles.get(name) !== file ? null : file);
    }
  }
  let started;
  let file;
  let observedCount = 0;
  const observations = new Map();
  for (const line of content.split('\n')) {
    const sample = /^(\S+Z) (TAP version 13|\s*# Subtest: (.*)|\s*duration_ms: ([0-9.]+))$/u.exec(line);
    if (!sample) continue;
    const timestamp = Date.parse(sample[1]);
    if (sample[2] === 'TAP version 13') started ??= timestamp;
    if (!started) continue;
    if (sample[3]) file = titles.get(sample[3]);
    if (sample[4] && file) {
      const previousSample = observations.get(file);
      observations.set(file, { timestamp, milliseconds: (previousSample?.milliseconds ?? 0) + Number(sample[4]) });
      file = undefined;
    }
  }
  assert.ok(started && observations.size >= 30, `expected test timing samples in ${job}`);
  let previousEnd = started;
  let previousIndex = -1;
  for (const [position, file] of owners.entries()) {
    const observed = observations.get(file);
    if (!observed) continue;
    assert.ok(observed.timestamp >= previousEnd, `sequential test output for ${file}`);
    const unassigned = owners.slice(previousIndex + 1, position);
    const estimatedGap = unassigned.reduce((sum, missing) => sum + (entries.get(missing)?.milliseconds ?? 0), 0);
    const milliseconds = Math.max(1, Math.round(observed.milliseconds), observed.timestamp - previousEnd - estimatedGap);
    const source = sources.get(file);
    entries.set(file, { file, bytes: source.length, sha256: digest(source), milliseconds });
    previousEnd = observed.timestamp;
    previousIndex = position;
    observedCount++;
  }
  logs.push({ job, shard: `${index + 1}/${jobIds.length}`, bytes: log.length, sha256: digest(log), observedCount });
}

const packet = { schemaVersion: 1, model: 'observed CI workload scheduling costs 1', run, revision, mergeCommit,
  logs, entries: [...entries.values()].sort((left, right) => left.file.localeCompare(right.file)),
  boundary: 'Advisory elapsed time between uniquely assigned, sequential TAP outputs includes test process overhead. Where logs are incomplete or titles are ambiguous, retain previous source-bound samples or the scheduler estimate. All original tests retain mandatory execution under the existing job deadline.' };
const output = new URL('../../parity/fixtures/ci-45049378352-translation-workload-costs.lino', import.meta.url);
const updatedOwners = partitionMeasuredWorkloads(files, jobIds.length, {
  group: 'translation', costs: packet, readSource: file => sources.get(file),
});
console.log(`Refreshed ${logs.reduce((sum, log) => sum + log.observedCount, 0)} file timings. Previous owners ${elapsed(initialOwners).join(', ')} seconds. Updated owners ${elapsed(updatedOwners).join(', ')} seconds.`);
if (mode === '--check') checkProjectData(output, packet);
else if (mode !== '--preview') writeProjectData(output, packet);
