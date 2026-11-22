import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { createGrammarParser, importEbnf, LinkNetwork, LinkQuery } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/grammar-search.json';
const fixture = readProjectData(new URL('../../' + fixtureFile, import.meta.url));
const parserFor = grammar => createGrammarParser(importEbnf(grammar));

function observed(requirementId, context, assertions) {
  recordIssue195Observations({
    requirementId, suffix: 'behavior', fixtureId: requirementId.toLowerCase(), fixtureFile,
    assertions, testName: context.name,
  });
}

test('grammar search reports exact source offsets and repeated captures', context => {
  const parser = parserFor(fixture.grammar);
  assert.equal(parser.parseTree(fixture.source).ok, false);
  assert.equal(parser.parseTree(fixture.matches[2].text).ok, true);
  const matches = parser.findAll(fixture.source, { rule: 'price', wordBoundaries: true, captures: ['amount', 'currency'] });
  assert.equal(matches.length, fixture.matches.length);
  for (const [index, expected] of fixture.matches.entries()) {
    const match = matches[index];
    const start = fixture.source.indexOf(expected.text);
    assert.deepEqual([match.start, match.end, match.text], [start, start + expected.text.length, expected.text]);
    assert.equal(match.tree.start, Buffer.byteLength(fixture.source.slice(0, start)));
    assert.equal(match.tree.end, Buffer.byteLength(fixture.source.slice(0, match.end)));
    for (const name of ['amount', 'currency']) {
      assert.deepEqual(match.captures[name].map(capture => capture.text), expected[name]);
      for (const capture of match.captures[name]) {
        assert.equal(fixture.source.slice(capture.start, capture.end), capture.text);
      }
    }
  }
  observed('I195-I235-GRAMMAR-SEARCH-SPANS', context, ['searchFindsOriginalSpans', 'wholeInputParseIsPreserved']);
  observed('I195-I235-GRAMMAR-SEARCH-CAPTURES', context, ['repeatedNamedCapturesRetainSourceSpans']);
});

test('grammar search controls word boundaries, selection and overlap', context => {
  const words = parserFor(fixture.boundaryGrammar);
  assert.deepEqual(words.findAll(fixture.boundarySource, { wordBoundaries: true }).map(match => match.start),
    fixture.boundaryStarts);
  assert.equal(words.findAll(fixture.boundarySource).length, 7);
  const choices = parserFor(fixture.selectionGrammar);
  for (const [key, options] of [['longest', {}], ['first', { matchPolicy: 'peg-first' }],
    ['overlapping', { overlapping: true }]]) {
    assert.deepEqual(choices.findAll(fixture.selectionSource, options).map(match => match.text), fixture.selection[key]);
  }
  assert.deepEqual(parserFor('start = "" ;').findAll('aaa'), []);
  assert.throws(() => choices.findAll('aaa', { rule: 'missing' }), { reason: 'reference' });
  const limited = createGrammarParser(importEbnf(fixture.selectionGrammar), { stepLimit: 1 });
  assert.throws(() => limited.findAll('aaa'), { reason: 'stepLimit' });
  observed('I195-I235-GRAMMAR-SEARCH-POLICY', context, ['wordBoundariesUseUnicode', 'matchPoliciesChooseAtEachStart', 'overlapAndEmptyMatchesAreControlled']);
});

test('grammar search composes with source text predicates in LinkQuery', context => {
  const parser = parserFor(fixture.grammar);
  const network = LinkNetwork.parse(fixture.html, 'HTML');
  const query = LinkQuery.fromSexpression(fixture.query);
  const matches = network.queryMatchesWith(query, parser.searchPredicate({ wordBoundaries: true }));
  assert.deepEqual(matches.map(match => network.capturedText(match.captures.first('candidate'))), [fixture.matches[1].text]);
  const cyclicNetwork = LinkNetwork.parseLosslessText('text', 'English');
  const wildcard = LinkQuery.fromSexpression('(_) @source (#grammar-match? @source)');
  assert.deepEqual(cyclicNetwork.queryMatchesWith(wildcard, parser.searchPredicate()), []);
  observed('I195-I235-GRAMMAR-SEARCH-QUERY', context, ['sourceTextQueryPredicateComposes']);
});
