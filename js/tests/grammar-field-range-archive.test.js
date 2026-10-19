import assert from 'node:assert/strict';
import test from 'node:test';
import { compileGrammar, serializeGrammar, deserializeGrammar } from '../src/grammar.js';
import { parseGrammarLinks, renderGrammarLinks, percentEncodeLinksText } from '../src/grammar-links.js';
import { parseNativeGrammar, renderNativeGrammar } from '../src/grammar-interchange.js';
import { checkGrammarSelectionMappedTree } from '../src/grammar-runtime.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks, readGlobalConceptSpaceLinks,
  createGlobalConceptSpaceParser } from '../src/global-concept-space.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks, readStructuredDataLino } from '../src/structured-data-links.js';

function fieldArchiveGrammar(policy = 'any-grammar', mode = 'include') {
  return parseGrammarLinks(`(grammar (start document))\n(rule document normal (${policy} (seq (literal %3C) (capture labeled body (literal %CF%80)) (literal %3E)) (grammar-field-candidate Child field-ranges (source-field-selector body ${mode} 0 1))))\n`);
}

function fieldArchiveChild(source) {
  return parseGrammarLinks(`(grammar (start decoded))\n(rule decoded normal (literal ${percentEncodeLinksText(source)}))\n`);
}

function fieldArchiveCandidate(grammar) {
  return grammar.startRule().expression.items[0];
}

// Retain the original records until the API under test performs its own
// preflight. Calling normalized() here would hide the data being tested.
function fieldArchiveDocument(grammar) {
  return { schemaVersion: 1, ...Object.fromEntries(Object.entries(grammar).map(([key, value]) =>
    [key, key === 'rules' ? [...value.values()] : value])) };
}

function fieldArchiveBoundaries(grammar) {
  return {
    compile: () => compileGrammar(grammar),
    links: () => renderGrammarLinks(grammar),
    native: () => renderNativeGrammar(grammar),
    jsonWrite: () => serializeGrammar(grammar),
    jsonRead: () => deserializeGrammar(fieldArchiveDocument(grammar)),
    global: () => createGlobalConceptSpace([{ id: 'Host', language: 'Host', grammar }], { records: [] }),
  };
}

function fieldArchiveStoredVariants(value) {
  const packet = encodeStructuredDataLinks(value), text = packet.network.toLino();
  const network = LinkNetwork.fromLino(text);
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(network));
  assert.equal(binary.toLino(), text);
  return [readStructuredDataLino(text), decodeStructuredDataLinks(binary, packet.root)];
}

function fieldArchivePhysicalBytes(tree) {
  if (tree.type === 'missing') return [];
  if (tree.type === 'token' || tree.type === 'error') return tree.text === null
    ? [...Buffer.from(tree.hex, 'hex')] : [...Buffer.from(tree.text)];
  return (tree.sourceChildren ?? tree.children ?? (tree.root ? [tree.root] : [])).flatMap(fieldArchivePhysicalBytes);
}

function fieldArchiveEmbedding(tree) {
  if (tree.type === 'embed' && tree.selection?.mapping === 'field-ranges') return tree;
  for (const child of tree.children ?? (tree.root ? [tree.root] : [])) {
    const found = fieldArchiveEmbedding(child);
    if (found) return found;
  }
  return null;
}

test('field candidate and selector definitions survive each native codec and source-free global archive', () => {
  for (const policy of ['any-grammar', 'all-grammar']) for (const mode of ['include', 'exclude']) {
    const host = fieldArchiveGrammar(policy, mode), selected = mode === 'include' ? 'π' : '<>';
    const child = fieldArchiveChild(selected), links = renderGrammarLinks(host), native = renderNativeGrammar(host);
    assert.equal(renderGrammarLinks(parseGrammarLinks(links)), links);
    assert.equal(renderNativeGrammar(parseNativeGrammar(native)), native);
    for (const restored of [host, parseGrammarLinks(links), parseNativeGrammar(native),
      deserializeGrammar(serializeGrammar(host)), deserializeGrammar(fieldArchiveDocument(host))]) {
      // These codecs normalize grammar-level source metadata. The closed
      // expression fields and their order are the round-trip contract here.
      assert.deepEqual(restored.startRule().expression, host.startRule().expression);
      const outcome = compileGrammar(restored, { resolveGrammar: () => child }).parseTree('<π>');
      assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
      const embedding = fieldArchiveEmbedding(outcome.tree);
      assert.deepEqual(fieldArchivePhysicalBytes(outcome.tree), [...Buffer.from('<π>')]);
      assert.deepEqual(fieldArchivePhysicalBytes(embedding.root), [...Buffer.from(selected)]);
      assert.deepEqual(embedding.selection.sourceMapping.selector, fieldArchiveCandidate(host).selector);
      assert.deepEqual(embedding.selection.sourceMapping.occurrences,
        [{ path: ['children', 1], start: 1, end: 3 }]);
      assert.equal(checkGrammarSelectionMappedTree(outcome), outcome);
    }
    const sources = [{ id: 'Host', language: 'Host', grammar: host },
      { id: 'Child', language: 'Child', grammar: child }];
    const space = createGlobalConceptSpace(sources, { records: [] });
    const expected = compileGrammar(host, { grammarName: 'Host', globalConceptSpace: space,
      resolveGrammar: name => sources.find(source => source.id === name)?.grammar }).parseTree('<π>');
    assert.equal(expected.ok, true);
    const archive = globalConceptSpaceToLinks(space);
    host.rules.clear();
    child.rules.clear();
    sources.length = 0;
    for (const stored of fieldArchiveStoredVariants({ archive })) {
      const actual = createGlobalConceptSpaceParser(stored.archive, 'Host', { records: [] }).parseTree('<π>');
      assert.deepEqual(actual, expected);
      assert.equal(checkGrammarSelectionMappedTree(actual), actual);
    }
  }
});

test('closed field records refuse undeclared or malformed data before all grammar projection boundaries', () => {
  const mutations = [
    ['candidate extra', candidate => { candidate.extra = true; }],
    ['candidate undefined extra', candidate => { candidate.extra = undefined; }],
    ['candidate symbol extra', candidate => { candidate[Symbol('extra')] = true; }],
    ['candidate hidden extra', candidate => { Object.defineProperty(candidate, 'extra', { value: true }); }],
    ['selector extra', candidate => { candidate.selector.extra = true; }],
    ['selector undefined extra', candidate => { candidate.selector.extra = undefined; }],
    ['selector symbol extra', candidate => { candidate.selector[Symbol('extra')] = true; }],
    ['selector hidden extra', candidate => { Object.defineProperty(candidate.selector, 'extra', { value: true }); }],
    ['missing selector', candidate => { delete candidate.selector; }],
    ['missing label', candidate => { delete candidate.selector.label; }],
    ['missing first', candidate => { delete candidate.selector.first; }],
    ['missing count', candidate => { delete candidate.selector.count; }],
    ['wrong candidate kind', candidate => { candidate.kind = 'grammar-range-candidate'; }],
    ['wrong selector kind', candidate => { candidate.selector.kind = 'source-range'; }],
    ['wrong mode', candidate => { candidate.selector.mode = 'all'; }],
    ['wrong mapping', candidate => { candidate.mapping = 'included-ranges'; }],
    ['nontext name', candidate => { candidate.name = 7; }],
    ['empty name', candidate => { candidate.name = ''; }],
    ['boxed mapping', candidate => { candidate.mapping = new String('field-ranges'); }],
    ['empty label', candidate => { candidate.selector.label = ''; }],
    ['boxed label', candidate => { candidate.selector.label = new String('body'); }],
    ['array selector', candidate => { candidate.selector = []; }],
    ['sparse selector', candidate => { candidate.selector = new Array(5); }],
    ['inherited selector', candidate => { Object.setPrototypeOf(candidate.selector, { inherited: true }); }],
    ['inherited candidate', candidate => { Object.setPrototypeOf(candidate, { inherited: true }); }],
  ];
  for (const [name, mutate] of mutations) {
    const host = fieldArchiveGrammar();
    mutate(fieldArchiveCandidate(host));
    for (const [boundary, operation] of Object.entries(fieldArchiveBoundaries(host))) {
      assert.throws(operation, undefined, name + ':' + boundary);
    }
  }
  for (const [key, values] of [['first', ['0', -1, 0.5, NaN, Infinity, -Infinity, 4294967296, null, undefined]],
    ['count', ['1', -1, 0, -0, 0.5, NaN, Infinity, -Infinity, 4294967296, null, undefined]]]) {
    for (const value of values) {
      const host = fieldArchiveGrammar();
      fieldArchiveCandidate(host).selector[key] = value;
      for (const [boundary, operation] of Object.entries(fieldArchiveBoundaries(host))) {
        assert.throws(operation, undefined, key + ':' + String(value) + ':' + boundary);
      }
    }
  }
});

test('field form accessors, serialization hooks and numeric coercions are never invoked by grammar readers or writers', () => {
  for (const location of ['candidate', 'selector', 'rule']) for (const transformation of ['accessor', 'toJSON']) {
    let calls = 0;
    const host = fieldArchiveGrammar(), candidate = fieldArchiveCandidate(host);
    const rule = { ...host.startRule() };
    host.rules.set(rule.name, rule);
    const owner = location === 'candidate' ? candidate : location === 'selector' ? candidate.selector : rule;
    const key = location === 'candidate' ? 'selector' : location === 'selector' ? 'first' : 'expression';
    const value = owner[key];
    if (transformation === 'accessor') Object.defineProperty(owner, key, {
      enumerable: true, get() { calls += 1; return value; },
    });
    else owner.toJSON = () => { calls += 1; return {}; };
    for (const [boundary, operation] of Object.entries(fieldArchiveBoundaries(host))) {
      // Rule-level raw-data preflight precedes normalization in these APIs.
      // Expression codecs own candidate/selector descriptors themselves.
      if (location === 'rule' && (boundary === 'links' || boundary === 'native')) continue;
      assert.throws(operation, undefined, location + ':' + transformation + ':' + boundary);
      assert.equal(calls, 0, location + ':' + transformation + ':' + boundary);
    }
  }
  for (const key of ['first', 'count']) {
    let calls = 0;
    const host = fieldArchiveGrammar();
    fieldArchiveCandidate(host).selector[key] = { valueOf() { calls += 1; return 1; },
      toString() { calls += 1; return '1'; }, toJSON() { calls += 1; return 1; } };
    for (const operation of Object.values(fieldArchiveBoundaries(host))) assert.throws(operation);
    assert.equal(calls, 0);
  }
});

test('global archive writers and readers validate the original closed field definitions before rebuilding grammars', () => {
  for (const mutate of [candidate => { candidate.extra = undefined; },
    candidate => { candidate.selector[Symbol('extra')] = true; },
    candidate => { candidate.selector.count = '1'; },
    candidate => { Object.defineProperty(candidate.selector, 'extra', { value: true }); }]) {
    const host = fieldArchiveGrammar();
    const space = createGlobalConceptSpace([{ id: 'Host', language: 'Host', grammar: host }], { records: [] });
    const definition = space.concepts.flatMap(concept => concept.definitions)
      .find(entry => entry.origin === 'Host:document').definition;
    mutate(definition.expression.items[0]);
    assert.throws(() => globalConceptSpaceToLinks(space));
  }
  const host = fieldArchiveGrammar(), child = fieldArchiveChild('π');
  const archive = globalConceptSpaceToLinks(createGlobalConceptSpace([
    { id: 'Host', language: 'Host', grammar: host }, { id: 'Child', language: 'Child', grammar: child },
  ], { records: [] }));
  const selector = '(source-field-selector body include 0 1)';
  assert.ok(archive.includes(selector));
  for (const replacement of ['(source-field-selector body include -1 1)',
    '(source-field-selector body include 0 0)', '(source-field-selector body include 0.5 1)',
    '(source-field-selector body include 0 4294967296)', '(source-field-selector body other 0 1)',
    '(source-field-selector % include 0 1)', '(source-field-selector body include 0 1 extra)']) {
    const malformed = archive.replace(selector, replacement);
    assert.throws(() => readGlobalConceptSpaceLinks(malformed, { records: [] }));
    assert.throws(() => createGlobalConceptSpaceParser(malformed, 'Host', { records: [] }));
  }
});

test('field selector integer zero is canonical across both codecs, candidate agreement and source-free replay', () => {
  const host = fieldArchiveGrammar('all-grammar'), first = fieldArchiveCandidate(host);
  first.selector.first = -0;
  host.startRule().expression.items.push({ ...first, name: 'Twin', selector: { ...first.selector, first: 0 } });
  assert.equal(Object.is(first.selector.first, -0), true);
  const child = fieldArchiveChild('π');
  for (const restored of [host, parseGrammarLinks(renderGrammarLinks(host)),
    parseNativeGrammar(renderNativeGrammar(host)), deserializeGrammar(serializeGrammar(host))]) {
    const outcome = compileGrammar(restored, { resolveGrammar: () => child }).parseTree('<π>');
    assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
    assert.equal(fieldArchiveEmbedding(outcome.tree).selection.alternatives.length, 1);
    assert.equal(Object.is(fieldArchiveEmbedding(outcome.tree).selection.sourceMapping.selector.first, 0), true);
    assert.equal(Object.is(outcome.sourcePlanes[1].derivation.selector.first, 0), true);
    for (const stored of fieldArchiveStoredVariants({ tree: outcome.tree, sourcePlanes: outcome.sourcePlanes })) {
      assert.equal(Object.is(stored.sourcePlanes[1].derivation.selector.first, 0), true);
      assert.equal(checkGrammarSelectionMappedTree(stored), stored);
    }
  }
  assert.equal(Object.is(first.selector.first, -0), true, 'normalizing a view does not mutate the original grammar');
});
