import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { compileGrammar, createDynamicMetaLanguage, DynamicMetaLanguage, inferGrammarAttributeAction,
  metaLanguageArchiveToBinary, readMetaLanguageArchiveBinary, runDynamicLanguageCommand } from '../src/index.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';
import { encodeFormalConceptDefinition, proveFormalConceptEquivalence,
  verifyFormalConceptEquivalence } from '../src/formal-concept-definition.js';
import { javaScriptTranslationOutput, rustTranslationOutput } from './support/translation-behavior.js';
import { nativeRustChecksEnabled } from './support/native-rust-checks.js';

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

test('installed inferred actions translate after archive reload and invalidate old semantic certificates', () => {
  const { grammar, numeric, request, unique } = fixture;
  const selection = { path: [], kind: request.rule, attribute: request.attribute };
  const source = { id: 'observed words', language: 'Observed words', edition: '1' };
  const session = createDynamicMetaLanguage([]);
  const restored = (language) => new DynamicMetaLanguage(readMetaLanguageArchiveBinary(metaLanguageArchiveToBinary(language.archive)));
  const translated = (language, input, expected) => {
    const parsed = language.parse(source.id, input);
    assert.ok(parsed.ok);
    assert.equal(parsed.tree.attributes[request.attribute], expected);
    const javascript = language.translateAttributes(source.id, input, 'JavaScript', [selection, selection]);
    const rust = language.translateAttributes(source.id, input, 'Rust', [selection, selection]);
    assert.equal(javascript.parsed.tree.attributes[request.attribute], expected);
    assert.equal(javascript.output.language, 'JavaScript');
    assert.equal(rust.output.language, 'Rust');
    const lines = String(expected) + '\n' + String(expected) + '\n';
    const archive = metaLanguageArchiveToBinary(language.archive);
    const selected = structuredDataToLino([selection, selection]);
    const bytes = new Map([['installed.lino.bin', archive], ['input', new TextEncoder().encode(input)]]);
    for (const { code, language: target } of [javascript.output, rust.output]) {
      const command = runDynamicLanguageCommand(
        ['translate-attributes', 'installed.lino.bin', '--input-binary', '--source', source.id,
          '--input', 'input', '--target', target, '--selections', 'selections.lino'],
        { readFile: name => { assert.equal(name, 'selections.lino'); return selected; },
          readBytes: name => { assert.ok(bytes.has(name)); return bytes.get(name); } },
      );
      assert.deepEqual(command, { exitCode: 0, stdout: code, stderr: '' });
    }
    assert.equal(javaScriptTranslationOutput(javascript.output.code), lines);
    if (nativeRustChecksEnabled()) assert.equal(rustTranslationOutput(rust.output.code).replaceAll('\r\n', '\n'), lines);
    return encodeFormalConceptDefinition(javascript.links);
  };
  const inferred = inferGrammarAttributeAction(parseGrammarLinks(grammar), request, unique.observations);
  const installed = restored(session.extend([{ ...source, grammar: inferred.grammar }]));
  const first = translated(installed, unique.holdout, unique.value);
  const certificate = proveFormalConceptEquivalence(first, first);
  assert.ok(certificate);
  assert.equal(verifyFormalConceptEquivalence(first, first, certificate), true);

  const listing = renderGrammarLinks(installed.sources[0].grammar);
  const changedListing = listing.replace(unique.candidate, unique.mutation);
  assert.notEqual(changedListing, listing);
  const changed = restored(installed.extend([{ ...source, grammar: parseGrammarLinks(changedListing) }], { replace: true }));
  const second = translated(changed, unique.holdout, unique.mutatedValue);
  assert.equal(verifyFormalConceptEquivalence(first, second, certificate), false);
  assert.equal(proveFormalConceptEquivalence(first, second), null);
  translated(installed, unique.holdout, unique.value);

  const inferredNumeric = inferGrammarAttributeAction(parseGrammarLinks(numeric.grammar), request, numeric.observations);
  const numbers = restored(changed.extend([{ ...source, grammar: inferredNumeric.grammar }], { replace: true }));
  translated(numbers, numeric.holdout, numeric.value);
  assert.throws(() => installed.translateAttributes(source.id, unique.negative, 'JavaScript', [selection]), /accepted unambiguous/u);
  assert.throws(() => installed.translateAttributes(source.id, unique.holdout, 'Rust', [{ ...selection, path: [99] }]), /concrete attributed/u);
});
