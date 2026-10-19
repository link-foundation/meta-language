// Physical scheduling changes have a separate exact inverse. Every command,
// directory, failure predicate and global step position remains authenticated.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readStructuredDataLino } from '../../src/structured-data-links.js';

const bytes = readFileSync(new URL('../../../parity/fixtures/generated-package-owner-history.lino', import.meta.url));
assert.equal(createHash('sha256').update(bytes).digest('hex'),
  'dfe02b295de4f7d10c717ceb14754c303d99f1137f105073b430c66241385a90');
const history = readStructuredDataLino(bytes.toString('utf8'));
assert.equal(history.changes.length, 3);
for (const value of [history.before, history.after]) {
  assert.equal(Buffer.byteLength(value.source), value.bytes);
  assert.equal(createHash('sha256').update(value.source).digest('hex'), value.sha256);
}

export function restoreGeneratedPackageOwnerHistory(source) {
  assert.equal(typeof source, 'string');
  let original = source.includes('node-version: 24.22.0') ? restoreCiNodeRuntimeHistory(source) : source;
  for (const change of history.changes) {
    assert.equal(original.split(change.after).length, 2, 'each current complete check has exactly one admitted physical owner');
    assert.equal(original.includes(change.before), false, 'the obsolete complete owner cannot coexist with its replacement');
    assert.equal(change.after.replace("matrix.generated_slice == 'first'", "matrix.generated_slice == 'second'"), change.before,
      'only the exact physical selector changes; complete commands and all other fields remain');
    original = original.replace(change.after, change.before);
  }
  return original;
}

import {restoreCiNodeRuntimeHistory} from './ci-node-runtime-history.js';
