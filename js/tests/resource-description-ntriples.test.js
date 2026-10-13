import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId, LinkMetadata, LinkType } from '../src/primitives.js';
import { importResourceDescriptionGraph, decodeResourceDescriptionGraph, mergeResourceDescriptionGraphs } from '../src/resource-description-graphs.js';
import { importResourceDescriptionNTriples, emitResourceDescriptionNTriples, readResourceDescriptionNTriplesSource,
  RESOURCE_DESCRIPTION_NTRIPLES_PROFILE, ResourceDescriptionNTriplesError } from '../src/resource-description-ntriples.js';
import { readResourceDescriptionNTriplesScalar, readResourceDescriptionNTriplesToken } from '../src/resource-description-ntriples-rules.js';
import { readResourceDescriptionNTriplesTransition, readResourceDescriptionNTriplesRole } from '../src/resource-description-ntriples-characters.js';
import { parseProgrammingLanguage } from '../src/programming-language-parser.js';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

const fixture = readProjectData(new URL('../../parity/fixtures/resource-description-ntriples.lino', import.meta.url));
const reload = graph => ({ ...graph, network: LinkNetwork.fromLino(graph.network.toLino()) });
const decode = graph => decodeResourceDescriptionGraph(graph.network, graph.root);
const archive = item => ({ network: LinkNetwork.fromLino(item.archive), root: LinkId.from(item.root), syntaxRoot: LinkId.from(item.syntaxRoot) });
const sortedTriples = triples => triples.map(triple => JSON.stringify(triple)).sort();
function replaceDefinition(network, id, before, after) {
  const encoded = encodeURIComponent;
  return LinkNetwork.fromLino(network.toLino().split('\n').map(line => line.startsWith('(' + id + ': ')
    ? line.replace('(def: ' + encoded(before) + ')', '(def: ' + encoded(after) + ')') : line).join('\n'));
}
function replaceReferences(network, id, refs) {
  return LinkNetwork.fromLino(network.toLino().split('\n').map(line => line.startsWith('(' + id + ': ')
    ? '(' + id + ': ' + refs.map(String).join(' ') + ' ' + line.slice(line.indexOf('(meta:')) : line).join('\n'));
}

test('independent N-Triples source oracles import and reload with all original syntax', () => {
  for (const item of fixture.cases) {
    for (const graph of [reload(importResourceDescriptionNTriples(item.source, { sourceIdentifier: 'syntax-fixture' })), archive(item)]) {
      assert.deepEqual(sortedTriples(decode(graph).triples), sortedTriples(item.triples), item.id);
      assert.equal(readResourceDescriptionNTriplesSource(graph.network, graph.syntaxRoot), item.source, item.id);
      assert.equal(emitResourceDescriptionNTriples(graph.network, graph.root).source, item.canonical, item.id);
    }
  }
  const duplicate = archive(fixture.cases.find(item => item.id === 'unicode-language-duplicates'));
  assert.equal(decode(duplicate).triples.length, 1);
  assert.deepEqual(decode(duplicate).occurrences.map(value => value.sourceLanguage), ['EN-us', 'en-US']);
});

test('canonical N-Triples conversion restores scoped blank identities through actual linked correspondences', () => {
  for (const item of fixture.cases) {
    const initial = archive(item);
    const emitted = emitResourceDescriptionNTriples(initial.network, initial.root);
    const correspondence = reload(emitted.correspondence);
    const returned = reload(importResourceDescriptionNTriples(emitted.source, { sourceIdentifier: 'another-source', correspondence }));
    assert.deepEqual(sortedTriples(decode(returned).triples), sortedTriples(item.triples), item.id);
  }
  const separate = ['first-scope', 'second-scope'].map(sourceIdentifier => importResourceDescriptionNTriples('_:same <urn:p> <urn:o> .', { sourceIdentifier }));
  const merged = mergeResourceDescriptionGraphs(separate);
  const converted = emitResourceDescriptionNTriples(merged.network, merged.root);
  const restored = importResourceDescriptionNTriples(converted.source, { sourceIdentifier: 'third-scope', correspondence: reload(converted.correspondence) });
  assert.deepEqual(sortedTriples(decode(restored).triples), sortedTriples(decode(merged).triples));
  assert.equal(decode(restored).triples.length, 2);
  const item = fixture.cases.find(item => item.id === 'scoped-blank-label');
  const plain = importResourceDescriptionNTriples(item.canonical, { sourceIdentifier: 'other-scope' });
  assert.ok(decode(plain).triples[0].filter(term => term.kind === 'blank').every(term => term.sourceIdentifier === 'other-scope'));
});

test('edited semantic links change canonical syntax without replaying retained source', () => {
  const item = fixture.cases.find(item => item.id === 'adjacent-identifiers');
  const initial = archive(item);
  const value = initial.network.links().find(link => link.metadata().language === 'meta-language resource description graph' && link.metadata().definition === 'urn:o');
  const network = replaceDefinition(initial.network, value.id(), 'urn:o', 'urn:edited');
  assert.equal(readResourceDescriptionNTriplesSource(network, initial.syntaxRoot), item.source);
  assert.equal(emitResourceDescriptionNTriples(network, initial.root).source, '<urn:s> <urn:p> <urn:edited> .\n');
  const onlySemantics = importResourceDescriptionGraph(decode({ network, root: initial.root }).triples, { sourceIdentifier: 'no-syntax' });
  assert.equal(emitResourceDescriptionNTriples(onlySemantics.network, onlySemantics.root).source, '<urn:s> <urn:p> <urn:edited> .\n');
});

test('malformed syntax and unsupported identifier profiles have precise separate diagnostics', () => {
  for (const item of [...fixture.malformed, ...fixture.unsupported]) {
    assert.throws(() => importResourceDescriptionNTriples(item.source, { sourceIdentifier: 'test' }), error => error instanceof ResourceDescriptionNTriplesError
      && error.kind === item.kind && Number.isSafeInteger(error.offset) && error.offset >= 0 && error.offset <= item.source.length, item.id);
  }
  for (const source of ['\uD800', '<urn:s> <urn:p> "\uDC00" .', '# \uD800']) {
    assert.throws(() => importResourceDescriptionNTriples(source, { sourceIdentifier: 'test' }), /invalid Unicode scalar/u);
  }
  const source = '<urn:s> <urn:p> "\\uD800" .';
  assert.throws(() => importResourceDescriptionNTriples(source, { sourceIdentifier: 'test' }), error => error.offset === source.indexOf('\\'));
});

test('lexical scalar kernel preserves astral escapes, dot boundaries and literal controls', () => {
  const units = value => Array.from({ length: value.length }, (_, index) => value.charCodeAt(index));
  assert.deepEqual(readResourceDescriptionNTriplesScalar(units('\\U0001F980'), 0, 3), [10, 0x1f980, 0]);
  assert.deepEqual(readResourceDescriptionNTriplesScalar(units('🦀'), 0, 0), [2, 0x1f980, 0]);
  assert.deepEqual(readResourceDescriptionNTriplesToken(units('_:x..'), 0), [2, 3, 0]);
  assert.deepEqual(readResourceDescriptionNTriplesScalar(units('\\uDFFF'), 0, 1), [0, -1, 4]);
  const graph = importResourceDescriptionNTriples('<urn:s> <urn:p> "a\tb" .', { sourceIdentifier: 's' });
  assert.equal(emitResourceDescriptionNTriples(graph.network, graph.root).source, '<urn:s> <urn:p> "a\tb" .\n');
});

test('foreign syntax roles, concealed fields and forged blank correspondences are refused', () => {
  const item = fixture.cases.find(item => item.id === 'scoped-blank-label');
  const graph = archive(item);
  graph.network.insertFieldRelation(graph.syntaxRoot, 'concealed', graph.root);
  assert.throws(() => readResourceDescriptionNTriplesSource(graph.network, graph.syntaxRoot), /concealed fields/u);
  const identities = { network: LinkNetwork.fromLino(item.correspondenceArchive), root: LinkId.from(item.correspondenceRoot) };
  const pair = identities.network.links().find(link => link.metadata().term === RESOURCE_DESCRIPTION_NTRIPLES_PROFILE.terms.correspondence);
  const foreign = identities.network.insertLink([], LinkMetadata.new().withLinkType(LinkType.Semantic).withNamed(true)
    .withLanguage('meta-language resource description graph').withTerm('blank node'));
  identities.network = replaceReferences(identities.network, pair.id(), [foreign, pair.references()[1]]);
  assert.throws(() => importResourceDescriptionNTriples(item.canonical, { sourceIdentifier: 's', correspondence: identities }), /not owned/u);
  const duplicate = { network: LinkNetwork.fromLino(item.correspondenceArchive), root: LinkId.from(item.correspondenceRoot) };
  const pairs = duplicate.network.link(duplicate.network.link(duplicate.root).references()[1]);
  duplicate.network = replaceReferences(duplicate.network, pairs.id(), [pairs.references()[0], pairs.references()[0]]);
  assert.throws(() => importResourceDescriptionNTriples(item.canonical, { sourceIdentifier: 's', correspondence: duplicate }), /one to one/u);
});

test('source and token resources are bounded without a full corpus', () => {
  assert.throws(() => importResourceDescriptionNTriples('<urn:s> <urn:p> "' + 'x'.repeat(65537) + '" .', { sourceIdentifier: 's' }), error => error.kind === 'resourceLimit');
  assert.throws(() => importResourceDescriptionNTriples(' '.repeat(262145), { sourceIdentifier: 's' }), error => error.kind === 'resourceLimit');
  assert.throws(() => importResourceDescriptionNTriples('\n '.repeat(5001), { sourceIdentifier: 's' }), error => error.kind === 'resourceLimit');
});

test('reviewed native transition tables translate back and retain all scalar equality decisions', () => {
  const source = readFileSync(new URL('../../rust/src/resource_description_ntriples_rules/part_0.rs', import.meta.url), 'utf8');
  const tree = parseProgrammingLanguage(source, 'Rust').tree;
  const values = [...Array.from({ length: 11 }, (_, index) => index), -0, -1, .5, NaN, Infinity, -Infinity];
  for (const [name, expected] of [
    ['read_resource_description_n_triples_transition', readResourceDescriptionNTriplesTransition],
    ['read_resource_description_n_triples_role', readResourceDescriptionNTriplesRole],
  ]) {
    const item = tree.children.find(node => node.term === 'function_item' && node.children.some(child =>
      child.field === 'name' && source.slice(child.span.byteRange.start, child.span.byteRange.end) === name));
    assert.ok(item, 'the original shared scalar classifier remains represented');
    const range = item.span.byteRange;
    const program = checkProgram(parseRust(source.slice(range.start, range.end) + '\nfn main() {}'));
    const code = emitJavaScript(program).text;
    const actual = new Function(code + '\nreturn ' + name + ';')();
    for (const state of values) for (const kind of values) assert.ok(Object.is(actual(state, kind), expected(state, kind)), name + ': ' + state + ', ' + kind);
  }
});
