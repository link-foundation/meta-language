import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readStructuredDataLino } from '../../src/structured-data-links.js';

const bytes = readFileSync(new URL('../../../parity/fixtures/aggregate-failure-history.lino', import.meta.url));
assert.equal(createHash('sha256').update(bytes).digest('hex'),
  'd51d9406957f56b0ee5cf17e21bffec144143c50c57a817a34f56e606b7e158e');
const history = readStructuredDataLino(bytes.toString('utf8'));
for (const value of [history.before, history.after]) {
  assert.equal(Buffer.byteLength(value.source), value.bytes);
  assert.equal(createHash('sha256').update(value.source).digest('hex'), value.sha256);
}

export function restoreAggregateFailureReportHistory(source) {
  assert.equal(typeof source, 'string');
  assert.equal(source.split(history.stepAfter).length, 2, 'the complete current failure guard must occur exactly once');
  assert.equal(source.includes(history.stepBefore), false, 'the obsolete guard cannot coexist with its replacement');
  return source.replace(history.stepAfter, history.stepBefore);
}
