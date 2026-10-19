import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../src/structured-data-links.js';

const bytes=readFileSync(new URL('../../../parity/naming/ci-node-runtime-source-history.lino',import.meta.url));
assert.equal(createHash('sha256').update(bytes).digest('hex'),'ca3566e0989eb61227ce2785c6f8a77bce9949bcb944847fabd2815b4a58279a');
const history=readStructuredDataLino(bytes.toString('utf8'));

export function restoreCiNodeRuntimeHistory(source) {
  assert.equal(typeof source,'string');
  source=restorePlatformTestDeadlineHistory(source);
  const owners=history.owners.filter(owner=>source.startsWith(owner.header+'\n'));assert.equal(owners.length,1,'one original workflow header is required');
  assert.equal([...source.matchAll(/^\s*node-version: 24\.22\.0$/gmu)].length,owners[0].count,'every original major-24 setup has the exact supported patch');
  assert.equal(/^\s*node-version: 24$/mu.test(source),false,'floating selections cannot coexist with exact runtime pins');
  return source.replace(/^(\s*node-version: )24\.22\.0$/gmu,(_match,prefix)=>prefix+'24');
}

import {restorePlatformTestDeadlineHistory} from './platform-test-deadline-history.js';
