// Usage: node experiments/ci-workload-costs/compare-translation-shard-widths.mjs <passing log> <timed-out log>
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync, statSync } from 'node:fs';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { partitionGroupFiles, partitionMeasuredWorkloads, partitionTests, testFiles } from '../../js/scripts/test-groups.mjs';
import { testFileWallCosts } from './compare-translation-wall-costs.mjs';

const [passingLog, timedOutLog] = process.argv.slice(2);
assert.ok(passingLog && timedOutLog, 'pass both exact shard 7/8 logs');
const root = new URL('../../', import.meta.url);
const timedOutRevision = '331e4b5f158f68ec9b2bf2c3e8dacb6c6c44c341';
const gitOptions = { cwd: root, maxBuffer: 2 * 1024 * 1024 };
const changed = new Set(execFileSync('git', ['diff', '--name-only', timedOutRevision, '--', 'js/tests'], gitOptions)
  .toString('utf8').trim().split('\n'));
const readPredecessor = file => changed.has(file)
  ? execFileSync('git', ['--no-replace-objects', 'show', `${timedOutRevision}:${file}`], gitOptions)
  : readFileSync(new URL(file, root));
const prior = readProjectData(new URL('parity/fixtures/ci-45186623729-translation-wall-costs.lino', root));
const names = partitionTests(testFiles()).groups.translation;
const files = names.map(file => `js/tests/${file}`);
const eighths = partitionMeasuredWorkloads(files, 8, { costs: prior, readSource: readPredecessor });
const measurements = new Map(prior.entries.map(row => [row.file, row.milliseconds]));

for (const [log, revision, completed] of [
  [passingLog, 'b4d330d38d40020c1e3bcaf81c9ce2ca02712e4e', true],
  [timedOutLog, '231b05eaf0343080cc9dc2df2186e03aca746b18', false],
]) {
  assert.ok(statSync(log).size <= 1024 * 1024, 'expected a bounded single-job log');
  const bytes = readFileSync(log);
  const source = bytes.toString('utf8');
  assert.ok(source.includes(`js.yml@refs/pull/249/merge (${revision})`));
  assert.ok(source.includes('TEST_GROUP_SHARD: 7/8'));
  assert.equal(/^\S+ # fail 0$/mu.test(source), completed);
  const testTimes = testFileWallCosts(eighths[6].map(file => file.slice('js/'.length)), bytes);
  for (const [file, { elapsed, reported }] of testTimes) {
    const address = `js/${file}`;
    assert.ok(measurements.has(address), `missing recorded cost for ${address}`);
    measurements.set(address, Math.max(measurements.get(address), Math.ceil(Math.max(elapsed, reported))));
  }
  const end = (completed ? /^(\S+) # duration_ms /mu : /^(\S+) ##\[error\]The operation was canceled\./mu).exec(source)[1];
  const start = /^(\S+) ##\[group\]Run node \.\.\/scripts\/with-cache-cleanup\.mjs/mu.exec(source)[1];
  console.log(`${revision}: ${Date.parse(end) - Date.parse(start)} ms elapsed in the test step`);
}

const tenths = Array.from({ length: 10 }, (_, index) =>
  partitionGroupFiles(names, `${index + 1}/10`).map(file => `js/tests/${file}`));
const candidates = [eighths, partitionMeasuredWorkloads(files, 9, { costs: prior }), tenths];
for (const shards of candidates) {
  assert.deepEqual(shards.flat().sort(), files);
  const loads = shards.map(shard => shard.reduce((sum, file) => sum + measurements.get(file), 0));
  const peak = Math.max(...loads);
  console.log(`${shards.length} shards: ${loads.join(', ')} ms, maximum ${peak} ms`);
  if (shards === tenths) assert.ok(peak <= 500_000, 'the active assignment exceeds its advisory test budget');
}
