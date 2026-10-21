import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {generateGrammarTestPartitionHistory} from '../scripts/generate-grammar-test-partition-history.mjs';
import {restoreGrammarTestPartitionHistory} from '../scripts/grammar-test-partition-history.mjs';
import {partitionTests,partitionGroupFiles,testFiles} from '../scripts/test-groups.mjs';
import {restoreRawOracleContractPartitionHistory} from '../scripts/raw-oracle-contract-partition-history.mjs';

test('six required grammar cells preserve every file exactly once and every complete previous command and deadline',()=>{
  const history=generateGrammarTestPartitionHistory({check:true});
  const path='.github/workflows/js.yml',source=restoreRawOracleContractPartitionHistory(readFileSync(new URL('../../'+path,import.meta.url),'utf8'),path);
  assert.equal(restoreGrammarTestPartitionHistory(source,path),history.source);
  const before=parseWorkflow(history.source),after=parseWorkflow(source);
  assert.equal(after.length,before.length);
  for(const original of before){
    const current=after.find(row=>row.id===original.id);assert.ok(current);
    assert.deepEqual(current.steps,original.steps);
    assert.equal(current.header,original.id==='tests'
      ?original.header.replace(history.edit.removed,history.edit.inserted):original.header);
  }
  const header=after.find(row=>row.id==='tests').header;
  assert.match(header,/^    timeout-minutes: 15$/mu);assert.match(header,/fail-fast: false/u);
  assert.deepEqual([...header.matchAll(/\{group: grammar, partition: (\d+), shards: (\d+)\}/gu)]
    .map(([,index,count])=>[Number(index),Number(count)]),[[1,6],[2,6],[3,6],[4,6],[5,6],[6,6]]);
  const partition=partitionTests(testFiles());assert.deepEqual(partition.unassigned,[]);assert.deepEqual(partition.ambiguous,[]);
  const files=partition.groups.grammar;
  const previous=Array.from({length:3},(_,index)=>partitionGroupFiles(files,`${index+1}/3`));
  const current=Array.from({length:6},(_,index)=>partitionGroupFiles(files,`${index+1}/6`));
  assert.ok(current.every(cell=>cell.length>0));
  assert.deepEqual(current.flat().sort(),previous.flat().sort());assert.deepEqual(current.flat().sort(),files);
  assert.equal(new Set(current.flat()).size,files.length);
});

test('unknown commands deadlines duplicate cells and partial matrices never acquire historical scheduling authentication',()=>{
  const path='.github/workflows/js.yml',source=readFileSync(new URL('../../'+path,import.meta.url),'utf8');
  for(const changed of [source+'\n# unknown\n',source.replace('timeout-minutes: 15','timeout-minutes: 16'),
    source.replace('{group: grammar, partition: 6, shards: 6}','{group: grammar, partition: 5, shards: 6}'),
    source.replace('          - {group: grammar, partition: 6, shards: 6}\n','')]){
    assert.notEqual(changed,source);assert.equal(restoreGrammarTestPartitionHistory(changed,path),changed);
  }
  assert.equal(restoreGrammarTestPartitionHistory(source,'unrelated.js'),source);
  assert.throws(()=>restoreGrammarTestPartitionHistory(null,path));
});
