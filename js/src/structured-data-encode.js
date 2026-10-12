// Dependency-free structured graph encoding and its canonical LiNo writer.
import { Link, LinkId, LinkMetadata } from './primitives.js';
import { encodeNetworkLino } from './lino-serialization-write.js';
import { encoder, fail, key, metadata, limitsOf, wellFormedString } from './structured-data-model.js';

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


/** Serialize plain data as canonical inspectable Links Notation. */
export function structuredDataToLino(value, options = {}) { return encodeStructuredDataLinks(value, options).network.toLino(); }

