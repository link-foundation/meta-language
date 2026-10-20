import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../scripts/project-data.mjs';

const file=new URL('../../../parity/fixtures/raw-contract-partition-history.lino',import.meta.url);
assert.equal(createHash('sha256').update(readFileSync(file)).digest('hex'),'0662928a7fd00a655e8a25ecd839a89c1d488ea30d45d0780ffcdb85e1f3f588');
const history=readProjectData(file);

/** Invert only complete recorded scheduling changes; unrelated edits stay visible. */
export function restoreRawContractPartitionHistory(source) {
  assert.equal(typeof source,'string');
  if(!source.startsWith('name: JavaScript\n'))return source;
  for(const change of history.changes){
    if(source.includes(change.after)){
      assert.equal(source.split(change.after).length,2,'one complete current raw-contract owner');
      source=source.replace(change.after,()=>change.before);
    }else assert.equal(source.split(change.before).length,2,'one complete original raw-contract owner');
  }
  assert.equal(source.includes('ORIGINAL_LINK_RAW_SEPARATE_SUITES'),false,'partial or duplicate suite selection is unsupported');
  for(const suite of history.suites)assert.equal(source.includes('group: '+suite.group),false,'unrecorded matrix duplicates are unsupported');
  return source;
}

export function restoreRawContractTestSource(source) {
  const {before,after}=history.testChange;
  const current=history.source.source.replace(before,()=>after);
  assert.ok(source===current||source===history.source.source,'only complete reviewed raw contract source is admitted');
  return history.source.source;
}
