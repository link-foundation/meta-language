// Bootstrap-safe structured data: every collection, entry and scalar is a link.
// These leaf imports deliberately do not initialize programming language parsers.
import { Link, LinkId, LinkMetadata, LinkType } from './primitives.js';
import { StreamParser } from 'links-notation';
import { decodeCanonicalStatements, encodeNetworkLino } from './lino-serialization.js';
import { acceptStructuredDataShape, readStructuredDataKind } from './structured-data-link-rules.js';

const language = 'meta-language structured data';
const terms = Object.freeze({ root: 'structured data', object: 'object value', array: 'array value', pair: 'key value pair',
  string: 'string value', units: 'Unicode code unit string', unit: 'Unicode code unit', number: 'number value', boolean: 'boolean value', null: 'null value' });
const defaults = Object.freeze({ nodeLimit: 1_000_000, depthLimit: 256, byteLimit: 128 * 1024 * 1024 });
const encoder = new TextEncoder();
const fail = (message) => { throw new TypeError(`structured data: ${message}`); };
const key = (id) => LinkId.from(id).asU64();
const metadata = (kind, definition) => LinkMetadata.new().withLinkType(LinkType.Semantic)
  .withLanguage(language).withNamed(true).withTerm(terms[kind]).withDefinition(definition);

/** A bootstrap-safe link store using the public link primitives and serializer. */
export class StructuredDataNetwork {
  constructor() { this._links = new Map(); this._nextId = 1; }
  insertLink(references = [], data = LinkMetadata.new()) {
    const id = LinkId.from(this._nextId++);
    this._links.set(key(id), new Link(id, references, data));
    return id;
  }
  link(id) { return this._links.get(key(id)); }
  links() { return [...this._links.values()].sort((left, right) => key(left.id()) - key(right.id())); }
  toLino() { return encodeNetworkLino(this.links(), new Set()); }
}

function limitsOf(options) {
  const limits = { ...defaults, ...options };
  for (const name of Object.keys(defaults)) if (!Number.isSafeInteger(limits[name]) || limits[name] < 1) fail(`invalid ${name}`);
  return limits;
}

function wellFormedString(value) {
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const following = value.charCodeAt(++index);
      if (!(following >= 0xdc00 && following <= 0xdfff)) return false;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) return false;
  }
  return true;
}

/** Encode plain data, preserving collection order, sharing and binary64 values. */
export function encodeStructuredDataLinks(value, options = {}) {
  const limits = limitsOf(options);
  const network = new StructuredDataNetwork();
  const scalarIds = new Map();
  const identities = new WeakMap();
  const active = new WeakSet();
  const heights = new Map();
  let bytes = 0;
  const heightOf = references => references.reduce((height, reference) => Math.max(height, 1 + heights.get(key(reference))), 0);
  const insert = (kind, definition, references = []) => {
    if (network._links.size >= limits.nodeLimit) fail('node resource limit exceeded');
    if (definition !== undefined) bytes += encoder.encode(definition).length;
    if (bytes > limits.byteLimit) fail('byte resource limit exceeded');
    const id = network.insertLink(references, metadata(kind, definition));
    heights.set(key(id), heightOf(references));
    return id;
  };
  function scalar(kind, definition) {
    const identity = `${kind}\u0000${definition ?? ''}`;
    if (scalarIds.has(identity)) return scalarIds.get(identity);
    const id = insert(kind, definition);
    scalarIds.set(identity, id);
    return id;
  }
  function string(value, depth) {
    if (depth > limits.depthLimit) fail('depth resource limit exceeded');
    if (wellFormedString(value)) return scalar('string', value);
    if (depth + 1 > limits.depthLimit) fail('depth resource limit exceeded');
    const identity = `units\u0000${value}`;
    if (scalarIds.has(identity)) return scalarIds.get(identity);
    const id = insert('units', undefined, Array.from({ length: value.length }, (_, index) => scalar('unit', String(value.charCodeAt(index)))));
    scalarIds.set(identity, id);
    return id;
  }
  function encode(item, depth) {
    if (depth > limits.depthLimit) fail('depth resource limit exceeded');
    if (item === null) return scalar('null');
    if (typeof item === 'string') return string(item, depth);
    if (typeof item === 'boolean') return scalar('boolean', String(item));
    if (typeof item === 'number') {
      if (!Number.isFinite(item)) fail('a number must be finite binary64');
      return scalar('number', Object.is(item, -0) ? '-0' : String(item));
    }
    if (!item || typeof item !== 'object') fail('unsupported host value');
    if (active.has(item)) fail('structural cycle');
    if (identities.has(item)) {
      const id = identities.get(item);
      if (depth + heights.get(key(id)) > limits.depthLimit) fail('depth resource limit exceeded');
      return id;
    }
    const array = Array.isArray(item);
    if (!array && ![Object.prototype, null].includes(Object.getPrototypeOf(item))) fail('unsupported host object');
    const names = Reflect.ownKeys(item);
    if (names.some(name => typeof name !== 'string')) fail('symbol field');
    for (const name of names) {
      const descriptor = Object.getOwnPropertyDescriptor(item, name);
      if (!Object.hasOwn(descriptor, 'value') || (!descriptor.enumerable && !(array && name === 'length'))) fail('non-data field');
    }
    if (array && (names.length !== item.length + 1 || !Array.from({ length: item.length }, (_, index) => String(index)).every(name => Object.hasOwn(item, name)))) fail('sparse array or array field');
    const id = insert(array ? 'array' : 'object');
    identities.set(item, id);
    active.add(item);
    const references = array ? item.map(child => encode(child, depth + 1))
      : Object.keys(item).map(name => insert('pair', undefined, [string(name, depth + 2), encode(item[name], depth + 2)]));
    active.delete(item);
    network.link(id).setReferences(references);
    heights.set(key(id), heightOf(references));
    return id;
  }
  const body = encode(value, 1);
  const root = insert('root', '1', [body]);
  return { network, root };
}

/** Decode a closed typed data graph; cycles, duplicate fields and foreign roles fail. */
export function decodeStructuredDataLinks(network, root, options = {}) {
  const limits = limitsOf(options);
  const links = network.links();
  if (links.length > limits.nodeLimit) fail('node resource limit exceeded');
  const active = new Set();
  const reached = new Set();
  const memo = new Map();
  const costs = new Map();
  let bytes = 0;
  let visits = 0;
  function inspect(id, depth) {
    const initialVisits = visits;
    const initialBytes = bytes;
    if (depth > limits.depthLimit || ++visits > limits.nodeLimit) fail('traversal resource limit exceeded');
    const identity = key(id);
    if (!Number.isSafeInteger(identity)) fail('invalid link identity');
    if (active.has(identity)) fail('structural cycle');
    const link = network.link(id);
    if (!link) fail('dangling reference');
    if (memo.has(identity)) {
      const cost = costs.get(identity);
      if (depth + cost.height > limits.depthLimit) fail('depth resource limit exceeded');
      visits += cost.nodes - 1;
      bytes += cost.bytes;
      if (visits > limits.nodeLimit || bytes > limits.byteLimit) fail('expanded data resource limit exceeded');
      return memo.get(identity);
    }
    const data = link.metadata();
    const references = link.references();
    const kind = readStructuredDataKind(data.term ?? '');
    if (data.linkType !== LinkType.Semantic || data.language !== language || !data.named || data.span
      || Object.values(data.flags ?? {}).some(Boolean) || !acceptStructuredDataShape(kind, String(references.length), data.definition !== undefined)) fail('invalid typed link');
    if (data.definition !== undefined) bytes += encoder.encode(data.definition).length;
    if (bytes > limits.byteLimit) fail('byte resource limit exceeded');
    reached.add(identity);
    active.add(identity);
    const child = reference => inspect(reference, depth + 1);
    let value;
    if (kind === 'root') {
      if (identity !== key(root) || data.definition !== '1') fail('invalid schema root');
      value = child(references[0]);
    } else if (kind === 'array') value = references.map(child);
    else if (kind === 'object') {
      const target = {};
      const names = [];
      const seen = new Set();
      for (const reference of references) {
        const pairLink = network.link(reference);
        if (pairLink?.metadata().term !== terms.pair) fail('object entry is not a key value pair');
        const pair = child(reference);
        if (typeof pair[0] !== 'string' || seen.has(pair[0])) fail('invalid or duplicate object key');
        seen.add(pair[0]); names.push(pair[0]);
        Object.defineProperty(target, pair[0], { value: pair[1], enumerable: true, configurable: true, writable: true });
      }
      // Ordinary JSON objects stay ordinary and remain structuredClone-compatible.
      // Only an explicitly edited order that JavaScript objects cannot express
      // needs the ordered view (for example integer-looking keys 2 before 1).
      value = Object.keys(target).every((name, index) => name === names[index]) ? target
        : new Proxy(target, { ownKeys: object => [...names.filter(name => Object.hasOwn(object, name)), ...Reflect.ownKeys(object).filter(name => !seen.has(name))] });
    } else if (kind === 'pair') value = references.map(child);
    else if (kind === 'string') {
      value = data.definition ?? '';
      if (!wellFormedString(value)) fail('a UTF-8 string contains an unpaired code unit');
    } else if (kind === 'units') {
      const units = references.map(reference => {
        if (network.link(reference)?.metadata().term !== terms.unit) fail('a code unit string contains a foreign role');
        return child(reference);
      });
      value = units.map(unit => String.fromCharCode(unit)).join('');
    } else if (kind === 'unit') {
      if (!/^(0|[1-9][0-9]*)$/u.test(data.definition)) fail('invalid Unicode code unit');
      value = Number(data.definition);
      if (!Number.isInteger(value) || value > 65535) fail('invalid Unicode code unit');
    } else if (kind === 'number') {
      if (!/^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:e[+-]?[0-9]+)?$/u.test(data.definition)) fail('invalid number');
      value = Number(data.definition);
      if (!Number.isFinite(value) || (Object.is(value, -0) ? '-0' : String(value)) !== data.definition) fail('number is not canonical finite binary64');
    } else if (kind === 'boolean') {
      if (!['true', 'false'].includes(data.definition)) fail('invalid boolean');
      value = data.definition === 'true';
    } else if (kind === 'null') value = null;
    else fail('unknown structured data kind');
    active.delete(identity);
    memo.set(identity, value);
    costs.set(identity, { nodes: visits - initialVisits, bytes: bytes - initialBytes,
      height: references.reduce((height, reference) => Math.max(height, 1 + costs.get(key(reference)).height), 0) });
    return value;
  }
  if (network.link(root)?.metadata().term !== terms.root) fail('missing schema root');
  const value = inspect(root, 0);
  if (reached.size !== links.length) fail('unreachable data links');
  // Entry and code-unit records can occur only in their owning typed collections.
  for (const link of links) {
    const role = readStructuredDataKind(link.metadata().term ?? '');
    for (const reference of link.references()) {
      const childRole = readStructuredDataKind(network.link(reference)?.metadata().term ?? '');
      if ((childRole === 'pair' && role !== 'object') || (childRole === 'unit' && role !== 'units')) fail('misplaced structural role');
    }
  }
  return value;
}

/** Serialize plain data as canonical inspectable Links Notation. */
export function structuredDataToLino(value, options = {}) { return encodeStructuredDataLinks(value, options).network.toLino(); }

/** Read inspectable data without initializing any programming language runtime. */
export function readStructuredDataLino(source, options = {}) {
  const limits = limitsOf(options);
  if (encoder.encode(source).length > limits.byteLimit) fail('byte resource limit exceeded');
  const ids = new Set();
  const network = new StructuredDataNetwork();
  const roots = [];
  const parser = new StreamParser({ comments: false, collect: false, maxInputSize: limits.byteLimit, maxBufferSize: limits.byteLimit, maxDepth: 8 });
  parser.on('error', error => fail(`Links Notation parse error: ${error.message}`));
  parser.on('link', statement => {
    if (ids.size >= limits.nodeLimit) fail('node resource limit exceeded');
    if (!/^[1-9][0-9]*$/u.test(statement.id ?? '') || !Number.isSafeInteger(Number(statement.id)) || ids.has(statement.id)) fail('invalid or duplicate link identity');
    ids.add(statement.id);
    const sublinks = statement.values.filter(value => value.values.length > 0);
    if (sublinks.length !== 1 || sublinks[0].id !== 'meta') fail('invalid metadata envelope');
    const fields = new Set();
    for (const field of sublinks[0].values) {
      if (!['t', 'n', 'term', 'def', 'lang'].includes(field.id) || fields.has(field.id)) fail('unknown or duplicate metadata field');
      fields.add(field.id);
      if (field.values.length !== 1 || field.values[0].values.length !== 0) fail('invalid metadata scalar');
      if (field.id === 'n' && field.values[0].id !== '1') fail('invalid named flag');
    }
    if (!['t', 'n', 'term', 'lang'].every(field => fields.has(field))) fail('missing metadata field');
    for (const reference of statement.values.filter(value => value.values.length === 0)) {
      if (!/^[1-9][0-9]*$/u.test(reference.id ?? '') || !Number.isSafeInteger(Number(reference.id))) fail('invalid reference identity');
    }
    const [record] = decodeCanonicalStatements([statement]);
    network._links.set(record.id, new Link(record.id, record.references, record.metadata));
    network._nextId = Math.max(network._nextId, record.id + 1);
    if (record.metadata.term === terms.root) roots.push(record.id);
  });
  for (let offset = 0; offset < source.length; offset += 64 * 1024) parser.write(source.slice(offset, offset + 64 * 1024));
  parser.end();
  if (roots.length !== 1) fail('exactly one schema root is required');
  return decodeStructuredDataLinks(network, LinkId.from(roots[0]), limits);
}
