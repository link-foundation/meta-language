import assert from 'node:assert/strict';
import test from 'node:test';
import { compileGrammar, serializeGrammar, deserializeGrammar } from '../src/grammar.js';
import { parseGrammarLinks, renderGrammarLinks, percentEncodeLinksText } from '../src/grammar-links.js';
import { parseNativeGrammar, renderNativeGrammar } from '../src/grammar-interchange.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { checkGrammarSelectionMappedTree } from '../src/grammar-runtime.js';
import { sameGrammarTree, sameGrammarOutput } from '../src/grammar-runtime/tree-equality.js';
import { Executor, memoryBudget, stepBudget } from '../src/grammar-runtime/executor.js';
import { mapGrammarSelectionFieldRangeSource, mapGrammarSelectionIncludedRangeSource,
  withGrammarSelectionSources } from '../src/grammar-runtime/selection-source-mapping.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks,
  createGlobalConceptSpaceParser } from '../src/global-concept-space.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks, readStructuredDataLino } from '../src/structured-data-links.js';

function fieldRangeJsonGrammar() {
  return parseGrammarLinks(nativeGrammarText('native-json'));
}

// Wrap an imported grammar's actual start expression. Every original production,
// declaration and alias remains grammar data; no generated grammar is edited.
function fieldRangeGrammar(grammar, names = ['Json'], first = 0, count = 1, mode = 'include', policy = 'any-grammar') {
  const host = parseGrammarLinks(renderGrammarLinks(grammar)), root = host.startRule();
  host.rules.set(root.name, { ...root, expression: { kind: policy, item: root.expression,
    items: names.map(name => ({ kind: 'grammar-field-candidate', name, mapping: 'field-ranges',
      selector: { kind: 'source-field-selector', label: 'value', mode, first, count } })) } });
  return host;
}

function fieldRangeEmbedding(tree) {
  if (tree.type === 'embed' && tree.selection?.mapping === 'field-ranges') return tree;
  for (const child of [...(tree.children ?? []), ...(tree.sourceChildren ?? []), ...(tree.root ? [tree.root] : [])]) {
    const found = fieldRangeEmbedding(child);
    if (found) return found;
  }
  return null;
}

function fieldRangePhysical(tree) {
  if (tree.type === 'missing') return [];
  if (tree.type === 'token' || tree.type === 'error') return tree.text === null
    ? [...Buffer.from(tree.hex, 'hex')] : [...new TextEncoder().encode(tree.text)];
  return (tree.sourceChildren ?? tree.children ?? (tree.root ? [tree.root] : [])).flatMap(fieldRangePhysical);
}

function fieldRangeStored(value) {
  const encoded = encodeStructuredDataLinks(value), source = encoded.network.toLino();
  const network = LinkNetwork.fromLino(source);
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(network));
  assert.equal(binary.toLino(), network.toLino());
  return [readStructuredDataLino(source), decodeStructuredDataLinks(binary, encoded.root)];
}

test('field selections follow actual shipped JSON captures with varying values and UTF-8 widths', () => {
  const json = fieldRangeJsonGrammar();
  for (const [source, first, expected, ranges, path] of [
    ['{"k":1}', 0, '1', [[5, 6]], ['children', 0, 'children', 1, 'children', 2]],
    ['{"k":{"x":23}}', 0, '{"x":23}', [[5, 13]], ['children', 0, 'children', 1, 'children', 2]],
    ['{"a":1,"b":2}', 1, '2', [[11, 12]], ['children', 0, 'children', 3, 'children', 2]],
    ['{"é":"β"}', 0, '"β"', [[6, 10]], ['children', 0, 'children', 1, 'children', 2]],
  ]) {
    const host = fieldRangeGrammar(json, ['Json'], first);
    const listing = renderGrammarLinks(host);
    assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
    const restored = deserializeGrammar(serializeGrammar(host));
    assert.deepEqual([...restored.rules.values()].map(rule => rule.expression),
      [...host.rules.values()].map(rule => rule.expression));
    const native = renderNativeGrammar(host);
    assert.equal(renderNativeGrammar(parseNativeGrammar(native)), native);
    assert.deepEqual(parseNativeGrammar(native).startRule().expression, host.startRule().expression);
    const outcome = compileGrammar(host, { resolveGrammar: () => json }).parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
    const embedded = fieldRangeEmbedding(outcome.tree), mapping = embedded.selection.sourceMapping;
    assert.deepEqual(mapping.ranges, ranges);
    assert.deepEqual(mapping.occurrences, [{ path, start: ranges[0][0], end: ranges[0][1] }]);
    assert.deepEqual(embedded.root, compileGrammar(json).parseTree(expected).tree);
    assert.deepEqual(fieldRangePhysical(outcome.tree), [...Buffer.from(source)]);
    assert.deepEqual(fieldRangePhysical(embedded.root), [...Buffer.from(expected)]);
    assert.equal(outcome.sourcePlanes.length, 2);
    assert.equal(outcome.sourcePlanes[1].derivation.operation, 'field-ranges');
    assert.deepEqual(outcome.sourcePlanes[1].derivation.selector, host.startRule().expression.items[0].selector);
    assert.deepEqual(outcome.sourcePlanes[1].derivation.ranges, ranges);
    assert.deepEqual(outcome.sourcePlanes[1].derivation.witness.children.flatMap(fieldRangePhysical), [...Buffer.from(source)]);
    for (const stored of fieldRangeStored({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes })) {
      assert.equal(checkGrammarSelectionMappedTree(stored), stored);
    }
  }
});

test('any and all field candidates retain every witness and share one completed derivation', () => {
  const json = fieldRangeJsonGrammar();
  for (const policy of ['any-grammar', 'all-grammar']) {
    const parser = compileGrammar(fieldRangeGrammar(json, ['Json', 'JsonTwin'], 0, 1, 'include', policy),
      { resolveGrammar: () => json });
    for (const [source, expected] of [['{"k":1}', '1'], ['{"k":23}', '23']]) {
      const outcome = parser.parseTree(source);
      assert.equal(outcome.ok, true, JSON.stringify(outcome));
      const embedded = fieldRangeEmbedding(outcome.tree);
      assert.equal(outcome.sourcePlanes.length, 2);
      assert.equal(embedded.selection.alternatives.length, 1);
      assert.deepEqual(embedded.root, embedded.selection.alternatives[0].root);
      assert.deepEqual(fieldRangePhysical(embedded.root), [...Buffer.from(expected)]);
      assert.equal(checkGrammarSelectionMappedTree(outcome), outcome);
    }
  }
});

test('source-free global concept archives preserve executable field selector definitions', () => {
  const json = fieldRangeJsonGrammar(), host = fieldRangeGrammar(json);
  const sources = [{ id: 'Host', language: 'Host', grammar: host }, { id: 'Json', language: 'Json', grammar: json }];
  const space = createGlobalConceptSpace(sources, { records: [] });
  const expected = compileGrammar(host, { grammarName: 'Host', globalConceptSpace: space,
    resolveGrammar: name => sources.find(source => source.id === name)?.grammar }).parseTree('{"k":{"x":23}}');
  assert.equal(expected.ok, true, JSON.stringify(expected));
  const archive = globalConceptSpaceToLinks(space);
  host.rules.clear();
  json.rules.clear();
  sources.length = 0;
  for (const stored of fieldRangeStored({ archive })) {
    const actual = createGlobalConceptSpaceParser(stored.archive, 'Host', { records: [] }).parseTree('{"k":{"x":23}}');
    assert.deepEqual(actual, expected);
    assert.equal(checkGrammarSelectionMappedTree(actual), actual);
  }
});

test('coordinated range and valid child forgeries cannot replace the selected field occurrence', () => {
  const source = '{"a":1,"b":2}', json = fieldRangeJsonGrammar();
  const original = compileGrammar(fieldRangeGrammar(json), { resolveGrammar: () => json }).parseTree(source);
  assert.equal(original.ok, true, JSON.stringify(original));
  const donorHost = parseGrammarLinks(`(grammar (start document))\n(rule document normal (any-grammar (literal ${percentEncodeLinksText(source)}) (grammar-range-candidate Json included-ranges (source-range 11 12))))\n`);
  const donor = compileGrammar(donorHost, { resolveGrammar: () => json }).parseTree(source);
  assert.equal(donor.ok, true);
  const donorEmbed = donor.tree.children[0];
  assert.deepEqual(fieldRangePhysical(donorEmbed.root), [50]);
  const forged = structuredClone({ tree: original.tree, sourcePlanes: original.sourcePlanes });
  const embedded = fieldRangeEmbedding(forged.tree), mapping = embedded.selection.sourceMapping;
  const origin = forged.sourcePlanes[1].derivation;
  mapping.ranges = structuredClone(donorEmbed.selection.sourceMapping.ranges);
  mapping.intervals = structuredClone(donorEmbed.selection.sourceMapping.intervals);
  origin.ranges = structuredClone(mapping.ranges);
  mapping.occurrences = [{ path: ['children', 0, 'children', 3, 'children', 2], start: 11, end: 12 }];
  origin.occurrences = structuredClone(mapping.occurrences);
  embedded.root = structuredClone(donorEmbed.root);
  for (const stored of fieldRangeStored(forged)) assert.throws(() => checkGrammarSelectionMappedTree(stored), TypeError);
  for (const mutate of [
    bundle => { bundle.sourcePlanes[1].derivation.witness.children[0].attributes = { note: 'forged' }; },
    bundle => { fieldRangeEmbedding(bundle.tree).sourceChildren[0].attributes = { note: 'forged' }; },
    bundle => { bundle.sourcePlanes[1].derivation.witness.children[0].unrepresented = true; },
  ]) {
    const altered = structuredClone({ tree: original.tree, sourcePlanes: original.sourcePlanes });
    mutate(altered);
    for (const stored of fieldRangeStored(altered)) assert.throws(() => checkGrammarSelectionMappedTree(stored), TypeError);
  }
  const signed = structuredClone({ tree: original.tree, sourcePlanes: original.sourcePlanes });
  signed.sourcePlanes[1].derivation.witness.children[0].attributes = { marker: -0 };
  fieldRangeEmbedding(signed.tree).sourceChildren[0].attributes = { marker: -0 };
  for (const stored of fieldRangeStored(signed)) {
    assert.equal(checkGrammarSelectionMappedTree(stored), stored);
    fieldRangeEmbedding(stored.tree).sourceChildren[0].attributes.marker = 0;
    assert.throws(() => checkGrammarSelectionMappedTree(stored), TypeError);
  }
});

test('failed field child attempts retain replayable physical CST witnesses after a fallback succeeds', () => {
  const source = '{"k":1}', json = fieldRangeJsonGrammar(), host = fieldRangeGrammar(json, ['Refusing']);
  const root = host.startRule();
  host.rules.set(root.name, { ...root, expression: { kind: 'choice', ordered: true,
    items: [root.expression, { kind: 'literal', value: source }] } });
  const refusing = parseGrammarLinks('(grammar (start absent))\n(rule absent normal (literal absent))\n');
  const outcome = compileGrammar(host, { resolveGrammar: () => refusing }).parseTree(source);
  assert.equal(outcome.ok, true);
  assert.equal(fieldRangeEmbedding(outcome.tree), null);
  assert.equal(outcome.sourcePlanes.length, 2);
  assert.equal(outcome.sourcePlanes[1].derivation.witness.children[0].kind, 'object');
  for (const stored of fieldRangeStored({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes })) {
    assert.equal(checkGrammarSelectionMappedTree(stored), stored);
    const forged = structuredClone(stored);
    forged.sourcePlanes[1].derivation.witness.children[0].children[1].children[2].field = 'unselected';
    assert.throws(() => checkGrammarSelectionMappedTree(forged), TypeError);
  }
});

test('empty or unavailable field windows refuse before child lookup and cannot create full source views', () => {
  const json = fieldRangeJsonGrammar();
  for (const [source, first, count] of [['{}', 0, 1], ['{"k":1}', 1, 1], ['{"k":1}', 0, 2]]) {
    let lookups = 0;
    const outcome = compileGrammar(fieldRangeGrammar(json, ['Json'], first, count),
      { resolveGrammar() { lookups += 1; return json; } }).parseTree(source);
    assert.equal(outcome.ok, false);
    assert.equal(lookups, 0);
    assert.equal(outcome.rejection.reason, 'invalidEmbeddingMapping');
  }
  const full = parseGrammarLinks('(grammar (start document))\n(rule document normal (any-grammar (capture labeled value (literal x)) (grammar-field-candidate Json field-ranges (source-field-selector value include 0 1))))\n');
  const outcome = compileGrammar(full, { resolveGrammar() { throw new Error('no child lookup before contraction'); } }).parseTree('x');
  assert.equal(outcome.ok, false);
  assert.equal(outcome.rejection.reason, 'invalidEmbeddingMapping');
});

test('recursive actual JSON fields compose with JSON string decoding across four independent source planes', () => {
  const json = fieldRangeJsonGrammar(), outer = fieldRangeGrammar(json, ['Quoted']);
  const inner = fieldRangeGrammar(json, ['Leaf']);
  const quoted = parseGrammarLinks(renderGrammarLinks(json)), rule = quoted.startRule();
  quoted.rules.set(rule.name, { ...rule, expression: { kind: 'any-grammar', item: rule.expression,
    items: [{ kind: 'grammar-candidate', name: 'Inner', mapping: 'json-string' }] } });
  const grammars = { Quoted: quoted, Inner: inner, Leaf: json };
  const source = '{"k":"{\\"k\\":2}"}';
  const outcome = compileGrammar(outer, { resolveGrammar: name => grammars[name] }).parseTree(source);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  assert.deepEqual(outcome.sourcePlanes.map(plane => [plane.identity, plane.byteLength,
    plane.derivation?.operation ?? null, plane.derivation?.input ?? null]), [
    [0, 17, null, null], [1, 11, 'field-ranges', 0], [2, 7, 'json-string', 1], [3, 1, 'field-ranges', 2],
  ]);
  const first = fieldRangeEmbedding(outcome.tree), last = fieldRangeEmbedding(first.root);
  assert.equal(first.selection.sourceMapping.output, 1);
  assert.equal(last.selection.sourceMapping.input, 2);
  assert.equal(last.selection.sourceMapping.output, 3);
  assert.deepEqual(fieldRangePhysical(outcome.tree), [...Buffer.from(source)]);
  assert.deepEqual(fieldRangePhysical(first.root), [...Buffer.from('"{\\"k\\":2}"')]);
  assert.deepEqual(fieldRangePhysical(last.root), [50]);
  assert.deepEqual(last.root, compileGrammar(json).parseTree('2').tree);
  for (const stored of fieldRangeStored({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes })) {
    assert.equal(checkGrammarSelectionMappedTree(stored), stored);
    const forged = structuredClone(stored);
    forged.sourcePlanes[3].derivation.input = 0;
    assert.throws(() => checkGrammarSelectionMappedTree(forged), TypeError);
  }
});

test('both tree comparisons retain actual field selection metadata physical captures and alternative trees', () => {
  const json = fieldRangeJsonGrammar();
  const outcome = compileGrammar(fieldRangeGrammar(json, ['Json', 'Twin']), { resolveGrammar: () => json })
    .parseTree('{"a":1,"b":2}');
  assert.equal(outcome.ok, true);
  for (const [name, compare] of [['sameGrammarTree', sameGrammarTree],
    ['sameGrammarOutput', (first, second) => sameGrammarOutput([first], [second])]]) {
    const privateOnly = structuredClone(outcome.tree);
    fieldRangeEmbedding(privateOnly).program = { private: 'ignored' };
    assert.equal(compare(outcome.tree, privateOnly), true);
    for (const mutate of [
      mapping => { mapping.selector.label = 'other'; },
      mapping => { mapping.selector.mode = 'exclude'; },
      mapping => { mapping.selector.first = 1; },
      mapping => { mapping.selector.count = 2; },
      mapping => { mapping.occurrences[0].path[1] = 1; },
      mapping => { mapping.occurrences[0].start = 4; },
      mapping => { mapping.occurrences[0].end = 7; },
      mapping => { mapping.ranges[0][1] = 7; },
      mapping => { mapping.output = 9; },
      mapping => { mapping.intervals[0][0] = 1; },
    ]) {
      const forged = structuredClone(outcome.tree);
      mutate(fieldRangeEmbedding(forged).selection.sourceMapping);
      assert.equal(compare(outcome.tree, forged), false, `${name}: ${mutate}`);
    }
    for (const mutate of [
      embedded => { embedded.sourceChildren[0].children[1].children[2].field = 'other'; },
      embedded => { embedded.selection.alternatives[0].root.children[0].end -= 1; },
    ]) {
      const forged = structuredClone(outcome.tree);
      mutate(fieldRangeEmbedding(forged));
      assert.equal(compare(outcome.tree, forged), false, `${name}: ${mutate}`);
    }
  }
});

test('field caches retain actual witness identity signed zero and immutable completed attempt definitions', () => {
  const source = '{"k":1}', json = compileGrammar(fieldRangeJsonGrammar()).parseTree(source);
  assert.equal(json.ok, true);
  const bytes = new TextEncoder().encode(source), options = {};
  const control = { bytes, budget: stepBudget(options, bytes.length, memoryBudget(options)),
    maxDepth: 128, depth: 0, step: Executor.prototype.step, retain: Executor.prototype.retain };
  const selector = { kind: 'source-field-selector', label: 'value', mode: 'include', first: 0, count: 1 };
  const children = structuredClone(json.tree.children);
  children[0].attributes = { marker: -0 };
  const first = mapGrammarSelectionFieldRangeSource(control, 0, 7, selector, children);
  assert.equal(first.mapping.output, 1);
  assert.equal(mapGrammarSelectionFieldRangeSource(control, 0, 7, selector, children).bytes, first.bytes);
  children[0].attributes.marker = 0;
  const changed = mapGrammarSelectionFieldRangeSource(control, 0, 7, selector, children);
  assert.equal(changed.mapping.output, 2);
  assert.notEqual(changed.bytes, first.bytes);
  const independent = mapGrammarSelectionFieldRangeSource(control, 0, 7, selector, structuredClone(children));
  assert.equal(independent.mapping.output, 3);
  assert.notEqual(independent.bytes, changed.bytes);
  const explicit = mapGrammarSelectionIncludedRangeSource(control, 0, 7, [[5, 6]]);
  assert.equal(explicit.mapping.output, 4);
  for (const view of [first, changed, independent, explicit]) assert.deepEqual([...view.bytes], [49]);
  const bundle = withGrammarSelectionSources({ tree: json.tree }, control.budget.memory);
  assert.equal(Object.is(bundle.sourcePlanes[1].derivation.witness.children[0].attributes.marker, -0), true);
  assert.equal(Object.is(bundle.sourcePlanes[2].derivation.witness.children[0].attributes.marker, 0), true);
  for (const stored of fieldRangeStored(bundle)) assert.equal(checkGrammarSelectionMappedTree(stored), stored);
});
