import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../scripts/project-data.mjs';

const bytes=readFileSync(new URL('../../../parity/naming/platform-test-deadline-source-history.lino',import.meta.url));
assert.equal(createHash('sha256').update(bytes).digest('hex'),'f460f5d583e1a005cce8abff041bdf935f2b172bfcf8e7102af98c065dd4e1e5');
const history=readProjectData(new URL('../../../parity/naming/platform-test-deadline-source-history.lino',import.meta.url));

export function restorePlatformTestDeadlineHistory(source) {
  assert.equal(typeof source,'string');
  source=restoreEvidenceTranslationSourceHistory('.github/workflows/ci.yml',source);
  source=restoreConformancePartitionHistory(source);
  for(const change of history.changes){
    if(!source.includes(change.marker)&&!source.includes(change.after))continue;
    assert.equal(source.split(change.after).length,2,'one complete current bounded deadline block is required');
    assert.equal(source.includes(change.before),false,'current and previous deadline blocks cannot coexist');
    source=source.replace(change.after,change.before);
  }
  return source;
}

import {restoreConformancePartitionHistory} from './conformance-partition-history.js';

import {restoreEvidenceTranslationSourceHistory} from '../../scripts/evidence-translation-source-history.mjs';
