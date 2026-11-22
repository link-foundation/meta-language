import { createHash, randomUUID } from 'node:crypto';
import { linkSync, lstatSync, readFileSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readProjectData, serializeProjectData, writeProjectData } from './project-data.mjs';
import { readStructuredDataLino } from '../src/structured-data-links.js';

const originalPartitionModel = 'Bounded native source receipt partitions 1';
const partitionModel = 'Content-addressed native source receipt partitions 1';
const maximumPartBytes = 72 * 1024 * 1024;
// Complete nominal projection decisions include shared immutable type rows.
// The current authored proof has 343,791 links and 1,055,646 expanded visits.
const maximumConstructorProofVisits = 1_200_000;
const fields = ['constructorMatchSharing', 'emitted', 'files'];
const partName = (file, index, digest) => new URL(file.href.replace(/\.lino$/u,
  `-part-${index + 1}${digest ? `-${digest}` : ''}.lino`));
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');

function readPartBytes(path, row) {
  const stat = lstatSync(path);
  if (!stat.isFile() || stat.isSymbolicLink() || stat.size !== row.bytes || realpathSync(path) !== fileURLToPath(path)) {
    throw new TypeError(`unexpected native source receipt part ${row.field}`);
  }
  const bytes = readFileSync(path);
  if (sha256(bytes) !== row.sha256) throw new TypeError(`changed native source receipt part ${row.field}`);
  return bytes;
}

/** Put large independent fields in exact, independently readable Links assets. */
export function writePartitionedNativeSourceReceipt(file, value) {
  const header = { ...value };
  const parts = [];
  for (const [index, field] of fields.entries()) {
    if (!Object.hasOwn(header, field)) continue;
    const contents = { [field]: header[field] };
    delete header[field];
    const text = serializeProjectData(contents, file);
    const bytes = Buffer.byteLength(text);
    if (bytes > maximumPartBytes) throw new RangeError(`native source receipt ${field} exceeds its Links asset bound`);
    const row = { field, bytes, sha256: sha256(text) };
    const path = partName(file, index, row.sha256);
    const temporary = new URL(path.href + '.' + randomUUID() + '.tmp');
    try {
      writeFileSync(temporary, text, { encoding: 'utf8', flag: 'wx' });
      try { linkSync(temporary, path); }
      catch (error) {
        if (error.code !== 'EEXIST') throw error;
        readPartBytes(path, row);
      }
    } finally {
      rmSync(temporary, { force: true });
    }
    parts.push(row);
    globalThis.gc?.();
  }
  writeProjectData(file, { ...header, nativeReceiptParts: { model: partitionModel, parts } });
}

/** Load the exact requested fields from the same receipt and verify their bytes. */
export function readNativeSourceReceipt(file, selectedFields = fields) {
  const header = readProjectData(file);
  const directory = header.nativeReceiptParts;
  if (directory === undefined) return header;
  if (![partitionModel, originalPartitionModel].includes(directory.model)
    || !Array.isArray(directory.parts) || directory.parts.length > fields.length) {
    throw new TypeError('unknown or malformed native source receipt partition directory');
  }
  const result = { ...header };
  delete result.nativeReceiptParts;
  const seen = new Set();
  for (const row of directory.parts) {
    const index = fields.indexOf(row.field);
    if (index === -1 || seen.has(row.field) || Object.hasOwn(header, row.field)
      || !Number.isSafeInteger(row.bytes) || row.bytes < 1 || row.bytes > maximumPartBytes
      || typeof row.sha256 !== 'string' || !/^[a-f0-9]{64}$/u.test(row.sha256)) {
      throw new TypeError('invalid native source receipt partition');
    }
    seen.add(row.field);
    if (!selectedFields.includes(row.field)) continue;
    const path = partName(file, index, directory.model === partitionModel ? row.sha256 : undefined);
    const limits = { byteLimit: maximumPartBytes };
    if (directory.model === partitionModel && row.field === 'constructorMatchSharing') {
      limits.nodeLimit = maximumConstructorProofVisits;
    }
    const part = readStructuredDataLino(readPartBytes(path, row).toString('utf8'), limits);
    if (!Object.hasOwn(part, row.field) || Object.keys(part).length !== 1) throw new TypeError('invalid native source receipt part body');
    result[row.field] = part[row.field];
  }
  if (!seen.has('files') || !seen.has('emitted')) throw new TypeError('native source receipt must retain its emission and files');
  return result;
}
