// Recover advisory translation scheduling costs from retained CI package tests.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionTests } from '../../js/scripts/test-groups.mjs';

const run = 45080642295;
const revision = '016ed6eed0c376a682299dd6463533a620a6a36f';
const root = new URL('../../', import.meta.url);
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = args => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2 * 1024 * 1024 });
const directory = process.argv[2];
assert.ok(directory && process.argv.length === 3, 'pass the directory containing the retained translation job logs');
const paths = readdirSync(directory).filter(file => new RegExp(`^javascript-translation-${run}-[0-9]+\\.log$`, 'u').test(file)).sort();
assert.ok(paths.length >= 5 && paths.length <= 6);
const names = git(['ls-tree', '-r', '--name-only', revision, 'js/tests']).toString('utf8').split('\n');
const files = partitionTests(names.filter(file => /^js\/tests\/[^/]+\.test\.js$/u.test(file)).map(file => file.slice(9)).sort()).groups.translation;
const partitions = new Set();
const entries = [];
const logs = paths.map(path => {
  const bytes = readFileSync(join(directory, path));
  assert.ok(bytes.length > 0 && bytes.length <= 1024 * 1024);
  const source = bytes.toString('utf8');
  assert.ok(source.includes(`.github/workflows/js.yml@refs/pull/249/merge (${revision})`));
  assert.ok(source.split('\n').some(line => line.endsWith(`Z ${revision}`)), 'the simulated merge retained the same checkout');
  const partition = Number(/TEST_GROUP_SHARD: ([1-6])\/6/u.exec(source)?.[1]);
  assert.ok(Number.isInteger(partition) && !partitions.has(partition));
  partitions.add(partition);
  const members = files.filter((_, offset) => offset % 6 === partition - 1).map(name => `js/tests/${name}`);
  const inputs = new Map(members.map(file => [file, git(['show', `${revision}:${file}`])]));
  const owners = new Map();
  for (const [file, input] of inputs) {
    for (const [, , title] of input.toString('utf8').matchAll(/\btest\((?:\{[^\n]*?\}, *)? *(['"`])([^\n]*?)\1 *[,)]/gu)) {
      if (!owners.has(title)) owners.set(title, file);
      else if (owners.get(title) !== file) owners.set(title, null);
    }
  }
  const costs = new Map();
  let owner;
  for (const line of source.split('\n')) {
    const title = /# Subtest: (.+)$/u.exec(line);
    if (title) owner = owners.get(title[1]);
    const duration = /\bduration_ms: ([0-9.]+)/u.exec(line);
    if (duration && owner) {
      costs.set(owner, (costs.get(owner) ?? 0) + Math.max(1, Math.round(Number(duration[1]))));
      owner = undefined;
    }
  }
  for (const [file, milliseconds] of costs) {
    const input = inputs.get(file);
    entries.push({ file, bytes: input.length, sha256: digest(input), milliseconds });
  }
  return { job: Number(/-([0-9]+)\.log$/u.exec(path)[1]), partition, bytes: bytes.length, sha256: digest(bytes) };
});
assert.ok(entries.length >= 200, 'most unchanged translation files have uniquely attributed measurements');
assert.equal(new Set(entries.map(row => row.file)).size, entries.length);
entries.sort((left, right) => left.file.localeCompare(right.file));
writeProjectData(new URL(`parity/fixtures/ci-${run}-translation-costs.lino`, root), {
  schemaVersion: 1, model: 'observed CI workload scheduling costs 1', run, revision, logs, entries,
  boundary: 'Only TAP durations uniquely attributable to the historical file owners are sampled. A canceled job contributes the intervals already printed. Nonobserved and changed files use size estimates. All current files are still mandatory.',
});
console.log(`Captured ${entries.length} advisory translation costs from ${logs.length} retained job logs.`);
