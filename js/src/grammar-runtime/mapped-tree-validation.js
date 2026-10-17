// Validate source correspondence after generic Links decoding. This checks
// mapping/ownership, not that either CST is licensed by a language grammar.
import { Executor, DEFAULT_MAX_DEPTH, memoryBudget, stepBudget, NestingTooDeep } from './executor.js';
import { decodeAt, encodeText, textOf } from './text.js';
import { mapGrammarSelectionSource } from './selection-source-mapping.js';
import { readGrammarSelectionMappingFieldOrder, acceptGrammarSelectionSourceDefinition,
  acceptGrammarSelectionSourceDerivation, acceptGrammarSelectionSourceInterval,
  acceptGrammarSelectionSourceRoot, acceptGrammarSelectionMappingReference,
  acceptGrammarSelectionMappingOccurrence, acceptGrammarSelectionPhysicalSpan,
  readGrammarSelectionTreeChildren, acceptGrammarSelectionMappingProfile,
  acceptGrammarSelectionMappingDescriptor } from '../translation/grammar-selection-mapping-rules.js';

const fail = detail => { throw new TypeError(`mapped grammar CST: ${detail}`); };

function exactKeys(value, keys) {
  return value && typeof value === 'object' && !Array.isArray(value)
    && Object.keys(value).length === keys.length && keys.every((key, index) => Object.keys(value)[index] === key);
}

// Reflect on descriptors before reading any public property. Copying ordinary
// plain data into null-prototype records also prevents absent optional fields
// from resolving inherited accessors. This is a host-data boundary, not an
// assertion that generic Links decoding authenticated the mapping.
function dataEntries(value) {
  if (!value || typeof value !== 'object') fail('ordinary plain data is required');
  const array = Array.isArray(value), prototype = Object.getPrototypeOf(value);
  if (array ? prototype !== Array.prototype && prototype !== null
    : prototype !== Object.prototype && prototype !== null) fail('unsupported host prototype');
  const names = Reflect.ownKeys(value), entries = [];
  for (const name of names) {
    if (typeof name !== 'string') fail('symbol data field');
    const descriptor = Object.getOwnPropertyDescriptor(value, name);
    if (!Object.hasOwn(descriptor, 'value') || (!descriptor.enumerable && !(array && name === 'length'))) fail('non-data field');
    entries.push([name, descriptor.value]);
  }
  if (array) {
    const length = entries.find(([name]) => name === 'length')?.[1];
    if (names.length !== length + 1 || !names.slice(0, -1).every((name, index) => name === String(index))
        || names[names.length - 1] !== 'length') fail('sparse array or array field');
  }
  return entries;
}

/** Uses the executor's actual unchanged defaults and reservation operations.
 * Source definitions occur once in an ordered table. References are local
 * table positions, not host object identities or LinkNetwork identifiers.
 */
export function checkGrammarSelectionMappedTree(bundle, options = {}) {
  if (Array.isArray(bundle) || Array.isArray(options)) fail('ordinary plain records are required');
  const header = new Map(dataEntries(bundle)), originalTree = header.get('tree');
  if (!originalTree || Array.isArray(originalTree)) fail('a complete physical root CST is required');
  const rootHeader = new Map(dataEntries(originalTree)), end = rootHeader.get('end');
  if (!Number.isSafeInteger(end) || end < 0 || end > 4294967295) fail('a complete physical root CST is required');
  const settings = Object.create(null);
  for (const [key, value] of dataEntries(options)) settings[key] = value;
  for (const key of ['memoryLimit', 'stepLimit', 'maxDepth']) if (Object.hasOwn(settings, key)
    && (!Number.isSafeInteger(settings[key]) || settings[key] < 0)) fail('invalid resource option');
  if (Object.hasOwn(settings, 'errorRecovery') && typeof settings.errorRecovery !== 'boolean') fail('invalid resource option');
  const memory = memoryBudget(settings), maximumDepth = settings.maxDepth ?? DEFAULT_MAX_DEPTH;
  const control = { bytes: null, budget: stepBudget(settings, end, memory),
    step: Executor.prototype.step, retain: Executor.prototype.retain };
  const depth = level => {
    control.step();
    if (level > maximumDepth) throw new NestingTooDeep();
  };
  const active = new Set(), copies = new Map();
  const snapshot = (value, level) => {
    depth(level);
    if (value === null || typeof value === 'string' || typeof value === 'boolean'
        || typeof value === 'number' && Number.isFinite(value)) return value;
    if (!value || typeof value !== 'object') fail('unsupported host data');
    if (active.has(value)) fail('cyclic host data');
    if (copies.has(value)) return copies.get(value);
    const entries = dataEntries(value), copy = Array.isArray(value) ? [] : Object.create(null);
    control.retain(entries.length + 1);
    copies.set(value, copy);
    active.add(value);
    for (const [key, child] of entries) {
      control.step();
      if (Array.isArray(value) && key === 'length') continue;
      Object.defineProperty(copy, key, { value: snapshot(child, level + 1), enumerable: true });
    }
    active.delete(value);
    return copy;
  };
  const { tree, sourcePlanes } = snapshot(bundle, 0);
  if (!Array.isArray(sourcePlanes) || sourcePlanes.length === 0) fail('the source-plane table is required');
  const physical = (nodes, begin, end, level) => {
    const bytes = [];
    let cursor = begin;
    const visit = (node, nesting) => {
      depth(nesting);
      control.retain(1);
      if (!node || typeof node !== 'object') fail('invalid physical tree');
      const begin = cursor;
      const role = readGrammarSelectionTreeChildren(node.type, node.sourceChildren !== undefined);
      if (role === 'leaf') {
        let count;
        if (typeof node.text === 'string') count = Buffer.byteLength(node.text, 'utf8');
        else if (node.text === null && typeof node.hex === 'string' && /^(?:[0-9a-f]{2})*$/u.test(node.hex)) count = node.hex.length / 2;
        else fail('invalid physical leaf bytes');
        if (!acceptGrammarSelectionPhysicalSpan(node.start, node.end, cursor, count, false)) fail('physical leaf span differs');
        control.retain(count + 1);
        const stored = typeof node.text === 'string' ? encodeText(node.text)
          : Uint8Array.from(node.hex.match(/../gu) ?? [], word => Number.parseInt(word, 16));
        if (typeof node.text === 'string' && textOf(stored, 0, stored.length) !== node.text) fail('non-scalar physical leaf text');
        for (const byte of stored) { control.step(); bytes.push(byte); }
        cursor = node.end;
      } else if (role === 'missing') {
        if (!acceptGrammarSelectionPhysicalSpan(node.start, node.end, cursor, 0, true)) fail('invalid missing span');
      } else if (role === 'children') {
        if (!Array.isArray(node.children)) fail('missing concrete children');
        for (const child of node.children) visit(child, nesting + 1);
      } else if (role === 'root' || role === 'sourceChildren') {
        if (role === 'sourceChildren') {
          if (!Array.isArray(node.sourceChildren)) fail('invalid physical source children');
          for (const child of node.sourceChildren) visit(child, nesting + 1);
        } else visit(node.root, nesting + 1);
      } else fail('unknown concrete node kind');
      if (role !== 'leaf' && role !== 'missing' && !acceptGrammarSelectionPhysicalSpan(node.start,
        node.end, begin, cursor - begin, false)) fail('physical container span differs');
    };
    for (const node of nodes) visit(node, level);
    if (cursor !== end) fail('incomplete physical source plane');
    return Uint8Array.from(bytes);
  };
  // Parsing root invocations consume the entire source. Embedded roots have
  // their own explicit plane; their byte offsets are never added to a parent.
  const rootBytes = physical([tree], 0, tree?.end, 0);
  for (let position = 0; position < rootBytes.length;) {
    control.step();
    const scalar = decodeAt(rootBytes, position);
    if (scalar.codePoint < 0) fail('the root source plane is not UTF-8');
    position += scalar.length;
  }
  const planes = [];
  for (let identity = 0; identity < sourcePlanes.length; identity += 1) {
    control.step();
    control.retain(1);
    const definition = sourcePlanes[identity];
    if (!exactKeys(definition, readGrammarSelectionMappingFieldOrder('source').split('|'))) fail('invalid ordered source definition');
    if (!acceptGrammarSelectionSourceRoot(identity, definition.derivation === null)) fail('invalid source derivation presence');
    if (identity === 0) {
      if (!acceptGrammarSelectionSourceDefinition(definition.kind, definition.identity,
        identity, definition.encoding, definition.byteLength, rootBytes.length)) fail('invalid root source definition');
      planes.push({ identity, bytes: rootBytes });
      continue;
    }
    const origin = definition.derivation;
    if (!exactKeys(origin, readGrammarSelectionMappingFieldOrder('derivation').split('|'))) fail('unowned source derivation');
    const parent = typeof origin.input === 'number' ? planes[origin.input] : null;
    if (!parent || !acceptGrammarSelectionSourceDerivation(origin.operation, origin.input, identity,
      origin.start, origin.end, parent.bytes.length)) fail('unowned source derivation');
    control.bytes = parent.bytes;
    const derived = mapGrammarSelectionSource(control, origin.start, origin.end);
    if (derived.reason || derived.mapping.output !== identity || !acceptGrammarSelectionSourceDefinition(definition.kind,
      definition.identity, identity, definition.encoding, definition.byteLength, derived.bytes.length)) fail('invalid source derivation');
    planes.push({ identity, bytes: derived.bytes });
  }
  const walk = (node, plane, level) => {
    depth(level);
    if (node.type === 'node') {
      for (const child of node.children) walk(child, plane, level + 1);
      return;
    }
    if (node.type !== 'embed') return;
    const mapped = node.selection?.mapping === 'json-string';
    if (!acceptGrammarSelectionMappingProfile(node.selection?.mapping ?? 'identity',
      node.sourceChildren !== undefined, node.selection?.sourceMapping !== undefined)) fail('unknown mapping profile');
    if (!mapped) {
      walk(node.root, plane, level + 1);
      for (const alternative of node.selection?.alternatives ?? []) walk(alternative.root, plane, level + 1);
      return;
    }
    const mapping = node.selection.sourceMapping;
    if (!exactKeys(mapping, readGrammarSelectionMappingFieldOrder('mapping').split('|'))
        || !acceptGrammarSelectionMappingDescriptor(mapping.kind, mapping.operation)
        || !Array.isArray(node.sourceChildren) || mapping.input === mapping.output
        || !acceptGrammarSelectionMappingOccurrence(mapping.input, plane.identity,
          node.selection.start, node.start, node.selection.end, node.end)) fail('invalid mapped boundary');
    if (!acceptGrammarSelectionMappingReference(mapping.input, plane.identity, mapping.output, planes.length)) fail('unowned source-plane reference');
    const output = sourcePlanes[mapping.output], origin = output.derivation;
    if (!acceptGrammarSelectionMappingOccurrence(origin.input, mapping.input, origin.start, node.start,
      origin.end, node.end)) fail('mapping belongs to another source occurrence');
    control.bytes = plane.bytes;
    const derived = mapGrammarSelectionSource(control, node.start, node.end);
    if (derived.reason) fail(derived.reason);
    if (!Array.isArray(mapping.intervals) || mapping.intervals.length !== derived.mapping.intervals.length
        || !mapping.intervals.every((row, index) => Array.isArray(row)
          && acceptGrammarSelectionSourceInterval(row, derived.mapping.intervals[index]))) fail('ordered source intervals differ');
    if (derived.mapping.output !== mapping.output) fail('source reference differs from its actual derivation');
    const childPlane = planes[mapping.output];
    for (const child of node.sourceChildren) walk(child, plane, level + 1);
    for (const root of [node.root, ...node.selection.alternatives.map(alternative => alternative.root)]) {
      const stored = physical([root], 0, derived.bytes.length, level + 1);
      if (stored.length !== derived.bytes.length || stored.some((byte, index) => byte !== derived.bytes[index])) fail('decoded tree bytes differ');
      walk(root, childPlane, level + 1);
    }
  };
  walk(tree, planes[0], 0);
  return bundle;
}
