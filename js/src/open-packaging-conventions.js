import { inflateRawSync } from 'node:zlib';
import { documentStreamFitsBudget, wordprocessingRelationshipOrder } from './document-container-policy.js';

const localHeader = 0x04034b50;
const centralHeader = 0x02014b50;
const endOfDirectory = 0x06054b50;
const packageText = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true });

export function hasZipHeader(bytes) {
  return bytes.length >= 4 && bytes[0] === 0x50 && bytes[1] === 0x4b &&
    ((bytes[2] === 3 && bytes[3] === 4) || (bytes[2] === 5 && bytes[3] === 6));
}

export function readBinaryString(text) {
  const bytes = new Uint8Array(text.length);
  for (let index = 0; index < text.length; index += 1) {
    const byte = text.charCodeAt(index);
    if (byte > 255) return null;
    bytes[index] = byte;
  }
  return bytes;
}

// PKWARE APPNOTE 6.3.10, sections 4.3.7, 4.3.12 and 4.3.16. The central
// directory owns sizes and checksums, including for data-descriptor entries.
function zipDirectory(bytes) {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.length);
  const u16 = offset => view.getUint16(offset, true);
  const u32 = offset => view.getUint32(offset, true);
  let footer = bytes.length - 22;
  const minimum = Math.max(0, footer - 65535);
  while (footer >= minimum && (u32(footer) !== endOfDirectory || footer + 22 + u16(footer + 20) !== bytes.length)) footer -= 1;
  if (footer < minimum) return { error: 'missing ZIP end of central directory', offset: bytes.length };
  const count = u16(footer + 10);
  const directory = u32(footer + 16);
  if (u16(footer + 4) || u16(footer + 6) || count !== u16(footer + 8) || count === 65535 ||
      directory + u32(footer + 12) !== footer) return { error: 'invalid or unsupported ZIP central directory', offset: footer };
  const entries = [];
  const names = new Set();
  let offset = directory;
  for (let index = 0; index < count; index += 1) {
    if (offset + 46 > footer || u32(offset) !== centralHeader) return { error: 'invalid ZIP central entry', offset };
    const flags = u16(offset + 8);
    const method = u16(offset + 10);
    const checksum = u32(offset + 16);
    const compressed = u32(offset + 20);
    const uncompressed = u32(offset + 24);
    const nameLength = u16(offset + 28);
    const local = u32(offset + 42);
    const next = offset + 46 + nameLength + u16(offset + 30) + u16(offset + 32);
    if (next > footer || local + 30 > directory || u32(local) !== localHeader || u16(offset + 34) ||
        flags !== u16(local + 6) || method !== u16(local + 8) || nameLength !== u16(local + 26)) {
      return { error: 'invalid ZIP entry header', offset };
    }
    const start = local + 30 + nameLength + u16(local + 28);
    const end = start + compressed;
    const nameBytes = bytes.subarray(offset + 46, offset + 46 + nameLength);
    if (end > directory || !nameBytes.every((byte, index) => byte === bytes[local + 30 + index])) {
      return { error: 'invalid ZIP entry data range or name', offset: local };
    }
    let name;
    try { name = packageText.decode(nameBytes); }
    catch { return { error: 'invalid OPC part name', offset: local }; }
    if (names.has(name) || !name || name.includes('\\') || name.startsWith('/') ||
        name.split('/').some(component => component === '..' || component === '.')) {
      return { error: 'invalid or duplicate OPC part name', offset: local };
    }
    names.add(name);
    entries.push({ name, start, end, local, method, flags, checksum, uncompressed });
    offset = next;
  }
  if (offset !== footer) return { error: 'unrecognized ZIP central directory data', offset };
  const ordered = [...entries].sort((left, right) => left.local - right.local);
  if (ordered.some((entry, index) => index > 0 && entry.local < ordered[index - 1].end)) {
    return { error: 'overlapping ZIP entries', offset: directory };
  }
  return { entries };
}

let checksumTable;

function zipChecksum(bytes) {
  checksumTable ??= Uint32Array.from({ length: 256 }, (_, index) => {
    let value = index;
    for (let bit = 0; bit < 8; bit += 1) value = (value >>> 1) ^ (value & 1 ? 0xedb88320 : 0);
    return value;
  });
  let remainder = 0xffffffff;
  for (const byte of bytes) remainder = (remainder >>> 8) ^ checksumTable[(remainder ^ byte) & 0xff];
  return (remainder ^ 0xffffffff) >>> 0;
}

function decodeZipEntry(bytes, { start, end, method, flags, uncompressed, checksum }) {
  if ((flags & 0x2041) || (method !== 0 && method !== 8)) throw new Error('unsupported OPC part compression or encryption');
  let decoded = bytes.subarray(start, end);
  if (method === 8) {
    try { decoded = inflateRawSync(decoded, { maxOutputLength: uncompressed + 1 }); }
    catch { throw new Error('invalid or oversized deflated OPC part'); }
  }
  if (decoded.length !== uncompressed || zipChecksum(decoded) !== checksum) throw new Error('OPC part size or checksum mismatch');
  return decoded;
}

export function readNamedZipEntries(bytes, names) {
  const directory = zipDirectory(bytes);
  if (directory.error) throw new Error(directory.error);
  const result = new Map();
  for (const entry of directory.entries) {
    if (!names.has(entry.name)) continue;
    if (entry.uncompressed !== names.get(entry.name)) throw new Error(`unexpected ZIP size for ${entry.name}`);
    result.set(entry.name, decodeZipEntry(bytes, entry));
  }
  if (result.size !== names.size) throw new Error('missing requested ZIP entries');
  return result;
}

export function readWordprocessingPackage(bytes, parseRelationships) {
  const directory = zipDirectory(bytes);
  if (directory.error) return { parts: [], errors: [directory] };
  const parts = [];
  const errors = [];
  let textBytes = 0;
  const entries = new Map(directory.entries.map(entry => [entry.name, entry]));
  function readPart(entry) {
    const { start, end, uncompressed } = entry;
    if (!documentStreamFitsBudget(end - start, uncompressed, textBytes)) throw new Error('OPC part exceeds the decoded size limit');
    // Charge the announced output before decoding so corrupt parts also use the budget.
    textBytes += uncompressed;
    const decoded = decodeZipEntry(bytes, entry);
    try { return packageText.decode(decoded); }
    catch { throw new Error('invalid UTF-8 OPC part'); }
  }
  const diagnostics = (entry, error) => errors.push({ error: `${entry.name}: ${error.message}`, offset: entry.start, end: entry.end });
  const readRelations = (entry, accept, sourceName) => {
    const targets = [];
    try {
      for (const { type, target, mode } of parseRelationships(readPart(entry))) {
        const order = wordprocessingRelationshipOrder(type ?? '');
        if (!accept(order) || mode === 'External') continue;
        const name = mode === undefined || mode === 'Internal' ? resolvePackageTarget(sourceName, target) : null;
        if (!name || !entries.has(name)) {
          diagnostics(entry, new Error(`missing or invalid WordprocessingML relationship target: ${target ?? ''}`));
          continue;
        }
        targets.push({ name, order });
      }
    } catch (error) { diagnostics(entry, error); }
    return targets;
  };
  const packageRelationships = entries.get('_rels/.rels');
  const mainTargets = packageRelationships ? readRelations(packageRelationships, order => order === 0, '') : [];
  if (mainTargets.length > 1) diagnostics(packageRelationships, new Error('multiple WordprocessingML document relationships'));
  const main = packageRelationships ? (mainTargets.length === 1 ? mainTargets[0].name : null) : 'word/document.xml';
  const slash = main?.lastIndexOf('/') ?? -1;
  const partRelationships = main && entries.get(`${main.slice(0, slash + 1)}_rels/${main.slice(slash + 1)}.rels`);
  const selected = new Map();
  if (main && entries.has(main)) selected.set(main, 0);
  if (partRelationships) {
    for (const { name, order } of readRelations(partRelationships, value => value > 0 && value < 4, main)) {
      if (!selected.has(name)) selected.set(name, order);
    }
  } else if (!packageRelationships) {
    for (const name of entries.keys()) {
      if (/^word\/(?:header\d*|footer\d*|footnotes|endnotes)\.xml$/u.test(name)) selected.set(name, wordPartOrder(name));
    }
  }
  const ordered = [...selected].map(([name, order]) => ({ entry: entries.get(name), order }));
  ordered.sort((left, right) => left.order - right.order || left.entry.start - right.entry.start);
  for (const { entry, order } of ordered) {
    const { name, start, end, method } = entry;
    try { parts.push({ name, order, start, end, compression: method === 8 ? 'deflate' : 'stored', text: readPart(entry) }); }
    catch (error) { diagnostics(entry, error); }
  }
  if (!parts.some(part => part.order === 0)) errors.push({ error: 'missing WordprocessingML document part', offset: 0 });
  return { parts, errors };
}

function resolvePackageTarget(sourceName, target) {
  if (!target || /[\u0000-\u0020\u007f\\:#?]/u.test(target)) return null;
  const segments = target.startsWith('/') || !sourceName ? [] : sourceName.split('/').slice(0, -1);
  for (const segment of target.replace(/^\//u, '').split('/')) {
    if (!segment) return null;
    if (segment === '.') continue;
    if (segment === '..') {
      if (!segments.length) return null;
      segments.pop();
    } else segments.push(segment);
  }
  return segments.join('/') || null;
}

function wordPartOrder(name) {
  if (name.startsWith('word/header')) return 1;
  if (name.startsWith('word/footer')) return 2;
  return 3;
}
