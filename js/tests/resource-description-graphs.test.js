import assert from 'node:assert/strict';
import test from 'node:test';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { importResourceDescriptionGraph, decodeResourceDescriptionGraph, mergeResourceDescriptionGraphs, verifyResourceDescriptionGraphWitnesses, RESOURCE_DESCRIPTION_SCHEMA as schema } from '../src/resource-description-graphs.js';
import { acceptResourceDescriptionIdentifier, acceptResourceDescriptionLanguage } from '../src/resource-description-rules.js';
import { readProjectData } from '../scripts/project-data.mjs';

const identifier = value => ({ kind: 'identifier', value });
const literal = (value, datatype, language) => ({ kind: 'literal', value, ...(datatype === undefined ? {} : { datatype }), ...(language === undefined ? {} : { language }) });
const blank = value => ({ kind: 'blank', value });
const predicate = identifier('urn:property');
const units = text => Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));
const reload = graph => ({ network: LinkNetwork.fromLino(graph.network.toLino()), root: graph.root });
const decode = graph => decodeResourceDescriptionGraph(graph.network, graph.root);
const links = (graph, kind) => [...graph.network.links()].filter(link => link.metadata().term === schema.terms[kind]);

test('shared semantic graph fixture independently reloads and reproduces both source merges', () => {
  const expected = readProjectData(new URL('../../parity/fixtures/resource-description-graph-expectations.lino', import.meta.url));
  const archived = { network: LinkNetwork.fromLino(readFileSync(new URL('../../parity/fixtures/resource-description-graph.lino', import.meta.url), 'utf8')), root: LinkId.fromU64(expected.root) };
  assert.deepEqual(decode(archived), expected.decoded);
  assert.equal(verifyResourceDescriptionGraphWitnesses(archived.network, archived.root), true);
  const regenerated = mergeResourceDescriptionGraphs(expected.inputs.map(input => importResourceDescriptionGraph(input.triples, input)));
  assert.deepEqual(decode(regenerated), expected.decoded);
  assert.equal(regenerated.network.toLino(), archived.network.toLino());
});

test('resource description rules and semantic fixtures retain automated generator freshness', () => {
  for (const file of ['generate-resource-description-rules.mjs', 'generate-resource-description-fixture.mjs']) {
    const result = spawnSync(process.execPath, [new URL('../scripts/' + file, import.meta.url).pathname, '--check'],
      { encoding: 'utf8', timeout: 20000, env: { ...process.env, TRANSLATION_CHECK_RUST: '0', SELF_TRANSLATION_CHECK_RUST: '0' } });
    assert.equal(result.status, 0, result.stderr || result.stdout);
  }
});

test('RDF abstract graph terms and source-qualified occurrences survive source-free LiNo reload', () => {
  const source = [[identifier('urn:subject'), predicate, literal('hello', undefined, 'EN-gb')],
    [blank('same'), predicate, literal('1', 'http://www.w3.org/2001/XMLSchema#integer')],
    [identifier('urn:subject'), predicate, literal('01', 'http://www.w3.org/2001/XMLSchema#integer')],
    [identifier('urn:unicode:🦀'), predicate, literal('空\0\n')],
    [identifier('urn:subject'), predicate, literal('ill-typed', 'http://www.w3.org/2001/XMLSchema#integer')]];
  const created = importResourceDescriptionGraph(source, { sourceIdentifier: 'source A' });
  const loaded = reload(created);
  const result = decode(loaded);
  assert.equal(result.model, schema.model);
  assert.equal(result.triples.length, 5);
  assert.equal(result.occurrences.length, 5);
  assert.deepEqual(result, decode(created));
  assert.ok(result.triples.some(triple => triple[2].value === '01'));
  assert.ok(result.triples.some(triple => triple[2].value === 'ill-typed'), 'RDF ill-typed lexical forms remain RDF terms rather than being dropped');
  assert.ok(result.triples.some(triple => triple[0].sourceIdentifier === 'source A'));
  assert.ok(result.triples.some(triple => triple[2].language === 'en-gb'));
  assert.ok(result.occurrences.some(value => value.sourceLanguage === 'EN-gb'));
});

test('duplicate statements share actual term and statement links while preserving ordered occurrences', () => {
  const statement = [identifier('urn:subject'), predicate, literal('same')];
  const graph = importResourceDescriptionGraph([statement, statement], { sourceIdentifier: 'source A' });
  assert.equal(links(graph, 'statement').length, 1);
  assert.equal(links(graph, 'literal').length, 1);
  assert.equal(links(graph, 'occurrence').length, 2);
  const result = decode(reload(graph));
  assert.equal(result.triples.length, 1);
  assert.deepEqual(result.occurrences.map(value => value.position), [0, 1]);
});

test('a linked lexical edit changes export without consulting the original structured terms', () => {
  const graph = reload(importResourceDescriptionGraph([[identifier('urn:subject'), predicate, literal('before')]], { sourceIdentifier: 'source A' }));
  const lexical = graph.network.link(links(graph, 'literal')[0].references()[0]);
  lexical.setMetadata(lexical.metadata().withDefinition('after'));
  assert.equal(decode(reload(graph)).triples[0][2].value, 'after');
});

test('merge preserves scoped blank nodes, distinct literal terms, axioms and source equality witnesses', () => {
  const first = importResourceDescriptionGraph([[blank('same'), predicate, literal('1', 'http://www.w3.org/2001/XMLSchema#integer')],
    [identifier('urn:left'), identifier('http://www.w3.org/2002/07/owl#sameAs'), identifier('urn:right')],
    [identifier('urn:subject'), predicate, literal('same')]], { sourceIdentifier: 'source A' });
  const second = importResourceDescriptionGraph([[blank('same'), predicate, literal('01', 'http://www.w3.org/2001/XMLSchema#integer')],
    [identifier('urn:subject'), predicate, literal('same')]], { sourceIdentifier: 'source B' });
  const merged = reload(mergeResourceDescriptionGraphs([reload(first), reload(second)]));
  const result = decode(merged);
  assert.equal(result.triples.length, 4);
  assert.equal(result.occurrences.length, 5);
  assert.equal(result.triples.filter(triple => triple[0].kind === 'blank').length, 2);
  assert.deepEqual(new Set(result.triples.filter(triple => triple[0].kind === 'blank').map(triple => triple[0].sourceIdentifier)), new Set(['source A', 'source B']));
  const axiom = result.triples.find(triple => triple[1].value.endsWith('#sameAs'));
  assert.equal(axiom[0].value, 'urn:left');
  assert.equal(axiom[2].value, 'urn:right', 'an OWL predicate does not silently authorize identity sharing under the RDF term model');
  assert.ok(links(merged, 'witness').length > 0);
  assert.equal(verifyResourceDescriptionGraphWitnesses(merged.network, merged.root), true);
});

test('counterfeit or dangling equality premises, foreign models and concealed fields are rejected', () => {
  const input = importResourceDescriptionGraph([[identifier('urn:subject'), predicate, literal('one')]], { sourceIdentifier: 'source A' });
  const merged = mergeResourceDescriptionGraphs([input]);
  const counterfeit = reload(merged);
  const witness = links(counterfeit, 'witness')[0];
  const literalTarget = links(counterfeit, 'literal').at(-1).id();
  witness.setReferences([witness.references()[0], literalTarget, witness.references()[2]]);
  assert.throws(() => decode(counterfeit), /witness/u);
  const dangling = reload(input);
  links(dangling, 'statement')[0].setReferences([LinkId.fromU64(999999), ...links(dangling, 'statement')[0].references().slice(1)]);
  assert.throws(() => decode(dangling), /reference|term/u);
  const foreign = reload(input);
  links(foreign, 'model')[0].setMetadata(links(foreign, 'model')[0].metadata().withDefinition('OWL identity entailment'));
  assert.throws(() => decode(foreign), /premises/u);
  const concealed = reload(input);
  concealed.network.insertFieldRelation(concealed.root, 'unexpected semantic field', concealed.root);
  assert.throws(() => decode(concealed), /conceal/u);
});

test('role violations, unknown term fields, malformed Unicode and inconsistent language datatypes refuse import', () => {
  const bad = [
    [literal('subject'), predicate, literal('object')],
    [identifier('urn:subject'), blank('property'), literal('object')],
    [identifier('urn:subject'), predicate, identifier('relative/path')],
    [identifier('urn:subject'), predicate, literal('\ud800')],
    [identifier('urn:subject'), predicate, literal('hi', schema.stringDatatype, 'en')],
    [identifier('urn:subject'), predicate, literal('hi', schema.languageDatatype)],
    [identifier('urn:subject'), predicate, literal('hi', undefined, 'en-a')],
    [identifier('urn:subject'), predicate, { kind: 'identifier', value: 'urn:a', opaquePayload: '{}' }],
  ];
  for (const triple of bad) assert.throws(() => importResourceDescriptionGraph([triple], { sourceIdentifier: 'source A' }));
  let accessed = false;
  const accessor = { kind: 'literal', get value() { accessed = true; return 'hidden'; } };
  assert.throws(() => importResourceDescriptionGraph([[identifier('urn:subject'), predicate, accessor]], { sourceIdentifier: 'source A' }), /accessor/u);
  assert.equal(accessed, false);
  assert.throws(() => importResourceDescriptionGraph([[blank('x'), predicate, literal('x')]], { sourceIdentifier: '' }), /source/u);
});

test('identifier and language lexical rules preserve exact spelling and reject malformed or unsupported forms', () => {
  for (const value of ['urn:a', 'https://example.test/空?x=🦀#part', 'urn:a%20b', 'scheme:']) assert.equal(acceptResourceDescriptionIdentifier(units(value)), true, value);
  for (const value of ['relative', 'urn:a b', 'urn:a%GG', 'https://user@other@example.test/a', 'http://[::1]/', 'urn:a#b#c']) assert.equal(acceptResourceDescriptionIdentifier(units(value)), false, value);
  for (const value of ['en', 'en-US', 'zh-Hant-TW', 'de-CH-1901', 'en-a-example-x-private', 'x-private']) assert.equal(acceptResourceDescriptionLanguage(units(value), false), true, value);
  for (const value of ['', 'e', 'en-', 'en-a', 'en-a-one-a-two', 'sl-rozaj-rozaj', 'en-💬']) assert.equal(acceptResourceDescriptionLanguage(units(value), false), false, value);
});

test('source provenance cycles and explicit graph traversal budgets reject promptly', () => {
  const graph = importResourceDescriptionGraph([[identifier('urn:subject'), predicate, literal('same')]], { sourceIdentifier: 'source A' });
  assert.throws(() => decodeResourceDescriptionGraph(graph.network, graph.root, { nodeLimit: 2 }), /limit/u);
  const malformed = reload(graph);
  const sources = malformed.network.link(malformed.network.link(malformed.root).references()[3]);
  sources.setReferences([malformed.root]);
  assert.throws(() => decode(malformed), /cyclic/u);
});
