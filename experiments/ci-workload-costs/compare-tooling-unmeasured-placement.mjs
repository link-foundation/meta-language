// Replay the archived tooling timings against both placements of the current files.
// Usage: node experiments/ci-workload-costs/compare-tooling-unmeasured-placement.mjs
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { partitionMeasuredWorkloads, partitionTests, testFiles } from '../../js/scripts/test-groups.mjs';

const root = new URL('../../', import.meta.url);
const costs = readProjectData(new URL('parity/fixtures/ci-45186623729-tooling-wall-costs.lino', root));
const historical = new Map(costs.entries.map(entry => [entry.file, entry]));
const files = partitionTests(testFiles()).groups.tooling.map(file => `js/tests/${file}`);
const sources = new Map(files.map(file => [file, readFileSync(new URL(file, root))]));
const unmeasured = new Set(files.filter(file => {
  const bytes = sources.get(file);
  const previous = historical.get(file);
  return previous?.sha256 !== createHash('sha256').update(bytes).digest('hex');
}));

for (const prioritizeUnmeasured of [false, true]) {
  const shards = partitionMeasuredWorkloads(files, 4, {
    costs, prioritizeUnmeasured, readSource: file => sources.get(file),
  });
  assert.deepEqual(shards.flat().sort(), files);
  const unknownCounts = shards.map(shard => shard.filter(file => unmeasured.has(file)).length);
  const historicalLoads = shards.map(shard => shard.reduce((total, file) =>
    total + (historical.get(file)?.milliseconds ?? 0), 0));
  const label = prioritizeUnmeasured ? 'unmeasured first' : 'estimated duration first';
  console.log(`${label}: ${unknownCounts.join(', ')} unmeasured files per shard`);
  console.log(`${label}: ${historicalLoads.join(', ')} ms replayed historical time per shard`);
}
