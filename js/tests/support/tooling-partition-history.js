import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../scripts/project-data.mjs';

const file=new URL('../../../parity/fixtures/tooling-partition-history.lino',import.meta.url);
assert.equal(createHash('sha256').update(readFileSync(file)).digest('hex'),'38f762c95779e155373632f4951f7bc24fc30fa7091e7e12e2b8c2bf75ed2164');
const policy=readProjectData(file);

export function restoreToolingPartitionHistory(source) {
  assert.equal(typeof source,'string');
  if(!source.startsWith('name: JavaScript\n'))return source;
  const {before,after}=policy.change;
  if(source.includes(after)){
    assert.equal(source.split(after).length,2,'one complete current tooling matrix is required');
    assert.equal([...source.matchAll(/\{group: tooling,/gu)].length,policy.shards,'every tooling partition has one explicit owner');
    return source.replace(after,()=>before);
  }
  assert.equal(source.split(before).length,2,'one complete preceding matrix owner is required');
  assert.equal([...source.matchAll(/\{group: tooling,/gu)].length,0,'partial or unsupported tooling partition matrices refuse');
  return source;
}
