// One small graph with cycles, full metadata and exact observable lookup state.
// Legacy bytes use an independent DataView writer of the existing v1 layout;
// CI also checks them against the actual optional native Doublets boundary.
import assert from 'node:assert/strict';
import { LinkNetwork } from '../src/network.js';
import { ByteRange, LinkMetadata, LinkFlags, LinkType, Point, SourceSpan } from '../src/primitives.js';
import { encodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { readProjectData, writeProjectData } from './project-data.mjs';

export function createBinaryNetworkFixture() {
  const network = new LinkNetwork();
  const first = network.insertTypedPoint(LinkType.Semantic, 'original');
  network.setTerm(first, 'renamed');
  network.link(first).setMetadata(network.link(first).metadata().withNamed(true).withDefinition('formal linked definition\0😀')
    .withLanguage('Русский').withSpan(new SourceSpan(new ByteRange(1, 23), new Point(2, 3), new Point(4, 5)))
    .withFlags(LinkFlags.error().withMissing().withExtra()));
  const second = network.insertLink([first], LinkMetadata.new().withLinkType(LinkType.Relation).withDefinition(''));
  network.link(first).setReferences([second, first, second]);
  network.insertConceptExpression('meaning', 'JavaScript', 'selected syntax');
  network.insertConceptExpression('😀 concept', 'en', 'astral syntax');
  network.insertConceptExpression('\uE000 concept', 'en', 'private use syntax');
  network.deleteLink(network.insertLinkWithOptionalId(100, [], LinkMetadata.new()));
  return network;
}

const path = new URL('../../parity/fixtures/link-network-binary.lino', import.meta.url);
const network = createBinaryNetworkFixture();
const types = [undefined, LinkType.Dynamic, LinkType.Reference, LinkType.Relation, LinkType.Language, LinkType.Grammar,
  LinkType.Type, LinkType.Concept, LinkType.Syntax, LinkType.Field, LinkType.Trivia, LinkType.SourceToken, LinkType.Document,
  LinkType.Semantic, LinkType.Region, LinkType.Object];
const encoder = new TextEncoder();
const word = value => { const bytes = new Uint8Array(8); new DataView(bytes.buffer).setBigUint64(0, BigInt(value), true); return [...bytes]; };
const optional = value => value === undefined ? [0] : [1, ...word(encoder.encode(value).length), ...encoder.encode(value)];
const legacy = [...encoder.encode('MLDSNP01'), ...word(network.len())];
for (const link of network.links()) {
  const metadata = link.metadata();
  const flags = metadata.flags;
  const bytes = [1, types.indexOf(metadata.linkType), Number(metadata.named), Number(flags.isError) + Number(flags.hasError) * 2 + Number(flags.isMissing) * 4 + Number(flags.isExtra) * 8,
    Number(Boolean(metadata.term !== undefined && network.findTerm(metadata.term)?.equals(link.id()))), 0,
    ...optional(metadata.term), ...optional(metadata.definition), ...optional(metadata.language), Number(metadata.span !== undefined)];
  if (metadata.span) for (const value of [metadata.span.byteRange.start, metadata.span.byteRange.end, metadata.span.start.row, metadata.span.start.column, metadata.span.end.row, metadata.span.end.column]) bytes.push(...word(value));
  legacy.push(...word(link.id().asU64()), ...word(link.references().length));
  for (const reference of link.references()) legacy.push(...word(reference.asU64()));
  legacy.push(...word(bytes.length), ...bytes);
}
const fixture = { schemaVersion: 1, graph: network.toLino(), version2Bytes: [...encodeLinkNetworkBinary(network)], legacyVersion1Bytes: legacy,
  termMap: [...network._terms].map(([term, id]) => [term, id.asU64()]), selectedSyntax: [...network._conceptSyntax], nextIdentity: network._nextId,
  scope: 'one cyclic network; all metadata fields; exact mutated term map; selected syntax; deleted high-ID allocation history',
  oracleBoundary: 'v1 bytes are independently framed with DataView; actual native Doublets framing and v2 native parity execute in CI' };
if (process.argv.includes('--check')) assert.deepEqual(readProjectData(path), fixture);
else writeProjectData(path, fixture);
console.log('Shared binary network fixture:', network.len(), 'links,', fixture.version2Bytes.length, 'v2 bytes');
