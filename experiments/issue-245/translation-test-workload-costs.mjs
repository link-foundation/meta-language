import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionTests } from '../../js/scripts/test-groups.mjs';

const run = 45076523061;
const revision = '432bc6adde4a01fcc1a2547a8f2b611cf26c1166';
const root = new URL('../../', import.meta.url);
const digest = value => createHash('sha256').update(value).digest('hex');
const directory = process.argv[2];
assert.equal(process.argv.length, 3, 'provide the directory with the retained complete CI logs');
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2 * 1024 * 1024 });
const files = partitionTests(git('ls-tree', '--name-only', revision + ':js/tests').toString().trim().split('\n')
  .filter(file => file.endsWith('.test.js'))).groups.translation;
const sources = new Map();
const titles = new Map();
for (const file of files) {
  const source = git('show', revision + ':js/tests/' + file);
  sources.set(file, source);
  for (const match of source.toString().matchAll(/\btest\(\s*(['"])([^\n]*?)\1\s*,/gu)) {
    const title = match[2];
    if (titles.has(title) && titles.get(title) !== file) titles.set(title, null);
    else titles.set(title, file);
  }
}
const { jobs } = JSON.parse(readFileSync(join(directory, run + '-jobs.json'), 'utf8'));
const shards = jobs.flatMap(job => {
  const match = /^JavaScript \/ JavaScript Tests \(translation, (\d)\/6\)$/u.exec(job.name);
  return match ? [{ number: Number(match[1]), job }] : [];
}).sort((left, right) => left.number - right.number);
assert.deepEqual(shards.map(shard => shard.number), [1, 2, 3, 4, 5, 6]);
const costs = new Map();
const logs = shards.map(({ number, job }) => {
  assert.equal(job.run_id, run);
  assert.equal(job.run_attempt, 2);
  const bytes = readFileSync(join(directory, 'new-ci-job-' + job.id + '.log'));
  const text = bytes.toString();
  assert.ok(bytes.length <= 524288 && text.includes(revision) && text.includes('Cleaning up orphan processes'));
  const owned = new Set(files.filter((_, index) => index % 6 === number - 1));
  let file;
  for (const line of text.split('\n')) {
    const subtest = /# Subtest: (.*)$/u.exec(line);
    if (subtest) {
      const name = subtest[1];
      file = name.startsWith('tests/') ? name.slice(6) : titles.get(name);
    }
    const elapsed = /\bduration_ms: ([0-9.]+)/u.exec(line);
    if (!elapsed || !file) continue;
    assert.ok(owned.has(file), 'the exact original test cohort owns the measured file');
    costs.set(file, (costs.get(file) ?? 0) + Number(elapsed[1]));
    file = undefined;
  }
  return { job: job.id, number, conclusion: job.conclusion, bytes: bytes.length, sha256: digest(bytes) };
});
const entries = files.filter(file => costs.has(file)).map(file => {
  const bytes = sources.get(file);
  return { file: 'js/tests/' + file, bytes: bytes.length, sha256: digest(bytes), milliseconds: Math.max(1, Math.round(costs.get(file))) };
});
assert.ok(entries.length > files.length * 0.8, 'bound the portion left to size estimates');
writeProjectData(new URL('parity/fixtures/ci-45076523061-translation-test-workload-costs.lino', root), {
  schemaVersion: 1, run, revision, logs, entries,
  boundary: 'Advisory sums of recorded TAP test durations with independently identified source ownership. Incomplete or ambiguous observations are omitted. This is not test execution evidence.',
});
