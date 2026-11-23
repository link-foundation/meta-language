import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads, partitionTests } from '../../js/scripts/test-groups.mjs';

const [directory, option, ...extra] = process.argv.slice(2);
assert.ok(directory && extra.length === 0 && (option === undefined || option === '--write'));
const root = new URL('../../', import.meta.url);
const baseline = readProjectData(new URL('parity/fixtures/ci-39615628279-workload-costs.lino', root));
const snapshots = [
  { run: 45215384144, head: 'f92830dae2cad6f4cd9cb679bd09ab471635a4a6',
    revision: 'e6fd033e357df12c76a2714a463a769964307d00', jobs: 'attempt2-jobs-pages',
    jobIds: [137835443261, 137835442411, 137835442577, 137835443499, 137835443326, 137835443124] },
  { run: 45250356617, head: '4ecca483361d5f47ad38698fe0d24653f6ef641f',
    revision: '3224c9a1d9e26d7eebe31e7cbfd563555bd69b9b', jobs: 'jobs',
    jobIds: [137958366334, 137958366071, 137958365747] },
];

function git(...args) {
  return execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2_000_000 });
}

function hash(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function measure(log, assigned, sources) {
  const owners = new Map();
  for (const file of assigned) {
    owners.set(file.slice(3), file);
    for (const match of sources.get(file).toString().matchAll(/\btest\(\s*(['"])([^\n]*?)\1\s*,/gu)) {
      const title = match[2];
      owners.set(title, owners.has(title) && owners.get(title) !== file ? null : file);
    }
  }
  const samples = new Map();
  let beginning, start, end, currentFile, title, lastResult;
  for (const line of log.split('\n')) {
    const time = Date.parse(line.slice(0, 28));
    if (line.endsWith('TAP version 13')) start = beginning = time;
    if (start === undefined) continue;
    const subtest = /# Subtest: (.*)$/u.exec(line);
    if (subtest) title = subtest[1];
    if (/Z (?:not )?ok \d+ - /u.test(line)) {
      const file = owners.get(title);
      if (file && currentFile !== file) {
        if (currentFile) {
          assert.ok(assigned.indexOf(currentFile) < assigned.indexOf(file));
          samples.set(currentFile, Math.max(1, lastResult - beginning));
          beginning = lastResult;
        }
        currentFile = file;
      }
      lastResult = time;
      title = undefined;
    }
    if (/Z # duration_ms [0-9.]+$/u.test(line)) end = time;
  }
  assert.ok(start < end && currentFile, 'the complete TAP stream has an identifiable last file');
  samples.set(currentFile, Math.max(1, end - beginning));
  return { samples, testMilliseconds: end - start };
}

const entries = new Map();
const logs = [];
let previous;
for (const { run, head, revision, jobs, jobIds } of snapshots) {
  assert.equal(git('rev-parse', revision + '^2').toString().trim(), head);
  assert.equal(git('rev-parse', revision + '^{tree}').toString(), git('rev-parse', head + '^{tree}').toString());
  const captured = JSON.parse(readFileSync(join(directory, `ci-${run}-${jobs}.json`))).flatMap(page => page.jobs);
  const files = partitionTests(git('ls-tree', '--name-only', `${revision}:js/tests`).toString().trim().split('\n')
    .filter(file => file.endsWith('.test.js'))).groups.grammar.map(file => 'js/tests/' + file);
  const sources = new Map(files.map(file => [file, git('show', `${revision}:${file}`)]));
  const shards = partitionMeasuredWorkloads(files, 6, { costs: baseline, readSource: file => sources.get(file) });
  previous = shards;
  for (const id of jobIds) {
    const job = captured.find(row => row.id === id);
    assert.ok(job);
    const number = Number(/^JavaScript \/ JavaScript Tests \(grammar, ([1-6])\/6\)$/u.exec(job.name)?.[1]);
    assert.equal(job.run_id, run);
    assert.equal(job.run_attempt, 2);
    assert.equal(job.head_sha, head);
    assert.ok(job.conclusion === 'success' || job.conclusion === 'cancelled');
    const bytes = readFileSync(join(directory, `ci-${run}-job-${id}.log`));
    const log = bytes.toString('utf8');
    assert.ok(bytes.length < 2_000_000 && log.includes(revision) && log.includes('Cleaning up orphan processes'));
    assert.ok(log.includes(`TEST_GROUP_SHARD: ${number}/6`));
    assert.match(log, /Z # fail 0\r?\n/u);
    const { samples, testMilliseconds } = measure(log, shards[number - 1], sources);
    const setupMilliseconds = Date.parse(job.steps.find(step => step.name === 'Run the test group').started_at)
      - Date.parse(job.started_at);
    assert.ok(setupMilliseconds > 0 && testMilliseconds > 0);
    logs.push({ run, head, revision, id, number, conclusion: job.conclusion, bytes: bytes.length, sha256: hash(bytes),
      setupMilliseconds, testMilliseconds, measuredFiles: samples.size, scheduledFiles: shards[number - 1].length });
    for (const [file, milliseconds] of samples) {
      const source = sources.get(file);
      const sha256 = hash(source);
      const older = entries.get(file);
      entries.set(file, { file, bytes: source.length, sha256,
        milliseconds: older?.sha256 === sha256 ? Math.max(milliseconds, older.milliseconds) : milliseconds });
    }
  }
}

const files = previous.flat().sort();
const measuredLoads = shards => shards.map(shard => shard.reduce((total, file) => total + (entries.get(file)?.milliseconds ?? 0), 0));
const costs = { ...baseline, entries: [...new Map([...baseline.entries.map(row => [row.file, row]), ...entries]).values()] };
const balanced = partitionMeasuredWorkloads(files, 6, { costs });
console.log(JSON.stringify({ logs, measured: entries.size, total: files.length,
  unmeasured: files.filter(file => !entries.has(file)), previous: measuredLoads(previous), balanced: measuredLoads(balanced) }, null, 2));
if (option === '--write') {
  assert.ok(entries.size >= files.length * 0.85);
  assert.ok(logs.find(entry => entry.id === 137958366334).testMilliseconds > 750_000);
  writeProjectData(new URL('parity/fixtures/ci-45250356617-grammar-test-wall-costs.lino', root), {
    schemaVersion: 1, run: 45250356617, revision: snapshots.at(-1).revision, logs,
    entries: files.flatMap(file => entries.has(file) ? [entries.get(file)] : []),
    boundary: 'Advisory sequential JavaScript grammar file costs from complete retained CI test streams. Each source digest uses the largest measured time for its exact bytes. The latest complete observation replaces an older source version. Unidentified and changed files retain the existing size-based estimate. Samples choose owners and never establish test completion.',
  });
}
