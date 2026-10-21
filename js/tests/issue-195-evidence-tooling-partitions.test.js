import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateToolingPartitionHistory} from '../scripts/generate-tooling-partition-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {partitionTests,partitionGroupFiles,testFiles} from '../scripts/test-groups.mjs';
import {restoreToolingPartitionHistory} from './support/tooling-partition-history.js';

const root=new URL('../../',import.meta.url);
const history=readProjectData(new URL('parity/fixtures/tooling-partition-history.lino',root));
const read=file=>restoreCiRepairSourceHistory(file,restoreSchedulingSourceHistory(file,readFileSync(new URL(file,root),'utf8')));

test('four required tooling cells retain every current file exactly once and every original command and deadline',()=>{
  generateToolingPartitionHistory({check:true});
  const source=read(history.workflow.path);assert.equal(source.split(history.change.after).length,2);
  const previous=restoreToolingPartitionHistory(source);assert.equal(previous,history.workflow.source);
  verifyGitArtifactCertificate(history.workflow.certificate,Buffer.from(previous),{revision:history.revision,path:history.workflow.path});
  const before=parseWorkflow(previous),after=parseWorkflow(source);
  assert.equal(after.length,before.length);
  for(const old of before){
    const current=after.find(row=>row.id===old.id);assert.ok(current);assert.deepEqual(current.steps,old.steps);
    assert.equal(current.header,old.id==='tests'?old.header.replace(history.change.before,history.change.after):old.header);
  }
  const header=after.find(row=>row.id==='tests').header;
  assert.match(header,/^ {4}timeout-minutes: 15$/mu);assert.match(header,/fail-fast: false/u);
  assert.deepEqual([...header.matchAll(/\{group: tooling, partition: (\d+), shards: (\d+)\}/gu)]
    .map(([,part,count])=>[Number(part),Number(count)]),[[1,4],[2,4],[3,4],[4,4]]);
  const partition=partitionTests(testFiles());assert.deepEqual(partition.unassigned,[]);assert.deepEqual(partition.ambiguous,[]);
  const files=partition.groups.tooling;
  const cells=Array.from({length:history.shards},(_,index)=>partitionGroupFiles(files,`${index+1}/${history.shards}`));
  assert.ok(cells.every(cell=>cell.length>0));assert.equal(cells.flat().length,files.length);
  assert.equal(new Set(cells.flat()).size,files.length);assert.deepEqual(cells.flat().sort(),files);
  assert.ok(cells.flat().includes('issue-195-evidence-tooling-partitions.test.js'));
  assert.ok(cells.flat().includes('issue-195-evidence-link-foundation.test.js'));
});

test('partial duplicate mixed and unsupported tooling cells refuse without hiding unrelated gate edits',()=>{
  const source=read(history.workflow.path),{before,after}=history.change;
  for(const replacement of [after+after,after.replace('partition: 3','partition: 2'),
    after.replace('shards: 4','shards: 5'),after.replace('          - {group: tooling, partition: 4, shards: 4}\n','')]){
    assert.throws(()=>restoreToolingPartitionHistory(source.replace(after,replacement)),assert.AssertionError);
  }
  assert.equal(restoreToolingPartitionHistory(source.replace(after,before)),history.workflow.source);
  for(const [old,next]of [['timeout-minutes: 15','timeout-minutes: 16'],['fail-fast: false','fail-fast: true']]){
    const changed=source.replace(old,next);assert.notEqual(changed,source);
    assert.equal(restoreToolingPartitionHistory(changed),history.workflow.source.replace(old,next));
  }
  assert.equal(restoreToolingPartitionHistory(source+'\n# unrelated\n'),history.workflow.source+'\n# unrelated\n');
  assert.equal(restoreToolingPartitionHistory('name: Other\n'),'name: Other\n');
  assert.throws(()=>restoreToolingPartitionHistory(null),assert.AssertionError);
});

test('tooling partition adapters reconstruct both complete preceding raw guards and every retained assertion',()=>{
  const previous=file=>execFileSync('git',['show',history.revision+':'+file],{cwd:root,encoding:'utf8',maxBuffer:128*1024});
  for(const [file,before,after,insertion]of [
    ['js/tests/support/raw-contract-partition-history.js',"  assert.equal(typeof source,'string');",
      "  assert.equal(typeof source,'string');\n  source=restoreToolingPartitionHistory(source);",
      "\nimport {restoreToolingPartitionHistory} from './tooling-partition-history.js';\n"],
    ['js/tests/issue-195-evidence-raw-contract-partitions.test.js',"  const workflow=read('.github/workflows/js.yml');",
      "  const workflow=restoreToolingPartitionHistory(read('.github/workflows/js.yml'));",
      "\nimport {restoreToolingPartitionHistory} from './support/tooling-partition-history.js';\n"],
    ['js/tests/test-groups.test.js',"  const workflow = await readFile(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');",
      "  const workflow = restoreRawContractPartitionHistory(await readFile(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8'));",
      "\nimport {restoreRawContractPartitionHistory} from './support/raw-contract-partition-history.js';\n"],
  ]){
    const source=read(file);assert.equal(source.split(after).length,2);assert.equal(source.split(insertion).length,2);
    assert.equal(source.replace(insertion,'').replace(after,before),previous(file));
  }
});

import {restoreCiRepairSourceHistory} from '../scripts/ci-repair-source-history.mjs';

import {restoreSchedulingSourceHistory} from '../scripts/scheduling-source-history.mjs';
