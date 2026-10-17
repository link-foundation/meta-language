import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from '../src/discovered-concept-links.js';
import { acceptLegacyDiscoveredConceptFormat } from '../src/discovered-concept-rules.js';
import { deserializeGrammar, Grammar, parseWithGrammar } from '../src/grammar.js';
import { readFormalConceptDefinition } from '../src/formal-concept-definition.js';
import { LinkNetwork } from '../src/network.js';
import { decodeStructuredDataLinks, encodeStructuredDataLinks } from '../src/structured-data-links.js';

const fixture = readProjectData(new URL('../../parity/fixtures/discovered-concept-links.json', import.meta.url));
const legacyFile = new URL('../../parity/fixtures/discovered-concept-links-legacy.lino', import.meta.url);
const legacyFixture = readProjectData(legacyFile);
const reload = (data = fixture) => ({ network: LinkNetwork.fromLino(data.links), root: data.root });
const role = (network, term) => network.links().filter(link => link.metadata().language === 'meta-language discovered concepts' && link.metadata().term === term);

function withDiscoveredSourceFormats(data) {
  const formats = new Map(data.sources.map(source => [source.id, source.grammar.sourceFormat]));
  return { ...data.expectedSpace, sources: data.expectedSpace.sources.map(source => ({ ...source,
    ...(formats.get(source.id) == null ? {} : { sourceFormat: formats.get(source.id) }) })) };
}

function discoveredTreeBytes(tree) {
  return tree.type === 'node' ? tree.children.flatMap(discoveredTreeBytes) : [...new TextEncoder().encode(tree.text)];
}

// Edit only the actual linked data envelope. Other formal islands, candidate
// origins and certificate membership remain untouched and fully revalidated.
function editDiscoveredEnvelope(packet, edit) {
  const { network, root } = packet;
  const dataRoot = network.link(root).references()[0];
  const ids = new Set();
  const pending = [dataRoot];
  const island = new LinkNetwork();
  for (let at = 0; at < pending.length; at += 1) {
    const id = pending[at];
    if (ids.has(id.asU64())) continue;
    ids.add(id.asU64());
    const link = network.link(id);
    island.insertLinkWithOptionalId(id, link.references(), link.metadata().clone());
    pending.push(...link.references());
  }
  const value = decodeStructuredDataLinks(island, dataRoot);
  edit(value);
  const replacement = encodeStructuredDataLinks(value);
  const changed = new LinkNetwork();
  for (const link of network.links()) if (!ids.has(link.id().asU64())) {
    changed.insertLinkWithOptionalId(link.id(), link.references(), link.metadata().clone());
  }
  const remapped = new Map(replacement.network.links().map(link => [link.id().asU64(), changed.insertLink([], link.metadata().clone())]));
  for (const link of replacement.network.links()) changed.link(remapped.get(link.id().asU64())).setReferences(
    link.references().map(id => remapped.get(id.asU64())));
  const references = changed.link(root).references();
  references[0] = remapped.get(replacement.root.asU64());
  changed.link(root).setReferences(references);
  return { network: changed, root };
}

test('discovered productions reload and execute while checked identities share recursively', () => {
  const { network, root } = reload();
  const loaded = readDiscoveredConceptLinks(network, root);
  assert.deepEqual(loaded.space, fixture.expectedSpace);
  assert.equal(loaded.space.mappings['first:root'], loaded.space.mappings['second:page']);
  assert.equal(loaded.space.mappings['first:atom'], loaded.space.mappings['second:letter']);
  assert.notEqual(loaded.space.mappings['first:root'], loaded.space.mappings['alternative:root']);
  assert.equal(role(network, 'discovered concept candidate').length, 9);
  assert.equal(role(network, 'discovered concept identity').length, loaded.space.concepts.length);
  for (const [index, id] of ['first', 'second'].entries()) {
    const source = loaded.sources.find(source => source.id === id);
    assert.deepEqual(parseWithGrammar(source.grammar, fixture.parseSource), fixture.expectedTrees[index]);
    assert.deepEqual(source.grammar.ruleNames(), fixture.sources[index].grammar.rules.map(rule => rule.name));
    assert.deepEqual(source.grammar.rule(source.grammar.start).sourceNames, fixture.sources[index].grammar.rules[0].sourceNames);
  }
  assert.deepEqual(loaded.space.failures.at(-1), { members: ['first:root', 'alternative:root'], reason: 'no-one-to-one-proof' });
  const rootCandidate = role(network, 'discovered concept candidate').find(link => link.metadata().definition === 'first:root');
  const dependencies = network.link(rootCandidate.references()[3]).references();
  assert.ok(dependencies.some(id => id.equals(rootCandidate.references()[1])), 'recursive dependency is an actual global identity reference');
  const legacy = reload(legacyFixture);
  const restored = readDiscoveredConceptLinks(legacy.network, legacy.root);
  assert.deepEqual(restored.space, withDiscoveredSourceFormats(legacyFixture));
  const { formalDefinitions: legacyDefinitions, ...legacySpace } = restored.space;
  const { formalDefinitions: currentDefinitions, ...currentSpace } = loaded.space;
  assert.deepEqual(legacySpace, currentSpace, 'legacy and current archives revalidate the same semantic concept space');
  assert.deepEqual(Object.keys(legacyDefinitions), Object.keys(currentDefinitions));
  for (const alias of Object.keys(legacyDefinitions)) assert.deepEqual(
    readFormalConceptDefinition(legacyDefinitions[alias]), readFormalConceptDefinition(currentDefinitions[alias]),
    'formal meaning is compared by the existing rooted graph reader, independent of packet-local link identities');
  for (const [index, id] of ['first', 'second'].entries()) {
    const source = restored.sources.find(source => source.id === id);
    assert.equal(source.grammar.sourceFormat, legacyFixture.sources[index].grammar.sourceFormat);
    const tree = parseWithGrammar(source.grammar, legacyFixture.parseSource);
    assert.deepEqual(tree, legacyFixture.expectedTrees[index]);
    assert.deepEqual(discoveredTreeBytes(tree), [...new TextEncoder().encode(legacyFixture.parseSource)]);
  }
  assert.equal(legacy.network.toLino(), legacyFixture.links, 'revalidation does not rewrite the original archive bytes');
});

test('formal definitions remain actual links and unsupported interpretations survive losslessly', () => {
  const { network, root } = reload();
  const loaded = readDiscoveredConceptLinks(network, root);
  assert.equal(role(network, 'discovered formal definition').length, Object.keys(fixture.formalDefinitions).length);
  for (const [alias, definition] of Object.entries(loaded.formalDefinitions)) {
    const original = LinkNetwork.fromLino(fixture.formalDefinitions[alias].links);
    assert.equal(definition.network.links().length, original.links().length);
    assert.ok(definition.network.link(definition.root));
    assert.deepEqual([...definition.network._terms.keys()].sort(), [...original._terms.keys()].sort());
    for (const term of original._terms.keys()) assert.ok(definition.network.findTerm(term), 'source-owned term lookup survives reload');
  }
  assert.ok(loaded.formalDefinitions['alternative:root'].network.links().some(link => link.metadata().definition === 'unresolved interpretation'));
  assert.ok(!Object.hasOwn(loaded.space.formalDefinitions, 'alternative:root'));
  assert.ok(!network.links().some(link => link.metadata().definition?.includes('(1 (')));
});

test('copying and source-free revalidation are deterministic for all explicit candidates', () => {
  const sources = fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
  const packet = createDiscoveredConceptLinks(sources, fixture);
  assert.equal(packet.root.asU64(), fixture.root);
  assert.equal(packet.network.toLino(), fixture.links);
  assert.deepEqual(readDiscoveredConceptLinks(packet.network, packet.root).space, fixture.expectedSpace);
  assert.equal(createHash('sha256').update(readFileSync(legacyFile)).digest('hex'),
    '69daf66d8a97bedf33b84dcfe1dc5ca40718e86ba8c71034f7f3a1596b78e84e', 'the original fixture is a byte-for-byte retained input');
  for (const field of ['sources', 'records', 'equivalences', 'formalDefinitions', 'parseSource', 'expectedTrees']) {
    assert.deepEqual(fixture[field], legacyFixture[field], `${field} is unchanged by automatic certificate regeneration`);
  }
  for (const source of packet.space.sources) {
    assert.equal(source.sourceFormat, fixture.sources.find(original => original.id === source.id).grammar.sourceFormat);
  }
  const legacy = reload(legacyFixture);
  const restored = readDiscoveredConceptLinks(legacy.network, legacy.root);
  const current = createDiscoveredConceptLinks(restored.sources, { ...legacyFixture, formalDefinitions: restored.formalDefinitions });
  assert.equal(current.network.toLino(), fixture.links, 'a retained archive produces the exact current certificate without original Grammar objects');
  assert.equal(current.root.asU64(), fixture.root);
  const dataRoot = current.network.link(current.root).references()[0];
  const island = new LinkNetwork(), seen = new Set(), pending = [dataRoot];
  for (let at = 0; at < pending.length; at += 1) {
    const id = pending[at];
    if (seen.has(id.asU64())) continue;
    seen.add(id.asU64());
    const link = current.network.link(id);
    island.insertLinkWithOptionalId(id, link.references(), link.metadata().clone());
    pending.push(...link.references());
  }
  const envelope = decodeStructuredDataLinks(island, dataRoot);
  for (const concept of envelope.space.concepts) assert.strictEqual(concept.definition, concept.definitions[0].definition,
    'the primary definition and its first owned entry retain one actual container identity');
  const encoded = encodeStructuredDataLinks(envelope);
  assert.equal(encoded.network.links().length, island.links().length);
  const copied = { ...envelope, space: { ...envelope.space, concepts: envelope.space.concepts.map(concept => ({
    ...concept, definition: JSON.parse(JSON.stringify(concept.definition)),
  })) } };
  assert.deepEqual(copied, envelope, 'copying primary definitions preserves values but changes their owned identities');
  assert.equal(encodeStructuredDataLinks(copied).network.links().length, encoded.network.links().length + 56,
    'four literal definitions and two recursive definitions expand by exactly 56 container and entry nodes');
  for (const [first, second] of [[{ value: 'same' }, { value: 'same' }], [['same'], ['same']]]) {
    const distinct = encodeStructuredDataLinks([first, second, first]);
    const references = distinct.network.link(distinct.network.link(distinct.root).references()[0]).references();
    assert.notEqual(references[0].asU64(), references[1].asU64(), 'equal but separately owned containers remain separate');
    assert.equal(references[0].asU64(), references[2].asU64(), 'repeated ownership retains the actual shared container');
    const decoded = decodeStructuredDataLinks(distinct.network, distinct.root);
    assert.deepEqual(decoded[0], decoded[1]);assert.notStrictEqual(decoded[0], decoded[1]);assert.strictEqual(decoded[0], decoded[2]);
    assert.ok(distinct.network.links().length > encodeStructuredDataLinks([first, first, first]).network.links().length);
  }
  const facets = encodeStructuredDataLinks([{ value: -0 }, { value: 0 }, { first: 1, second: 2 }, { second: 2, first: 1 }]);
  const decoded = decodeStructuredDataLinks(facets.network, facets.root);
  assert.equal(Object.is(decoded[0].value, -0), true);assert.equal(Object.is(decoded[1].value, 0), true);
  assert.notStrictEqual(decoded[0], decoded[1]);assert.notStrictEqual(decoded[2], decoded[3]);
  assert.deepEqual(Object.keys(decoded[2]), ['first', 'second']);assert.deepEqual(Object.keys(decoded[3]), ['second', 'first']);
  const zeros = facets.network.links().filter(link => link.metadata().term === 'number value'
    && ['-0', '0'].includes(link.metadata().definition));
  assert.equal(zeros.length, 2);assert.notEqual(zeros[0].id().asU64(), zeros[1].id().asU64());
});

test('formal registrations remain owned and unambiguous after source-free reload', () => {
  for (const edit of [
    (network, registration) => registration.setReferences([...registration.references(), registration.references()[0]]),
    (network, registration, root) => registration.setReferences([root]),
  ]) {
    const { network, root } = reload();
    const declaration = role(network, 'discovered formal definition')[0];
    const registration = network.link(declaration.references()[2]);
    assert.ok(registration.references().length > 0);
    edit(network, registration, root);
    assert.throws(() => readDiscoveredConceptLinks(network, root), /registered definition point/u);
  }
  const sources = fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
  const alias = 'fixture.recursive-expression';
  const original = LinkNetwork.fromLino(fixture.formalDefinitions[alias].links);
  const point = original.insertPoint('original registration');
  original.setTerm(point, 'changed metadata');
  assert.throws(() => createDiscoveredConceptLinks(sources, { ...fixture, formalDefinitions: {
    ...fixture.formalDefinitions, [alias]: { network: original, root: fixture.formalDefinitions[alias].root },
  } }), /inconsistent registered definition term/u);
});

test('foreign origins, recursive identities and definition inventories are refused', () => {
  for (const edit of [
    network => {
      const candidates = role(network, 'discovered concept candidate');
      const references = candidates[0].references(); references[0] = candidates[1].references()[0]; candidates[0].setReferences(references);
    },
    network => {
      const candidate = role(network, 'discovered concept candidate')[0];
      const dependencies = network.link(candidate.references()[3]); dependencies.setReferences([candidate.references()[1]]);
    },
    network => {
      const definitions = role(network, 'discovered formal definition');
      const inventory = network.link(definitions[0].references()[1]); inventory.setReferences([...inventory.references(), definitions[1].references()[0]]);
    },
    network => {
      const identity = role(network, 'discovered concept identity')[0]; identity.setMetadata(identity.metadata().withDefinition('counterfeit identity'));
    },
  ]) {
    const { network, root } = reload(); edit(network);
    assert.throws(() => readDiscoveredConceptLinks(network, root), /discovered concepts:/u);
  }
  const certificateEdits = [
    value => { value.space.sources[0].sourceFormat = 'peg'; },
    value => { delete value.space.sources[0].sourceFormat; },
    value => { value.space.sources[0].context.matching = 'peg'; },
    value => { value.space.proofs[0].correspondence[0][1] = value.space.proofs[0].correspondence[0][0]; },
    value => { value.space.sources[0].id = 'counterfeit source'; },
  ];
  for (const edit of certificateEdits) {
    const changed = editDiscoveredEnvelope(reload(), edit);
    assert.throws(() => readDiscoveredConceptLinks(changed.network, changed.root), /stale or counterfeit global concept certificate/u);
  }
  for (const edit of [
    value => { value.space.sources[0].sourceFormat = 'bnf'; },
    value => { value.sources.find(source => source.id === 'first').grammar.sourceFormat = 'peg'; },
    value => { value.sources.find(source => source.id === 'first').id = 'counterfeit source'; },
    value => { value.space.sources[0].context.matching = 'peg'; },
    value => { value.space.proofs[0].correspondence[0][1] = value.space.proofs[0].correspondence[0][0]; },
  ]) {
    const changed = editDiscoveredEnvelope(reload(legacyFixture), edit);
    assert.throws(() => readDiscoveredConceptLinks(changed.network, changed.root), /stale or counterfeit global concept certificate/u);
  }
});

test('source kind provenance survives and redundant references cannot exceed the traversal bound', () => {
  for (const [explicit, matching, expected] of [[false, 'generalized', true], [false, 'peg', false], [true, 'generalized', true], [true, 'peg', true]]) {
    assert.equal(acceptLegacyDiscoveredConceptFormat(explicit, matching), expected);
  }
  for (const matching of [null, 'generalized', 'peg']) {
    const grammar = new Grammar('root', [['root', { expression: { kind: 'literal', value: 'π' } }]],
      'peg', matching === null ? null : { matching });
    const packet = createDiscoveredConceptLinks([{ id: 'authored', language: 'authored', grammar }]);
    assert.equal(readDiscoveredConceptLinks(packet.network, packet.root).sources[0].grammar.sourceFormat, 'peg');
    const legacy = editDiscoveredEnvelope(packet, value => {
      for (const source of value.space.sources) delete source.sourceFormat;
    });
    if (matching === null) {
      assert.throws(() => readDiscoveredConceptLinks(legacy.network, legacy.root), /stale or counterfeit global concept certificate/u,
        'an implicit PEG default cannot be certified by a legacy generalized-default certificate');
    } else {
      const restored = readDiscoveredConceptLinks(legacy.network, legacy.root);
      assert.deepEqual(restored.space, packet.space);
      assert.deepEqual(discoveredTreeBytes(parseWithGrammar(restored.sources[0].grammar, 'π')), [...new TextEncoder().encode('π')]);
      const changed = editDiscoveredEnvelope(legacy, value => {
        value.space.sources[0].context.matching = matching === 'peg' ? 'generalized' : 'peg';
      });
      assert.throws(() => readDiscoveredConceptLinks(changed.network, changed.root), /stale or counterfeit global concept certificate/u);
    }
  }
  const sources = fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
  sources[0].grammar.kinds = [{ name: 'expression node', sourceNames: [{ source: 'independent discovery', name: 'expression alias' }] }];
  const packet = createDiscoveredConceptLinks(sources, fixture);
  assert.deepEqual(readDiscoveredConceptLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root).sources[0].grammar.kinds, sources[0].grammar.kinds);
  const { network, root } = reload();
  const sequence = role(network, 'discovered concept sequence')[0];
  sequence.setReferences(Array(100001).fill(network.links()[0].id()));
  assert.throws(() => readDiscoveredConceptLinks(network, root), /resource limit/u);
});

test('changing a formal premise invalidates the archived sharing certificate', () => {
  for (const data of [fixture, legacyFixture]) {
    const { network, root } = reload(data);
    const declared = role(network, 'discovered formal definition').find(link => link.metadata().definition === 'fixture.recursive-expression');
    const definition = network.link(declared.references()[0]);
    definition.setMetadata(definition.metadata().withLanguage('counterfeit source semantics'));
    assert.throws(() => readDiscoveredConceptLinks(network, root), /stale or counterfeit global concept certificate/u);
  }
});

test('formal declaration keys cannot mutate the host object prototype', () => {
  const sources = fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
  const formalDefinitions = { ...fixture.formalDefinitions };
  Object.defineProperty(formalDefinitions, '__proto__', { value: fixture.formalDefinitions['fixture.letter'], enumerable: true });
  const packet = createDiscoveredConceptLinks(sources, { ...fixture, formalDefinitions });
  const loaded = readDiscoveredConceptLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  assert.ok(Object.hasOwn(loaded.formalDefinitions, '__proto__'));
  assert.ok(loaded.formalDefinitions.__proto__.network instanceof LinkNetwork);
  assert.equal(Object.getPrototypeOf(loaded.formalDefinitions), null);
});
