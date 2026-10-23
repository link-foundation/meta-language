// Complete graph archives. Source buffers and parser instances are not inputs
// to reconstruction; identities, cycles and disconnected links are retained.
import { LinkNetwork } from './network.js';
import { LinkFlags, LinkId, LinkMetadata, LinkType } from './primitives.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks } from './structured-data-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from './link-network-binary.js';
import { StreamParser } from 'links-notation';
import { decodeCanonicalStatements } from './lino-serialization.js';

const language = 'meta-language complete graph archive';
const rootTerm = 'complete meta-language archive';
const groupTerm = 'complete source network';
const kinds = new Set(['concrete syntax', 'abstract syntax', 'grammar',
  'global concept space', 'formal concept definition', 'ontology', 'taxonomy',
  'resource description graph', 'resource description dataset', 'typed program',
  'discovered concepts', 'decorators', 'grammar inference']);
const encoder = new TextEncoder();
const identity = value => LinkId.from(value).asU64();
const fail = message => { throw new TypeError(`meta-language archive: ${message}`); };
const metadata = term => new LinkMetadata({ linkType: LinkType.Semantic,
  named: true, term, language });

function bounds(options) {
  const result = { nodeLimit: options.nodeLimit ?? 100000,
    referenceLimit: options.referenceLimit ?? 1000000,
    byteLimit: options.byteLimit ?? 128 * 1024 * 1024 };
  for (const value of Object.values(result)) {
    if (!Number.isSafeInteger(value) || value < 1) fail('invalid resource limit');
  }
  return result;
}

function inspect(network, limits) {
  if (!(network instanceof LinkNetwork)) fail('expected a LinkNetwork');
  const links = network.links();
  if (links.length > limits.nodeLimit) fail('node resource limit exceeded');
  const identities = new Set(links.map(link => identity(link.id())));
  if (identities.size !== links.length) fail('duplicate graph identity');
  let references = 0, bytes = 0;
  for (const link of links) {
    const data = link.metadata();
    if (!Object.values(LinkType).includes(data.linkType) || typeof data.named !== 'boolean'
      || !['isError', 'hasError', 'isMissing', 'isExtra'].every(flag => typeof data.flags?.[flag] === 'boolean')) fail('invalid graph metadata profile');
    if (data.span) {
      const { byteRange, start, end } = data.span;
      if (![byteRange?.start, byteRange?.end, start?.row, start?.column, end?.row, end?.column]
        .every(value => Number.isSafeInteger(value) && value >= 0) || byteRange.end < byteRange.start) fail('invalid graph source span');
    }
    references += link.references().length;
    for (const reference of link.references()) {
      if (!identities.has(identity(reference))) fail('dangling graph reference');
    }
    for (const field of ['term', 'language', 'definition']) {
      const text = link.metadata()[field];
      if (text !== undefined) {
        if (typeof text !== 'string' || !text.isWellFormed()) fail('invalid graph metadata');
        bytes += encoder.encode(text).length;
      }
    }
    if (references > limits.referenceLimit || bytes > limits.byteLimit) fail('graph resource limit exceeded');
  }
  if (!Number.isSafeInteger(network._nextId) || network._nextId < 1
    || links.some(link => identity(link.id()) >= network._nextId)) fail('invalid allocator state');
  const terms = [...network._terms].map(([term, id]) => {
    if (typeof term !== 'string' || !term.isWellFormed() || !identities.has(identity(id))) fail('invalid term registration');
    bytes += encoder.encode(term).length;
    return { term, identity: identity(id) };
  });
  const syntax = [...network._conceptSyntax].map(([key, value]) => {
    if (typeof key !== 'string' || typeof value !== 'string'
      || !key.isWellFormed() || !value.isWellFormed()) fail('invalid selected syntax');
    const at = key.indexOf('\0');
    if (at < 0 || key.indexOf('\0', at + 1) !== -1) fail('ambiguous selected syntax key');
    bytes += encoder.encode(key).length + encoder.encode(value).length;
    return { concept: key.slice(0, at), language: key.slice(at + 1), value };
  });
  if (bytes > limits.byteLimit) fail('byte resource limit exceeded');
  return { links, state: { identities: links.map(link => identity(link.id())),
    nextIdentity: network._nextId, terms, syntax } };
}

function copyLinks(target, links, limits) {
  if (target.links().length + links.length > limits.nodeLimit) fail('node resource limit exceeded');
  const mapping = new Map(links.map(link => [identity(link.id()), target.insertLink([], link.metadata().clone())]));
  for (const link of links) target.link(mapping.get(identity(link.id())))
    .setReferences(link.references().map(reference => mapping.get(identity(reference))));
  return mapping;
}

function copy(target, source, limits) {
  const { links, state } = inspect(source, limits);
  const mapping = copyLinks(target, links, limits);
  return { mapping, state };
}

function entryRecord(entry, group) {
  if (!entry || typeof entry.identifier !== 'string' || entry.identifier.length === 0
    || !entry.identifier.isWellFormed() || !kinds.has(entry.kind)) fail('invalid entry identifier or kind');
  const root = identity(entry.root);
  if (!entry.network.link(root)) fail('entry root is absent');
  return { identifier: entry.identifier, kind: entry.kind, group, root };
}

/** Archive complete networks once per shared object identity. Independently
 * supplied networks are never identified by matching names or equal samples.
 */
export function createMetaLanguageArchive(entries, options = {}) {
  const limits = bounds(options);
  if (!Array.isArray(entries) || entries.length === 0 || entries.length > limits.nodeLimit) fail('expected bounded nonempty entries');
  const network = new LinkNetwork(), groups = [], groupRoots = [], names = new Set(), shared = new Map();
  const records = entries.map(entry => {
    if (!entry || !(entry.network instanceof LinkNetwork)) fail('invalid entry network');
    let group = shared.get(entry.network);
    if (group === undefined) {
      group = groups.length;
      const { mapping, state } = copy(network, entry.network, limits);
      const inventory = network.insertLink([...mapping.values()], metadata(groupTerm));
      groups.push(state);
      groupRoots.push(inventory);
      shared.set(entry.network, group);
    }
    const record = entryRecord(entry, group);
    if (names.has(record.identifier)) fail('duplicate entry identifier');
    names.add(record.identifier);
    return record;
  });
  const descriptor = encodeStructuredDataLinks({ schemaVersion: 1, entries: records, groups }, limits);
  const mapping = copyLinks(network, descriptor.network.links(), limits);
  const root = network.insertLink([mapping.get(identity(descriptor.root)), ...groupRoots], metadata(rootTerm));
  inspect(network, limits);
  return { network, root };
}

function references(network, root, term) {
  const link = network.link(root), value = link?.metadata();
  if (!link || value.language !== language || value.term !== term
    || value.linkType !== LinkType.Semantic || value.named !== true
    || value.span || value.definition !== undefined || Object.values(value.flags).some(Boolean)) fail(`invalid ${term}`);
  return link.references();
}

function restore(network, inventory, state, claimed, limits) {
  if (!state || !Array.isArray(state.identities) || !Array.isArray(state.terms)
    || !Array.isArray(state.syntax) || inventory.length !== state.identities.length) fail('invalid source network descriptor');
  const mapping = new Map(), original = new Set();
  for (let index = 0; index < inventory.length; index += 1) {
    const from = identity(inventory[index]), to = identity(state.identities[index]);
    if (mapping.has(from) || claimed.has(from) || original.has(to)) fail('overlapping or duplicate graph inventory');
    mapping.set(from, LinkId.from(to));
    original.add(to);
    claimed.add(from);
  }
  const result = new LinkNetwork();
  for (const id of inventory) {
    const link = network.link(id);
    if (!link) fail('missing source network member');
    const targets = link.references().map(reference => {
      const target = mapping.get(identity(reference));
      if (!target) fail('foreign or dangling source network reference');
      return target;
    });
    result.insertLinkWithOptionalId(mapping.get(identity(id)), targets, link.metadata().clone());
  }
  for (const record of state.terms) {
    if (!record || typeof record.term !== 'string' || !original.has(identity(record.identity))
      || result._terms.has(record.term)) fail('invalid or duplicate term registration');
    result._terms.set(record.term, LinkId.from(record.identity));
  }
  for (const record of state.syntax) {
    if (!record || !['concept', 'language', 'value'].every(field => typeof record[field] === 'string')
      || record.concept.includes('\0') || record.language.includes('\0')) fail('invalid selected syntax');
    const key = `${record.concept}\0${record.language}`;
    if (result._conceptSyntax.has(key)) fail('duplicate selected syntax');
    result._conceptSyntax.set(key, record.value);
  }
  result._nextId = state.nextIdentity;
  inspect(result, limits);
  return result;
}

/** Eagerly reconstruct every source node, including disconnected provenance.
 * Entries sharing a source group share the same reconstructed network object.
 */
export function readMetaLanguageArchive(network, root, options = {}) {
  const limits = bounds(options);
  inspect(network, limits);
  const roots = references(network, root, rootTerm);
  if (roots.length < 2) fail('missing descriptor or source groups');
  const claimed = new Set([identity(root), ...roots.map(identity)]);
  // Descriptor containers cannot also be members of an archived source group.
  const pending = [roots[0]], descriptorMembers = new Set();
  for (let index = 0; index < pending.length; index += 1) {
    const id = identity(pending[index]);
    if (descriptorMembers.has(id)) continue;
    if (descriptorMembers.size >= limits.nodeLimit) fail('descriptor resource limit exceeded');
    descriptorMembers.add(id);
    claimed.add(id);
    const link = network.link(id);
    if (!link) fail('dangling descriptor reference');
    for (const reference of link.references()) pending.push(reference);
  }
  if (new Set(roots.map(identity)).size !== roots.length || roots.some(id => identity(id) === identity(root))) fail('overlapping archive roots');
  for (const id of roots.slice(1)) if (descriptorMembers.has(identity(id))) fail('descriptor aliases a source group');
  const descriptorNetwork = new LinkNetwork();
  for (const id of descriptorMembers) {
    const link = network.link(id);
    descriptorNetwork.insertLinkWithOptionalId(id, link.references(), link.metadata().clone());
  }
  const descriptor = decodeStructuredDataLinks(descriptorNetwork, roots[0], limits);
  if (descriptor?.schemaVersion !== 1 || !Array.isArray(descriptor.entries)
    || !Array.isArray(descriptor.groups) || descriptor.groups.length !== roots.length - 1
    || descriptor.entries.length === 0) fail('invalid archive descriptor');
  const groups = descriptor.groups.map((state, index) => restore(network,
    references(network, roots[index + 1], groupTerm), state, claimed, limits));
  const names = new Set(), used = new Set();
  const entries = descriptor.entries.map(record => {
    if (!record || !Number.isSafeInteger(record.group) || record.group < 0 || record.group >= groups.length) fail('invalid entry source group');
    const entry = { ...record, network: groups[record.group] };
    entryRecord(entry, record.group);
    if (names.has(record.identifier)) fail('duplicate entry identifier');
    names.add(record.identifier);
    used.add(record.group);
    return { identifier: record.identifier, kind: record.kind, network: entry.network, root: LinkId.from(record.root) };
  });
  if (used.size !== groups.length) fail('unreferenced source group');
  return { entries, networks: groups };
}

export function metaLanguageArchiveToText(archive, options = {}) {
  readMetaLanguageArchive(archive.network, archive.root, options);
  const text = archive.network.toLino();
  if (encoder.encode(text).length > bounds(options).byteLimit) fail('byte resource limit exceeded');
  return text;
}

function findRoot(network) {
  const referenced = new Set(network.links().flatMap(link => link.references().map(identity)));
  const roots = network.links().filter(link => link.metadata().language === language
    && link.metadata().term === rootTerm && !referenced.has(identity(link.id()))).map(link => link.id());
  if (roots.length !== 1) fail('exactly one archive root is required');
  return roots[0];
}

export function readMetaLanguageArchiveText(text, options = {}) {
  const limits = bounds(options);
  if (typeof text !== 'string' || encoder.encode(text).length > limits.byteLimit) fail('invalid or oversized Links text');
  const network = new LinkNetwork(), seen = new Set();
  let referenceCount = 0;
  const parser = new StreamParser({ comments: false, collect: false,
    maxInputSize: limits.byteLimit, maxBufferSize: limits.byteLimit, maxDepth: 16 });
  parser.on('error', error => fail(`Links parse error: ${error.message}`));
  parser.on('link', statement => {
    if (seen.size >= limits.nodeLimit) fail('node resource limit exceeded');
    if (!/^[1-9][0-9]*$/u.test(statement.id ?? '') || seen.has(statement.id)) fail('invalid or duplicate Links identity');
    seen.add(statement.id);
    const envelopes = statement.values.filter(value => value.id === 'meta' && value.values.length > 0);
    if (envelopes.length !== 1) fail('exactly one metadata envelope is required');
    const fields = new Map();
    for (const field of envelopes[0].values) {
      if (fields.has(field.id)) fail('duplicate graph metadata field');
      fields.set(field.id, field);
    }
    const named = fields.get('n');
    if (!fields.has('t') || !named || named.values.length !== 1
      || !['0', '1'].includes(named.values[0].id)) fail('missing or invalid graph metadata');
    referenceCount += statement.values.filter(value => value.values.length === 0).length;
    if (referenceCount > limits.referenceLimit) fail('reference resource limit exceeded');
    const [record] = decodeCanonicalStatements([statement]);
    if (!record) fail('invalid canonical network statement');
    const flags = fields.get('flags');
    if (flags) {
      const bits = Number(flags.values[0]?.id);
      if (!Number.isInteger(bits) || bits < 0 || bits > 15) fail('unknown graph flag bits');
      // General LiNo reading infers hasError for error/missing links. An exact
      // archive retains all four stored bits independently, as binary does.
      record.metadata = record.metadata.withFlags(new LinkFlags({ isError: bits % 2 === 1,
        hasError: Math.floor(bits / 2) % 2 === 1, isMissing: Math.floor(bits / 4) % 2 === 1,
        isExtra: Math.floor(bits / 8) % 2 === 1 }));
    }
    network.insertLinkWithOptionalId(record.id, record.references, record.metadata);
  });
  for (let at = 0; at < text.length; at += 8192) parser.write(text.slice(at, at + 8192));
  parser.end();
  const root = findRoot(network);
  return { network, root, ...readMetaLanguageArchive(network, root, options) };
}

export function metaLanguageArchiveToBinary(archive, options = {}) {
  readMetaLanguageArchive(archive.network, archive.root, options);
  return encodeLinkNetworkBinary(archive.network, { ...options, recordLimit: bounds(options).nodeLimit });
}

export function readMetaLanguageArchiveBinary(bytes, options = {}) {
  const network = decodeLinkNetworkBinary(bytes, { ...options, recordLimit: bounds(options).nodeLimit });
  const root = findRoot(network);
  return { network, root, ...readMetaLanguageArchive(network, root, options) };
}

/** Persistent extension: previous roots survive unless explicitly replaced or
 * removed. Neither additions nor replacement names imply semantic equality.
 */
export function extendMetaLanguageArchive(archive, additions, options = {}) {
  const { entries } = readMetaLanguageArchive(archive.network, archive.root, options);
  if (!Array.isArray(additions) || !Array.isArray(options.remove ?? [])) fail('invalid archive extension');
  const selected = new Map(entries.map(entry => [entry.identifier, entry]));
  const removed = new Set();
  for (const identifier of options.remove ?? []) {
    if (removed.has(identifier) || !selected.delete(identifier)) fail('unknown or duplicate removed entry');
    removed.add(identifier);
  }
  const added = new Set();
  for (const entry of additions) {
    if (!entry || added.has(entry.identifier)) fail('duplicate added entry');
    if (selected.has(entry.identifier) && options.replace !== true) fail('replacement requires an explicit selection');
    added.add(entry.identifier);
    selected.set(entry.identifier, entry);
  }
  return createMetaLanguageArchive([...selected.values()], options);
}
