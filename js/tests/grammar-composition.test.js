// Exercise production composition using the complete shipped grammar inputs.
import assert from 'node:assert/strict';
import test from 'node:test';
import { DecoratorSet, DecoratorError } from '../src/decorators.js';
import { Grammar, compileGrammar } from '../src/grammar.js';
import { decorateGrammar } from '../src/grammar-decorators.js';
import { parseGrammarLinks, renderGrammarLinks, renderLinksExpression } from '../src/grammar-links.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { checkGrammarSelectionMappedTree } from '../src/grammar-runtime.js';
import { Executor } from '../src/grammar-runtime/executor.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks,
  createGlobalConceptSpaceParser } from '../src/global-concept-space.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks,
  readStructuredDataLino } from '../src/structured-data-links.js';
import { readGrammarCompositionFieldOrder, readGrammarCompositionContextIssue,
  acceptGrammarCompositionCandidateProfile, readGrammarCompositionCandidateIssue,
  readGrammarCompositionDisposition } from '../src/translation/grammar-rule-composition-rules.js';

function fieldCandidate(name, first = 0, count = 1, label = 'value') {
  return { kind: 'grammar-field-candidate', name, mapping: 'field-ranges',
    selector: { kind: 'source-field-selector', label, mode: 'include', first, count } };
}
function jsonCandidate(name) {
  return { kind: 'grammar-candidate', name, mapping: 'json-string' };
}
function compositionSet(grammar, candidates, mode = 'any-grammar', extra = []) {
  const value = renderLinksExpression({ kind: mode, item: { kind: 'empty' }, items: candidates });
  return new DecoratorSet([{ id: 'compose', level: 'grammar-rule',
    when: [['name', grammar.startRule().name]], actions: [{ op: 'set', field: 'composition', value }] }, ...extra]);
}
function composed(grammar, candidates, mode = 'any-grammar') {
  return decorateGrammar(grammar, compositionSet(grammar, candidates, mode));
}
function jsonGrammar() {
  return parseGrammarLinks(nativeGrammarText('native-json'));
}
function physicalBytes(tree) {
  if (tree.type === 'missing') return [];
  if (tree.type === 'token' || tree.type === 'error') return tree.text === null
    ? [...Buffer.from(tree.hex, 'hex')] : [...Buffer.from(tree.text)];
  return (tree.sourceChildren ?? tree.children ?? (tree.root ? [tree.root] : [])).flatMap(physicalBytes);
}
// Projection for this TEST ONLY: remove just the new mapped embedding nodes,
// retaining every original physical node/token/field/attribute and its span.
function originalPhysicalTrees(tree) {
  if (tree.type === 'embed' && tree.sourceChildren) return tree.sourceChildren.flatMap(originalPhysicalTrees);
  if (tree.type === 'node') return [{ ...tree, children: tree.children.flatMap(originalPhysicalTrees) }];
  return [tree];
}
function firstOriginalField(tree, label) {
  if (tree.field === label && tree.trivia !== true) return tree;
  for (const child of tree.children ?? []) {
    const found = firstOriginalField(child, label);
    if (found) return found;
  }
  return null;
}
function embeddings(tree) {
  const own = tree.type === 'embed' ? [tree] : [];
  const children = [...(tree.children ?? []), ...(tree.sourceChildren ?? []), ...(tree.root ? [tree.root] : [])];
  return own.concat(children.flatMap(embeddings));
}
function storedVariants(value) {
  const packet = encodeStructuredDataLinks(value), text = packet.network.toLino();
  const graph = LinkNetwork.fromLino(text);
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(graph));
  assert.equal(binary.toLino(), graph.toLino());
  return [readStructuredDataLino(text), decodeStructuredDataLinks(binary, packet.root)];
}
function document(grammar) {
  return { schemaVersion: 1, ...Object.fromEntries(Object.entries(grammar)
    .map(([key, value]) => [key, key === 'rules' ? [...value.values()] : value])) };
}
function parserFor(host, sources, options = {}) {
  return compileGrammar(host, { resolveGrammar: name => sources[name], ...options });
}
function requireMapped(outcome, source) {
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  assert.deepEqual(physicalBytes(outcome.tree), [...Buffer.from(source)]);
  assert.equal(checkGrammarSelectionMappedTree(outcome), outcome);
  return embeddings(outcome.tree);
}

test('composition decisions are closed and reject context candidate and source-name collisions', () => {
  assert.equal(readGrammarCompositionFieldOrder('selection'), 'kind|item|items');
  assert.equal(readGrammarCompositionFieldOrder('unknown'), '');
  for (const mode of ['any-grammar', 'all-grammar']) {
    assert.equal(readGrammarCompositionContextIssue('normal', mode, 'empty', false), '');
  }
  for (const args of [['silent', 'any-grammar', 'empty', false], ['normal', 'choice', 'empty', false],
    ['normal', 'any-grammar', 'literal', false], ['normal', 'any-grammar', 'empty', true]]) {
    assert.notEqual(readGrammarCompositionContextIssue(...args), '');
  }
  assert.equal(acceptGrammarCompositionCandidateProfile('grammar-field-candidate', 'field-ranges'), true);
  assert.equal(acceptGrammarCompositionCandidateProfile('grammar-candidate', 'json-string'), true);
  assert.equal(acceptGrammarCompositionCandidateProfile('grammar-candidate', 'identity'), false);
  assert.equal(readGrammarCompositionCandidateIssue(['grammar-candidate'], ['json-string'], ['Json'], ['']), '');
  assert.equal(readGrammarCompositionCandidateIssue(['grammar-field-candidate', 'grammar-field-candidate'],
    ['field-ranges', 'field-ranges'], ['A', 'B'], ['same', 'same']), '');
  for (const args of [[[], [], [], []], [['grammar-candidate'], [], ['A'], ['']],
    [['grammar-candidate'], ['json-string'], [''], ['']],
    [['grammar-candidate', 'grammar-candidate'], ['json-string', 'json-string'], ['A', 'A'], ['', '']],
    [['grammar-candidate', 'grammar-candidate'], ['json-string', 'field-ranges'], ['A', 'B'], ['', '']],
    [['grammar-field-candidate', 'grammar-field-candidate'], ['field-ranges', 'field-ranges'], ['A', 'B'], ['first', 'second']],
    [['grammar-field-candidate'], ['json-string'], ['A'], ['']],
    [['grammar-candidate'], ['included-ranges'], ['A'], ['']]]) {
    assert.notEqual(readGrammarCompositionCandidateIssue(...args), '');
  }
  assert.equal(readGrammarCompositionDisposition('any-grammar', 'full', 'any-grammar', 'full'), 1);
  assert.equal(readGrammarCompositionDisposition('any-grammar', 'first', 'any-grammar', 'second'), 2);
  assert.equal(readGrammarCompositionDisposition('all-grammar', 'full', 'any-grammar', 'full'), 2);
});

test('composition materializes the original expression once and preserves old fallback metadata and renaming', () => {
  const grammar = parseGrammarLinks('(grammar (start root))\n'
    + '(rule root normal (choice ordered (ref item) (literal %222%22)) (concept grammar.root) (source-names (own original_root)))\n'
    + '(rule item normal (literal %221%22))\n');
  const root = grammar.startRule();
  grammar.rules.set(root.name, Object.freeze({ ...root, doc: 'Original complete rule documentation.' }));
  grammar.kinds = [{ name: 'kept', sourceNames: [{ source: 'own', name: 'old_kept' }] }];
  const before = renderGrammarLinks(grammar), original = grammar.startRule().expression;
  const beforeDocument = structuredClone(document(grammar));
  assert.equal(decorateGrammar(grammar), grammar);
  const noMatch = new DecoratorSet([{ id: 'none', level: 'grammar-rule', when: [['name', 'absent']],
    actions: [{ op: 'set', field: 'composition', value: 'not parsed because it did not match' }] }]);
  assert.equal(decorateGrammar(grammar, noMatch), grammar);
  const set = compositionSet(grammar, [jsonCandidate('Json')]);
  const wrapped = decorateGrammar(grammar, set);
  assert.equal(wrapped.startRule().expression.item, original);
  assert.deepEqual(wrapped.startRule().expression.item.items, original.items);
  assert.equal(wrapped.startRule().doc, grammar.startRule().doc);
  assert.deepEqual(wrapped.startRule().sourceNames, grammar.startRule().sourceNames);
  assert.deepEqual(wrapped.declarations, grammar.declarations);
  assert.deepEqual(wrapped.kinds, grammar.kinds);
  assert.equal(renderGrammarLinks(grammar), before);
  assert.equal(decorateGrammar(wrapped, set), wrapped);
  assert.equal(decorateGrammar(grammar, set.remove('compose')), grammar);
  const importerSet = new DecoratorSet(set.decorators.map(entry => ({ ...entry, level: 'importer',
    when: [...entry.when, ['format', grammar.sourceFormat ?? '']] })));
  assert.equal(decorateGrammar(grammar, importerSet, 'importer').startRule().expression.item, original);
  const ordered = new DecoratorSet([
    { id: 'later', order: 2, level: 'grammar-rule', when: [['name', grammar.startRule().name]],
      actions: [{ op: 'replace', field: 'composition', from: 'Json', to: 'RenamedSource' }] },
    ...set.decorators.map(entry => ({ ...entry, order: 1 })),
  ]);
  assert.equal(decorateGrammar(grammar, ordered).startRule().expression.items[0].name, 'RenamedSource');
  const json = jsonGrammar();
  for (const source of ['"1"', '"2"']) {
    const baseline = compileGrammar(grammar).parseTree(source);
    const outcome = parserFor(wrapped, { Json: json }).parseTree(source);
    requireMapped(outcome, source);
    assert.deepEqual(originalPhysicalTrees(outcome.tree), [baseline.tree]);
    const options = { resolveGrammar: () => json, decorators: set };
    assert.deepEqual(compileGrammar(grammar, options).parseTree(source), outcome);
    assert.deepEqual(compileGrammar(wrapped, options).parseTree(source), outcome,
      'compile with the same authoring policy does not add another wrapper or source plane');
  }
  const rename = compositionSet(grammar, [jsonCandidate('item')], 'any-grammar', [
    { id: 'rename', level: 'grammar-rule', when: [['name', 'item']], actions: [{ op: 'set', field: 'name', value: 'leaf' }] },
  ]);
  const renamed = decorateGrammar(grammar, rename);
  assert.equal(renamed.startRule().expression.item.items[0].name, 'leaf');
  assert.equal(renamed.startRule().expression.items[0].name, 'item');
  const oldOnly = decorateGrammar(grammar, rename.remove('compose'));
  for (const [name, rule] of oldOnly.rules) {
    const actual = renamed.rules.get(name);
    assert.deepEqual({ ...actual, expression: rule.expression }, rule);
  }
  const collision = compositionSet(grammar, [jsonCandidate('Json')], 'any-grammar', [
    { id: 'collide', level: 'grammar-rule', when: [['name', 'item']], actions: [{ op: 'set', field: 'name', value: 'root' }] },
  ]);
  assert.throws(() => decorateGrammar(grammar, collision), /capture the existing name/);
  const distinct = composed(wrapped, [jsonCandidate('Other')]);
  assert.equal(distinct.startRule().expression.item, wrapped.startRule().expression);
  const identity = parseGrammarLinks('(grammar (start root))\n(rule root normal (any-grammar (literal %221%22) (grammar-candidate Old identity)))\n');
  assert.equal(composed(identity, [jsonCandidate('Json')]).startRule().expression.item, identity.startRule().expression);
  const orderedCandidates = composed(grammar, [jsonCandidate('First'), jsonCandidate('Second')]);
  assert.equal(composed(orderedCandidates, [jsonCandidate('First'), jsonCandidate('Second')]), orderedCandidates);
  assert.equal(composed(orderedCandidates, [jsonCandidate('Second'), jsonCandidate('First')])
    .startRule().expression.item, orderedCandidates.startRule().expression);
  const selected = composed(grammar, [fieldCandidate('Json')]);
  assert.equal(composed(selected, [fieldCandidate('Json', 1)]).startRule().expression.item,
    selected.startRule().expression, 'a distinct valid occurrence window is not hidden as idempotent');
  assert.deepEqual(document(grammar), beforeDocument, 'no original grammar data is mutated');
});

test('composition rejects malformed authoring templates and never hides forged idempotence data', () => {
  const grammar = jsonGrammar();
  const invalid = [
    '', // Empty is the no-composition value and is checked separately below.
    '(any-grammar empty (grammar-candidate Json json-string)) empty',
    '(any-grammar (literal x) (grammar-candidate Json json-string))',
    '(seq empty)',
    '(any-grammar empty)',
    '(any-grammar empty (grammar-candidate Json identity))',
    '(any-grammar empty (grammar-candidate Json json-string) (grammar-candidate Json json-string))',
    '(any-grammar empty (grammar-candidate Json json-string) (grammar-field-candidate Child field-ranges (source-field-selector value include 0 1)))',
    '(any-grammar empty (grammar-field-candidate Json field-ranges (source-field-selector value include 0 0)))',
    '(any-grammar empty (grammar-field-candidate Json field-ranges (source-field-selector value include 0 1)) (grammar-field-candidate Other field-ranges (source-field-selector value include 1 1)))',
    '(any-grammar empty (grammar-candidate Json json-string extra))',
  ];
  for (const value of invalid.slice(1)) {
    const set = new DecoratorSet([{ id: 'bad', level: 'grammar-rule', when: [['name', grammar.startRule().name]],
      actions: [{ op: 'set', field: 'composition', value }] }]);
    assert.throws(() => decorateGrammar(grammar, set), DecoratorError, value);
  }
  const emptySet = new DecoratorSet([{ id: 'empty', level: 'grammar-rule',
    actions: [{ op: 'set', field: 'composition', value: '' }] }]);
  assert.equal(decorateGrammar(grammar, emptySet), grammar);
  const guarded = compositionSet(grammar, [fieldCandidate('Json')], 'any-grammar', [
    { id: 'change-tree', level: 'executor', actions: [{ op: 'set', field: 'kind', value: 'changed' }] },
  ]);
  assert.throws(() => decorateGrammar(grammar, guarded), /source-witness proof/);
  const materialized = composed(jsonGrammar(), [fieldCandidate('Json')]);
  const resolveGrammar = () => jsonGrammar();
  const assertWitnessRefusal = error => error instanceof DecoratorError
    && /source-witness proof/u.test(error.message);
  for (const level of ['executor', 'recovery']) {
    const treePolicy = new DecoratorSet([{ id: 'tree-' + level, level,
      actions: [{ op: 'set', field: level === 'executor' ? 'kind' : 'reason', value: 'changed' }] }]);
    for (const source of [materialized, document(materialized)]) {
      assert.throws(() => compileGrammar(source, { resolveGrammar, decorators: treePolicy })
        .parseTree('{"k":1}'), assertWitnessRefusal,
      'a pre-materialized grammar cannot bypass the mapped source-witness guard');
      const parser = compileGrammar(source, { resolveGrammar });
      assert.throws(() => parser.parseTree('{"k":1}', { decorators: treePolicy }),
        assertWitnessRefusal, 'per-parse decorators cannot publish an altered mapped witness');
    }
  }
  const nonnormal = compositionSet(grammar, [fieldCandidate('Json')], 'any-grammar', [
    { id: 'kind', level: 'grammar-rule', when: [['name', grammar.startRule().name]], actions: [{ op: 'set', field: 'kind', value: 'silent' }] },
  ]);
  assert.throws(() => decorateGrammar(grammar, nonnormal), /normal syntactic rule/);
  for (const mutation of [
    candidate => { candidate.extra = undefined; },
    candidate => { candidate.selector.extra = true; },
    candidate => { candidate.selector.count = 0; },
  ]) {
    const base = jsonGrammar(), set = compositionSet(base, [fieldCandidate('Json')]);
    const wrapped = decorateGrammar(base, set);
    mutation(wrapped.startRule().expression.items[0]);
    assert.throws(() => decorateGrammar(wrapped, set), DecoratorError);
  }
  let getterCalls = 0;
  const base = jsonGrammar(), set = compositionSet(base, [fieldCandidate('Json')]);
  const wrapped = decorateGrammar(base, set);
  Object.defineProperty(wrapped.startRule().expression.items[0].selector, 'label', {
    enumerable: true, get() { getterCalls += 1; return 'value'; },
  });
  assert.throws(() => decorateGrammar(wrapped, set), DecoratorError);
  assert.equal(getterCalls, 0);
  for (const mutate of [
    expression => { expression.items.length = 2; },
    expression => { delete expression.items[0]; },
    expression => { expression.items.extra = true; },
    expression => { Object.setPrototypeOf(expression.items, Object.create(Array.prototype)); },
    expression => { Object.setPrototypeOf(expression.items[0], { inherited: true }); },
    expression => { Object.setPrototypeOf(expression.items[0].selector, { inherited: true }); },
    expression => { expression.items[0][Symbol('hidden candidate data')] = true; },
    expression => { Object.defineProperty(expression.items[0], 'name', { value: 'Json', enumerable: false }); },
    expression => { Object.defineProperty(expression.items, '0', {
      enumerable: true, get() { getterCalls += 1; return fieldCandidate('Json'); },
    }); },
    expression => { Object.defineProperty(expression.items[0], 'kind', {
      enumerable: true, get() { getterCalls += 1; return 'grammar-field-candidate'; },
    }); },
    expression => { Object.defineProperty(expression, 'item', {
      enumerable: true, get() { getterCalls += 1; return { kind: 'empty' }; },
    }); },
  ]) {
    const base = jsonGrammar(), policy = compositionSet(base, [fieldCandidate('Json')]);
    const candidate = decorateGrammar(base, policy);
    mutate(candidate.startRule().expression);
    assert.throws(() => decorateGrammar(candidate, policy), DecoratorError);
    assert.equal(getterCalls, 0, 'malformed idempotence data is refused without executing accessors');
  }
});

test('opt-in composition uses full shipped JavaScript TypeScript and Rust CSTs with UTF-8 and any all alternatives', () => {
  const payload = '{"é":"β"}', quoted = JSON.stringify(payload), json = jsonGrammar();
  const quotedGrammar = composed(jsonGrammar(), [jsonCandidate('Json')]);
  for (const [id, source] of [
    ['native-javascript', '// 前\nconst data = ' + quoted + ';'],
    ['native-typescript', '// 前\nconst data: string = ' + quoted + ';'],
    ['native-rust', '// 前\nfn main(){let data = ' + quoted + ';}'],
  ]) {
    const grammar = parseGrammarLinks(nativeGrammarText(id));
    const original = renderGrammarLinks(grammar), baseline = compileGrammar(grammar).parseTree(source);
    assert.equal(baseline.ok, true, id + ': ' + JSON.stringify(baseline.rejection));
    const capture = firstOriginalField(baseline.tree, 'value');
    assert.ok(capture, id + ' actual original value capture');
    // Native CST captures can own padding before the quoted literal. Preserve
    // that original field span and all of its bytes; never trim the view.
    const selected = Buffer.from(source).subarray(capture.start, capture.end);
    const at = Buffer.from(source).indexOf(Buffer.from(quoted));
    assert.ok(at >= capture.start);
    assert.equal(capture.end, at + Buffer.byteLength(quoted));
    for (const mode of ['any-grammar', 'all-grammar']) {
      const host = composed(grammar, [fieldCandidate('QuotedA'), fieldCandidate('QuotedB')], mode);
      const outcome = parserFor(host, { QuotedA: quotedGrammar, QuotedB: quotedGrammar, Json: json }).parseTree(source);
      const embedded = requireMapped(outcome, source);
      assert.deepEqual(originalPhysicalTrees(outcome.tree), [baseline.tree], id + ' original physical CST');
      const first = embedded.find(node => node.selection.mapping === 'field-ranges');
      assert.ok(first, id + ' actual field selection');
      assert.equal(first.selection.alternatives.length, 1);
      assert.deepEqual(first.selection.candidates.map(candidate => [candidate.language, candidate.relation]),
        [['QuotedA', 'accepted'], ['QuotedB', 'accepted']]);
      assert.deepEqual(physicalBytes(first.root), [...selected]);
      const mapping = first.selection.sourceMapping;
      const origin = outcome.sourcePlanes[mapping.output].derivation;
      assert.deepEqual(mapping.ranges.map(([left, right]) => [origin.start + left, origin.start + right]),
        [[capture.start, capture.end]]);
      const decoded = embeddings(first.root).find(node => node.selection.mapping === 'json-string');
      assert.deepEqual(decoded.root, compileGrammar(json).parseTree(payload).tree);
      assert.deepEqual(physicalBytes(decoded.root), [...Buffer.from(payload)]);
      assert.equal(renderGrammarLinks(grammar), original, id + ' shipped grammar unchanged');
    }
  }
  const grammar = parseGrammarLinks(nativeGrammarText('native-javascript'));
  const other = JSON.stringify('{"n":2}');
  const source = 'const first = ' + quoted + '; const second = ' + other + ';';
  const baseline = compileGrammar(grammar).parseTree(source);
  assert.equal(baseline.ok, true);
  const secondDeclaration = baseline.tree.children.filter(child => child.type === 'node')[1];
  assert.ok(secondDeclaration);
  const secondCapture = firstOriginalField(secondDeclaration, 'value');
  assert.ok(secondCapture);
  const outcome = parserFor(composed(grammar, [fieldCandidate('Quoted', 1)]), { Quoted: quotedGrammar, Json: json }).parseTree(source);
  const first = requireMapped(outcome, source).find(node => node.selection.mapping === 'field-ranges');
  assert.deepEqual(originalPhysicalTrees(outcome.tree), [baseline.tree]);
  assert.deepEqual(physicalBytes(first.root), [...Buffer.from(source).subarray(secondCapture.start, secondCapture.end)]);
});

test('recursive composition retains independent source planes and source-free text binary grammar and result archives', () => {
  const host = composed(jsonGrammar(), [fieldCandidate('Quoted')]);
  const quoted = composed(jsonGrammar(), [jsonCandidate('Inner')]);
  const inner = composed(jsonGrammar(), [fieldCandidate('Leaf')]);
  const leaf = jsonGrammar(), sources = { Host: host, Quoted: quoted, Inner: inner, Leaf: leaf };
  const source = '{"k":"{\\"k\\":2}"}';
  const parser = parserFor(host, sources), budgets = new Set(), memories = new Set();
  const step = Executor.prototype.step;
  Executor.prototype.step = function () {
    budgets.add(this.budget);
    memories.add(this.budget.memory);
    return step.call(this);
  };
  let expected;
  try { expected = parser.parseTree(source); }
  finally { Executor.prototype.step = step; }
  const nodes = requireMapped(expected, source);
  assert.equal(budgets.size, 1, 'composition never restarts the shared parse budget');
  assert.equal(memories.size, 1, 'composition never restarts source-plane memory accounting');
  for (const [options, reason] of [[{ stepLimit: 0 }, 'stepLimit'], [{ memoryLimit: 0 }, 'memoryBudget'], [{ maxDepth: 0 }, 'nestingDepth']]) {
    const limited = parser.parseTree(source, options);
    assert.equal(limited.ok, false);
    assert.equal(limited.rejection.reason, reason);
  }
  assert.deepEqual(expected.sourcePlanes.map(plane => [plane.identity, plane.byteLength,
    plane.derivation?.operation ?? null, plane.derivation?.input ?? null]), [
    [0, 17, null, null], [1, 11, 'field-ranges', 0], [2, 7, 'json-string', 1], [3, 1, 'field-ranges', 2],
  ]);
  assert.deepEqual(physicalBytes(nodes[nodes.length - 1].root), [50]);
  const definitions = Object.fromEntries(Object.entries(sources).map(([name, grammar]) => [name, document(grammar)]));
  const policy = compositionSet(jsonGrammar(), [fieldCandidate('Quoted')]);
  assert.equal(DecoratorSet.fromLino(policy.toLino()).toLino(), policy.toLino());
  for (const stored of storedVariants({ definitions, policy: policy.toLino(),
    result: { tree: expected.tree, sourcePlanes: expected.sourcePlanes } })) {
    assert.equal(DecoratorSet.fromLino(stored.policy).toLino(), policy.toLino());
    // Existing compileGrammar accepts a complete plain Grammar document.
    const restored = compileGrammar(stored.definitions.Host, { resolveGrammar: name => stored.definitions[name] }).parseTree(source);
    assert.deepEqual(restored, expected);
    assert.equal(checkGrammarSelectionMappedTree(stored.result), stored.result);
    assert.deepEqual(stored.result, { tree: expected.tree, sourcePlanes: expected.sourcePlanes });
    const forged = structuredClone(stored.result);
    forged.sourcePlanes[3].derivation.input = 0;
    assert.throws(() => checkGrammarSelectionMappedTree(forged), TypeError);
    const coordinated = structuredClone(stored.result);
    const mapping = embeddings(coordinated.tree).find(node => node.selection.mapping === 'field-ranges').selection.sourceMapping;
    mapping.selector.first = 1;
    coordinated.sourcePlanes[1].derivation.selector.first = 1;
    assert.throws(() => checkGrammarSelectionMappedTree(coordinated), TypeError);
  }
  const sourceRecords = Object.entries(sources).map(([id, grammar]) => ({ id, language: id, grammar }));
  const space = createGlobalConceptSpace(sourceRecords, { records: [] });
  const globalExpected = parserFor(host, sources, { grammarName: 'Host', globalConceptSpace: space }).parseTree(source);
  requireMapped(globalExpected, source);
  const archive = globalConceptSpaceToLinks(space);
  // Drop actual source definitions; runtime policies and private pointers are
  // absent from the archive-only parser. Every rule definition is in Links.
  for (const grammar of Object.values(sources)) grammar.rules.clear();
  for (const key of Object.keys(sources)) delete sources[key];
  sourceRecords.length = 0;
  for (const stored of storedVariants({ archive })) {
    const actual = createGlobalConceptSpaceParser(stored.archive, 'Host', { records: [] }).parseTree(source);
    assert.deepEqual(actual, globalExpected);
    assert.equal(checkGrammarSelectionMappedTree(actual), actual);
  }
});

test('composition keeps no-match child-refusal profile boundaries and explicit existing fallback behavior', () => {
  const json = jsonGrammar(), quoted = composed(jsonGrammar(), [jsonCandidate('Json')]);
  const host = composed(jsonGrammar(), [fieldCandidate('Quoted')]);
  let lookups = 0;
  const absent = compileGrammar(host, { resolveGrammar() { lookups += 1; return quoted; } }).parseTree('{}');
  assert.equal(compileGrammar(json).parseTree('{}').ok, true);
  assert.equal(absent.ok, false);
  assert.equal(absent.rejection.reason, 'invalidEmbeddingMapping');
  assert.equal(lookups, 0);
  assert.ok(!absent.sourcePlanes || absent.sourcePlanes.length === 1);
  const refused = parserFor(host, { Quoted: quoted, Json: json }).parseTree('{"k":"not json"}');
  assert.equal(refused.ok, false);
  assert.equal(refused.sourcePlanes.length, 3);
  assert.deepEqual(refused.sourcePlanes.map(plane => plane.derivation?.operation ?? null), [null, 'field-ranges', 'json-string']);
  if (refused.tree === null) assert.throws(() => checkGrammarSelectionMappedTree(refused), TypeError);
  for (const source of ["const data = '{\"k\":1}';", String.raw`const data = "\x31";`]) {
    const grammar = parseGrammarLinks(nativeGrammarText('native-javascript'));
    assert.equal(compileGrammar(grammar).parseTree(source).ok, true);
    const outcome = parserFor(composed(grammar, [fieldCandidate('Quoted')]), { Quoted: quoted, Json: json }).parseTree(source);
    assert.equal(outcome.ok, false, source + ' is outside json-string');
  }
  const rust = parseGrammarLinks(nativeGrammarText('native-rust'));
  const raw = 'fn main(){let data = r#"{\"k\":1}"#;}';
  assert.equal(compileGrammar(rust).parseTree(raw).ok, true);
  assert.equal(parserFor(composed(rust, [fieldCandidate('Quoted')]), { Quoted: quoted, Json: json }).parseTree(raw).ok, false);
  // Preserve an explicit original enclosing fallback. The adapter changes only
  // attempt; the entire full JSON root expression remains the fallback arm.
  const complete = jsonGrammar(), root = complete.startRule();
  const fallback = new Grammar(root.name, [...complete.rules].map(([name, rule]) => [name,
    name === root.name ? { ...rule, expression: { kind: 'choice', ordered: true,
      items: [{ kind: 'ref', name: 'attempt' }, rule.expression] } } : rule]).concat([
    ['attempt', { kind: 'normal', expression: root.expression }],
  ]), complete.sourceFormat, complete.declarations);
  fallback.kinds = complete.kinds;
  const attemptSet = new DecoratorSet([{ id: 'attempt-only', level: 'grammar-rule', when: [['name', 'attempt']],
    actions: [{ op: 'set', field: 'composition', value: renderLinksExpression({ kind: 'any-grammar', item: { kind: 'empty' },
      items: [fieldCandidate('Refusing')] }) }] }]);
  const refusing = parseGrammarLinks('(grammar (start absent))\n(rule absent normal (literal absent))\n');
  const retained = decorateGrammar(fallback, attemptSet);
  assert.equal(retained.startRule().expression, fallback.startRule().expression);
  assert.equal(retained.rules.get('attempt').expression.item, root.expression);
  const outcome = parserFor(retained, { Refusing: refusing }).parseTree('{"k":1}');
  requireMapped(outcome, '{"k":1}');
  assert.equal(embeddings(outcome.tree).length, 0);
  assert.equal(outcome.sourcePlanes.length, 2);
  assert.equal(outcome.sourcePlanes[1].derivation.operation, 'field-ranges');
  const cycle = parseGrammarLinks('(grammar (start cycle))\n(rule cycle normal (any-grammar (literal x) (grammar-candidate Cycle identity)))\n');
  const cyclic = parserFor(composed(jsonGrammar(), [jsonCandidate('Cycle')]), { Cycle: cycle }).parseTree('"x"');
  assert.equal(cyclic.ok, false);
  assert.equal(cyclic.rejection.reason, 'nonConsumingEmbeddingCycle');
});
