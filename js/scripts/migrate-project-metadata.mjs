// Migrate only owned repository metadata with the shared, lossless converter.
// Foreign source bytes, native fixtures and runtime mirrors have separate owners.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, unlinkSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { convertStructuredData } from './convert-structured-data.mjs';
import { structuredDataToLino } from '../src/structured-data-links.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const args = process.argv.slice(2);
const receiptAt = args.indexOf('--receipt');
const receiptFile = receiptAt < 0 ? null : args[receiptAt + 1];
if (receiptAt >= 0 && !receiptFile) throw new Error('--receipt needs a path');
const files = execFileSync('git', ['ls-files', '*.json'], { cwd: root, encoding: 'utf8' }).trim().split('\n')
  .filter(file => file.startsWith('parity/') && !file.startsWith('parity/fixtures/') && !file.startsWith('parity/sources/'))
  .filter(file => !args.includes('--exclude-language-features') || file !== 'parity/language-features.json')
  .filter(file => existsSync(join(root, file)));
for (const file of files) {
  if (existsSync(join(root, file.replace(/\.json$/u, '.lino')))) throw new Error(`duplicate metadata storage: ${file}`);
}
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const converted = [];
for (const logicalPath of files) {
  const source = readFileSync(join(root, logicalPath));
  // Conversion independently decodes and compares values and every key order.
  const lino = convertStructuredData(source.toString('utf8'));
  const physicalPath = logicalPath.replace(/\.json$/u, '.lino');
  writeFileSync(join(root, physicalPath), lino, { flag: 'wx' });
  const bytes = readFileSync(join(root, physicalPath));
  if (!bytes.equals(Buffer.from(lino))) throw new Error(`metadata write differs: ${physicalPath}`);
  unlinkSync(join(root, logicalPath));
  converted.push({ logicalPath, physicalPath, originalBytes: source.length, convertedBytes: bytes.length,
    originalSha256: digest(source), convertedSha256: digest(bytes) });
  console.log(`converted ${logicalPath}`);
}
if (receiptFile) writeFileSync(resolve(receiptFile), structuredDataToLino({ schemaVersion: 1, conversions: converted }));
console.log(`Converted ${converted.length} metadata records through the shared codec; values and ordering preserved.`);
