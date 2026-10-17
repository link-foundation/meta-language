// Bounded regeneration: only ANTLR reverse cases and their four derived Links
// fields. Historical payloads are authenticated from their immutable Git blob.
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { refreshAntlrReverseFixtureLexers } from './antlr-reverse-fixture-lexers.mjs';

const root = new URL('../../', import.meta.url);
const logicalPath = 'parity/fixtures/grammar-importers.json';
const physicalPath = logicalPath.replace(/\.json$/u, '.lino');
const digest = (value) => createHash('sha256').update(value).digest('hex');

export function generateAntlrReverseFixture({ check = false } = {}) {
  const baselines = readProjectData(new URL('parity/fixtures/structured-data-migration-baselines.lino', root));
  const revisions = readProjectData(new URL('parity/fixtures/structured-data-migration-revisions.lino', root));
  const baseline = baselines.find((entry) => entry.logicalPath === logicalPath);
  const revision = revisions.find((entry) => entry.logicalPath === logicalPath)?.migrationRevision;
  if (!baseline || !/^[0-9a-f]{40}$/u.test(revision ?? '')) throw new Error('ANTLR fixture refresh needs its exact immutable migration baseline');
  const original = readStructuredDataLino(execFileSync('git', ['show', `${revision}:${physicalPath}`],
    { cwd: fileURLToPath(root), encoding: 'utf8', maxBuffer: 1024 * 1024, timeout: 10_000 }));
  if (digest(JSON.stringify(original)) !== baseline.decodedSha256) throw new Error('ANTLR fixture original does not match its frozen decoded hash');
  const result = refreshAntlrReverseFixtureLexers(original);
  const file = new URL(physicalPath, root);
  if (check) {
    if (!isDeepStrictEqual(readProjectData(file), result)) throw new Error('ANTLR reverse fixture lexical declarations are stale');
  } else writeProjectData(file, result);
  return result;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  generateAntlrReverseFixture({ check: process.argv.includes('--check') });
  console.log('ANTLR reverse fixture retains its authenticated original payload and generated lexical declarations');
}
