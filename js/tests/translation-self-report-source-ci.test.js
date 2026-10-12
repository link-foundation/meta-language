import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';

import test from 'node:test';

import { sourceCoverage } from '../scripts/generate-self-translation-report.mjs';
import { parseProgrammingLanguage } from '../src/programming-language-parser.js';
import { selfTranslate } from '../src/self-translation.js';

const fixture = readProjectData(new URL('../../parity/fixtures/self-translation-report-source-ci.json', import.meta.url));

test('the authentic report CI module retains native source items and positive translation at default limits', () => {
  assert.equal(fixture.provenance.runId, 38240388521);
  assert.equal(fixture.provenance.jobId, 114789033111);
  assert.equal(fixture.module, fixture.reportedMeasurement.module);
  assert.equal(createHash('sha256').update(fixture.source).digest('hex'), fixture.sourceSha256);
  assert.equal(fixture.sourceSha256, fixture.reportedMeasurement.sourceSha256);
  assert.equal(Buffer.byteLength(fixture.source), fixture.reportedMeasurement.coverage.sourceBytes);
  assert.deepEqual(fixture.reportedMeasurement.items, { translated: 0, carried: 1, comment: 0 });

  const parsed = parseProgrammingLanguage(fixture.source, 'JavaScript');
  assert.equal(parsed.rootTerm, 'program');
  assert.equal(parsed.tree.flags.hasError, false);
  assert.ok(parsed.tree.children.some(node => node.term === 'import_statement'));
  assert.ok(parsed.tree.children.some(node => node.term === 'export_statement'));
  assert.equal(parsed.tokens.map(token => token.text).join(''), fixture.source);

  // This checks the failed module alone; CI still produces and validates the
  // complete report and its dependency closure against the observed commit.
  const translation = selfTranslate(fixture.source, 'JavaScript', 'Rust');
  assert.ok(translation.items.some(item => item.status === 'translated'), fixture.module);
  assert.ok(translation.items.length > fixture.reportedMeasurement.items.carried);
  assert.equal(translation.items.some(item => item.term === 'ERROR'), false);
  const coverage = sourceCoverage(fixture.source, translation.items);
  assert.equal(coverage.sourceBytes, fixture.reportedMeasurement.coverage.sourceBytes);
  assert.equal(coverage.itemBytes + coverage.layoutBytes, coverage.sourceBytes);
  assert.equal(coverage.unrepresentedBytes, 0);
  assert.deepEqual(coverage.unrepresentedRanges, []);
});
