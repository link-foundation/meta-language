// Large reports remain typed linked data. Each bounded fragment describes
// portions of the original object graph, including repeated object references.
import { createHash, randomUUID } from 'node:crypto';
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from 'node:fs';
import { basename, dirname, join } from 'node:path';
import { structuredDataToLino, readStructuredDataLino } from '../src/structured-data-links.js';
import { defaults, encoder, limitsOf, wellFormedString } from '../src/structured-data-model.js';
import { readProjectData, resolveProjectDataPath } from './project-data.mjs';

const FORMAT = 'meta-language grammar report fragments';
const PART = 'meta-language grammar report object segments';
const digest = text => createHash('sha256').update(text).digest('hex');
const fail = message => { throw new TypeError(`grammar report: ${message}`); };

function reportLimits(options) {
  const limits = limitsOf(options);
  for (const name of Object.keys(defaults)) if (limits[name] > defaults[name]) fail(`cannot increase ${name}`);
  return limits;
}

// Expanded costs bound the codec's actual retained graph and repeated-reference
// traversal. Memoized heights preserve its original depth and cycle policy.
function inspect(value, limits) {
  const active = new WeakSet();
  const memo = new WeakMap();
  function visit(item, depth = 1) {
    if (depth > limits.depthLimit) fail('depth resource limit exceeded');
    if (item === null || typeof item === 'boolean') return { nodes: 1, bytes: 5, height: 0 };
    if (typeof item === 'number') {
      if (!Number.isFinite(item)) fail('a number must be finite binary64');
      return { nodes: 1, bytes: String(item).length, height: 0 };
    }
    if (typeof item === 'string') {
      const height = wellFormedString(item) ? 0 : 1;
      if (depth + height > limits.depthLimit) fail('depth resource limit exceeded');
      return { nodes: height === 0 ? 1 : 1 + item.length, bytes: encoder.encode(item).length, height };
    }
    if (!item || typeof item !== 'object') fail('unsupported host value');
    if (active.has(item)) fail('structural cycle');
    if (memo.has(item)) {
      const cost = memo.get(item);
      if (depth + cost.height > limits.depthLimit) fail('depth resource limit exceeded');
      return cost;
    }
    const array = Array.isArray(item);
    if (!array && ![Object.prototype, null].includes(Object.getPrototypeOf(item))) fail('unsupported host object');
    const names = Reflect.ownKeys(item);
    for (const name of names) {
      if (typeof name !== 'string') fail('symbol field');
      const descriptor = Object.getOwnPropertyDescriptor(item, name);
      if (!Object.hasOwn(descriptor, 'value') || (!descriptor.enumerable && !(array && name === 'length'))) fail('non-data field');
    }
    if (array && (names.length !== item.length + 1 || names.slice(0, -1).some((name, index) => name !== String(index)))) fail('sparse array or array field');
    active.add(item);
    const cost = { nodes: 1, bytes: 0, height: 0 };
    for (const name of array ? Array.from({ length: item.length }, (_, index) => String(index)) : Object.keys(item)) {
      const child = visit(item[name], depth + (array ? 1 : 2));
      const key = array ? { nodes: 0, bytes: 0, height: 0 } : visit(name, depth + 2);
      cost.nodes = Math.min(Number.MAX_SAFE_INTEGER, cost.nodes + child.nodes + key.nodes + (array ? 0 : 1));
      cost.bytes = Math.min(Number.MAX_SAFE_INTEGER, cost.bytes + child.bytes + key.bytes);
      cost.height = Math.max(cost.height, child.height + (array ? 1 : 2));
      if (!array) cost.height = Math.max(cost.height, key.height + 2);
    }
    active.delete(item);
    memo.set(item, cost);
    return cost;
  }
  const cost = visit(value);
  if (cost.height + 1 > limits.depthLimit) fail('depth resource limit exceeded');
  return { cost, measure: visit };
}

function atomic(file, text) {
  mkdirSync(dirname(file), { recursive: true });
  const temporary = join(dirname(file), `.${basename(file)}.${randomUUID()}.tmp`);
  try { writeFileSync(temporary, text, { flag: 'wx' }); renameSync(temporary, file); }
  finally { if (existsSync(temporary)) rmSync(temporary); }
}

export function writeFactoredGrammarReport(file, report, { limits: options } = {}) {
  const limits = reportLimits(options);
  const actual = resolveProjectDataPath(file, { format: 'lino' });
  const { cost } = inspect(report, limits);
  const nodeBudget = Math.min(50_000, Math.floor(limits.nodeLimit / 3));
  const byteBudget = Math.min(1024 * 1024, Math.floor(limits.byteLimit / 3));
  if (nodeBudget < 24 || byteBudget < 256) fail('fragment limits cannot hold the typed report envelope');
  if (cost.nodes + 1 <= nodeBudget && report?.format !== FORMAT) {
    atomic(actual, structuredDataToLino(report, limits));
    return actual;
  }

  const pending = [];
  const identities = new WeakMap();
  const reference = value => {
    if (value !== null && typeof value === 'object') {
      let identity = identities.get(value);
      if (identity === undefined) { identity = pending.length + 1; identities.set(value, identity); pending.push(value); }
      return { reference: identity };
    }
    return { value };
  };
  const root = reference(report);
  const parts = [];
  const staging = join(dirname(actual), `.${basename(actual)}.${randomUUID()}.parts`);
  mkdirSync(staging, { recursive: true });
  let segments = [];
  let segmentCost = 12;
  let segmentBytes = 0;
  const flush = () => {
    if (!segments.length) return;
    const text = structuredDataToLino({ format: PART, schemaVersion: 1, segments }, limits);
    const sha256 = digest(text);
    const name = `${sha256}.lino`;
    writeFileSync(join(staging, name), text);
    parts.push({ file: name, sha256 });
    segments = []; segmentCost = 12; segmentBytes = 0;
  };
  const append = segment => {
    const cost = inspect(segment, limits).cost;
    if (cost.nodes + 12 > nodeBudget || cost.bytes > limits.byteLimit / 2) fail('one report segment exceeds the fragment budget');
    if (segmentCost + cost.nodes > nodeBudget || segmentBytes + cost.bytes > byteBudget) flush();
    segments.push(segment); segmentCost += cost.nodes; segmentBytes += cost.bytes;
  };
  try {
    for (let index = 0; index < pending.length; index += 1) {
      const value = pending[index];
      const array = Array.isArray(value);
      const names = array ? Array.from({ length: value.length }, (_, at) => String(at)) : Object.keys(value);
      let offset = 0;
      let entries = [];
      let size = 13;
      let bytes = 0;
      for (const name of names) {
        const item = array ? reference(value[name]) : [name, reference(value[name])];
        const cost = inspect(item, limits).cost;
        if (entries.length && (size + cost.nodes + 12 > nodeBudget || bytes + cost.bytes > byteBudget)) {
          append({ identity: index + 1, kind: array ? 'array' : 'object', offset, entries });
          offset += entries.length; entries = []; size = 13; bytes = 0;
        }
        entries.push(item); size += cost.nodes; bytes += cost.bytes;
      }
      append({ identity: index + 1, kind: array ? 'array' : 'object', offset, entries });
    }
    flush();
    const directory = `${basename(actual)}.parts-${digest(parts.map(part => part.sha256).join('\n'))}`;
    const target = join(dirname(actual), directory);
    const manifest = structuredDataToLino({ format: FORMAT, schemaVersion: 1, root, directory, parts }, limits);
    if (existsSync(target)) {
      if (!lstatSync(target).isDirectory()) fail('foreign fragment directory');
      const names = readdirSync(target);
      if (names.length !== parts.length || names.some(name => !parts.some(part => part.file === name))) fail('unexpected report fragment');
      for (const part of parts) if (!lstatSync(join(target, part.file)).isFile() || digest(readFileSync(join(target, part.file))) !== part.sha256) fail('report fragment checksum mismatch');
      rmSync(staging, { recursive: true });
    }
    else renameSync(staging, target);
    atomic(actual, manifest);
    return actual;
  } finally { if (existsSync(staging)) rmSync(staging, { recursive: true }); }
}

export function readFactoredGrammarReport(file, { limits: options } = {}) {
  const limits = reportLimits(options);
  const actual = resolveProjectDataPath(file);
  const manifest = readProjectData(actual, { limits });
  if (manifest?.format !== FORMAT) return manifest;
  if (manifest.schemaVersion !== 1 || Object.keys(manifest).join(',') !== 'format,schemaVersion,root,directory,parts') fail('foreign fragment index');
  if (!Array.isArray(manifest.parts) || !manifest.parts.length) fail('missing report fragments');
  const directory = `${basename(actual)}.parts-${digest(manifest.parts.map(part => part.sha256).join('\n'))}`;
  if (manifest.directory !== directory) fail('foreign fragment directory');
  const folder = join(dirname(actual), directory);
  if (!lstatSync(folder).isDirectory()) fail('foreign fragment directory');
  const expected = new Set();
  for (const part of manifest.parts) {
    if (Object.keys(part).join(',') !== 'file,sha256' || !/^[a-f0-9]{64}$/u.test(part.sha256) || part.file !== `${part.sha256}.lino` || expected.has(part.file)) fail('foreign or duplicate report fragment');
    expected.add(part.file);
  }
  const present = readdirSync(folder);
  if (present.length !== expected.size || present.some(name => !expected.has(name))) fail('missing or unexpected report fragment');
  const objects = new Map();
  const metadata = new Map();
  const readPart = part => {
    if (!lstatSync(join(folder, part.file)).isFile()) fail('foreign report fragment');
    const text = readFileSync(join(folder, part.file), 'utf8');
    if (digest(text) !== part.sha256) fail('report fragment checksum mismatch');
    const block = readStructuredDataLino(text, limits);
    if (Object.keys(block).join(',') !== 'format,schemaVersion,segments' || block.format !== PART || block.schemaVersion !== 1 || !Array.isArray(block.segments)) fail('foreign report fragment');
    return block;
  };
  // The first bounded pass allocates every identity. The second populates
  // them, so forward references retain sharing without retaining all parts.
  for (const part of manifest.parts) {
    const block = readPart(part);
    for (const segment of block.segments) {
      if (Object.keys(segment).join(',') !== 'identity,kind,offset,entries' || !Number.isSafeInteger(segment.identity) || segment.identity < 1
        || !['array', 'object'].includes(segment.kind) || !Number.isSafeInteger(segment.offset) || segment.offset < 0 || !Array.isArray(segment.entries)) fail('invalid object segment');
      let info = metadata.get(segment.identity);
      if (!info) {
        if (segment.identity !== metadata.size + 1 || segment.offset !== 0) fail('invalid object identity order');
        info = { kind: segment.kind, length: 0, names: new Set(), order: [], references: [], scalarHeight: 0 };
        metadata.set(segment.identity, info);
        objects.set(segment.identity, segment.kind === 'array' ? [] : {});
      }
      if (info.kind !== segment.kind || info.length !== segment.offset || (segment.offset > 0 && segment.entries.length === 0)) fail('duplicate or discontinuous object segment');
      info.length += segment.entries.length;
      if (info.kind === 'object') for (const entry of segment.entries) {
        if (!Array.isArray(entry) || entry.length !== 2 || typeof entry[0] !== 'string' || info.names.has(entry[0])) fail('invalid or duplicate report field');
        info.names.add(entry[0]); info.order.push(entry[0]);
        Object.defineProperty(objects.get(segment.identity), entry[0], { value: undefined, enumerable: true, writable: true, configurable: true });
      }
    }
  }
  for (const [identity, info] of metadata) if (info.kind === 'object') {
    const object = objects.get(identity);
    if (Object.keys(object).some((name, index) => name !== info.order[index])) {
      objects.set(identity, new Proxy(object, { ownKeys: target => [
        ...info.order.filter(name => Object.hasOwn(target, name)),
        ...Reflect.ownKeys(target).filter(name => !info.names.has(name)),
      ] }));
    }
  }
  const valueOf = (descriptor, references) => {
    if (!descriptor || typeof descriptor !== 'object' || Array.isArray(descriptor)) fail('invalid value reference');
    const keys = Object.keys(descriptor);
    if (keys.length === 1 && keys[0] === 'value' && (descriptor.value === null || ['boolean', 'string', 'number'].includes(typeof descriptor.value))) return descriptor.value;
    if (keys.length === 1 && keys[0] === 'reference' && Number.isSafeInteger(descriptor.reference) && objects.has(descriptor.reference)) {
      references.push(descriptor.reference); return objects.get(descriptor.reference);
    }
    fail('dangling or foreign value reference');
  };
  for (const part of manifest.parts) for (const segment of readPart(part).segments) {
    const object = objects.get(segment.identity);
    const info = metadata.get(segment.identity);
    for (const entry of segment.entries) {
      if (info.kind === 'array') object.push(valueOf(entry, info.references));
      else {
        Object.defineProperty(object, entry[0], { value: valueOf(entry[1], info.references), enumerable: true, writable: true, configurable: true });
      }
      const descriptor = info.kind === 'array' ? entry : entry[1];
      if (Object.hasOwn(descriptor, 'value')) info.scalarHeight = Math.max(info.scalarHeight,
        (info.kind === 'array' ? 1 : 2) + (typeof descriptor.value === 'string' && !wellFormedString(descriptor.value) ? 1 : 0));
      if (info.kind === 'object') info.scalarHeight = Math.max(info.scalarHeight, wellFormedString(entry[0]) ? 2 : 3);
    }
  }
  const roots = [];
  const result = valueOf(manifest.root, roots);
  const active = new Set();
  const reached = new Set();
  const heights = new Map();
  const visit = identity => {
    if (active.has(identity)) fail('structural cycle');
    if (heights.has(identity)) return heights.get(identity);
    active.add(identity); reached.add(identity);
    const info = metadata.get(identity);
    const height = info.references.reduce((maximum, child) => Math.max(maximum, visit(child) + (info.kind === 'array' ? 1 : 2)), info.scalarHeight);
    active.delete(identity); heights.set(identity, height); return height;
  };
  for (const root of roots) if (visit(root) + 1 > limits.depthLimit) fail('depth resource limit exceeded');
  if (reached.size !== objects.size) fail('unreachable report objects');
  return result;
}
