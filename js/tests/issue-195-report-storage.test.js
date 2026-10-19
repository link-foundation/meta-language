import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';
import { canonicalJson, parityDigests, PARITY_ARTIFACT_FILES, writeParityDigestArtifacts } from '../scripts/issue-195-parity-evidence.mjs';
import { readStructuredDataRecords } from '../src/structured-data-records.js';
import { stageFiles } from '../scripts/issue-195-evidence-stages.mjs';

test('persisted parity and stage reports retain exact values and canonical hash domains', async () => {
  const directory = mkdtempSync(path.join(tmpdir(), 'report-storage-'));
  try {
    const observations = { javascript: { schemaVersion: 1, positive: [{ source: 'α', nodes: ['original wire snapshot'] }] },
      rust: { schemaVersion: 1, positive: [{ source: 'β', nodes: ['changed wire snapshot'] }] } };
    const expected = parityDigests(observations);
    await writeParityDigestArtifacts(directory, observations);
    for (const role of ['digests', 'differences']) {
      const bytes = readFileSync(path.join(directory, PARITY_ARTIFACT_FILES[role]), 'utf8');
      const decoded = readStructuredDataRecords(bytes);
      assert.deepEqual(decoded, expected[role]);
      assert.equal(createHash('sha256').update(canonicalJson(decoded)).digest('hex'), createHash('sha256').update(canonicalJson(expected[role])).digest('hex'));
    }
    const report = { schemaVersion: 1, commit: 'a'.repeat(40), checkpoint: 'pre-merge', groups: { parity: { artifacts: ['same-source.rs'] } }, error: null };
    const physical = writeProjectData(path.join(directory, 'legacy-logical-report.json'), report, { format: 'lino' });
    assert.equal(physical, path.join(directory, 'legacy-logical-report.lino'));
    assert.deepEqual(readProjectData(path.join(directory, 'legacy-logical-report.json')), report);
    assert.deepEqual(stageFiles('runtime-parity'), { record: 'work/stage-runtime-parity.lino', observations: 'work/stage-runtime-parity-observations.lino' });
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
