#!/usr/bin/env node
// Transform fixture storage only. No grammar, oracle, or corpus is executed.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readdirSync, readFileSync, unlinkSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { convertStructuredData } from './convert-structured-data.mjs';
import { checkProjectData, readProjectData } from './project-data.mjs';
import { structuredDataToLino } from '../src/structured-data-links.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const directory = join(root, 'parity/fixtures/native-grammars');
const digest = value => createHash('sha256').update(JSON.stringify(value)).digest('hex');

export function migrateNativeFixtureData({ convert = false } = {}) {
  const names = readdirSync(directory).filter(name => /\.(json|lino)$/u.test(name)).sort();
  const stems = new Set();
  const baselines = [];
  for (const name of names) {
    const stem = name.replace(/\.(json|lino)$/u, '');
    assert.ok(!stems.has(stem), `${stem} has duplicate fixture storage`);
    stems.add(stem);
    const logicalPath = `parity/fixtures/native-grammars/${stem}.json`;
    const file = join(directory, name);
    let value;
    if (name.endsWith('.json')) {
      assert.ok(convert, `${logicalPath} still uses JSON storage`);
      const source = readFileSync(file, 'utf8');
      value = JSON.parse(source);
      const physical = join(directory, `${stem}.lino`);
      // The shared converter proves values and key order before writing.
      writeFileSync(physical, convertStructuredData(source), { flag: 'wx' });
      unlinkSync(file);
    } else value = readProjectData(file);
    const decoded = readProjectData(join(root, logicalPath));
    assert.deepEqual(decoded, value, `${logicalPath} preserves decoded fixture data`);
    assert.equal(digest(decoded), digest(value), `${logicalPath} preserves key order`);
    checkProjectData(join(root, logicalPath), decoded);
    baselines.push({ logicalPath, decodedSha256: digest(value) });
  }
  assert.ok(baselines.length > 0, 'native fixtures exist');
  return baselines;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  const baselineAt = args.indexOf('--baseline');
  const baselineFile = baselineAt < 0 ? null : args[baselineAt + 1];
  if (baselineAt >= 0 && !baselineFile) throw new Error('--baseline needs an output LiNo path');
  const recognized = args.filter((_, index) => baselineAt < 0 || index !== baselineAt && index !== baselineAt + 1);
  if (recognized.some(arg => arg !== '--convert' && arg !== '--check') || args.includes('--convert') && args.includes('--check')) {
    throw new Error('usage: migrate-native-fixture-data.mjs [--convert|--check] [--baseline output.lino]');
  }
  const baselines = migrateNativeFixtureData({ convert: args.includes('--convert') });
  if (baselineFile) writeFileSync(baselineFile, structuredDataToLino(baselines));
  console.log(`Validated ${baselines.length} native fixture files; decoded aggregate SHA-256 ${digest(baselines)}`);
}
