// The downstream consumer matrix (docs/downstream-consumers.md): the usage and
// the requirements of relative-meta-logic and formal-ai are mapped to
// meta-language capabilities, ledger rows and tests, and a mapping to a ledger
// row or a test that does not exist is rejected.
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { test } from 'node:test';

import {
  CONSUMER_MATRIX,
  DOWNSTREAM_CONSUMERS,
  downstreamByRow,
  parseConsumerMatrix,
  validateConsumerMatrix,
} from '../scripts/issue-195-downstream.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const repositoryFile = (relative) => new URL(`../../${relative}`, import.meta.url);
const markdown = readFileSync(repositoryFile(CONSUMER_MATRIX), 'utf8');
const manifest = JSON.parse(readFileSync(repositoryFile('parity/issue-195-requirements.json'), 'utf8'));
const context = {
  rowIds: new Set(manifest.atomicRequirements.map(({ id }) => id)),
  fileExists: (relative) => existsSync(repositoryFile(relative)),
};

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-DOWNSTREAM-CONSUMER-MATRIX',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-downstream-consumer-matrix',
    fixtureFile: CONSUMER_MATRIX,
    assertions,
    testName,
    runtime: 'tooling',
  });
}

// A consumer section with one mapping row.
const section = (heading, row) => [
  `## ${heading}`, '',
  '| Consumer usage or requirement | meta-language capability | Ledger rows | Tests | Status |',
  '|---|---|---|---|---|',
  row, '',
].join('\n');
const scope = [
  '## Audit scope', '',
  `- relative-meta-logic, <https://github.com/link-foundation/relative-meta-logic>, inspected at commit \`${'a'.repeat(40)}\`.`,
  `- formal-ai, <https://github.com/link-assistant/formal-ai>, inspected at commit \`${'b'.repeat(40)}\`.`, '',
].join('\n');
const covered = '| Parsing | `LinkNetwork` | `I195-DOWNSTREAM-CONSUMER-MATRIX` | `js/tests/downstream-consumers.test.js` | covered by tests |';

test('the matrix maps the usage of both downstream consumers at an inspected commit', () => {
  const { consumers } = parseConsumerMatrix(markdown);
  assert.deepEqual(Object.keys(consumers), Object.keys(DOWNSTREAM_CONSUMERS));
  assert.equal(consumers['formal-ai'].repository, 'link-assistant/formal-ai');
  for (const consumer of Object.values(consumers)) {
    assert.ok(consumer.present);
    assert.match(consumer.commit, /^[0-9a-f]{40}$/u);
    assert.ok(consumer.mappings.length > 0);
    for (const mapping of consumer.mappings) assert.ok(mapping.rows.length > 0, mapping.usage);
  }
  assert.ok(consumers['relative-meta-logic'].mappings.some(({ status }) => status === 'covered by tests'));
  observe(['relativeMetaLogicUsageMapped', 'formalAiUsageMapped'],
    'the matrix maps the usage of both downstream consumers at an inspected commit');
});

test('every mapped ledger row and test of the matrix exists', () => {
  const matrix = parseConsumerMatrix(markdown);
  assert.deepEqual(validateConsumerMatrix(matrix, context), []);
  // The consumers whose usage a row covers are part of the row's traceability.
  const byRow = downstreamByRow(matrix);
  for (const row of manifest.atomicRequirements.filter(({ id }) => byRow.has(id))) {
    for (const consumer of byRow.get(row.id)) assert.ok(row.traceability.downstream.includes(consumer), row.id);
  }
  observe(['everyMappingResolves'], 'every mapped ledger row and test of the matrix exists');
});

test('a mapping to a missing ledger row or test, or without a row, is rejected', () => {
  const valid = parseConsumerMatrix(`${scope}${section('relative-meta-logic', covered)}${section('formal-ai', covered)}`);
  assert.deepEqual(validateConsumerMatrix(valid, context), []);
  const broken = (row) => validateConsumerMatrix(
    parseConsumerMatrix(`${scope}${section('relative-meta-logic', row)}${section('formal-ai', covered)}`), context);
  assert.match(broken(covered.replace('I195-DOWNSTREAM-CONSUMER-MATRIX', 'I195-NO-SUCH-ROW')).join('\n'),
    /missing ledger row I195-NO-SUCH-ROW/u);
  assert.match(broken(covered.replace('downstream-consumers.test.js', 'no-such.test.js')).join('\n'),
    /missing test js\/tests\/no-such\.test\.js/u);
  assert.match(broken(covered.replace('`I195-DOWNSTREAM-CONSUMER-MATRIX`', 'none')).join('\n'), /maps no ledger row/u);
  assert.match(broken(covered.replace('`js/tests/downstream-consumers.test.js`', 'none')).join('\n'),
    /covered by tests but names none/u);
  assert.match(broken(covered.replace('covered by tests', 'done')).join('\n'), /unknown status done/u);
  const missing = validateConsumerMatrix(parseConsumerMatrix(`${scope}${section('relative-meta-logic', covered)}`), context);
  assert.match(missing.join('\n'), /has no formal-ai section/u);
});
