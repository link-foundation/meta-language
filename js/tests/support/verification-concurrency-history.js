// Legacy conservation checks inspect an authenticated inverse of only the
// approved queue correction. Actual current groups must match first, including
// every matrix coordinate and the unchanged release-write cancellation policy.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readStructuredDataLino } from '../../src/structured-data-links.js';

const bytes = readFileSync(new URL('../../../parity/fixtures/verification-concurrency-history.lino', import.meta.url));
assert.equal(createHash('sha256').update(bytes).digest('hex'),
  '4a6d40babe12f205ee90c2295fd189d55e81e39781a2c1df4c6e38789503a261');
const history = readStructuredDataLino(bytes.toString('utf8'));
for (const file of history.files) {
  for (const capture of [file.before, file.after]) {
    assert.equal(Buffer.byteLength(capture.source), capture.bytes);
    assert.equal(createHash('sha256').update(capture.source).digest('hex'), capture.sha256);
  }
  let original = file.after.source;
  for (const change of file.changes) {
    assert.equal(original.split(change.after).length - 1, change.occurrences);
    original = original.replaceAll(change.after, change.before);
  }
  assert.equal(original, file.before.source, 'complete original workflow source inverse');
}

export function restoreVerificationConcurrencyHistory(source, name) {
  assert.equal(typeof source, 'string');
  source=restoreEvidenceTranslationSourceHistory('.github/workflows/ci.yml',source);
  const file = history.files.find(file => file.name === name);
  assert.ok(file, 'only the three authenticated verification workflows have a historical inverse');
  const groups = [...source.matchAll(/^(\s*group: .+)\n(\s*cancel-in-progress: (?:true|false))$/gmu)].map(([source]) => source);
  assert.deepEqual(groups.sort(), [...file.groups].sort(), 'current queue identities, matrix dimensions and cancellation policies must match before historical comparison');
  let original = source;
  for (const change of file.changes) {
    assert.equal(original.split(change.after).length - 1, change.occurrences, 'every approved queue or comment correction must be present exactly');
    original = original.replaceAll(change.after, change.before);
  }
  return original;
}

import {restoreEvidenceTranslationSourceHistory} from '../../scripts/evidence-translation-source-history.mjs';
