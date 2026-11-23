import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork, LinkType, naturalLanguageValues } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/natural-language-values.lino';
const { fixtures } = readProjectData(new URL(`../../${fixtureFile}`, import.meta.url));

test('natural-language values preserve the original spelling, spans, and alternatives', (context) => {
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
  recordIssue195Observations({
    requirementId: 'I195-I242-NATURAL-LANGUAGE-VALUES', suffix: 'behavior',
    fixtureId: 'i195-i242-natural-language-values', fixtureFile,
    assertions: ['localizedExactNumbers', 'currencyRanges', 'inflectedDurations', 'dateAlternatives', 'utf8SourceSpans', 'explicitZeroInNumberWordCompounds'],
    testName: context.name,
  });
});
