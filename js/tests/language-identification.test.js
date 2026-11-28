import assert from 'node:assert/strict';
import test from 'node:test';

import { identifyLanguage } from 'meta-language';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/language-identification.lino';
const fixture = readProjectData(new URL(`../../${fixtureFile}`, import.meta.url));

function observe(section, assertions, testName) {
  recordIssue195Observations({
    requirementId: `I195-LANGUAGE-IDENTIFICATION-${section.toUpperCase()}`,
    suffix: 'behavior', fixtureId: `language-identification:${section}`, fixtureFile, assertions, testName,
  });
}

test('package entry point identifies an English sentence', ({ name }) => {
  assert.equal(identifyLanguage(fixture.ranking.source).language, 'English');
  observe('export', ['packageEntryPointIdentifiesLanguage'], name);
});

test('candidates are ranked and remain visible when the threshold rejects them', ({ name }) => {
  const { source, candidates: expected, rejectingThreshold } = fixture.ranking;
  const report = identifyLanguage(source);
  assert.deepEqual(report.candidates.map(({ language }) => language), expected.map(([language]) => language));
  for (const [index, { confidence }] of report.candidates.entries()) {
    assert.ok(confidence >= 0 && confidence <= 1);
    assert.ok(Math.abs(confidence - expected[index][1]) < 1e-14);
  }
  const rejected = identifyLanguage(source, { threshold: rejectingThreshold });
  assert.equal(rejected.language, null);
  assert.equal(rejected.spans[0].language, null);
  assert.deepEqual(rejected.candidates, report.candidates);
  assert.deepEqual(rejected.spans[0].candidates, report.spans[0].candidates);
  assert.equal(identifyLanguage(source, { threshold: report.candidates[0].confidence }).language, 'English');
  for (const threshold of [-0.1, 1.1, NaN, Infinity, -Infinity]) {
    assert.throws(() => identifyLanguage(source, { threshold }), RangeError);
  }
  observe('ranking', ['candidatesRankedWithConfidence', 'thresholdSelectsUnknown'], name);
});

test('Cyrillic rejection distinguishes Russian from unsupported languages', ({ name }) => {
  const { russian, unsupported, foreignLetters } = fixture.cyrillic;
  const russianReport = identifyLanguage(russian);
  assert.equal(russianReport.language, 'Russian');
  assert.deepEqual(identifyLanguage(russian.normalize('NFD')).candidates, russianReport.candidates);
  const expectUnknown = (source, label) => {
    const report = identifyLanguage(source, { threshold: 0 });
    assert.equal(report.language, null, label);
    assert.deepEqual(report.candidates, [], label);
    assert.equal(report.spans.length, 1, label);
    assert.equal(report.spans[0].language, null, label);
  };
  for (const [language, source] of unsupported) expectUnknown(source, language);
  for (const letter of foreignLetters) {
    expectUnknown(`${russian} ${letter}`, letter);
    expectUnknown(`${russian} ${letter.toUpperCase()}`, letter);
  }
  observe('cyrillic', ['russianEvidenceRetained', 'unsupportedCyrillicRemainsUnknown'], name);
});

test('mixed scripts expose byte spans, including minority and unknown runs', ({ name }) => {
  for (const { source, spans } of fixture.mixed) {
    const report = identifyLanguage(source);
    const bytes = Buffer.from(source, 'utf8');
    assert.deepEqual(report.spans.map(({ start, end, language }) => [
      start, end, bytes.subarray(start, end).toString('utf8'), language,
    ]), spans);
  }
  const report = identifyLanguage(fixture.mixed[0].source);
  assert.equal(report.language, null);
  assert.ok(report.candidates.some(({ language }) => language === 'Russian'));
  assert.ok(report.spans[0].candidates.some(({ language }) => language === 'English'));
  observe('mixed', ['mixedScriptSpansHaveLanguagesAndByteOffsets'], name);
});

test('short, neutral and bounded input follow the documented evidence policy', ({ name }) => {
  for (const source of fixture.short.unknown) {
    const report = identifyLanguage(source, { threshold: 0 });
    assert.equal(report.language, null, source);
    assert.deepEqual(report.candidates, [], source);
  }
  for (const [source, language] of fixture.short.known) {
    assert.equal(identifyLanguage(source).language, language, source);
  }
  assert.deepEqual(identifyLanguage('👋'.repeat(2048) + fixture.ranking.source).spans, []);
  observe('short', ['shortAndNonTextInputsUseDocumentedPolicy'], name);
});
