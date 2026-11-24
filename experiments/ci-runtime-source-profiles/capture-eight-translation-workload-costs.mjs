import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join, posix } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkProjectData, readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads, partitionTests } from '../../js/scripts/test-groups.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const revision = 'd1a0528619f9eaa960dd43c7576aaf22a127e5cd';
const mergeCommit = 'c29d2b626cd05c209c814cc78cf654668ed8d29c';
const run = 45251703034;
const jobIds = [137962761889, 137962762397, 137962761153, 137962761177,
  137962762491, 137962761682, 137962763011, 137962762635];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root });
const [directory, mode] = process.argv.slice(2);
assert.ok(directory && [undefined, '--check'].includes(mode));
assert.equal(git('rev-parse', mergeCommit + '^2').toString().trim(), revision);
const sourcePaths = git('ls-tree', '--name-only', `${mergeCommit}:js/tests`).toString().trimEnd().split('\n');
const files = partitionTests(sourcePaths.filter(file => file.endsWith('.test.js'))).groups.translation.map(file => 'js/tests/' + file);
const sources = new Map(files.map(file => [file, git('show', `${mergeCommit}:${file}`)]));
const previous = readProjectData(new URL('../../parity/fixtures/ci-45049378352-translation-workload-costs.lino', import.meta.url));
const owners = partitionMeasuredWorkloads(files, jobIds.length, {
  group: 'translation', costs: previous, readSource: file => sources.get(file),
});
const entries = new Map(previous.entries.map(row => [row.file, row]));
const dependencies = [];
const wrappedFixtures = new Map();
for (const [file, bytes] of sources) {
  const source = bytes.toString('utf8');
  const wrapper = /^(?:\/\/[^\n]*\n)*import '((?:\.\.\/){2}experiments\/[a-z0-9-]+\/[a-z0-9-]+\.test\.mjs)';\n$/u.exec(source);
  if (!wrapper) continue;
  const path = posix.join(posix.dirname(file), wrapper[1]);
  const imported = git('show', `${mergeCommit}:${path}`);
  wrappedFixtures.set(file, imported.toString('utf8'));
  dependencies.push({ file, path, bytes: imported.length, sha256: digest(imported) });
}
assert.ok(dependencies.length > 0, 'actual side-effect test wrappers must be measured by their imported test source');
const jobPages = JSON.parse(readFileSync(join(directory, `ci-${run}-attempt-1-jobs.json`), 'utf8'));
const jobs = jobPages.flatMap(page => page.jobs);
const logs = [];
for (const [index, job] of jobIds.entries()) {
  const shard = `${index + 1}/${jobIds.length}`;
  const matches = jobs.filter(row => row.id === job);
  assert.equal(matches.length, 1);
  const { run_id, head_sha, run_attempt, conclusion, name } = matches[0];
  assert.deepEqual([run_id, head_sha, run_attempt, conclusion, name], [
    run, revision, 1, 'success', `JavaScript / JavaScript Tests (translation, ${shard})`,
  ]);
  const log = readFileSync(join(directory, `ci-${run}-job-${job}-translation-${index + 1}-full.log`));
  const content = log.toString('utf8');
  assert.ok(content.includes(`Uses: link-foundation/meta-language/.github/workflows/js.yml@refs/pull/247/merge (${mergeCommit})`));
  const passed = /# pass ([0-9]+)\r?$/mu.exec(content);
  assert.ok(Number(passed?.[1]) > 0);
  assert.match(content, /# fail 0\r?$/mu);
  assert.match(content, /# cancelled 0\r?$/mu);
  const titles = new Map();
  const expected = new Map();
  for (const file of owners[index]) {
    const source = wrappedFixtures.get(file) ?? sources.get(file).toString('utf8');
    const names = new Set();
    for (const match of source.matchAll(/\b(?:test|it)\(\s*(['"`])([^\n]*?)\1\s*,/gu)) {
      const name = match[2];
      names.add(name);
      titles.set(name, titles.has(name) && titles.get(name) !== file ? null : file);
    }
    expected.set(file, names);
  }
  const observations = new Map();
  let title;
  let started;
  for (const line of content.split(/\r?\n/u)) {
    const sample = /^(\S+Z) (TAP version 13|\s*# Subtest: (.*)|\s*duration_ms: ([0-9.]+))$/u.exec(line);
    if (!sample) continue;
    const timestamp = Date.parse(sample[1]);
    if (sample[2] === 'TAP version 13') started ??= timestamp;
    if (!started) continue;
    if (sample[3]) title = sample[3];
    const file = titles.get(title);
    if (!sample[4] || !file) continue;
    const observed = observations.get(file) ?? { titles: new Set(), milliseconds: 0, timestamp };
    observed.titles.add(title);
    observed.milliseconds += Number(sample[4]);
    observed.timestamp = timestamp;
    observations.set(file, observed);
    title = undefined;
  }
  let observedCount = 0;
  let previousEnd = started;
  let previousIndex = -1;
  for (const [position, file] of owners[index].entries()) {
    const observed = observations.get(file);
    if (!observed) continue;
    if (wrappedFixtures.has(file)) assert.deepEqual(observed.titles, expected.get(file));
    assert.ok(observed.timestamp >= previousEnd, `sequential test output for ${file}`);
    const unassigned = owners[index].slice(previousIndex + 1, position);
    const estimatedGap = unassigned.reduce((sum, missing) => sum + (entries.get(missing)?.milliseconds ?? 0), 0);
    const milliseconds = Math.max(1, Math.round(observed.milliseconds), observed.timestamp - previousEnd - estimatedGap);
    const source = sources.get(file);
    entries.set(file, { file, bytes: source.length, sha256: digest(source), milliseconds });
    previousEnd = observed.timestamp;
    previousIndex = position;
    observedCount += 1;
  }
  assert.ok(observedCount >= 25);
  logs.push({ job, shard, bytes: log.length, sha256: digest(log), observedCount });
}
assert.ok(dependencies.every(({ file }) => entries.get(file)?.sha256 === digest(sources.get(file))));
const packet = {
  schemaVersion: 1, model: 'observed CI workload scheduling costs 1', run, revision, mergeCommit, logs,
  entries: [...entries.values()].sort((left, right) => left.file.localeCompare(right.file)), dependencies,
  boundary: 'Advisory times from uniquely assigned sequential output in eight completed JavaScript test jobs. Imported test sources are pinned alongside exact test wrappers. Timings include process overhead and are used only to assign complete files under the existing job deadline.',
};
const output = new URL('../../parity/fixtures/ci-45251703034-translation-workload-costs.lino', import.meta.url);
if (mode === '--check') checkProjectData(output, packet);
else writeProjectData(output, packet);
console.log(`Captured ${logs.reduce((sum, log) => sum + log.observedCount, 0)} timings from ${logs.length} complete jobs and ${dependencies.length} imported test sources.`);
