// Recover advisory report scheduling costs from the eight retained CI logs.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { writeProjectData } from '../../js/scripts/project-data.mjs';

const run = 45080642295;
const revision = '016ed6eed0c376a682299dd6463533a620a6a36f';
const root = new URL('../../', import.meta.url);
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = args => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2 * 1024 * 1024 });
const directory = process.argv[2];
assert.ok(directory && process.argv.length === 3, 'pass the directory containing the eight retained report job logs');
const paths = readdirSync(directory).filter(file => new RegExp(`^self-translation-report-${run}-[0-9]+\\.log$`, 'u').test(file)).sort();
assert.equal(paths.length, 8);
const costs = new Map();
const logs = paths.map(path => {
  const bytes = readFileSync(join(directory, path));
  assert.ok(bytes.length > 0 && bytes.length <= 512 * 1024);
  const source = bytes.toString('utf8');
  assert.ok(source.includes(`git checkout --progress --force ${revision}`), 'each log ran at the same recorded merge commit');
  let previous;
  for (const line of source.split('\n')) {
    const timestamp = Date.parse(line.slice(0, 28));
    if (previous === undefined && line.includes('self-translation report: resolving js/')) previous = timestamp;
    const recorded = /self-translation report: recorded (js\/src\/\S+\.js) \(/u.exec(line);
    if (recorded) {
      const [, file] = recorded;
      assert.ok(Number.isFinite(previous) && timestamp >= previous && !costs.has(file));
      costs.set(file, Math.max(1, timestamp - previous));
      previous = timestamp;
    }
  }
  return { job: Number(/-([0-9]+)\.log$/u.exec(path)[1]), bytes: bytes.length, sha256: digest(bytes) };
});
const modules = git(['ls-tree', '-r', '--name-only', revision, 'js/src']).toString('utf8').split('\n').filter(file => file.endsWith('.js'));
assert.deepEqual([...costs.keys()].sort(), modules.slice().sort(), 'all complete report sources recorded an interval');
const entries = [...costs].sort(([left], [right]) => left.localeCompare(right)).map(([file, milliseconds]) => {
  const bytes = git(['show', `${revision}:${file}`]);
  return { file, bytes: bytes.length, sha256: digest(bytes), milliseconds };
});
writeProjectData(new URL(`parity/fixtures/ci-${run}-report-costs.lino`, root), {
  schemaVersion: 1, model: 'observed CI workload scheduling costs 1', run, revision, logs, entries,
  boundary: 'Elapsed intervals between recorded modules include overlapping work. Source bytes determine whether a sample is applicable. These intervals assign scheduling owners and do not establish CI completion.',
});
console.log(`Captured ${entries.length} advisory report costs from ${logs.length} retained job logs.`);
