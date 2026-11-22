// Future controls, intentionally not executed during the complete draft phase.
import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { inferRegularGrammar, readRegularGrammarInferenceLinks, acceptInferredGrammarSymbols } from '../src/grammar-inference.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { createDynamicMetaLanguage } from '../src/dynamic-meta-language.js';
import { inferGrammarScanner, inferGrammarScannerClasses, inferGrammarScannerModeStack } from '../src/grammar-scanner-inference.js';
import { readRefinedRegularGrammarInferenceLinks, refineRegularGrammarInference } from '../src/grammar-inference-refinement.js';
import { inferStructuralGrammar, readStructuralGrammarInferenceLinks } from '../src/index.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { createGrammarParser as compileGrammar, renderSyntaxTree } from '../src/grammar-runtime.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtures = readProjectData(new URL('../../parity/fixtures/regular-grammar-inference.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + '; ' + fixtures.versionRanges.JavaScript);

for (const fixture of fixtures.cases) test(fixture.name + ': observations, independent controls and both complete codecs', () => {
  const original = inferRegularGrammar(fixture.positives, fixture.negatives, fixture.options);
  const text = LinkNetwork.fromLino(original.network.toLino());
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(original.network));
  for (const model of [original, readRegularGrammarInferenceLinks(text, original.root), readRegularGrammarInferenceLinks(binary, original.root)]) {
    for (const symbols of fixture.positives) assert.equal(acceptInferredGrammarSymbols(model, symbols), true);
    for (const symbols of fixture.negatives) assert.equal(acceptInferredGrammarSymbols(model, symbols), false);
    for (const control of fixture.controls) assert.equal(acceptInferredGrammarSymbols(model, control.symbols), control.accepted);
    assert.deepEqual(model.table, original.table);
    assert.deepEqual(model.grammar.normalized(), original.grammar.normalized());
  }
  const language = createDynamicMetaLanguage([original.source]);
  // The parser's normal concrete result retains every accepted source byte.
  const parsed = language.parse(original.source.id, fixture.positives[1].join(''));
  assert.equal(parsed.projection, 'concrete syntax');
  assert.ok(parsed.tree);
  const renderTree = tree => tree.type === 'node' ? tree.children.map(renderTree).join('') : tree.text;
  assert.equal(renderTree(parsed.tree), fixture.positives[1].join(''));
  assert.ok(parsed.concrete);
  assert.ok(parsed.archive);
  const corrupted = { ...original, table: [...original.table] };
  corrupted.table[1] = original.table.length;
  assert.throws(() => acceptInferredGrammarSymbols(corrupted, ['a']), /invalid model table/u);
});

for (const fixture of fixtures.refusals) test(fixture.name + ' remains an explicit refusal', () => {
  assert.throws(() => inferRegularGrammar(fixture.positives, fixture.negatives, fixture.options), error => error.message.includes(fixture.reason));
});

test('observed delimiter productions recur, preserve source bytes and refuse contradictory merges', context => {
  const fixture = fixtures.structural;
  const options = {delimiters: fixture.delimiters};
  const original = inferStructuralGrammar(fixture.positives, fixture.negatives, options);
  const text = LinkNetwork.fromLino(original.network.toLino());
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(original.network));
  const renderTree = tree => tree.type === 'node' ? tree.children.map(renderTree).join('') : tree.text;
  for (const network of [text, binary]) {
    const model = readStructuralGrammarInferenceLinks(network, original.root);
    assert.deepEqual(model.productions, fixture.productions);
    assert.deepEqual(model.grammar.normalized(), original.grammar.normalized());
    const library = createDynamicMetaLanguage([model.source]);
    const parser = compileGrammar(model.grammar);
    for (const input of fixture.accepted) {
      const result = library.parse(model.source.id, input);
      assert.equal(result.projection, 'concrete syntax');
      assert.equal(renderTree(result.tree), input);
    }
    for (const input of fixture.rejected) assert.equal(parser.parseTree(input).ok, false, input);
  }
  assert.throws(() => inferStructuralGrammar(fixture.positives, [fixture.counterexample], options), /merge accepts a negative/u);
  assert.throws(() => inferStructuralGrammar([...fixture.positives, fixture.unbalanced], [], options), /unbalanced delimiters/u);
  assert.throws(() => inferStructuralGrammar(fixture.positives, fixture.negatives, {...options, depthLimit: 1}), /nesting limit/u);
  assert.throws(() => inferStructuralGrammar(fixture.positives, fixture.negatives, {...options, productionLimit: 1}), /production limit/u);
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-INFERENCE-DELIMITER-PRODUCTIONS', suffix: 'behavior',
    fixtureId: 'grammar-inference-delimiter-productions', fixtureFile: 'parity/fixtures/regular-grammar-inference.lino',
    assertions: ['recursiveDelimiterProductionsAndConcreteBytesMatch', 'recursiveDelimiterArchiveReplays', 'invalidAndUnboundedDelimiterMergesAreRefused'],
    testName: context.name,
  });
});

test('scanner trace inference emits executable mode transitions and preserves source bytes', () => {
  const fixture = fixtures.scanner;
  const inferred = inferGrammarScanner(fixture.observations);
  assert.deepEqual(inferred.modes, ['inside']);
  assert.ok(inferred.listing.indexOf('(valid long)') < inferred.listing.indexOf('(valid short)'));
  assert.equal(inferred.listing.match(/\(valid content\)/gu).length, 1);
  const listing = '(grammar (start source))\n' + inferred.listing + fixture.syntax;
  const loaded = parseGrammarLinks(renderGrammarLinks(parseGrammarLinks(listing)));
  const parser = compileGrammar(loaded);
  const bytes = tree => tree.type === 'node' ? tree.children.map(bytes).join('') : tree.text;
  for (const { input, tree } of fixture.accepted) {
    const result = parser.parseTree(input);
    assert.equal(result.ok, true, input);
    assert.equal(renderSyntaxTree(result.tree), tree);
    assert.equal(bytes(result.tree), input);
  }
  for (const input of fixture.rejected) assert.equal(parser.parseTree(input).ok, false, input);
  assert.throws(() => inferGrammarScanner([...fixture.observations, fixture.conflicting]), /unsupported context sensitivity/u);
});

test('scanner character classes preserve gaps, literal precedence and archive refinement', () => {
  const fixture = fixtures.scannerCharacterClasses;
  const inferred = inferGrammarScannerClasses(fixture.observations);
  assert.ok(inferred.listing.includes(fixture.matcher));
  assert.ok(inferred.listing.lastIndexOf('(valid word)') < inferred.listing.indexOf('(valid member)'));
  const listing = '(grammar (start source))\n' + inferred.listing + fixture.syntax;
  const parser = compileGrammar(parseGrammarLinks(renderGrammarLinks(parseGrammarLinks(listing))));
  for (const {input, tree} of fixture.accepted) {
    const result = parser.parseTree(input);
    assert.equal(result.ok, true, input);
    assert.equal(renderSyntaxTree(result.tree), tree);
  }
  for (const input of fixture.rejected) assert.equal(parser.parseTree(input).ok, false, input);
  assert.throws(() => inferRegularGrammar([], [], {scannerCharacterClasses: true}), /require scanner observations/u);
  const original = inferRegularGrammar([['member'], ['word']], [[]], {
    scannerObservations: fixture.observations, scannerCharacterClasses: true,
  });
  const restored = readRegularGrammarInferenceLinks(LinkNetwork.fromLino(original.network.toLino()), original.root);
  assert.deepEqual(restored.grammar.normalized(), original.grammar.normalized());
  const refined = refineRegularGrammarInference(restored.network, restored.root, [['word', 'member']]);
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(refined.network));
  const reloaded = readRefinedRegularGrammarInferenceLinks(binary, refined.root);
  assert.deepEqual(reloaded.grammar.normalized(), refined.grammar.normalized());
  assert.ok(acceptInferredGrammarSymbols(reloaded, ['word', 'member']));
});

test('scanner mode stacks restore each actual parent and replay the selected archive', context => {
  const fixture = fixtures.scannerModeStack;
  for (const classes of [false, true]) {
    const inferred = inferGrammarScannerModeStack(fixture.observations, undefined, classes);
    for (const operation of fixture.operations) assert.ok(inferred.listing.includes(operation));
    assert.equal(inferred.listing.includes(fixture.matcher), classes);
    assert.equal(inferred.listing.match(/\(popMode\)/gu).length, 1);
    const listing = '(grammar (start source))\n' + inferred.listing + fixture.syntax;
    const parser = compileGrammar(parseGrammarLinks(renderGrammarLinks(parseGrammarLinks(listing))));
    for (const {input, tree} of fixture.accepted) {
      const result = parser.parseTree(input);
      assert.equal(result.ok, true, input);
      assert.equal(renderSyntaxTree(result.tree), tree);
    }
    for (const input of fixture.rejected) assert.equal(parser.parseTree(input).ok, false, input);
    const original = inferRegularGrammar([['begin', 'word', 'end', 'suffix']], [[]], {
      scannerModeStackObservations: fixture.observations, scannerCharacterClasses: classes,
    });
    const restored = readRegularGrammarInferenceLinks(LinkNetwork.fromLino(original.network.toLino()), original.root);
    assert.deepEqual(restored.grammar.normalized(), original.grammar.normalized());
    const addition = ['alternate', 'begin', 'member', 'end', 'suffix'];
    const refined = refineRegularGrammarInference(restored.network, restored.root, [addition]);
    const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(refined.network));
    const reloaded = readRefinedRegularGrammarInferenceLinks(binary, refined.root);
    assert.deepEqual(reloaded.grammar.normalized(), refined.grammar.normalized());
    assert.equal(acceptInferredGrammarSymbols(reloaded, addition), true);
  }
  assert.throws(() => inferGrammarScannerModeStack([...fixture.observations, fixture.conflicting]), /unsupported context sensitivity/u);
  assert.throws(() => inferGrammarScannerModeStack([fixture.unsupported]), /at most one native scanner mode operation/u);
  assert.throws(() => inferGrammarScannerModeStack([{...fixture.unsupported, after: []}]), /bounded nonempty/u);
  assert.throws(() => inferRegularGrammar([], [], {scannerObservations: [], scannerModeStackObservations: []}), /one scanner observation format/u);
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-INFERENCE-SCANNER-MODE-STACK', suffix: 'behavior',
    fixtureId: 'regular-grammar-inference-scanner-mode-stack', fixtureFile: 'parity/fixtures/regular-grammar-inference.lino',
    assertions: ['scannerModeStackTreesAndRejectionsMatch', 'scannerModeStackArchiveReplaysAndRefines', 'unsupportedScannerModeStackEffectsAreRefused'],
    testName: context.name,
  });
});
