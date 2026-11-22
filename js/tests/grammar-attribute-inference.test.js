import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { compileGrammar, inferGrammarAttributeAction, runDynamicLanguageCommand } from '../src/index.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/grammar-attribute-inference.lino';
const fixture = readProjectData(new URL('../../' + fixtureFile, import.meta.url));

test('concrete attribute observations constrain executable native actions', context => {
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + ' for ' + fixture.versionRanges.JavaScript);
  const original = parseGrammarLinks(fixture.grammar);
  const { unique, ambiguous, forged, unsupported, numeric, request } = fixture;
  const result = inferGrammarAttributeAction(original, request, unique.observations);
  assert.deepEqual(result.candidates, [unique.candidate]);
  assert.equal(result.diagnostic, null);
  const links = renderGrammarLinks(result.grammar);
  assert.ok(links.includes(unique.doc));
  assert.equal(original.rule(request.rule).action, undefined);
  for (const [listing, value] of [[links, unique.value], [links.replace(unique.candidate, unique.mutation), unique.mutatedValue]]) {
    const parser = compileGrammar(parseGrammarLinks(listing));
    const outcome = parser.parseTree(unique.holdout);
    assert.equal(outcome.ok, true);
    assert.equal(outcome.tree.attributes[request.attribute], value);
    assert.equal(parser.parseTree(unique.negative).ok, false);
  }

  const alternatives = inferGrammarAttributeAction(original, request, ambiguous.observations);
  assert.deepEqual(alternatives.candidates, ambiguous.candidates);
  assert.equal(alternatives.grammar, null);
  assert.ok(alternatives.diagnostic.includes(ambiguous.diagnostic));
  assert.throws(() => inferGrammarAttributeAction(original, request, forged.observations), error => error.message.includes(forged.diagnostic));
  const unresolved = inferGrammarAttributeAction(original, request, unsupported.observations);
  assert.deepEqual(unresolved.candidates, []);
  assert.equal(unresolved.grammar, null);
  assert.ok(unresolved.diagnostic.includes(unsupported.diagnostic));
  const numericInference = inferGrammarAttributeAction(parseGrammarLinks(numeric.grammar), request, numeric.observations);
  assert.deepEqual(numericInference.candidates, [numeric.candidate]);
  const numericParser = compileGrammar(numericInference.grammar);
  assert.equal(numericParser.parseTree('-0').tree.attributes[request.attribute], 0);
  assert.equal(numericParser.parseTree(numeric.holdout).tree.attributes[request.attribute], numeric.value);
  assertCommandResults();
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-INFERENCE-CONCRETE-ATTRIBUTES', suffix: 'behavior',
    fixtureId: 'grammar-attribute-inference', fixtureFile,
    assertions: ['concreteAttributeObservationsInferNativeAction', 'reloadedActionAndLinkedEditExecuteOnHoldout', 'ambiguousForgedAndUnsupportedTracesRemainExplicit', 'attributeInferenceCommandReturnsExecutableActionsAndAlternatives'],
    testName: context.name,
  });
});

function assertCommandResults() {
  const { grammar, request, unique, ambiguous, forged, unsupported, numeric } = fixture;
  for (const current of [unique, ambiguous, forged, unsupported, numeric]) {
    const inputs = new Map([['grammar.lino', current.grammar ?? grammar], ['observations.lino', structuredDataToLino({ request, observations: current.observations })]]);
    const outcome = runDynamicLanguageCommand(['infer-attribute', 'observations.lino', '--grammar', 'grammar.lino'], {
      readFile: path => inputs.get(path), readBytes: () => assert.fail('attribute inference reads native text only'),
    });
    if (current === forged) {
      assert.equal(outcome.exitCode, 2);
      assert.ok(outcome.stderr.includes(forged.diagnostic));
      continue;
    }
    assert.equal(outcome.exitCode, 0, outcome.stderr);
    const report = readStructuredDataLino(outcome.stdout);
    if (current.candidate) {
      assert.deepEqual(report.candidates, [current.candidate]);
      assert.equal(report.diagnostic, null);
      const holdout = compileGrammar(parseGrammarLinks(report.grammar)).parseTree(current.holdout);
      assert.equal(holdout.tree.attributes[request.attribute], current.value);
    } else {
      assert.deepEqual(report.candidates, current.candidates ?? []);
      assert.equal(report.grammar, null);
      assert.ok(report.diagnostic.includes(current.diagnostic));
    }
  }
}
