import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { partitionGroupFiles, partitionTests, testFiles } from '../scripts/test-groups.mjs';

test('four tooling shards redistribute the completed file workload that reached its deadline', () => {
  const packet = readProjectData(new URL('../../parity/fixtures/ci-45296878458-tooling-test-wall-costs.lino', import.meta.url));
  const expired = packet.logs.find(row => row.id === 138119600350);
  assert.equal(expired.run, packet.run);
  assert.equal(expired.revision, packet.revision);
  assert.equal(expired.number, 1);
  assert.equal(expired.conclusion, 'cancelled');
  assert.equal(expired.failedTests, 0);
  assert.ok(expired.setupMilliseconds + expired.testMilliseconds >= 900_000);
  for (const run of [45250356617, 45296878458]) {
    assert.deepEqual(packet.logs.filter(row => row.run === run).map(row => row.number), [1, 2, 3, 4]);
  }

  const files = partitionTests(testFiles()).groups.tooling;
  const assignments = Array.from({ length: 4 }, (_, index) => partitionGroupFiles(files, `${index + 1}/4`));
  assert.ok(assignments.every(shard => shard.length > 0));
  assert.deepEqual(assignments.flat().sort(), files);
  const costByFile = new Map(packet.entries.map(row => [row.file, row.milliseconds]));
  const measuredLoads = assignments.map(shard => shard.reduce((total, file) => total + (costByFile.get('js/tests/' + file) ?? 0), 0));
  assert.ok(Math.max(...measuredLoads) < expired.testMilliseconds * 0.8, measuredLoads.join(', '));
});
