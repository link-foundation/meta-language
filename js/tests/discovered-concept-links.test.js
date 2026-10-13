import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from '../src/discovered-concept-links.js';
import { deserializeGrammar, parseWithGrammar } from '../src/grammar.js';
import { LinkNetwork } from '../src/network.js';

const fixture = readProjectData(new URL('../../parity/fixtures/discovered-concept-links.json', import.meta.url));
const reload = () => ({ network: LinkNetwork.fromLino(fixture.links), root: fixture.root });
const role = (network, term) => network.links().filter(link => link.metadata().language === 'meta-language discovered concepts' && link.metadata().term === term);

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
});

test('source kind provenance survives and redundant references cannot exceed the traversal bound', () => {
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
  const { network, root } = reload();
  const declared = role(network, 'discovered formal definition').find(link => link.metadata().definition === 'fixture.recursive-expression');
  const definition = network.link(declared.references()[0]);
  definition.setMetadata(definition.metadata().withLanguage('counterfeit source semantics'));
  assert.throws(() => readDiscoveredConceptLinks(network, root), /stale or counterfeit global concept certificate/u);
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
