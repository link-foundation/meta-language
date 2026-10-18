// Finite composition controls over shipped grammars. Mapping validation is not
// a proof that a CST belongs to a grammar; standalone CST comparisons are separate.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import test from 'node:test';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks, renderGrammarLinks, percentEncodeLinksText } from '../src/grammar-links.js';
import { checkGrammarSelectionMappedTree } from '../src/grammar-runtime.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { LANGUAGE_CATALOG } from '../src/language-catalog.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks, readStructuredDataLino,
  structuredDataToLino } from '../src/structured-data-links.js';
import { readProjectData, readProjectFileBytes } from '../scripts/project-data.mjs';
import { originalGrammarSelectionCapture } from '../scripts/grammar-selection-capture-profile.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';
import { grammarSelectionMappingObservationSections } from './support/grammar-selection-mapping-cases.js';
import { nativeRows } from '../scripts/native-grammar-rows.mjs';
import { Executor, MemoryBudgetReached, StepLimitReached, NestingTooDeep } from '../src/grammar-runtime/executor.js';

const encoder = new TextEncoder();
const decoder = new TextDecoder('utf-8', { fatal: true });
const byteLength = source => encoder.encode(source).length;
const fixtureData = name => readProjectData(new URL(`../../parity/${name}`, import.meta.url));
const originalInventory = fixtureData('language-grammar-inventory.lino');
const originalPython = originalInventory.languages.find(entry => entry.name === 'Python');
assert.ok(originalPython, 'the unchanged inventory supplies the real Python source');
const oracleRows = fixtureData('fixtures/default-cst-expected.lino');
const currentRows = fixtureData('fixtures/native-default-cst-expected.lino');
const expectedPython = currentRows.languages.Python ?? oracleRows.languages.Python;
assert.ok(expectedPython, 'independent existing Python CST rows remain the oracle');
const mappedFixturePath = 'parity/fixtures/grammar-selection-json-string.lino';
const mappedFixture = fixtureData('fixtures/grammar-selection-json-string.lino');
assert.deepEqual(mappedFixture.observations, grammarSelectionMappingObservationSections());
assert.deepEqual(mappedFixture.inputs.python, { source: originalPython.source,
  recoverySource: originalPython.recoverySource, positiveRows: expectedPython.positive });
const mappedCapturePaths = [
  ['inventory', 'parity/language-grammar-inventory.lino'],
  ['originalConcreteOracle', 'parity/fixtures/default-cst-expected.lino'],
  ['nativeConcreteOracle', 'parity/fixtures/native-default-cst-expected.lino'],
  ['javascriptJsonGrammar', 'js/src/data/native-grammars/json.lino'],
  ['javascriptPythonGrammar', 'js/src/data/native-grammars/python.lino'],
  ['rustJsonGrammar', 'rust/src/data/native-grammars/json.lino'],
  ['rustPythonGrammar', 'rust/src/data/native-grammars/python.lino'],
  ['javascriptTests', 'js/tests/grammar-selection-json-string.test.js'],
  ['rustTests', 'rust/tests/unit/grammar_recursive_selection.rs'],
  ['rustTestTransport', 'rust/tests/unit/grammar_recursive_selection_mapping.rs'],
  ['fixtureAuthor', 'js/tests/support/grammar-selection-mapping-cases.js'],
  ['fixtureGenerator', 'js/scripts/generate-grammar-selection-mapping-fixtures.mjs'],
];
assert.deepEqual(mappedFixture.captures.map(({ id, path }) => [id, path]), mappedCapturePaths);
for (const capture of mappedFixture.captures) {
  const bytes = originalGrammarSelectionCapture(capture.path,
    readProjectFileBytes(new URL(`../../${capture.path}`, import.meta.url)));
  assert.equal(bytes.length, capture.bytes, capture.id + ': exact original capture byte count');
  assert.equal(createHash('sha256').update(bytes).digest('hex'), capture.sha256,
    capture.id + ': exact source bytes captured after the test hooks freeze');
}

function observeMappedProfile(section, assertions, testName) {
  const cell = mappedFixture.observations[section];
  assert.ok(assertions.length > 0 && assertions.every(assertion => cell.assertions.includes(assertion)));
  recordIssue195Observations({ requirementId: cell.requirementId, suffix: cell.suffix,
    fixtureId: cell.fixtureIds.javascript, fixtureFile: mappedFixturePath, assertions, testName });
}
const shippedJsonListing = nativeGrammarText('native-json');
const shippedPythonListing = nativeGrammarText('native-python');
const shippedPython = parseGrammarLinks(shippedPythonListing);
const standalonePython = compileGrammar(shippedPython);
const standaloneJson = compileGrammar(parseGrammarLinks(shippedJsonListing));

function mappedJsonHost(names, mode = 'any-grammar', mappings = names.map(() => 'json-string'), prefix = '') {
  const grammar = parseGrammarLinks(shippedJsonListing);
  const originalStart = grammar.startRule().name;
  const root = 'mapped_document';
  assert.equal(grammar.rules.has(root), false, 'the authored wrapper must not hide a shipped rule');
  const selection = { kind: mode, item: { kind: 'ref', name: originalStart },
    items: names.map((name, index) => ({ kind: 'grammar-candidate', name, mapping: mappings[index] })) };
  grammar.rules.set(root, Object.freeze({ name: root, kind: 'normal',
    expression: prefix ? { kind: 'seq', items: [{ kind: 'literal', value: prefix }, selection] } : selection,
    sourceNames: [{ source: 'authored', name: root }] }));
  grammar.start = root;
  return grammar;
}

function literalChild(source) {
  return parseGrammarLinks(`(grammar (start decoded))\n(rule decoded normal (literal ${percentEncodeLinksText(source)}))\n`);
}

function mappedEmbeddings(tree, found = []) {
  if (tree.type === 'embed' && tree.selection?.mapping === 'json-string') found.push(tree);
  for (const child of tree.children ?? []) mappedEmbeddings(child, found);
  for (const child of tree.sourceChildren ?? []) mappedEmbeddings(child, found);
  if (tree.root) mappedEmbeddings(tree.root, found);
  for (const witness of tree.selection?.alternatives ?? []) mappedEmbeddings(witness.root, found);
  return found;
}

function physicalBytes(tree) {
  if (tree.type === 'missing') return [];
  if (tree.type === 'token' || tree.type === 'error') return tree.text === null
    ? [...(tree.hex.match(/../gu) ?? [])].map(word => Number.parseInt(word, 16)) : [...encoder.encode(tree.text)];
  const children = tree.sourceChildren ?? tree.children ?? (tree.root ? [tree.root] : []);
  return children.flatMap(physicalBytes);
}

function assertConcreteLeaves(tree, source) {
  const bytes = encoder.encode(source);
  const visit = node => {
    if (node.type === 'token' || node.type === 'error') {
      assert.deepEqual(physicalBytes(node), [...bytes.subarray(node.start, node.end)], 'leaf text belongs to its actual source plane');
    }
    for (const child of node.sourceChildren ?? node.children ?? (node.root ? [node.root] : [])) visit(child);
  };
  visit(tree);
  assert.deepEqual(physicalBytes(tree), [...bytes], 'physical reconstruction owns every byte exactly once');
}

function grammarDocument(grammar) {
  // Preserve every own source field. The existing loader accepts rules arrays;
  // normalized() intentionally omits some source/concept provenance.
  return Object.fromEntries(Object.entries(grammar).map(([key, value]) =>
    [key, key === 'rules' ? [...value.values()] : value]));
}

function storedVariants(value) {
  const encoded = encodeStructuredDataLinks(value);
  const text = encoded.network.toLino();
  const network = LinkNetwork.fromLino(text);
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(network));
  assert.equal(binary.toLino(), network.toLino(), 'binary transport keeps every actual graph identity and metadata field');
  return [['text', readStructuredDataLino(text)], ['binary', decodeStructuredDataLinks(binary, encoded.root)]];
}

test('a mapped shipped JSON string retains its full physical CST and the original Python concrete tree', () => {
  const source = JSON.stringify(originalPython.source);
  const host = mappedJsonHost(['Python']);
  const parser = compileGrammar(host, { resolveGrammar: name => name === 'Python' ? shippedPython : undefined });
  const outcome = parser.parseTree(source);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  const directJson = standaloneJson.parseTree(source), directPython = standalonePython.parseTree(originalPython.source);
  assert.equal(directJson.ok, true, JSON.stringify(directJson.rejection));
  assert.equal(directPython.ok, true, JSON.stringify(directPython.rejection));
  const [embedded] = mappedEmbeddings(outcome.tree);
  assert.ok(embedded);
  assert.equal(mappedEmbeddings(outcome.tree).length, 1);
  assert.deepEqual(embedded.sourceChildren, [directJson.tree], 'the physical host is the real complete JSON grammar CST');
  assert.deepEqual(embedded.root, directPython.tree, 'the decoded child is the real complete standalone Python CST');
  assert.deepEqual(nativeRows(embedded.root, originalPython.source,
    LANGUAGE_CATALOG.nativeGrammars['native-python']), expectedPython.positive, 'all independently pinned Python CST rows remain');
  assertConcreteLeaves(outcome.tree, source);
  assertConcreteLeaves(embedded.root, originalPython.source);
  assert.equal(embedded.start, 0);
  assert.equal(embedded.end, byteLength(source));
  assert.equal(embedded.root.start, 0, 'decoded root positions are not physical host positions');
  assert.equal(embedded.root.end, byteLength(originalPython.source));
  assert.equal(outcome.sourcePlanes[0].byteLength, byteLength(source));
  assert.equal(outcome.sourcePlanes[1].byteLength, byteLength(originalPython.source));
  const bundle = { tree: outcome.tree, sourcePlanes: outcome.sourcePlanes };
  assert.equal(checkGrammarSelectionMappedTree(bundle), bundle, 'mapping correspondence is checked separately from CST correctness');
  assert.throws(() => parser.parse(source), /parseTree.*sourcePlanes/u, 'tree-only parsing must not discard the mapped registry');
  assert.deepEqual(standaloneJson.parseTree(source), directJson, 'ordinary JSON parsing is unchanged');
  observeMappedProfile('boundaries', ["shippedJsonPythonConcreteTreesRemainExact"],
    'a mapped shipped JSON string retains its full physical CST and the original Python concrete tree');
});

test('independent JSON escape intervals retain UTF-8 source planes and exact quote ownership', () => {
  const raw = '"a\\n\\u03C0\\uD83D\\uDE00"';
  const decoded = 'a\nπ😀';
  assert.equal(byteLength(raw), 23);
  assert.equal(byteLength(decoded), 8);
  assert.equal(raw, mappedFixture.inputs.escape.raw);
  assert.equal(decoded, mappedFixture.inputs.escape.decoded);
  assert.deepEqual([...encoder.encode(raw)], mappedFixture.inputs.escape.physicalBytes);
  assert.deepEqual([...encoder.encode(decoded)], mappedFixture.inputs.escape.decodedBytes);
  const intervals = [[0, 1, 0, 0], [1, 2, 0, 1], [2, 4, 1, 2], [4, 10, 2, 4],
    [10, 22, 4, 8], [22, 23, 8, 8]];
  assert.deepEqual(intervals, mappedFixture.inputs.escape.intervals);
  assert.deepEqual(['', 'λ<'], mappedFixture.inputs.escape.prefixes);
  for (const prefix of ['', 'λ<']) {
    const grammar = mappedJsonHost(['decoded'], 'any-grammar', ['json-string'], prefix);
    const child = literalChild(decoded);
    const outcome = compileGrammar(grammar, { resolveGrammar: () => child }).parseTree(prefix + raw);
    assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
    const [embedded] = mappedEmbeddings(outcome.tree), mapping = embedded.selection.sourceMapping;
    const offset = byteLength(prefix);
    assert.deepEqual(mapping.intervals, intervals.map(([a, b, c, d]) => [a + offset, b + offset, c, d]));
    assert.deepEqual(outcome.sourcePlanes, [
      { kind: 'grammar-source', identity: 0, encoding: 'UTF-8', byteLength: offset + 23, derivation: null },
      { kind: 'grammar-source', identity: 1, encoding: 'UTF-8', byteLength: 8,
        derivation: { operation: 'json-string', input: 0, start: offset, end: offset + 23 } },
    ]);
    assert.deepEqual({ input: mapping.input, output: mapping.output }, { input: 0, output: 1 });
    assertConcreteLeaves(outcome.tree, prefix + raw);
    assertConcreteLeaves(embedded.root, decoded);
    assert.equal(embedded.root.start, 0);
    assert.equal(embedded.root.end, 8);
    assert.equal(checkGrammarSelectionMappedTree({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes }).tree, outcome.tree);
  }
  for (const [raw, decoded] of [
    ['""', ''], ['"é😀"', 'é😀'], ['"\\\"\\\\\\/\\b\\f\\n\\r\\t"', '"\\/\b\f\n\r\t'],
  ]) {
    const child = literalChild(decoded);
    const outcome = compileGrammar(mappedJsonHost(['decoded']), { resolveGrammar: () => child }).parseTree(raw);
    assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
    assertConcreteLeaves(outcome.tree, raw);
    assertConcreteLeaves(mappedEmbeddings(outcome.tree)[0].root, decoded);
    checkGrammarSelectionMappedTree({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes });
  }
  observeMappedProfile('boundaries', ["jsonStringUtf8IntervalsAndQuoteOwnershipExact"],
    'independent JSON escape intervals retain UTF-8 source planes and exact quote ownership');
});

test('nested mapped registries survive inspectable text and binary archives and source-free shipped grammar replay', () => {
  const middle = mappedJsonHost(['Python']);
  const outer = mappedJsonHost(['Middle']);
  const grammars = new Map([['outer', outer], ['Middle', middle], ['Python', shippedPython]]);
  const innerSource = JSON.stringify(originalPython.source), source = JSON.stringify(innerSource);
  const outcome = compileGrammar(outer, { resolveGrammar: name => grammars.get(name) }).parseTree(source);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  const [first, second] = mappedEmbeddings(outcome.tree);
  assert.ok(first && second);
  assert.equal(mappedEmbeddings(outcome.tree).length, 2);
  assert.deepEqual(outcome.sourcePlanes.map(row => row.identity), [0, 1, 2]);
  assert.deepEqual(outcome.sourcePlanes.map(row => row.derivation), [null,
    { operation: 'json-string', input: 0, start: 0, end: byteLength(source) },
    { operation: 'json-string', input: 1, start: 0, end: byteLength(innerSource) }]);
  assert.equal(first.selection.sourceMapping.output, second.selection.sourceMapping.input, 'nested references resolve through one owned table');
  assert.equal(second.selection.sourceMapping.output, 2);
  assert.deepEqual(first.sourceChildren, [standaloneJson.parseTree(source).tree]);
  assert.deepEqual(second.sourceChildren, [standaloneJson.parseTree(innerSource).tree]);
  assert.deepEqual(second.root, standalonePython.parseTree(originalPython.source).tree);
  assertConcreteLeaves(outcome.tree, source);
  assertConcreteLeaves(first.root, innerSource);
  assertConcreteLeaves(second.root, originalPython.source);
  for (const grammar of grammars.values()) {
    const listing = renderGrammarLinks(grammar);
    assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing, 'all original grammar source/provenance links roundtrip');
  }
  const archive = { grammars: [...grammars].map(([name, grammar]) => ({ name, document: grammarDocument(grammar) })), outcome };
  for (const [encoding, restored] of storedVariants(archive)) {
    assert.deepEqual(restored, archive, encoding + ': the entire inspectable grammar and CST registry bundle survives');
    checkGrammarSelectionMappedTree({ tree: restored.outcome.tree, sourcePlanes: restored.outcome.sourcePlanes });
    const decodedGrammars = new Map(restored.grammars.map(entry => [entry.name, entry.document]));
    const replay = compileGrammar(decodedGrammars.get('outer'), { resolveGrammar: name => decodedGrammars.get(name) }).parseTree(source);
    assert.deepEqual(replay, restored.outcome, encoding + ': recompile only actual source-free grammar documents');
    assert.equal(mappedEmbeddings(replay.tree)[0].selection.sourceMapping.output,
      mappedEmbeddings(replay.tree)[1].selection.sourceMapping.input);
    assert.equal(decoder.decode(Uint8Array.from(physicalBytes(replay.tree))), source);
  }
  observeMappedProfile('boundaries', ["mappedSourceRegistryAndGrammarReplayTextAndBinary"],
    'nested mapped registries survive inspectable text and binary archives and source-free shipped grammar replay');
  observeMappedProfile('recursion', ["nestedMappedSourcesRetainActualDerivationOrder"],
    'nested mapped registries survive inspectable text and binary archives and source-free shipped grammar replay');
});

function mappingBundle(outcome) {
  return { tree: outcome.tree, sourcePlanes: outcome.sourcePlanes };
}

function firstTextLeaf(tree) {
  if ((tree.type === 'token' || tree.type === 'error') && typeof tree.text === 'string' && tree.text) return tree;
  for (const child of tree.sourceChildren ?? tree.children ?? (tree.root ? [tree.root] : [])) {
    const found = firstTextLeaf(child);
    if (found) return found;
  }
  return null;
}

test('mapped any and all retain real witnesses while mixed lexical and non-scalar profiles refuse honestly', () => {
  const source = JSON.stringify(originalPython.source);
  const resolver = name => name === 'Python' || name === 'PythonTwin' ? shippedPython
    : name === 'JSON' ? parseGrammarLinks(shippedJsonListing) : undefined;
  for (const mode of ['any-grammar', 'all-grammar']) {
    const parser = compileGrammar(mappedJsonHost(['Python', 'PythonTwin'], mode), { resolveGrammar: resolver });
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
    const [embedded] = mappedEmbeddings(outcome.tree);
    assert.deepEqual(embedded.selection.candidates.map(item => [item.language, item.relation]),
      [['Python', 'accepted'], ['PythonTwin', 'accepted']]);
    assert.deepEqual(embedded.selection.alternatives.map(item => item.language), ['PythonTwin']);
    assert.deepEqual(embedded.selection.alternatives[0].root, embedded.root, 'parallel witnesses retain the actual complete child CST');
    assert.equal(embedded.selection.provenance.rules[0].origin.split(':')[0], 'Python');
    assert.equal(embedded.selection.alternatives[0].provenance.rules[0].origin.split(':')[0], 'PythonTwin');
    assert.equal(embedded.selection.ambiguous, mode === 'any-grammar');
    assert.equal(outcome.sourcePlanes.length, 2, 'candidates share one explicit derived plane, not duplicate source output');
    assertConcreteLeaves(outcome.tree, source);
    checkGrammarSelectionMappedTree(mappingBundle(outcome));
    const strict = parser.parseTree(source, { ambiguity: 'reject' });
    assert.equal(strict.ok, mode === 'all-grammar');
    if (mode === 'any-grammar') assert.equal(strict.rejection.reason, 'ambiguity');
  }
  const siblings = mappedJsonHost(['Python']);
  const selection = siblings.startRule().expression;
  siblings.rules.set(siblings.start, Object.freeze({ ...siblings.startRule(),
    expression: { kind: 'seq', items: [selection, selection] } }));
  const firstProgram = 'x=1\n', secondProgram = 'x=2\n';
  assert.deepEqual({ firstProgram, secondProgram }, mappedFixture.inputs.selection.siblings);
  const siblingSource = JSON.stringify(firstProgram) + JSON.stringify(secondProgram);
  const siblingOutcome = compileGrammar(siblings, { resolveGrammar: resolver }).parseTree(siblingSource);
  assert.equal(siblingOutcome.ok, true, JSON.stringify(siblingOutcome.rejection));
  const [firstSibling, secondSibling] = mappedEmbeddings(siblingOutcome.tree);
  assert.deepEqual(firstSibling.root, standalonePython.parseTree(firstProgram).tree);
  assert.deepEqual(secondSibling.root, standalonePython.parseTree(secondProgram).tree);
  assert.notDeepEqual(firstSibling.root, secondSibling.root, 'equal local byte ranges never borrow another decoded source CST');
  assert.notEqual(firstSibling.selection.sourceMapping.output, secondSibling.selection.sourceMapping.output);
  assertConcreteLeaves(siblingOutcome.tree, siblingSource);
  checkGrammarSelectionMappedTree(mappingBundle(siblingOutcome));
  const any = compileGrammar(mappedJsonHost(['Python', 'JSON']), { resolveGrammar: resolver }).parseTree(source);
  assert.equal(any.ok, true, 'a rejected JSON candidate cannot poison an accepted Python witness');
  assert.deepEqual(mappedEmbeddings(any.tree)[0].selection.candidates.map(item => item.relation), ['accepted', 'rejected']);
  const all = compileGrammar(mappedJsonHost(['Python', 'JSON'], 'all-grammar'), { resolveGrammar: resolver }).parseTree(source);
  assert.equal(all.ok, false);
  assert.equal(all.rejection.reason, 'syntax');
  assert.equal(all.tree, null);
  assert.equal(all.sourcePlanes.length, 2, 'a completed derivation remains inspectable without inventing a successful CST');
  assert.throws(() => checkGrammarSelectionMappedTree(mappingBundle(all)), /mapped grammar CST: a complete physical root CST is required/u);
  const mixed = compileGrammar(mappedJsonHost(['Python', 'PythonTwin'], 'any-grammar', ['json-string', 'identity']),
    { resolveGrammar: resolver }).parseTree(source);
  assert.equal(mixed.rejection.reason, 'unsupportedEmbeddingMapping');
  assert.match(mixed.rejection.diagnostics[0].detail, /syntactic context and one common source mapping/u);
  const lexicalGrammar = mappedJsonHost(['Python']);
  lexicalGrammar.rules.set(lexicalGrammar.start, Object.freeze({ ...lexicalGrammar.startRule(), kind: 'token' }));
  const lexical = compileGrammar(lexicalGrammar, { resolveGrammar: resolver }).parseTree(source);
  assert.equal(lexical.ok, false);
  assert.equal(lexical.rejection.reason, 'unsupportedEmbeddingMapping');
  const unknown = compileGrammar(mappedJsonHost(['missing']), { resolveGrammar: resolver }).parseTree(source);
  assert.equal(unknown.rejection.reason, 'unknownGrammar');
  const unknownMapping = compileGrammar(mappedJsonHost(['Python'], 'any-grammar', ['quoted-escape']),
    { resolveGrammar: resolver }).parseTree(source);
  assert.equal(unknownMapping.rejection.reason, 'unsupportedEmbeddingMapping');
  for (const raw of ['"\\uD800"', '"\\uDC00"']) {
    assert.equal(standaloneJson.parseTree(raw).ok, true, 'lone surrogate spelling is valid JSON syntax');
    const result = compileGrammar(mappedJsonHost(['Python']), { resolveGrammar: resolver }).parseTree(raw);
    assert.equal(result.ok, false);
    assert.equal(result.rejection.reason, 'unsupportedEmbeddingMapping', 'the finite scalar profile must not silently replace non-scalar contents');
  }
  for (const raw of ['"\\q"', '"\\u{41}"', '"\\u12"', '"line\nbreak"']) {
    // An authored literal deliberately admits malformed text to isolate the
    // mapping refusal. This is not a substitute for the shipped JSON grammar.
    const grammar = parseGrammarLinks(`(grammar (start document))\n(rule document normal (any-grammar (literal ${percentEncodeLinksText(raw)}) (grammar-candidate decoded json-string)))\n`);
    const result = compileGrammar(grammar, { resolveGrammar: () => literalChild('unused') }).parseTree(raw);
    assert.equal(result.ok, false);
    assert.equal(result.rejection.reason, 'invalidEmbeddingMapping');
  }
  observeMappedProfile('selection', ["jsonStringAnyAllCandidateWitnessesPreserved", "jsonStringUnsupportedAndNonScalarProfilesRefused", "jsonStringLexicalAndMalformedInputsRefused"],
    'mapped any and all retain real witnesses while mixed lexical and non-scalar profiles refuse honestly');
});

test('generic data decoding preserves forgeries while the mapped-source checker rejects false correspondences', () => {
  const raw = '"a\\n\\u03C0\\uD83D\\uDE00"', child = literalChild('a\nπ😀');
  const outcome = compileGrammar(mappedJsonHost(['decoded']), { resolveGrammar: () => child }).parseTree(raw);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  const genuine = mappingBundle(outcome);
  checkGrammarSelectionMappedTree(genuine);
  const edits = [
    ['changed physical container start', value => { value.tree.start = 1; }],
    ['changed physical container end', value => { value.tree.end -= 1; }],
    ['changed decoded container start', value => { mappedEmbeddings(value.tree)[0].root.start = 1; }],
    ['changed decoded container end', value => { mappedEmbeddings(value.tree)[0].root.end -= 1; }],
    ['changed mapped embed start', value => { mappedEmbeddings(value.tree)[0].start += 1; }],
    ['changed mapped embed end', value => { mappedEmbeddings(value.tree)[0].end -= 1; }],
    ['absent registry', value => { delete value.sourcePlanes; }],
    ['empty registry', value => { value.sourcePlanes = []; }],
    ['null root', value => { value.tree = null; }],
    ['unknown source kind', value => { value.sourcePlanes[0].kind = 'unowned-source'; }],
    ['foreign encoding', value => { value.sourcePlanes[0].encoding = 'UTF-16'; }],
    ['duplicate source identity', value => { value.sourcePlanes[1].identity = 0; }],
    ['fractional source identity', value => { value.sourcePlanes[1].identity = 0.5; }],
    ['changed root length', value => { value.sourcePlanes[0].byteLength += 1; }],
    ['changed derived length', value => { value.sourcePlanes[1].byteLength += 1; }],
    ['reordered definitions', value => { value.sourcePlanes.reverse(); }],
    ['missing derivation', value => { value.sourcePlanes[1].derivation = null; }],
    ['forward derivation', value => { value.sourcePlanes[1].derivation.input = 1; }],
    ['unknown derivation', value => { value.sourcePlanes[1].derivation.operation = 'identity'; }],
    ['different occurrence', value => { value.sourcePlanes[1].derivation.start = 1; }],
    ['unknown definition field', value => { value.sourcePlanes[0].opaqueBinding = true; }],
    ['unknown derivation field', value => { value.sourcePlanes[1].derivation.opaqueBinding = true; }],
    ['unknown mapping profile', value => { mappedEmbeddings(value.tree)[0].selection.mapping = 'guessed-string'; }],
    ['missing physical CST', value => { delete mappedEmbeddings(value.tree)[0].sourceChildren; }],
    ['wrong input plane', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.input = 1; }],
    ['foreign output plane', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.output = 2; }],
    ['unknown mapping operation', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.operation = 'identity'; }],
    ['unknown mapping field', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.opaqueBinding = true; }],
    ['interval gap', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.intervals[1][0] += 1; }],
    ['interval overlap', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.intervals[1][1] += 1; }],
    ['interval reversal', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.intervals.reverse(); }],
    ['missing closing quote', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.intervals.pop(); }],
    ['fractional interval', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.intervals[1][2] = 0.5; }],
    ['negative interval', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.intervals[0][0] = -1; }],
    ['fifth interval column', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.intervals[0].push(0); }],
    ['surrogate width', value => { mappedEmbeddings(value.tree)[0].selection.sourceMapping.intervals[4][3] -= 1; }],
    ['edited physical bytes', value => {
      const token = firstTextLeaf(mappedEmbeddings(value.tree)[0].sourceChildren[0]);
      token.text = '!' + token.text.slice(1);
    }],
    ['edited decoded bytes', value => {
      const token = firstTextLeaf(mappedEmbeddings(value.tree)[0].root);
      token.text = 'b' + token.text.slice(1);
    }],
  ];
  assert.deepEqual(edits.map(([label]) => label), mappedFixture.runtimeCases.javascript.serializableForgeries);
  for (const [label, edit] of edits) {
    const value = readStructuredDataLino(structuredDataToLino(genuine));
    edit(value);
    for (const [encoding, decoded] of storedVariants(value)) {
      assert.deepEqual(decoded, value, label + ': generic ' + encoding + ' decoding preserves the edited data');
      assert.throws(() => checkGrammarSelectionMappedTree(decoded), /mapped grammar CST:/u,
        label + ': only the explicit domain checker authenticates source correspondence');
    }
  }
  for (const [label, objectAt, field] of [
    ['bundle tree', value => value, 'tree'],
    ['tree endpoint', value => value.tree, 'end'],
    ['tree children', value => value.tree, 'children'],
    ['source definition', value => value.sourcePlanes, '0'],
    ['source length', value => value.sourcePlanes[0], 'byteLength'],
  ]) {
    const value = readStructuredDataLino(structuredDataToLino(genuine));
    const owner = objectAt(value), original = owner[field];
    let reads = 0;
    Object.defineProperty(owner, field, { enumerable: true, configurable: true,
      get() { reads += 1; return original; } });
    assert.throws(() => checkGrammarSelectionMappedTree(value), /^TypeError: mapped grammar CST:/u, label + ': accessors refuse before reads');
    assert.equal(reads, 0, label + ': validation cannot execute an unowned getter');
  }
  for (const [label, edit] of [
    ['foreign prototype', value => { Object.setPrototypeOf(value.tree, { inheritedBinding: true }); }],
    ['sparse concrete children', value => { delete value.tree.children[0]; }],
    ['cyclic concrete tree', value => { value.tree.children.push(value.tree); }],
    ['extra array field', value => { value.sourcePlanes.opaqueBinding = true; }],
  ]) {
    const value = readStructuredDataLino(structuredDataToLino(genuine));
    edit(value);
    assert.throws(() => checkGrammarSelectionMappedTree(value), /^TypeError: mapped grammar CST:/u,
      label + ': the direct host boundary refuses before source interpretation');
  }
  // This counterfactual preserves every leaf span and mapping relation, but
  // prefixes a real invalid UTF-8 byte outside the quoted region. It must not
  // authenticate a root definition falsely labeled UTF-8.
  const invalidUtf8 = readStructuredDataLino(structuredDataToLino(genuine));
  const shiftPhysical = node => {
    node.start += 1; node.end += 1;
    if (node.selection?.mapping === 'json-string') {
      node.selection.start += 1; node.selection.end += 1;
      for (const row of node.selection.sourceMapping.intervals) { row[0] += 1; row[1] += 1; }
    }
    for (const child of node.sourceChildren ?? node.children ?? (node.root ? [node.root] : [])) shiftPhysical(child);
  };
  shiftPhysical(invalidUtf8.tree);
  invalidUtf8.tree = { type: 'node', kind: 'physical_prefix', start: 0, end: byteLength(raw) + 1,
    children: [{ type: 'token', kind: null, start: 0, end: 1, text: null, hex: 'ff' }, invalidUtf8.tree] };
  invalidUtf8.sourcePlanes[0].byteLength += 1;
  invalidUtf8.sourcePlanes[1].derivation.start += 1;
  invalidUtf8.sourcePlanes[1].derivation.end += 1;
  for (const [encoding, decoded] of storedVariants(invalidUtf8)) {
    assert.deepEqual(decoded, invalidUtf8, encoding + ': invalid bytes remain inspectable');
    assert.throws(() => checkGrammarSelectionMappedTree(decoded), /^TypeError: mapped grammar CST:/u,
      'a new UTF-8 plane cannot silently include an invalid byte outside the mapping');
  }

  observeMappedProfile('boundaries', ["serializableMappingForgeriesRefusedAfterReload"],
    'generic data decoding preserves forgeries while the mapped-source checker rejects false correspondences');
  observeMappedProfile('selection', ["invalidUtf8RootCannotClaimUtf8SourcePlane"],
    'generic data decoding preserves forgeries while the mapped-source checker rejects false correspondences');
});

test('derived parses share existing budgets and retain explicit source-qualified cycle and syntax failures', () => {
  const middle = mappedJsonHost(['decoded']), outer = mappedJsonHost(['Middle']);
  const child = literalChild('x');
  const parser = compileGrammar(outer, { resolveGrammar: name => name === 'Middle' ? middle : child });
  const source = JSON.stringify(JSON.stringify('x'));
  assert.equal(source, mappedFixture.inputs.nesting.twoLevels);
  const originalStep = Executor.prototype.step, budgets = new Set(), memories = new Set(), sources = new Map();
  let outcome;
  Executor.prototype.step = function () {
    budgets.add(this.budget);
    memories.add(this.budget.memory);
    if (this.bytes instanceof Uint8Array && this.expectations) {
      if (!sources.has(this.bytes)) sources.set(this.bytes, new Set());
      sources.get(this.bytes).add(this.expectations);
    }
    return originalStep.call(this);
  };
  try { outcome = parser.parseTree(source); }
  finally { Executor.prototype.step = originalStep; }
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  assert.equal(budgets.size, 1, 'derived sources never restart the shared step budget');
  assert.equal(memories.size, 1, 'derived sources never restart memory accounting');
  assert.equal(sources.size, 3, 'root and two decoded buffers are concrete separate planes');
  const contexts = [...sources.values()];
  for (let first = 0; first < contexts.length; first += 1) for (let second = first + 1; second < contexts.length; second += 1) {
    assert.equal([...contexts[first]].some(value => contexts[second].has(value)), false,
      'scanner expectations from different source buffers must remain independent');
  }
  for (const [options, reason] of [[{ stepLimit: 0 }, 'stepLimit'], [{ memoryLimit: 0 }, 'memoryBudget'], [{ maxDepth: 0 }, 'nestingDepth']]) {
    const limited = parser.parseTree(source, options);
    assert.equal(limited.ok, false);
    assert.equal(limited.rejection.reason, reason);
    assert.equal(limited.rejection.limit, 0);
  }
  for (const [options, kind] of [[{ stepLimit: 0 }, StepLimitReached], [{ memoryLimit: 0 }, MemoryBudgetReached], [{ maxDepth: 0 }, NestingTooDeep]]) {
    assert.throws(() => checkGrammarSelectionMappedTree(mappingBundle(outcome), options), kind,
      'the validator reuses the exact existing resource limits and error types');
  }
  const cyclic = parseGrammarLinks('(grammar (start document))\n(rule document normal (any-grammar (literal x) (grammar-candidate cycle identity)))\n');
  const cycleResult = compileGrammar(mappedJsonHost(['cycle']), { resolveGrammar: () => cyclic }).parseTree('"x"');
  assert.equal(cycleResult.ok, false);
  assert.equal(cycleResult.rejection.reason, 'nonConsumingEmbeddingCycle');
  const cycle = cycleResult.rejection.diagnostics.find(item => item.reason === 'nonConsumingEmbeddingCycle');
  assert.equal(cycle.source, 1);
  assert.equal(cycle.sourceStart, 0);
  assert.equal(cycle.sourceEnd, 1);
  assert.equal(cycle.offsetSemantics, 'covering-source-spelling');
  assert.deepEqual(cycle.sourceRoute.at(-1), {
    output: 1, outputStart: 0, outputEnd: 1, input: 0, inputStart: 1, inputEnd: 2, displayAnchor: 1,
  });
  const invalidBytes = Uint8Array.from([255, 34, 120, 34]);
  const prefixedGrammar = mappedJsonHost(['decoded']);
  prefixedGrammar.rules.set(prefixedGrammar.start, Object.freeze({ ...prefixedGrammar.startRule(),
    expression: { kind: 'seq', items: [{ kind: 'any' }, prefixedGrammar.startRule().expression] } }));
  const invalidPlane = compileGrammar(prefixedGrammar, { resolveGrammar: () => child }).parseTree(invalidBytes);
  assert.equal(invalidPlane.ok, false);
  assert.equal(invalidPlane.rejection.reason, 'unsupportedEmbeddingMapping');
  assert.equal(Object.hasOwn(invalidPlane, 'sourcePlanes'), false, 'no invalid physical plane is declared UTF-8');
  const identityGrammar = mappedJsonHost(['raw'], 'any-grammar', ['identity']);
  identityGrammar.rules.set(identityGrammar.start, Object.freeze({ ...identityGrammar.startRule(),
    expression: { kind: 'seq', items: [{ kind: 'any' }, identityGrammar.startRule().expression] } }));
  const identity = compileGrammar(identityGrammar, { resolveGrammar: () => literalChild('"x"') }).parseTree(invalidBytes);
  assert.equal(identity.ok, true, 'existing identity parsing still accepts and preserves byte input');
  assert.deepEqual(Object.keys(identity), ['ok', 'tree', 'ambiguities', 'rejection']);
  assert.deepEqual(physicalBytes(identity.tree), [...invalidBytes]);
  const invalidSource = JSON.stringify(originalPython.recoverySource);
  assert.equal(standalonePython.parseTree(originalPython.recoverySource).ok, false, 'retain the exact original invalid Python input');
  const rejected = compileGrammar(mappedJsonHost(['Python']), { resolveGrammar: () => shippedPython }).parseTree(invalidSource);
  assert.equal(rejected.ok, false);
  assert.equal(rejected.rejection.reason, 'syntax');
  assert.equal(rejected.tree, null, 'no fabricated token envelope claims a successful child CST');
  assert.equal(rejected.sourcePlanes.length, 2);
  const diagnostic = rejected.rejection.diagnostics.find(item => item.reason === 'mappedSyntax' && item.language === 'Python');
  assert.ok(diagnostic);
  assert.equal(diagnostic.source, 1);
  assert.equal(diagnostic.offsetSemantics, 'covering-source-spelling');
  assert.ok(diagnostic.sourceStart >= 0 && diagnostic.sourceStart <= byteLength(originalPython.recoverySource));
  assert.equal(diagnostic.sourceRoute.at(-1).input, 0);
  assert.ok(diagnostic.sourceRoute.at(-1).inputStart >= 0);
  assert.ok(diagnostic.sourceRoute.at(-1).inputEnd <= byteLength(invalidSource));
  assert.throws(() => checkGrammarSelectionMappedTree(mappingBundle(rejected)), /mapped grammar CST: a complete physical root CST is required/u,
    'a failed result can retain source trace without authenticating unavailable CST bytes');
  observeMappedProfile('recursion', ["mappedParsersRetainExistingLimitRefusals", "mappedCycleAndSyntaxFailuresRetainSourceRoutes", "failedMappedParseCannotAuthenticateMissingRoot"],
    'derived parses share existing budgets and retain explicit source-qualified cycle and syntax failures');
});

test('selection memo keys distinguish mapping profiles from exact delimiter-bearing grammar names', () => {
  const decodedSource = 'x', physicalSource = JSON.stringify(decodedSource);
  const identityName = 'json-string|decoded';
  assert.equal(decodedSource, mappedFixture.inputs.memo.decoded);
  assert.equal(physicalSource, mappedFixture.inputs.nesting.oneLevel);
  assert.equal(identityName, mappedFixture.inputs.memo.identityName);
  for (const warmMapping of ['identity', 'json-string']) {
    const selectedMapping = warmMapping === 'identity' ? 'json-string' : 'identity';
    const host = mappedJsonHost(['decoded']);
    const mapped = host.startRule().expression;
    const identity = { ...mapped, items: [
      { kind: 'grammar-candidate', name: identityName, mapping: 'identity' },
    ] };
    const selections = { identity, 'json-string': mapped };
    host.rules.set(host.start, Object.freeze({ ...host.startRule(), expression: {
      kind: 'seq', items: [
        { kind: 'and', item: selections[warmMapping] }, selections[selectedMapping],
      ],
    } }));
    const grammars = new Map([
      [identityName, literalChild(physicalSource)], ['decoded', literalChild(decodedSource)],
    ]);
    const outcome = compileGrammar(host, { resolveGrammar: name => grammars.get(name) })
      .parseTree(physicalSource);
    assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
    const retained = outcome.tree.children.filter(node => node.type === 'embed');
    assert.equal(retained.length, 1, 'the warming lookahead retains no selected CST');
    const [selected] = retained;
    assert.equal(selected.selection.mapping, selectedMapping);
    assert.equal(selected.language, selectedMapping === 'identity' ? identityName : 'decoded');
    const selectedSource = selectedMapping === 'identity' ? physicalSource : decodedSource;
    assert.deepEqual(selected.root, compileGrammar(grammars.get(selected.language)).parseTree(selectedSource).tree,
      'a completed memo entry belongs to its exact grammar name and mapping profile');
    assert.equal(outcome.sourcePlanes.length, 2, 'both profiles share the one actual completed derivation');
    assertConcreteLeaves(outcome.tree, physicalSource);
    checkGrammarSelectionMappedTree(mappingBundle(outcome));
  }
  observeMappedProfile('selection', ["mappedProfileMemoKeysKeepExactGrammarNames"],
    'selection memo keys distinguish mapping profiles from exact delimiter-bearing grammar names');
});
