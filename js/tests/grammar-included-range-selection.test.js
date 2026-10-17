import assert from 'node:assert/strict';
import test from 'node:test';
import { compileGrammar, serializeGrammar, deserializeGrammar } from '../src/grammar.js';
import { parseGrammarLinks, renderGrammarLinks, percentEncodeLinksText } from '../src/grammar-links.js';
import { parseNativeGrammar, renderNativeGrammar } from '../src/grammar-interchange.js';
import { checkGrammarSelectionMappedTree } from '../src/grammar-runtime.js';
import { sameGrammarOutput, sameGrammarTree } from '../src/grammar-runtime/tree-equality.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks, readStructuredDataLino } from '../src/structured-data-links.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks,
  createGlobalConceptSpaceParser } from '../src/global-concept-space.js';
import { DecoratorSet } from '../src/decorators.js';
import { readGrammarSelectionRangeLibraryStyles } from '../scripts/generate-grammar-selection-included-range-rules.mjs';
import {
  acceptGrammarSelectionIncludedRangeMapping,
  readGrammarSelectionIncludedRangeContext,
  acceptGrammarSelectionIncludedRangeCandidate,
  acceptGrammarSelectionIncludedRangeSourceRange,
  acceptGrammarSelectionIncludedRangeAgreement,
  readGrammarSelectionIncludedRangeReservation,
  readGrammarSelectionIncludedRangePlan,
  readGrammarSelectionIncludedRangePart,
  acceptGrammarSelectionIncludedRangeByteBoundary,
  readGrammarSelectionIncludedRangeFieldOrder,
} from '../src/translation/grammar-selection-included-range-rules.js';

function includedRangeGrammar(region, ranges, names = ['Inline'], mode = 'any-grammar', prefix = '> ', suffix = '\n') {
  const candidates = names.map(name => `(grammar-range-candidate ${name} included-ranges ${ranges.map(([start, end]) => `(source-range ${start} ${end})`).join(' ')})`);
  return parseGrammarLinks(`(grammar (start document))\n(rule document normal (seq (literal ${percentEncodeLinksText(prefix)}) (${mode} (ref physical) ${candidates.join(' ')}) (literal ${percentEncodeLinksText(suffix)})))\n(rule physical normal (literal ${percentEncodeLinksText(region)}))\n`);
}

function includedRangeLiteralGrammar(source) {
  return parseGrammarLinks(`(grammar (start decoded))\n(rule decoded normal (literal ${percentEncodeLinksText(source)}))\n`);
}

function includedRangePhysicalBytes(tree) {
  if (tree.type === 'missing') return [];
  if (tree.type === 'token' || tree.type === 'error') return tree.text === null
    ? [...Buffer.from(tree.hex, 'hex')] : [...new TextEncoder().encode(tree.text)];
  return (tree.sourceChildren ?? tree.children ?? (tree.root ? [tree.root] : [])).flatMap(includedRangePhysicalBytes);
}

function includedRangeEmbedding(tree) {
  if (tree.type === 'embed' && tree.selection.mapping === 'included-ranges') return tree;
  for (const child of [...(tree.children ?? []), ...(tree.sourceChildren ?? []), ...(tree.root ? [tree.root] : [])]) {
    const found = includedRangeEmbedding(child);
    if (found) return found;
  }
  return null;
}

function includedRangeStoredVariants(value) {
  const encoded = encodeStructuredDataLinks(value), text = encoded.network.toLino();
  const network = LinkNetwork.fromLino(text);
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(network));
  assert.equal(binary.toLino(), network.toLino());
  return [readStructuredDataLino(text), decodeStructuredDataLinks(binary, encoded.root)];
}

function includedRangeGrammarDocument(grammar) {
  return Object.fromEntries(Object.entries(grammar).map(([key, value]) =>
    [key, key === 'rules' ? [...value.values()] : value]));
}

test('source-free global archives execute the original included range grammar and physical CST', () => {
  for (const mode of ['any-grammar', 'all-grammar']) {
    const host = includedRangeGrammar('a+b', [[0, 1], [2, 3]], ['Inline'], mode, '', '');
    const child = includedRangeLiteralGrammar('ab');
    assert.equal(renderGrammarLinks(deserializeGrammar(serializeGrammar(host))), renderGrammarLinks(host));
    const sources = [{ id: 'Outer', language: 'Outer', grammar: host },
      { id: 'Inline', language: 'Inline', grammar: child }];
    const space = createGlobalConceptSpace(sources, { records: [] });
    const expected = compileGrammar(host, { grammarName: 'Outer', globalConceptSpace: space,
      resolveGrammar: name => sources.find(source => source.id === name)?.grammar }).parseTree('a+b');
    assert.equal(expected.ok, true);
    const archive = globalConceptSpaceToLinks(space);
    host.rules.clear();
    child.rules.clear();
    sources.length = 0;
    for (const { archive: stored } of includedRangeStoredVariants({ archive })) {
      const actual = createGlobalConceptSpaceParser(stored, 'Outer', { records: [] }).parseTree('a+b');
      assert.deepEqual(actual, expected);
      assert.deepEqual(includedRangePhysicalBytes(actual.tree), [97, 43, 98]);
      assert.deepEqual(includedRangePhysicalBytes(includedRangeEmbedding(actual.tree).root), [97, 98]);
      assert.equal(checkGrammarSelectionMappedTree(actual), actual);
    }
  }
});

test('closed source view forms refuse lossy serialization before coercions or property reads', () => {
  const mutations = [
    candidate => { candidate.opaqueBinding = true; },
    candidate => { candidate.opaqueBinding = undefined; },
    candidate => { candidate[Symbol('opaqueBinding')] = true; },
    candidate => { Object.defineProperty(candidate, 'opaqueBinding', { value: true }); },
    candidate => { candidate.ranges[0].opaqueBinding = true; },
    candidate => { candidate.ranges[0].opaqueBinding = undefined; },
    candidate => { candidate.ranges[0][Symbol('opaqueBinding')] = true; },
    candidate => { candidate.ranges[0].start = '0'; },
    candidate => { candidate.ranges[0].end = '1'; },
    candidate => { candidate.ranges[0].start = NaN; },
    candidate => { candidate.ranges[0].end = Infinity; },
    candidate => { candidate.ranges[0].end = 4294967296; },
    candidate => { candidate.name = 7; },
    candidate => { candidate.mapping = new String('included-ranges'); },
    candidate => { delete candidate.ranges[0]; },
    candidate => { candidate.ranges.opaqueBinding = true; },
    candidate => { candidate.ranges[Symbol('opaqueBinding')] = true; },
    candidate => { Object.setPrototypeOf(candidate.ranges[0], { inheritedBinding: true }); },
    candidate => { candidate.toJSON = () => ({ kind: candidate.kind, name: candidate.name,
      mapping: candidate.mapping, ranges: candidate.ranges }); },
  ];
  for (const mutate of mutations) {
    const host = includedRangeGrammar('a+b', [[0, 1], [2, 3]], ['Inline'], 'any-grammar', '', '');
    mutate(host.rules.get('document').expression.items[1].items[0]);
    assert.throws(() => compileGrammar(host));
    assert.throws(() => renderGrammarLinks(host));
    assert.throws(() => renderNativeGrammar(host));
    assert.throws(() => serializeGrammar(host));
    assert.throws(() => deserializeGrammar(includedRangeGrammarDocument(host)));
    assert.throws(() => createGlobalConceptSpace([{ id: 'Outer', language: 'Outer', grammar: host }], { records: [] }));
  }
  for (const location of ['range', 'array']) {
    for (const operation of ['compile', 'links', 'listing', 'structured', 'read', 'global']) {
      let reads = 0;
      const host = includedRangeGrammar('a+b', [[0, 1], [2, 3]], ['Inline'], 'any-grammar', '', '');
      const candidate = host.rules.get('document').expression.items[1].items[0];
      Object.defineProperty(location === 'range' ? candidate.ranges[0] : candidate.ranges,
        location === 'range' ? 'start' : '0', { enumerable: true, get() { reads += 1; return location === 'range' ? 0 : { kind: 'source-range', start: 0, end: 1 }; } });
      assert.throws(() => ({ compile: () => compileGrammar(host), links: () => renderGrammarLinks(host),
        listing: () => renderNativeGrammar(host), structured: () => serializeGrammar(host),
        read: () => deserializeGrammar(includedRangeGrammarDocument(host)), global: () => createGlobalConceptSpace([
          { id: 'Outer', language: 'Outer', grammar: host }], { records: [] }) })[operation]());
      assert.equal(reads, 0, location + ':' + operation);
    }
  }
  for (const transformation of ['accessor', 'toJSON']) {
    let reads = 0;
    const host = includedRangeGrammar('a+b', [[0, 1], [2, 3]], ['Inline'], 'any-grammar', '', '');
    const rule = { ...host.rules.get('document') }, expression = rule.expression;
    host.rules.set('document', rule);
    if (transformation === 'accessor') Object.defineProperty(rule, 'expression', {
      enumerable: true, get() { reads += 1; return expression; },
    });
    else rule.toJSON = () => { reads += 1; return { ...rule, expression }; };
    assert.throws(() => compileGrammar(host));
    assert.throws(() => serializeGrammar(host));
    assert.throws(() => deserializeGrammar(includedRangeGrammarDocument(host)));
    assert.throws(() => createGlobalConceptSpace([{ id: 'Outer', language: 'Outer', grammar: host }], { records: [] }));
    assert.equal(reads, 0, transformation);
  }
});

test('global archive writers refuse malformed source view definitions changed after construction', () => {
  for (const mutate of [candidate => { candidate.opaqueBinding = true; },
    candidate => { candidate.ranges[0].end = '1'; },
    candidate => { candidate.ranges.opaqueBinding = undefined; }]) {
    const host = includedRangeGrammar('a+b', [[0, 1], [2, 3]], ['Inline'], 'any-grammar', '', '');
    const space = createGlobalConceptSpace([{ id: 'Outer', language: 'Outer', grammar: host }], { records: [] });
    const definition = space.concepts.flatMap(concept => concept.definitions)
      .find(entry => entry.origin === 'Outer:document').definition;
    mutate(definition.expression.items[1].items[0]);
    assert.throws(() => globalConceptSpaceToLinks(space));
  }
});

test('included range plans retain every kept byte, excluded gap and final boundary without rewriting inputs', () => {
  for (const [ranges, length, expected] of [
    [[0, 4, 6, 9], 9, [0, 4, 0, 4, 4, 6, 4, 4, 6, 9, 4, 7, 9, 9, 7, 7]],
    [[1, 3], 5, [0, 1, 0, 0, 1, 3, 0, 2, 3, 5, 2, 2, 5, 5, 2, 2]],
    [[0, 1, 1, 3], 4, [0, 1, 0, 1, 1, 3, 1, 3, 3, 4, 3, 3, 4, 4, 3, 3]],
  ]) {
    const input = Object.freeze([...ranges]);
    assert.deepEqual(readGrammarSelectionIncludedRangePlan(input, length), [0, 0, 0, 0, ...expected]);
    assert.deepEqual(input, ranges);
    assert.equal(acceptGrammarSelectionIncludedRangeAgreement(input, [...ranges]), true);
  }
  for (const [ranges, length] of [
    [[], 5], [[0], 5], [[0, 0], 5], [[-1, 1], 5], [[1.5, 2], 5],
    [[2, 1], 5], [[0, 6], 5], [[2, 3, 1, 2], 5], [[0, 3, 2, 4], 5],
    [[0, 5], 5], [[0, 2, 2, 5], 5], [[0, 1], 0], [[0, 1], Infinity],
    [[0, 1], NaN], [[0, 1], 4294967296], [[0, NaN], 5],
  ]) assert.deepEqual(readGrammarSelectionIncludedRangePlan(ranges, length), [], 'invalid or non-contracting view');
  assert.equal(acceptGrammarSelectionIncludedRangeAgreement([0, 4], [0, 3]), false);
  assert.equal(acceptGrammarSelectionIncludedRangeAgreement([0, 4], [0, 4, 6, 9]), false);
  assert.equal(readGrammarSelectionIncludedRangeReservation(9, 4, false), 352);
  assert.equal(readGrammarSelectionIncludedRangeReservation(9, 4, true), 640);
  for (const cells of [0, 1, 19, Infinity, NaN]) {
    assert.equal(readGrammarSelectionIncludedRangeReservation(9, cells, false), -1);
  }
});

test('included ranges require their closed candidate and one common syntactic mapping', () => {
  assert.equal(acceptGrammarSelectionIncludedRangeMapping('included-ranges'), true);
  assert.equal(acceptGrammarSelectionIncludedRangeMapping('json-string'), false);
  assert.equal(acceptGrammarSelectionIncludedRangeCandidate('grammar-range-candidate', 'included-ranges'), true);
  assert.equal(acceptGrammarSelectionIncludedRangeCandidate('grammar-candidate', 'included-ranges'), false);
  assert.equal(acceptGrammarSelectionIncludedRangeCandidate('grammar-range-candidate', 'json-string'), false);
  for (const [mappings, inToken, expected] of [
    [['identity'], false, 0], [['json-string'], false, 0], [['future-view'], false, 0],
    [['included-ranges'], false, 1], [['included-ranges', 'included-ranges'], false, 1],
    [['included-ranges'], true, 2], [['included-ranges', 'identity'], false, 2],
    [['json-string', 'included-ranges'], false, 2], [['included-ranges', 'future-view'], false, 2],
  ]) assert.equal(readGrammarSelectionIncludedRangeContext(mappings, inToken), expected);
  assert.equal(acceptGrammarSelectionIncludedRangeSourceRange('source-range', 0, 4), true);
  for (const [kind, start, end] of [
    ['range', 0, 4], ['source-range', 0, 0], ['source-range', -1, 4],
    ['source-range', 0, 1.5], ['source-range', 0, 4294967296],
  ]) assert.equal(acceptGrammarSelectionIncludedRangeSourceRange(kind, start, end), false);
  assert.equal(readGrammarSelectionIncludedRangeFieldOrder('derivation'), 'operation|input|start|end|ranges');
  assert.equal(readGrammarSelectionIncludedRangeFieldOrder('range'), 'kind|start|end');
  assert.equal(readGrammarSelectionIncludedRangeFieldOrder('json-string'), '');
});

test('streamed scalar correspondence preserves the original multibyte excluded-range example', () => {
  const source = '*é\n> β*', bytes = new TextEncoder().encode(source), widths = new Map();
  let cursor = 0;
  for (const scalar of source) {
    const width = new TextEncoder().encode(scalar).length;
    widths.set(cursor, width);
    cursor += width;
  }
  const plan = readGrammarSelectionIncludedRangePlan([0, 4, 6, 9], bytes.length);
  const intervals = [], result = [];
  let row = 0, position = 0, output = 0, done = false;
  for (let step = 0; step < 20; step += 1) {
    const part = readGrammarSelectionIncludedRangePart(plan, row, position, output, widths.get(position) ?? 0);
    if (part[0] === 1) { done = true; break; }
    assert.equal(part.length, 8);
    assert.equal(part[0], 0);
    const [inputStart, inputEnd, outputStart, outputEnd] = part.slice(4);
    intervals.push([inputStart + 2, inputEnd + 2, outputStart, outputEnd]);
    if (outputEnd > outputStart) result.push(...bytes.subarray(inputStart, inputEnd));
    [row, position, output] = part.slice(1, 4);
  }
  assert.equal(done, true);
  assert.deepEqual(intervals, [
    [2, 2, 0, 0], [2, 3, 0, 1], [3, 5, 1, 3], [5, 6, 3, 4],
    [6, 8, 4, 4], [8, 10, 4, 6], [10, 11, 6, 7], [11, 11, 7, 7],
  ]);
  assert.deepEqual(result, [42, 195, 169, 10, 206, 178, 42]);
  assert.equal(new TextDecoder('utf-8', { fatal: true }).decode(Uint8Array.from(result)), '*é\nβ*');
  assert.deepEqual(readGrammarSelectionIncludedRangePart(plan, 0, 0, 1, 1), []);
  const provider = 'pub mod ml_array { pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> { index.and_then(|index| values.get(index)).cloned() } }\n';
  const numeric = provider + 'pub fn read_grammar_selection_included_range_part(plan: &[f64], row: f64) -> Vec<f64> { '
    + 'let end = (self::ml_array::optional_at(plan, self::ml_array::number_index(row))).unwrap_or_else(|| -1f64); '
    + 'let next = (self::ml_array::optional_at(plan, self::ml_array::number_index(row + 1f64))).unwrap_or_else(|| end); vec![end, next] }';
  for (const source of [numeric, numeric.replace('read_grammar_selection_included_range_part', 'read_grammar_selection_field_range_plan')]) {
    const styled = readGrammarSelectionRangeLibraryStyles(source);
    assert.equal(styled.actions.length, 1, 'one complete numeric source owner retains both scalar defaults');
    assert.equal(styled.source, source.replaceAll('.unwrap_or_else(||', '.unwrap_or('));
    const record = new DecoratorSet([{id: 'range-scalar-default-control', level: 'emitter', order: 0,
      when: [['format', 'Rust']], actions: styled.actions}]);
    assert.equal(record.decorate('emitter', {format: 'Rust', source}).source, styled.source);
    assert.equal(styled.source.replace(styled.actions[0].to, styled.actions[0].from), source);
    assert.deepEqual(readGrammarSelectionRangeLibraryStyles(styled.source), {source: styled.source, actions: []});
    for (const refused of [
      source.replace('plan: &[f64]', 'plan: &[Custom]'),
      source.replace('values.get(index)', 'values.custom(index)'),
      source.replace('|| -1f64', '|| /* retain */ -1f64'),
      source.replace('|| -1f64', '|| observe()'),
      'type f64=Custom;\n' + source, 'use custom::*;\n' + source,
      source.replace('pub fn read_', '#[custom]\npub fn read_'),
    ]) assert.deepEqual(readGrammarSelectionRangeLibraryStyles(refused), {source: refused, actions: []});
    const unknown = source.replace('|| end', '|| unknown');
    assert.ok(readGrammarSelectionRangeLibraryStyles(unknown).source.includes('.unwrap_or_else(|| unknown)'));
    const note = '\nconst NOTE:&str=r#".unwrap_or_else(|| end)"#;\n';
    assert.equal(readGrammarSelectionRangeLibraryStyles(source + note).source, styled.source + note);
  }
});

test('actual valid UTF-8 source boundaries exclude every continuation byte', () => {
  const source = '\0A' + Array.from({ length: 64 }, (_, index) => String.fromCodePoint(128 + index)).join('') + '€😀\uFEFF';
  const bytes = new TextEncoder().encode(source);
  const decoder = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true });
  for (let index = 0; index <= bytes.length; index += 1) {
    let boundary = false;
    try {
      decoder.decode(bytes.subarray(0, index));
      decoder.decode(bytes.subarray(index));
      boundary = true;
    } catch {}
    assert.equal(acceptGrammarSelectionIncludedRangeByteBoundary(bytes[index] ?? -1, index === bytes.length), boundary);
  }
  for (const byte of [-1, 0.5, 256, NaN, Infinity]) {
    assert.equal(acceptGrammarSelectionIncludedRangeByteBoundary(byte, false), false);
  }
  assert.equal(acceptGrammarSelectionIncludedRangeByteBoundary(-1, true), true);
});

test('real any and all selections keep the complete physical CST and a separate scalar source view', () => {
  const region = '*é\n> β*', decoded = '*é\nβ*', ranges = [[0, 4], [6, 9]];
  const child = includedRangeLiteralGrammar(decoded);
  const direct = compileGrammar(child).parseTree(decoded);
  assert.equal(direct.ok, true);
  for (const mode of ['any-grammar', 'all-grammar']) for (const prefix of ['> ', '前\n> ']) {
    const grammar = includedRangeGrammar(region, ranges, ['Inline', 'InlineTwin'], mode, prefix);
    const listing = renderGrammarLinks(grammar);
    assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
    assert.equal(renderGrammarLinks(parseNativeGrammar(renderNativeGrammar(grammar))), listing);
    const source = prefix + region + '\n';
    const outcome = compileGrammar(grammar, { resolveGrammar: () => child }).parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
    const embedded = includedRangeEmbedding(outcome.tree);
    assert.ok(embedded);
    const start = Buffer.byteLength(prefix);
    assert.deepEqual(includedRangePhysicalBytes(outcome.tree), [...Buffer.from(source)]);
    assert.deepEqual(includedRangePhysicalBytes(embedded.root), [42, 195, 169, 10, 206, 178, 42]);
    assert.deepEqual(embedded.root, direct.tree);
    assert.equal(embedded.selection.alternatives.length, 1);
    assert.deepEqual(embedded.selection.alternatives[0].root, direct.tree);
    assert.deepEqual(embedded.selection.sourceMapping.ranges, ranges);
    assert.deepEqual(embedded.selection.sourceMapping.intervals, [
      [0, 0, 0, 0], [0, 1, 0, 1], [1, 3, 1, 3], [3, 4, 3, 4],
      [4, 6, 4, 4], [6, 8, 4, 6], [8, 9, 6, 7], [9, 9, 7, 7],
    ].map(([begin, end, outputStart, outputEnd]) => [start + begin, start + end, outputStart, outputEnd]));
    assert.equal(outcome.sourcePlanes.length, 2);
    assert.deepEqual(outcome.sourcePlanes[1].derivation,
      { operation: 'included-ranges', input: 0, start, end: start + 9, ranges });
    const bundle = { tree: outcome.tree, sourcePlanes: outcome.sourcePlanes };
    assert.equal(checkGrammarSelectionMappedTree(bundle), bundle);
  }
});

test('included source views run the shipped JSON grammar and retain its complete standalone CST', () => {
  const source = '{"a":\n> 1}', decoded = '{"a":\n1}';
  const json = parseGrammarLinks(nativeGrammarText('native-json'));
  const direct = compileGrammar(json).parseTree(decoded);
  assert.equal(direct.ok, true);
  const host = includedRangeGrammar(source, [[0, 6], [8, 10]], ['JSON'], 'any-grammar', '', '');
  const outcome = compileGrammar(host, { resolveGrammar: () => json }).parseTree(source);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  const embedded = includedRangeEmbedding(outcome.tree);
  assert.deepEqual(embedded.root, direct.tree);
  assert.deepEqual(includedRangePhysicalBytes(outcome.tree), [...Buffer.from(source)]);
  assert.deepEqual(includedRangePhysicalBytes(embedded.root), [...Buffer.from(decoded)]);
  assert.deepEqual(JSON.parse(Buffer.from(includedRangePhysicalBytes(embedded.root)).toString()), { a: 1 });
  assert.equal(checkGrammarSelectionMappedTree({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes }).tree, outcome.tree);
});

test('nested JSON and included ranges preserve all source planes and replay from text and binary grammar archives', () => {
  const physical = '> *é\n> β*\n', source = JSON.stringify(physical);
  const child = includedRangeLiteralGrammar('*é\nβ*');
  const middle = includedRangeGrammar('*é\n> β*', [[0, 4], [6, 9]]);
  const outer = parseGrammarLinks(nativeGrammarText('native-json'));
  const original = outer.startRule().name;
  outer.rules.set('quoted_source', { name: 'quoted_source', kind: 'normal', expression: {
    kind: 'any-grammar', item: { kind: 'ref', name: original },
    items: [{ kind: 'grammar-candidate', name: 'Middle', mapping: 'json-string' }],
  } });
  outer.start = 'quoted_source';
  const grammars = new Map([['Outer', outer], ['Middle', middle], ['Inline', child]]);
  const outcome = compileGrammar(outer, { resolveGrammar: name => grammars.get(name) }).parseTree(source);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  assert.deepEqual(outcome.sourcePlanes.map(plane => plane.byteLength), [Buffer.byteLength(source), 12, 7]);
  assert.equal(outcome.sourcePlanes[1].derivation.operation, 'json-string');
  assert.deepEqual(Object.keys(outcome.sourcePlanes[1].derivation), ['operation', 'input', 'start', 'end']);
  assert.deepEqual(outcome.sourcePlanes[2].derivation,
    { operation: 'included-ranges', input: 1, start: 2, end: 11, ranges: [[0, 4], [6, 9]] });
  assert.deepEqual(includedRangePhysicalBytes(outcome.tree), [...Buffer.from(source)]);
  assert.deepEqual(includedRangePhysicalBytes(includedRangeEmbedding(outcome.tree).root), [42, 195, 169, 10, 206, 178, 42]);
  const bundle = { tree: outcome.tree, sourcePlanes: outcome.sourcePlanes };
  assert.equal(checkGrammarSelectionMappedTree(bundle), bundle);
  const archive = { grammars: [...grammars].map(([name, grammar]) => ({ name, document: includedRangeGrammarDocument(grammar) })), bundle };
  for (const decoded of includedRangeStoredVariants(archive)) {
    assert.equal(checkGrammarSelectionMappedTree(decoded.bundle), decoded.bundle);
    const restored = new Map(decoded.grammars.map(row => [row.name, row.document]));
    const replay = compileGrammar(restored.get('Outer'), { resolveGrammar: name => restored.get(name) }).parseTree(source);
    assert.equal(replay.ok, true, JSON.stringify(replay.rejection));
    assert.deepEqual({ tree: replay.tree, sourcePlanes: replay.sourcePlanes }, decoded.bundle);
  }
});

test('different source views do not share a child memo and failed attempts keep their actual source definitions', () => {
  const host = includedRangeGrammar('xy!', [[0, 1]], ['Letter'], 'any-grammar', '', '');
  const original = host.rules.get('document'), first = original.expression.items[1];
  const second = structuredClone(first);
  second.items[0].ranges = [{ kind: 'source-range', start: 1, end: 2 }];
  host.rules.set('document', { ...original, expression: { kind: 'choice', ordered: true, items: [first, second] } });
  const child = includedRangeLiteralGrammar('y');
  const outcome = compileGrammar(host, { resolveGrammar: () => child }).parseTree('xy!');
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  assert.equal(outcome.sourcePlanes.length, 3);
  assert.deepEqual(outcome.sourcePlanes.slice(1).map(plane => plane.derivation.ranges), [[[0, 1]], [[1, 2]]]);
  assert.equal(includedRangeEmbedding(outcome.tree).selection.sourceMapping.output, 2);
  assert.deepEqual(includedRangePhysicalBytes(includedRangeEmbedding(outcome.tree).root), [121]);
  assert.equal(checkGrammarSelectionMappedTree({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes }).tree, outcome.tree);
});

test('archived range forgeries fail source correspondence checks on the actual physical source', () => {
  const host = includedRangeGrammar('*é\n> β*', [[0, 4], [6, 9]]);
  const child = includedRangeLiteralGrammar('*é\nβ*');
  const outcome = compileGrammar(host, { resolveGrammar: () => child }).parseTree('> *é\n> β*\n');
  assert.equal(outcome.ok, true);
  const bundle = { tree: outcome.tree, sourcePlanes: outcome.sourcePlanes };
  const mutations = [
    stored => { includedRangeEmbedding(stored.tree).selection.sourceMapping.ranges = [[0, 3], [6, 9]]; },
    stored => { stored.sourcePlanes[1].derivation.ranges.reverse(); },
    stored => { stored.sourcePlanes[1].derivation.ranges[0][1] = 2; },
    stored => { stored.sourcePlanes[1].derivation.ranges[0].push(1); },
    stored => { includedRangeEmbedding(stored.tree).selection.sourceMapping.intervals[4][2] = 3; },
    stored => { includedRangeEmbedding(stored.tree).selection.sourceMapping.input = 1; },
    stored => { includedRangeEmbedding(stored.tree).selection.sourceMapping.hidden = 'extra'; },
    stored => { includedRangeEmbedding(stored.tree).root.end = 6; },
  ];
  for (const mutate of mutations) {
    const counterfeit = structuredClone(bundle);
    mutate(counterfeit);
    for (const decoded of includedRangeStoredVariants(counterfeit)) {
      assert.throws(() => checkGrammarSelectionMappedTree(decoded), /mapped grammar CST:/u);
    }
  }
  let reads = 0;
  const accessor = structuredClone(bundle);
  Object.defineProperty(accessor.sourcePlanes[1].derivation.ranges[0], '0', { enumerable: true, get() { reads += 1; return 0; } });
  assert.throws(() => checkGrammarSelectionMappedTree(accessor), /non-data field/u);
  assert.equal(reads, 0);
});

test('range profiles reject scalar cuts, nonprogress and inconsistent candidate plans before executing child grammars', () => {
  const region = '*é\n> β*';
  for (const ranges of [[[0, 2]], [[0, 10]], [[6, 9], [0, 4]], [[0, 5], [4, 9]], [[0, 9]], [[0, 4], [4, 9]]]) {
    let calls = 0;
    const parser = compileGrammar(includedRangeGrammar(region, ranges), {
      resolveGrammar: () => { calls += 1; return includedRangeLiteralGrammar('*é\nβ*'); },
    });
    const failure = parser.parseTree('> ' + region + '\n');
    assert.equal(failure.ok, false);
    assert.equal(calls, 0);
    assert.equal(failure.rejection.reason, 'invalidEmbeddingMapping');
    assert.equal(failure.rejection.diagnostics[0].reason, 'invalidEmbeddingMapping');
    assert.ok(failure.rejection.diagnostics.every(diagnostic => !Object.hasOwn(diagnostic, 'language')));
  }
  const host = includedRangeGrammar(region, [[0, 4], [6, 9]], ['Inline', 'Different'], 'all-grammar');
  const declaration = host.rules.get('document'), expression = structuredClone(declaration.expression);
  expression.items[1].items[1].ranges = [{ kind: 'source-range', start: 0, end: 1 }];
  host.rules.set('document', { ...declaration, expression });
  let calls = 0;
  assert.equal(compileGrammar(host, { resolveGrammar: () => { calls += 1; return includedRangeLiteralGrammar('*'); } }).parseTree('> ' + region + '\n').ok, false);
  assert.equal(calls, 0);
  const refusing = compileGrammar(includedRangeGrammar(region, [[0, 4], [6, 9]]), {
    resolveGrammar: () => includedRangeLiteralGrammar('!'),
  }).parseTree('> ' + region + '\n');
  assert.equal(refusing.ok, false);
  assert.equal(refusing.rejection.reason, 'syntax');
  assert.ok(refusing.rejection.diagnostics.some(diagnostic => diagnostic.reason === 'mappedSyntax' && diagnostic.language === 'Inline'));
  const parser = compileGrammar(includedRangeGrammar(region, [[0, 4], [6, 9]]), { resolveGrammar: () => includedRangeLiteralGrammar('*é\nβ*') });
  for (const [options, reason] of [[{ stepLimit: 0 }, 'stepLimit'], [{ memoryLimit: 0 }, 'memoryBudget'], [{ maxDepth: 0 }, 'nestingDepth']]) {
    const failure = parser.parseTree('> ' + region + '\n', options);
    assert.equal(failure.ok, false);
    assert.equal(failure.rejection.reason, reason);
  }
});

test('both CST comparisons preserve source range parameters, physical children and every derived witness', () => {
  const host = includedRangeGrammar('*é\n> β*', [[0, 4], [6, 9]], ['First', 'Second'], 'all-grammar');
  const child = includedRangeLiteralGrammar('*é\nβ*');
  const outcome = compileGrammar(host, { resolveGrammar: () => child }).parseTree('> *é\n> β*\n');
  assert.equal(outcome.ok, true);
  const reference = outcome.tree;
  assert.equal(sameGrammarTree(reference, structuredClone(reference)), true);
  assert.equal(sameGrammarOutput([reference], [structuredClone(reference)]), true);
  const edits = [
    embedded => { embedded.selection.sourceMapping.ranges[0][1] = 3; },
    embedded => { embedded.selection.sourceMapping.ranges.reverse(); },
    embedded => { embedded.selection.sourceMapping.intervals[4][2] = 3; },
    embedded => { embedded.selection.sourceMapping.output = 2; },
    embedded => { delete embedded.sourceChildren; },
    embedded => { embedded.sourceChildren[0].end -= 1; },
    embedded => { embedded.root.end -= 1; },
    embedded => { embedded.selection.alternatives[0].root.end -= 1; },
  ];
  for (const edit of edits) {
    const candidate = structuredClone(reference);
    edit(includedRangeEmbedding(candidate));
    assert.equal(sameGrammarTree(reference, candidate), false, edit.toString());
    assert.equal(sameGrammarOutput([reference], [candidate]), false, edit.toString());
  }
});

test('grammar integer zero is canonical while archived source range Numbers retain their signed zero', () => {
  const host = includedRangeGrammar('xy!', [[0, 1]], ['Letter'], 'any-grammar', '', '');
  host.rules.get('document').expression.items[1].items[0].ranges[0].start = -0;
  const outcome = compileGrammar(host, { resolveGrammar: () => includedRangeLiteralGrammar('x') }).parseTree('xy!');
  assert.equal(outcome.ok, true);
  assert.equal(Object.is(includedRangeEmbedding(outcome.tree).selection.sourceMapping.ranges[0][0], 0), true);
  const bundle = structuredClone({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes });
  includedRangeEmbedding(bundle.tree).selection.sourceMapping.ranges[0][0] = -0;
  bundle.sourcePlanes[1].derivation.ranges[0][0] = -0;
  for (const stored of includedRangeStoredVariants(bundle)) {
    assert.equal(Object.is(stored.sourcePlanes[1].derivation.ranges[0][0], -0), true);
    assert.equal(Object.is(includedRangeEmbedding(stored.tree).selection.sourceMapping.ranges[0][0], -0), true);
    assert.equal(checkGrammarSelectionMappedTree(stored), stored);
  }
});
