import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { partitionGroupFiles, partitionTests, testFiles } from '../scripts/test-groups.mjs';

test('six grammar shards rebalance the completed tests that exceeded the job deadline', () => {
  const packet = readProjectData(new URL('../../parity/fixtures/ci-45250356617-grammar-test-wall-costs.lino', import.meta.url));
  const expired = packet.logs.find(row => row.id === 137958366334);
  assert.equal(expired.run, packet.run);
  assert.equal(expired.revision, packet.revision);
  assert.equal(expired.number, 2);
  assert.equal(expired.conclusion, 'cancelled');
  assert.ok(expired.testMilliseconds > 750_000);
  assert.ok(expired.setupMilliseconds + expired.testMilliseconds >= 900_000);
  assert.deepEqual(packet.logs.filter(row => row.run === 45215384144).map(row => row.number), [1, 2, 3, 4, 5, 6]);

  const files = partitionTests(testFiles()).groups.grammar;
  const shards = Array.from({ length: 6 }, (_, index) => partitionGroupFiles(files, `${index + 1}/6`));
  assert.ok(shards.every(files => files.length > 0));
  assert.deepEqual(shards.flat().sort(), files);
  const millisecondsByFile = new Map(packet.entries.map(row => [row.file, row.milliseconds]));
  const measuredLoads = shards.map(shard => shard.reduce((load, file) => load + (millisecondsByFile.get('js/tests/' + file) ?? 0), 0));
  assert.ok(Math.max(...measuredLoads) < expired.testMilliseconds * 0.75, measuredLoads.join(', '));
});
