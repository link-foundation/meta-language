import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { partitionGroupFiles, partitionTests, testFiles } from '../scripts/test-groups.mjs';

test('the nine translation shards redistribute the measured workload that expired before feedback retention', () => {
  const files = partitionTests(testFiles()).groups.translation;
  const costs = readProjectData(new URL('../../parity/fixtures/ci-45118414652-translation-test-wall-costs.lino', import.meta.url));
  assert.equal(costs.logs.length, 9);
  const expired = costs.logs.find(row => row.job === 137506501617);
  assert.equal(expired.number, 3);
  assert.equal(expired.conclusion, 'cancelled');
  assert.equal(expired.completedTests, true);
  assert.ok(expired.measuredMilliseconds > 800000);

  const shards = Array.from({ length: 9 }, (_, index) => partitionGroupFiles(files, `${index + 1}/9`));
  assert.deepEqual(shards.flat().sort(), files);
  for (let cohort = 0; cohort < 3; cohort += 1) {
    const retained = [shards[cohort], shards[cohort + 3], shards[cohort + 6]].flat().sort();
    assert.deepEqual(retained, partitionGroupFiles(files, `${cohort + 1}/3`));
  }
  const millisecondsByFile = new Map(costs.entries.map(row => [row.file, row.milliseconds]));
  const measuredLoads = shards.map(shard => shard.reduce((load, file) => load + (millisecondsByFile.get('js/tests/' + file) ?? 0), 0));
  assert.ok(Math.max(...measuredLoads) < expired.measuredMilliseconds * 0.75, measuredLoads.join(', '));
});
