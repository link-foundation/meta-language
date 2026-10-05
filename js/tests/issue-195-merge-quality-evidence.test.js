// Requirement I195-MERGE-QUALITY-EVIDENCE: the published comparison of every
// native grammar with the pinned tree-sitter grammar it was merged from, on
// the corpus of that grammar. This suite recomputes the
// deterministic part, coverage, preserved features, correctness, recovery and
// shared reuse, and compares it with parity/fixtures/merge-quality-evidence.json;
// checks that docs/grammar/merge-quality-evidence.md publishes it with the
// time and memory of both runtimes in
// parity/fixtures/merge-quality-measurements.json; and measures the
// JavaScript native executor and oracle again. The Rust twin is
// rust/tests/unit/issue_195_merge_quality_evidence.rs, with the measurement
// in rust/tests/merge_quality.rs.
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { test } from 'node:test';

import {
  MERGE_QUALITY_DOCUMENT,
  MERGE_QUALITY_FIXTURE,
  MERGE_QUALITY_MEASUREMENTS,
  formatMergeQuality,
  measureJavaScript,
  mergeQualityReport,
  nativeGrammarEntries,
  renderMergeQualityDocument,
} from '../scripts/build-merge-quality-evidence.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-MERGE-QUALITY-EVIDENCE',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-merge-quality-evidence',
    fixtureFile: MERGE_QUALITY_FIXTURE,
    assertions,
    testName,
  });
}

test('merge quality evidence: the JavaScript report equals the published report', (context) => {
  const report = mergeQualityReport();
  assert.equal(formatMergeQuality(report), read(MERGE_QUALITY_FIXTURE));
  assert.deepEqual(report.grammars.map(({ grammar }) => grammar), nativeGrammarEntries().map(([id]) => id));
  assert.equal(report.grammars.length, 14);
  for (const { grammar, sources, corpus, mergeReport, coverage, correctness, recovery } of report.grammars) {
    // Every grammar names the sources it merges: a URL, or a vendored copy.
    assert.ok(sources.length > 0, grammar);
    for (const source of sources) assert.ok(source.startsWith('https://') || existsSync(new URL(`../../${source}`, import.meta.url)), source);
    // A grammar the importer merges is checked on the upstream test corpus of
    // its source grammar at the pinned revision.
    if (mergeReport !== null) assert.match(corpus, /^https:\/\/github\.com\/.+\/tree\/[0-9a-f]{40}\/.*corpus$/u, grammar);
    assert.ok(coverage.exercisedRules > 0 && coverage.checkedKinds > 0, grammar);
    assert.ok(correctness.matches >= 30 && correctness.rows > correctness.matches, grammar);
    // Every rejection the oracle recovers from is repaired natively.
    assert.equal(recovery.repaired, correctness.rejections, grammar);
    // A grammar the importer merged reports no approximation and no unsupported feature.
    if (mergeReport !== null) {
      assert.equal(coverage.approximations, 0, grammar);
      assert.equal(coverage.unsupported, 0, grammar);
    }
  }
  observe(['coverageMeasured', 'correctnessMeasured', 'recoveryMeasured'], context.name);
});

test('merge quality evidence: the comparison is published with the measurements of both runtimes', (context) => {
  const report = mergeQualityReport();
  const measurements = JSON.parse(read(MERGE_QUALITY_MEASUREMENTS));
  assert.deepEqual(measurements.grammars.map(({ grammar }) => grammar), report.grammars.map(({ grammar }) => grammar));
  for (const { grammar, javascript, rust } of measurements.grammars) {
    for (const side of ['native', 'oracle']) {
      assert.ok(javascript[side].parseMs > 0 && javascript[side].recoverMs > 0, `${grammar} ${side}`);
      assert.ok(Number.isInteger(javascript[side].peakKiB), `${grammar} ${side}`);
    }
    assert.ok(rust.native.parseMs > 0 && rust.native.peakHeapBytes > 0, grammar);
  }
  assert.equal(renderMergeQualityDocument(report, measurements), read(MERGE_QUALITY_DOCUMENT));
  observe(['comparisonPublished'], context.name);
});

test('merge quality evidence: the JavaScript native executor and oracle are measured on the corpus', (context) => {
  // Each side runs in its own process; JSON is the smallest corpus.
  const { native, oracle } = measureJavaScript('native-json');
  for (const side of [native, oracle]) {
    assert.ok(side.parseMs > 0 && side.recoverMs > 0);
    assert.ok(Number.isInteger(side.peakKiB) && side.peakKiB >= 0);
  }
  observe(['timeAndMemoryMeasured'], context.name);
});
