import { inflateRawSync } from 'node:zlib';
import { documentStreamFitsBudget } from './document-container-policy.js';

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

export function readWordprocessingPackage(bytes) {
  const directory = zipDirectory(bytes);
  if (directory.error) return { parts: [], errors: [directory] };
  const parts = [];
  const errors = [];
  let textBytes = 0;
  for (const entry of directory.entries) {
    if (!/^word\/(?:document|header\d*|footer\d*|footnotes|endnotes)\.xml$/u.test(entry.name)) continue;
    try {
      const { name, start, end, method, flags, uncompressed, checksum } = entry;
      if ((flags & 0x2041) || (method !== 0 && method !== 8)) throw new Error('unsupported OPC part compression or encryption');
      if (!documentStreamFitsBudget(end - start, uncompressed, textBytes)) throw new Error('OPC part exceeds the decoded size limit');
      // Charge the announced output before decoding so corrupt parts also use the budget.
      textBytes += uncompressed;
      const data = bytes.subarray(start, end);
      let decoded = data;
      if (method === 8) {
        try { decoded = inflateRawSync(data, { maxOutputLength: uncompressed + 1 }); }
        catch { throw new Error('invalid or oversized deflated OPC part'); }
      }
      if (decoded.length !== uncompressed || zipChecksum(decoded) !== checksum) throw new Error('OPC part size or checksum mismatch');
      let text;
      try { text = packageText.decode(decoded); }
      catch { throw new Error('invalid UTF-8 OPC part'); }
      parts.push({ name, start, end, compression: method === 8 ? 'deflate' : 'stored', text });
    } catch (error) {
      errors.push({ error: `${entry.name}: ${error.message}`, offset: entry.start, end: entry.end });
    }
  }
  if (!parts.some(part => part.name === 'word/document.xml')) errors.push({ error: 'missing WordprocessingML document part', offset: 0 });
  parts.sort((left, right) => wordPartOrder(left.name) - wordPartOrder(right.name) || left.start - right.start);
  return { parts, errors };
}

function wordPartOrder(name) {
  if (name === 'word/document.xml') return 0;
  if (name.startsWith('word/header')) return 1;
  if (name.startsWith('word/footer')) return 2;
  return 3;
}
