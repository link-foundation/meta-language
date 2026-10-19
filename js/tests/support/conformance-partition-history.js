import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../scripts/project-data.mjs';

const file=new URL('../../../parity/fixtures/javascript-conformance-deadline-2026-10-20.lino',import.meta.url);
const bytes=readFileSync(file);
assert.equal(createHash('sha256').update(bytes).digest('hex'),'45c56a90b7c82c4c7a798c5fc51d1209a1328cb3c3284e2e88a70d64571f253e');
const history=readProjectData(file);
assert.equal(history.oldCount,2);assert.equal(history.newCount,4);assert.equal(history.timeoutMinutes,15);
assert.equal(createHash('sha256').update(history.workflow.text).digest('hex'),history.workflow.sha256);

/** Restore exactly the reviewed matrix block; every unrelated edit stays visible. */
export function restoreConformancePartitionHistory(source) {
  assert.equal(typeof source,'string');
  if(!source.startsWith('name: JavaScript\n'))return source;
  const {before,after}=history.change;
  if(source.includes(after)){
    assert.equal(source.split(after).length,2,'one complete current conformance matrix is required');
    assert.equal(source.includes(before),false,'current and previous conformance blocks cannot coexist');
    const restored=source.replace(after,before);
    assert.equal([...restored.matchAll(/\{group: conformance,/gu)].length,history.oldCount);
    return restored;
  }
  assert.equal(source.split(before).length,2,'one complete supported conformance matrix is required');
  assert.equal([...source.matchAll(/\{group: conformance,/gu)].length,history.oldCount);
  return source;
}
