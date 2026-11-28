import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { readProjectData } from '../scripts/project-data.mjs';
import { recoverJson } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/json-recovery.lino';
const { cases } = readProjectData(new URL(`../../${fixtureFile}`, import.meta.url));

for (const group of new Set(cases.map(item => item.group))) {
  test(`JSON recovery ${group}`, (context) => {
    for (const { id, input, options, expected } of cases.filter(item => item.group === group)) {
      const result = recoverJson(input, options);
      assert.deepEqual(result, expected, id);
      const bytes = Buffer.from(input);
      let offset = 0;
      let replayed = '';
      for (const { kind, span, inserted, removed } of result.edits) {
        assert.ok(offset <= span.start && span.start <= span.end && span.end <= bytes.length, id);
        assert.equal(bytes.toString('utf8', span.start, span.end), removed, id);
        assert.ok(!options.allow || options.allow.includes(kind), id);
        replayed += bytes.toString('utf8', offset, span.start) + inserted;
        offset = span.end;
      }
      assert.equal(replayed + bytes.toString('utf8', offset), result.text, id);
      if (!result.issues.some(issue => issue.kind === 'unrecoveredSyntax')) {
        assert.deepEqual(JSON.parse(result.text), result.value, id);
      }
    }
    recordIssue195Observations({
      requirementId: `I195-JSON-RECOVERY-${group.toUpperCase()}`,
      suffix: 'behavior',
      fixtureId: `json-recovery:${group}`,
      fixtureFile,
      assertions: ['jsonRecoveryReportMatches', 'jsonRecoveryEditsReplay'],
      testName: context.name,
    });
  });
}

test('the packages use identical JSON recovery grammar and decorators', () => {
  for (const file of ['json-recovery-decorators.lino', 'json-recovery-grammar.lino']) {
    assert.deepEqual(readFileSync(new URL(`../src/data/${file}`, import.meta.url)),
      readFileSync(new URL(`../../rust/src/data/${file}`, import.meta.url)));
  }
});

test('unknown allow-list operations and non-Unicode source are rejected', () => {
  assert.throws(() => recoverJson('{}', { allow: ['rewrite'] }), TypeError);
  assert.throws(() => recoverJson('\ud800'), TypeError);
});

test('JSON recovery closes deeply nested objects within the native rule limit', () => {
  const depth = 60;
  const input = '{"value":'.repeat(depth) + '0';
  const result = recoverJson(input, { allow: ['closeOpenContainers'] });
  assert.equal(result.edits.length, depth);
  assert.equal(result.text, input + '}'.repeat(depth));
  assert.equal(result.complete, false);
});
