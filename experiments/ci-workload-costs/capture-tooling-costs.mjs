// Capture advisory test durations from the four tooling jobs on one CI checkout.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionTests } from '../../js/scripts/test-groups.mjs';

const run = 45112870920;
const revision = 'f1d961d1a9d72bd4e6c99178d20e0aadb0415b63';
const checkout = '8ac7901e0b94ffd8bba1634000204179c8a281e1';
const tree = '8b2b5a9ba150ef0ca9b438c72c5440f2645783ae';
const jobs = [137488959562, 137488959098, 137488959842, 137488959152];
const root = new URL('../../', import.meta.url);
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = args => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root, maxBuffer: 2 * 1024 * 1024 });
const directory = process.argv[2];
assert.ok(directory && process.argv.length === 3, 'pass the directory with the retained CI tooling logs');
assert.equal(git(['rev-parse', revision + '^{tree}']).toString().trim(), tree);

const files = git(['ls-tree', '-r', '--name-only', revision, 'js/tests']).toString().split('\n')
  .filter(file => /^js\/tests\/[^/]+\.test\.js$/u.test(file)).map(file => file.slice(9)).sort();
const tooling = partitionTests(files).groups.tooling;
const entries = [];
const logs = jobs.map((job, index) => {
  const partition = index + 1;
  const bytes = readFileSync(join(directory, `tooling-${partition}-of-4-${job}.log`));
  assert.ok(bytes.length > 0 && bytes.length <= 64 * 1024 * 1024);
  const source = bytes.toString('utf8');
  assert.ok(source.includes(`.github/workflows/js.yml@refs/pull/249/merge (${checkout})`));
  const lines = source.split('\n');
  assert.ok(lines.some(line => line.endsWith(`Z ${checkout}`)), 'the simulated merge retained the tested checkout');
  assert.ok(lines.some(line => line.endsWith(`TEST_GROUP_SHARD: ${partition}/4`)));
  const inputs = new Map(tooling.filter((_, offset) => offset % jobs.length === index)
    .map(name => {
      const file = 'js/tests/' + name;
      return [file, git(['show', `${revision}:${file}`])];
    }));
  const titles = new Map();
  for (const [file, input] of inputs) {
    for (const [, , title] of input.toString().matchAll(/\btest\((?:\{[^\n]*?\}, *)? *(['"`])([^\n]*?)\1 *[,)]/gu)) {
      if (!titles.has(title)) titles.set(title, file);
      else if (titles.get(title) !== file) titles.set(title, null);
    }
  }
  const costs = new Map();
  let owner;
  for (const line of lines) {
    const title = /^\S+Z # Subtest: (.+)$/u.exec(line);
    if (title) owner = titles.get(title[1]);
    const duration = /^\S+Z   duration_ms: ([0-9.]+)$/u.exec(line);
    if (duration && owner) {
      costs.set(owner, (costs.get(owner) ?? 0) + Math.max(1, Math.round(Number(duration[1]))));
      owner = undefined;
    }
  }
  for (const [file, milliseconds] of costs) {
    const input = inputs.get(file);
    entries.push({ file, bytes: input.length, sha256: digest(input), milliseconds });
  }
  return { job, partition, bytes: bytes.length, sha256: digest(bytes) };
});
assert.ok(entries.length > tooling.length / 2, 'most original files have attributable durations');
assert.equal(new Set(entries.map(row => row.file)).size, entries.length);
entries.sort((left, right) => left.file.localeCompare(right.file));
writeProjectData(new URL(`parity/fixtures/ci-${run}-tooling-costs.lino`, root), {
  schemaVersion: 1, model: 'observed CI workload scheduling costs 1', run, revision, checkout, tree, logs, entries,
  boundary: 'Only uniquely attributable top-level TAP durations are sampled. Canceled jobs contribute only printed intervals. Every test file remains required regardless of its scheduling estimate.',
});
console.log(`Captured ${entries.length} advisory tooling costs from ${logs.length} retained job logs.`);
