import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { readStructuredDataRecords } from '../src/structured-data-records.js';

test('observation writer preserves evidence fields and deduplicates namespaced Links records', () => {
  const directory = mkdtempSync(path.join(tmpdir(), 'issue-195-observation-links-'));
  const file = path.join(directory, 'records.lino');
  const module = new URL('./support/issue-195-observations.js', import.meta.url).href;
  const fixture = new URL('../../docs/vision.md', import.meta.url);
  const digest = createHash('sha256').update(readFileSync(fixture)).digest('hex');
  try {
    const child = spawnSync(process.execPath, ['--input-type=module', '-e', `
      import { recordIssue195Observations } from ${JSON.stringify(module)};
      const observation = {
        requirementId: 'writer-contract', suffix: 'behavior', fixtureId: 'writer-fixture',
        fixtureFile: 'docs/vision.md', assertions: ['field-preservation', 'deduplication'],
        testName: 'writer contract λ', runtime: 'tooling',
      };
      recordIssue195Observations(observation);
      recordIssue195Observations(observation);
    `], {
      env: { ...process.env, ISSUE_195_OBSERVATION_FILE: file, ISSUE_195_COMMIT: 'writer-commit' },
      encoding: 'utf8',
    });
    assert.equal(child.status, 0, child.stderr);
    const text = readFileSync(file, 'utf8');
    assert.equal(text.trim().split('\n').length, 2);
    assert.match(text, /\/record:/u);
    const records = readStructuredDataRecords(text);
    assert.deepEqual(records, ['field-preservation', 'deduplication'].map(assertionId => ({
      testId: 'writer-contract-tooling-behavior', assertionId, fixtureId: 'writer-fixture',
      fixtureDigest: digest, runtime: 'tooling', commit: 'writer-commit',
      outcome: 'passed', testName: 'writer contract λ',
    })));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
