import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork, LinkType, naturalLanguageValues } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/natural-language-values.lino';
const unicodeFixtureFile = 'parity/fixtures/natural-language-unicode-decimal-values.lino';
const writtenMultiplierFixtureFile = 'parity/fixtures/natural-language-written-multipliers.lino';
const namedCalendarDateFixtureFile = 'parity/fixtures/natural-language-named-calendar-dates.lino';

function checkValueFixtures(file) {
  const { fixtures } = readProjectData(new URL(`../../${file}`, import.meta.url));
  for (const { source, language, expected, absent = [] } of fixtures) {
    const network = LinkNetwork.parse(source, language);
    const bytes = Buffer.from(source);
    const entries = network.links().map(link => link.metadata())
      .filter(({ linkType, term }) => linkType === LinkType.Semantic && term.startsWith('value:'));
    assert.equal(network.reconstructText(), source);
    const values = entries.map(({ span, term, language: owner }) => {
      assert.equal(owner, language);
      const { start, end } = span.byteRange;
      assert.ok(start < end && end <= bytes.length);
      return [bytes.subarray(start, end).toString(), term];
    });
    assert.deepEqual(naturalLanguageValues(source, language).map(({ start, end, term }) => [bytes.subarray(start, end).toString(), term]), values);
    for (const entry of expected) assert.ok(values.some(value => value.every((part, index) => part === entry[index])), entry.join(' → '));
    for (const entry of absent) assert.ok(!values.some(value => value.every((part, index) => part === entry[index])), entry.join(' → '));
  }
}

test('natural-language values preserve the original spelling, spans, and alternatives', (context) => {
  checkValueFixtures(fixtureFile);
  recordIssue195Observations({
    requirementId: 'I195-I242-NATURAL-LANGUAGE-VALUES', suffix: 'behavior',
    fixtureId: 'i195-i242-natural-language-values', fixtureFile,
    assertions: ['localizedExactNumbers', 'currencyRanges', 'inflectedDurations', 'dateAlternatives', 'utf8SourceSpans', 'explicitZeroInNumberWordCompounds'],
    testName: context.name,
  });
});

test('Unicode decimal digits retain typed values and original UTF-8 spans', (context) => {
  checkValueFixtures(unicodeFixtureFile);
  recordIssue195Observations({
    requirementId: 'I195-I242-UNICODE-DECIMAL-VALUES', suffix: 'behavior',
    fixtureId: 'i195-i242-unicode-decimal-values', fixtureFile: unicodeFixtureFile,
    assertions: ['unicodeDecimalsAndGrouping', 'unicodeCalendarValidationAndAlternatives', 'unicodeValueSourceSpans'],
    testName: context.name,
  });
});

test('digit amounts with written multipliers preserve exact values and original spans', (context) => {
  checkValueFixtures(writtenMultiplierFixtureFile);
  recordIssue195Observations({
    requirementId: 'I195-I242-WRITTEN-MULTIPLIERS', suffix: 'behavior',
    fixtureId: 'i195-i242-written-multipliers', fixtureFile: writtenMultiplierFixtureFile,
    assertions: ['writtenMultipliersNormalizeExactValues', 'writtenMultiplierRangesPreserveEndpointScales', 'writtenMultipliersKeepOriginalSpans'],
    testName: context.name,
  });
});

test('named calendar dates and ranges preserve valid days and original spans', (context) => {
  checkValueFixtures(namedCalendarDateFixtureFile);
  recordIssue195Observations({
    requirementId: 'I195-I242-NAMED-CALENDAR-DATES', suffix: 'behavior',
    fixtureId: 'i195-i242-named-calendar-dates', fixtureFile: namedCalendarDateFixtureFile,
    assertions: ['namedCalendarDatesAndRanges', 'namedCalendarDateSpansAndValidation'],
    testName: context.name,
  });
});
