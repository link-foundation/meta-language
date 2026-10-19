#!/usr/bin/env node
// A bounded integration fixture, derived without running an inference experiment.
import { readProjectData, serializeProjectData, writeProjectData } from './project-data.mjs';
import { deserializeGrammar } from '../src/grammar.js';
import { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from '../src/discovered-concept-links.js';
import { LinkNetwork } from '../src/network.js';
import { LinkMetadata, LinkType } from '../src/primitives.js';

const baseline = readProjectData(new URL('../../parity/fixtures/global-concept-space.json', import.meta.url));
const sources = baseline.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
const alternative = structuredClone(baseline.sources[0]);
alternative.id = 'alternative';
alternative.grammar.rules[0].concept = 'fixture.unproved-expression';
alternative.grammar.rules[0].sourceNames = [{ source: 'independent discovery', name: 'possible expression' }];
sources.push({ ...alternative, grammar: deserializeGrammar(alternative.grammar) });
const unproved = LinkNetwork.fromLino(baseline.formalDefinitions['fixture.recursive-expression'].links);
unproved.link(baseline.formalDefinitions['fixture.recursive-expression'].root).setMetadata(
  unproved.link(baseline.formalDefinitions['fixture.recursive-expression'].root).metadata().withLanguage('unsupported discovery semantics'));
unproved.insertLink([], new LinkMetadata({ linkType: LinkType.Semantic, term: 'preserved discovery trace', language: 'independent discovery', definition: 'unresolved interpretation' }));
const formalDefinitions = { ...baseline.formalDefinitions, 'alternative:root': { network: unproved, root: baseline.formalDefinitions['fixture.recursive-expression'].root } };
const options = { records: baseline.records, equivalences: [...baseline.equivalences, ['first:root', 'alternative:root']], formalDefinitions };
const packet = createDiscoveredConceptLinks(sources, options);
const loaded = readDiscoveredConceptLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
const fixture = { schemaVersion: 1, id: 'discovered-concept-links', source: 'https://github.com/link-foundation/meta-language/issues/224',
  sources: [...baseline.sources, alternative], records: baseline.records, equivalences: options.equivalences,
  formalDefinitions: Object.fromEntries(Object.entries(formalDefinitions).map(([alias, definition]) => [alias, { links: definition.network?.toLino() ?? definition.links, root: definition.root }])),
  root: packet.root.asU64(), links: packet.network.toLino(), expectedSpace: loaded.space, parseSource: 'αα',
  expectedTrees: [
    { type: 'node', kind: 'root', start: 0, end: 4, children: [{ type: 'node', kind: 'atom', start: 0, end: 2, children: [{ type: 'token', kind: null, start: 0, end: 2, text: 'α' }] }, { type: 'node', kind: 'root', start: 2, end: 4, children: [{ type: 'node', kind: 'atom', start: 2, end: 4, children: [{ type: 'token', kind: null, start: 2, end: 4, text: 'α' }] }] }] },
    { type: 'node', kind: 'page', start: 0, end: 4, children: [{ type: 'node', kind: 'letter', start: 0, end: 2, children: [{ type: 'token', kind: null, start: 0, end: 2, text: 'α' }] }, { type: 'node', kind: 'page', start: 2, end: 4, children: [{ type: 'node', kind: 'letter', start: 2, end: 4, children: [{ type: 'token', kind: null, start: 2, end: 4, text: 'α' }] }] }] },
  ] };
const path = new URL('../../parity/fixtures/discovered-concept-links.json', import.meta.url);
if (process.argv.includes('--check')) {
  if (serializeProjectData(readProjectData(path), path) !== serializeProjectData(fixture, path)) throw new Error('discovered concept fixture drift');
  console.log('discovered concept fixture matches current integration');
} else writeProjectData(path, fixture);
