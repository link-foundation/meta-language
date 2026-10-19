import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../src/structured-data-links.js';

const bytes=readFileSync(new URL('../../../parity/naming/windows-linker-timeout-source-history.lino',import.meta.url));
assert.equal(createHash('sha256').update(bytes).digest('hex'),'3b729f41b57f4b82945c8e022d10f172e15e9c370fabbcb4a5da08c2b69ed51e');
const history=readStructuredDataLino(bytes.toString('utf8'));

export function restoreWindowsLinkerLookupHistory(source) {
  assert.equal(typeof source,'string');
  assert.equal(source.split(history.after).length,2,'one complete bounded Windows lookup retry block is required');
  assert.equal(source.includes(history.before),false,'the preceding lookup cannot coexist with its successor');
  return source.replace(history.after,history.before);
}
