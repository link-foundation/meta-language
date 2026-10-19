import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { restoreVerificationConcurrencyHistory } from './support/verification-concurrency-history.js';

test('historical workflow comparison authenticates every current queue before restoring only recorded edits', () => {
  const history = readProjectData(new URL('../../parity/fixtures/verification-concurrency-history.lino', import.meta.url));
  let count = 0;
  for (const file of history.files) {
    const actual = readFileSync(new URL('../../' + file.after.path, import.meta.url), 'utf8');
    const current = file.name === 'js.yml' ? restoreGeneratedPackageOwnerHistory(actual)
      : file.name === 'ci.yml' ? restoreAggregateFailureReportHistory(actual) : actual;
    assert.equal(current, file.after.source);
    assert.equal(restoreVerificationConcurrencyHistory(current, file.name), file.before.source);
    assert.throws(() => restoreVerificationConcurrencyHistory(file.before.source, file.name), /current queue identities/u);
    for (const change of file.changes) {
      if (!change.before.trimStart().startsWith('group: ')) continue;
      count += change.occurrences;
      for (const replacement of [change.before, '', change.after.replace('group: ', 'group: changed-'),
        change.after.replace(/-\$\{\{ matrix\.[^}]+\}\}/u, '')]) {
        if (replacement === change.after) continue;
        assert.throws(() => restoreVerificationConcurrencyHistory(current.replace(change.after, replacement), file.name));
      }
    }
    for (const [before, after] of [['timeout-minutes: ', 'timeout-minutes: 99 # '],
      ['fail-fast: false', 'fail-fast: true'], ['cancel-in-progress: true', 'cancel-in-progress: false']]) {
      assert.ok(current.includes(before));
      const changed = current.replace(before, after);
      if (before.startsWith('cancel-')) assert.throws(() => restoreVerificationConcurrencyHistory(changed, file.name));
      else assert.equal(restoreVerificationConcurrencyHistory(changed, file.name), file.before.source.replace(before, after),
        'unrelated workflow edits survive the inverse and remain visible to every original assertion');
    }
  }
  assert.equal(count, 36);
  assert.throws(() => restoreVerificationConcurrencyHistory('', 'unregistered.yml'));
  assert.throws(() => restoreVerificationConcurrencyHistory(null, 'ci.yml'));
});

import { restoreGeneratedPackageOwnerHistory } from './support/generated-package-owner-history.js';

import { restoreAggregateFailureReportHistory } from './support/aggregate-failure-history.js';
