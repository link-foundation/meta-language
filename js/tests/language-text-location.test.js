import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork, locate, wordNgramOverlap } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/text-location.lino';
const fixture = readProjectData(new URL('../../' + fixtureFile, import.meta.url));
const caseFoldingFixtureFile = 'parity/fixtures/text-location-case-folding.lino';
const caseFoldingFixture = readProjectData(new URL('../../' + caseFoldingFixtureFile, import.meta.url));

function observe(context, requirementId, assertions, evidenceFile = fixtureFile) {
  recordIssue195Observations({
    requirementId, fixtureId: requirementId.toLowerCase(), fixtureFile: evidenceFile,
    assertions, suffix: 'behavior', testName: context.name,
  });
}

function verifyExactCases(cases) {
  for (const { source, fragment, normalize, matches } of cases) {
    const network = LinkNetwork.parseLosslessText(source, 'English');
    const actual = locate(network, fragment, { normalize });
    assert.deepEqual(actual.map(({ text, normalizations }) => ({ text, normalizations })), matches);
    let from = 0;
    for (const { start, end, text, distance, edits } of actual) {
      assert.equal(start, source.indexOf(text, from));
      assert.equal(end, start + text.length);
      assert.equal(source.slice(start, end), text);
      assert.equal(distance, 0);
      assert.deepEqual(edits, []);
      from = start + 1;
    }
  }
}

test('quote location maps typography and normalization back to each original source range', context => {
  verifyExactCases(fixture.exact);
  assert.throws(() => locate('example', 'example', { normalize: ['unknown'] }), RangeError);
  observe(context, 'I195-I243-NORMALIZED-TEXT-LOCATION', ['allNormalizedMatchesKeepSourceRanges', 'neededNormalizationsAreReported']);
});

test('approximate location gives bounded grapheme and word edit scripts', context => {
  for (const { source, fragment, unit, match, operation, original, replacement } of fixture.approximate) {
    assert.deepEqual(locate(source, fragment), []);
    const found = locate(source, fragment, { maxEdits: 1, unit }).find(result => result.text === match);
    assert.ok(found, source);
    assert.equal(found.distance, 1);
    assert.equal(found.edits.length, 1);
    const [{ kind, fragment: editFragment, source: editSource }] = found.edits;
    assert.equal(kind, operation);
    assert.equal(editFragment?.text ?? null, original);
    assert.equal(editSource?.text ?? null, replacement);
    if (editFragment) assert.equal(fragment.slice(editFragment.start, editFragment.end), original);
    if (editSource) assert.equal(source.slice(editSource.start, editSource.end), replacement);
  }
  for (const budget of [-1, 0.1, Infinity]) assert.throws(() => locate('abc', 'abc', { maxEdits: budget }), RangeError);
  observe(context, 'I195-I243-APPROXIMATE-TEXT-LOCATION', ['editBudgetAndUnitControlMatches', 'reportedEditScriptsKeepSourceRanges']);
});

function verifyOverlapCases(cases) {
  for (const { left, right, size, normalize, matches } of cases) {
    const found = wordNgramOverlap(left, right, { size, normalize });
    assert.deepEqual(found.map(match => ({
      words: match.words,
      left: left.slice(match.left.start, match.left.end),
      right: right.slice(match.right.start, match.right.end),
    })), matches);
    assert.ok(found.every(match => match.words.length === size));
  }
}

test('word n-gram overlap retains all source pairs', context => {
  verifyOverlapCases(fixture.overlap);
  assert.throws(() => wordNgramOverlap('abc', 'abc', { size: 0 }), RangeError);
  observe(context, 'I195-I243-WORD-NGRAM-OVERLAP', ['wordNgramsReportEverySharedSourcePair']);
});

test('Unicode full case folding keeps original quote and n-gram spans', context => {
  verifyExactCases(caseFoldingFixture.exact);
  verifyOverlapCases(caseFoldingFixture.overlap);
  observe(context, 'I195-I243-UNICODE-CASE-FOLDING', [
    'fullCaseFoldingMatchesKeepSourceRanges', 'caseFoldingOverlapKeepsBothSourceRanges',
  ], caseFoldingFixtureFile);
});
