// Historical guards retain their original complete test command. Actual current
// compiler flags have an independent raw-workflow and shell-argument control.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../src/structured-data-links.js';

const bytes=readFileSync(new URL('../../../parity/naming/native-test-profile-source-history.lino',import.meta.url));
assert.equal(createHash('sha256').update(bytes).digest('hex'),'ead12b38d0e6df99d578166256a74050589a5f160520a81fd6f991542355e0f4');
const history=readStructuredDataLino(bytes.toString('utf8'));

export function restoreNativeTestProfileHistory(source) {
  assert.equal(typeof source,'string');
  source=restorePlatformTestDeadlineHistory(source);
  assert.equal(source.split(history.after).length,2,'the complete checked package profile occurs exactly once');
  assert.equal(source.includes(history.before),false,'the preceding command cannot coexist with its corrected profile');
  return source.replace(history.after,history.before);
}

import {restorePlatformTestDeadlineHistory} from './platform-test-deadline-history.js';
