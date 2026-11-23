import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads, partitionTests } from '../../js/scripts/test-groups.mjs';

const run = 45089575521;
const revision = '0deae514483822ffd2a42a4fa0f4c88744d0745d';
const root = new URL('../../', import.meta.url);
const source = path => git('show', `${revision}:${path}`);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const directory = process.argv[2];
const argumentsAfterDirectory = process.argv.slice(3);
assert.ok(directory && (argumentsAfterDirectory.length === 0 || argumentsAfterDirectory.join() === '--write'));

function git(...args) {
  return execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2_000_000 });
}

const files = partitionTests(git('ls-tree', '--name-only', `${revision}:js/tests`).toString().trim().split('\n')
  .filter(file => file.endsWith('.test.js'))).groups.translation.map(file => 'js/tests/' + file);
const sources = new Map(files.map(file => [file, source(file)]));
const baseline = readProjectData(new URL('parity/fixtures/ci-39615628279-workload-costs.lino', root));
const prior = readProjectData(new URL('parity/fixtures/ci-45076523061-translation-test-workload-costs.lino', root));
const costs = { ...baseline, entries: [...baseline.entries, ...prior.entries] };
const owners = [0, 1, 2].map(cohort => partitionMeasuredWorkloads(files.filter((_, index) => index % 3 === cohort), 2,
  { costs, readSource: file => sources.get(file) }));

function titlesFor(assigned) {
  const titles = new Map();
  for (const file of assigned) {
    titles.set(file.slice(3), file);
    for (const match of sources.get(file).toString().matchAll(/\btest\(\s*(['"])([^\n]*?)\1\s*,/gu)) {
      const title = match[2];
      titles.set(title, titles.has(title) && titles.get(title) !== file ? null : file);
    }
  }
  return titles;
}

function measure(log, assigned) {
  const titles = titlesFor(assigned);
  const elapsed = new Map();
  let testStart, end, file, beginning, lastResult, currentTitle;
  for (const line of log.split('\n')) {
    const time = Date.parse(line.slice(0, 28));
    if (line.endsWith('TAP version 13')) testStart = beginning = time;
    if (testStart === undefined) continue;
    const subtest = /# Subtest: (.*)$/u.exec(line);
    if (subtest) currentTitle = subtest[1];
    if (/Z (?:not )?ok \d+ - /u.test(line)) {
      const next = titles.get(currentTitle);
      if (next && next !== file) {
        if (file) {
          assert.ok(assigned.indexOf(next) > assigned.indexOf(file));
          assert.ok(!elapsed.has(file));
          elapsed.set(file, Math.max(1, lastResult - beginning));
          beginning = lastResult;
        }
        file = next;
      }
      lastResult = time;
    }
    if (/Z # duration_ms \d/u.test(line)) end = time;
  }
  assert.ok(Number.isFinite(testStart) && Number.isFinite(lastResult));
  if (end && file) elapsed.set(file, Math.max(1, end - beginning));
  return { elapsed, completedTests: end !== undefined, measuredMilliseconds: (end ?? lastResult) - testStart };
}

const { jobs } = JSON.parse(readFileSync(join(directory, `ci-${run}-jobs.json`), 'utf8'));
const entries = new Map();
const logs = [];
for (const job of jobs) {
  const match = /^JavaScript \/ JavaScript Tests \(translation, ([1-6])\/6\)$/u.exec(job.name);
  if (!match || !job.conclusion) continue;
  const number = Number(match[1]);
  assert.equal(job.run_id, run);
  const assigned = owners[(number - 1) % 3][Math.floor((number - 1) / 3)];
  const contents = readFileSync(join(directory, `ci-${run}-job-${job.id}.log`));
  const log = contents.toString();
  assert.ok(contents.length < 2_000_000 && log.includes(revision) && log.includes('Cleaning up orphan processes'));
  const { elapsed, completedTests, measuredMilliseconds } = measure(log, assigned);
  logs.push({ job: job.id, number, conclusion: job.conclusion, bytes: contents.length, sha256: hash(contents),
    completedTests, measuredMilliseconds, measuredFiles: elapsed.size });
  for (const [file, milliseconds] of elapsed) {
    assert.ok(!entries.has(file));
    const bytes = sources.get(file);
    entries.set(file, { file, bytes: bytes.length, sha256: hash(bytes), milliseconds });
  }
}

logs.sort((left, right) => left.number - right.number);
console.log(JSON.stringify({ logs, entries: entries.size, total: files.length }));
const updatedSamples = new Map(costs.entries.map(row => [row.file, row]));
for (const [file, sample] of entries) updatedSamples.set(file, sample);
const estimates = { ...costs, entries: [...updatedSamples.values()] };
for (const [cohort, partitions] of owners.entries()) {
  const selection = partitions.flat();
  const balanced = partitionMeasuredWorkloads(selection, 3, { costs: estimates, readSource: file => sources.get(file) });
  const measuredTotal = files => files.reduce((total, file) => total + (updatedSamples.get(file)?.milliseconds ?? 0), 0);
  console.log(JSON.stringify({ cohort: cohort + 1, previous: partitions.map(measuredTotal), balanced: balanced.map(measuredTotal) }));
}
if (argumentsAfterDirectory.length) {
  assert.ok(logs.length >= 5 && new Set(logs.map(row => row.number)).size === logs.length);
  assert.ok(entries.size > files.length * 0.75);
  writeProjectData(new URL(`parity/fixtures/ci-${run}-translation-test-wall-costs.lino`, root), {
    schemaVersion: 1, run, revision, logs, entries: files.flatMap(file => entries.has(file) ? [entries.get(file)] : []),
    boundary: 'Advisory file scheduling costs from CI TAP result timestamps in complete retained job logs. Only uniquely identified files in their actual sequential partition are sampled. Startup and unmatched output preceding the next owned file are included where observable. A partially observed final file and jobs still queued at capture time are omitted. Untimed files retain earlier or source-size estimates and remain assigned exactly once. These timings are not test execution evidence.',
  });
}
