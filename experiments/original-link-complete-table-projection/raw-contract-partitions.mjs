// Both required CI cells execute the original callbacks, with no skipped tests.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';

/** Select exact registered contracts; default execution retains the full suite. */
export function createOriginalLinkRawContractPartition(register,environment=process.env) {
  assert.equal(typeof register,'function');
  const profile=environment.ORIGINAL_LINK_RAW_CONTRACT_PARTITION;
  if(profile===undefined)return{test:register,runPlan:true,finish(){}};
  assert.ok(profile==='schema'||profile==='plan','unsupported raw contract partition');
  assert.equal(environment.CI,'true');assert.equal(environment.ORIGINAL_LINK_RAW_NATIVE,'0');
  assert.equal(environment.ORIGINAL_LINK_RAW_SEPARATE_SUITES,'1');
  const fixture=readStructuredDataLino(readFileSync(new URL('../../parity/fixtures/raw-native-contract-partitions.lino',import.meta.url),'utf8'));
  const rows=fixture.tests.filter(row=>row.partition!=='native');
  const selected=rows.filter(row=>row.partition===profile),seen=[],registered=[];
  assert.equal(new Set(rows.map(row=>row.name)).size,rows.length);
  return{
    runPlan:profile==='plan',
    test(name,...argumentsList){
      const row=rows.find(row=>row.name===name);assert.ok(row,'unregistered raw contract '+name);
      assert.equal(seen.includes(name),false,'duplicate raw contract '+name);seen.push(name);
      if(row.partition!==profile)return undefined;
      registered.push(name);return register(name,...argumentsList);
    },
    finish(){
      assert.deepEqual(registered,selected.map(row=>row.name),'every selected original callback must register exactly once');
      assert.deepEqual(seen,(profile==='schema'?selected:rows).map(row=>row.name),
        'the complete original source registration order must remain exact');
    },
  };
}
