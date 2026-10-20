import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateRawContractPartitionHistory} from '../scripts/generate-raw-contract-partition-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {restoreRawContractPartitionHistory,restoreRawContractTestSource} from './support/raw-contract-partition-history.js';

const root=new URL('../../',import.meta.url);
const read=file=>restoreCiRepairSourceHistory(file,readFileSync(new URL(file,root),'utf8'));
const history=readProjectData(new URL('parity/fixtures/raw-contract-partition-history.lino',root));

test('isolated raw contract cells retain every original command gate and deadline and restore the entire old workflow',()=>{
  generateRawContractPartitionHistory({check:true});
  const workflow=restoreToolingPartitionHistory(read('.github/workflows/js.yml'));
  assert.equal(restoreRawContractPartitionHistory(workflow),history.workflow.source);
  verifyGitArtifactCertificate(history.workflow.certificate,Buffer.from(history.workflow.source),{
    revision:history.revision,path:'.github/workflows/js.yml'});
  const before=parseWorkflow(history.workflow.source),after=parseWorkflow(workflow);
  assert.equal(after.length,before.length);
  const old=before.find(row=>row.id==='package-checks'),current=after.find(row=>row.id===old.id);
  assert.match(current.header,/^ {4}timeout-minutes: 10$/mu);
  assert.equal(current.header,old.header.replace(history.changes[0].before,history.changes[0].after));
  assert.equal(current.steps.length,old.steps.length+history.suites.length);
  const names=new Set(history.suites.map(row=>row.name));
  assert.deepEqual(current.steps.filter(row=>!names.has(row.name)).map(row=>row.name),old.steps.map(row=>row.name));
  for(const step of old.steps){
    const actual=current.steps.find(row=>row.name===step.name);assert.ok(actual);
    assert.equal(actual.text,step.name==='Run complete original link raw native oracle contracts'
      ?step.text.replace("          ORIGINAL_LINK_RAW_NATIVE: '0'","          ORIGINAL_LINK_RAW_NATIVE: '0'\n          ORIGINAL_LINK_RAW_SEPARATE_SUITES: '1'"):step.text);
  }
  const original=old.steps.find(row=>row.name==='Run complete original link raw native oracle contracts');
  for(const suite of history.suites){
    assert.equal(current.steps.filter(row=>row.name===suite.name).length,1);
    assert.equal(current.steps.find(row=>row.name===suite.name).text,original.text.replace(original.name,suite.name)
      .replace("matrix.group == 'source-model-raw'","matrix.group == '"+suite.group+"'")
      .replace('raw-native-observations.test.mjs',suite.file));
    assert.equal(current.header.split('group: '+suite.group+'\n').length,2);
  }
  for(const job of before.filter(row=>row.id!==old.id))assert.deepEqual(after.find(row=>row.id===job.id),job);
});

test('partial repeated and corrupted raw contract partitions refuse while unrelated workflow edits remain visible',()=>{
  const workflow=read('.github/workflows/js.yml');
  for(const change of history.changes)for(const replacement of ['',change.after+change.after,
    change.after.replace('raw-consuming-emission','unknown-consuming-emission')]){
    assert.throws(()=>restoreRawContractPartitionHistory(workflow.replace(change.after,replacement)),assert.AssertionError);
  }
  const altered=workflow.replace('timeout-minutes: 10','timeout-minutes: 11');
  assert.notEqual(altered,workflow);
  assert.equal(restoreRawContractPartitionHistory(altered),history.workflow.source.replace('timeout-minutes: 10','timeout-minutes: 11'));
  assert.equal(restoreRawContractPartitionHistory(workflow+'\n# unrelated edit\n'),history.workflow.source+'\n# unrelated edit\n');
  assert.throws(()=>restoreRawContractPartitionHistory(null),assert.AssertionError);
});

test('raw suite selection preserves all original assertions and permits isolation only in the explicit JavaScript CI mode',async()=>{
  const current=read('experiments/original-link-complete-table-projection/raw-native-observations.test.mjs');
  assert.equal(restoreRawContractTestSource(current),history.source.source);
  verifyGitArtifactCertificate(history.source.certificate,Buffer.from(history.source.source),{
    revision:history.revision,path:'experiments/original-link-complete-table-projection/raw-native-observations.test.mjs'});
  for(const altered of [current+' ',current.replace('211','210'),current.replace("CI,'true'","CI,'false'")]){
    assert.notEqual(altered,current);assert.throws(()=>restoreRawContractTestSource(altered));
  }
  // Replace only the module-loading boundary with a recorder; execute the
  // complete actual selection control without importing any heavy contracts.
  const AsyncFunction=Object.getPrototypeOf(async function(){}).constructor;
  const choose=new AsyncFunction('process','assert','load',history.testChange.after.replaceAll('await import(','await load('));
  for(const env of [{},{CI:'true',ORIGINAL_LINK_RAW_NATIVE:'1'},{CI:'true',ORIGINAL_LINK_RAW_NATIVE:'0'}]){
    const calls=[];await choose({env},assert,async path=>calls.push(path));
    assert.deepEqual(calls,history.suites.map(row=>'./'+row.file));
  }
  const calls=[];
  await choose({env:{CI:'true',ORIGINAL_LINK_RAW_NATIVE:'0',ORIGINAL_LINK_RAW_SEPARATE_SUITES:'1'}},assert,async path=>calls.push(path));
  assert.deepEqual(calls,[]);
  for(const env of [{ORIGINAL_LINK_RAW_SEPARATE_SUITES:'1'},
    {CI:'true',ORIGINAL_LINK_RAW_NATIVE:'1',ORIGINAL_LINK_RAW_SEPARATE_SUITES:'1'},
    {CI:'true',ORIGINAL_LINK_RAW_NATIVE:'0',ORIGINAL_LINK_RAW_SEPARATE_SUITES:'0'}]){
    await assert.rejects(choose({env},assert,async()=>{throw new Error('a refused selection cannot load a suite');}),assert.AssertionError);
  }
});

test('raw partition adapters retain both entire preceding conformance guards without changing their assertions',()=>{
  const previous=file=>execFileSync('git',['show',history.revision+':'+file],{cwd:root,encoding:'utf8',maxBuffer:128*1024});
  for(const [file,before,after,insertion]of [
    ['js/tests/support/conformance-partition-history.js',"  assert.equal(typeof source,'string');",
      "  assert.equal(typeof source,'string');\n  source=restoreRawContractPartitionHistory(source);",
      "\nimport {restoreRawContractPartitionHistory} from './raw-contract-partition-history.js';\n"],
    ['js/tests/issue-195-evidence-conformance-history.test.js',"  const source=readFileSync(new URL('.github/workflows/js.yml',root),'utf8');",
      "  const source=restoreRawContractPartitionHistory(readFileSync(new URL('.github/workflows/js.yml',root),'utf8'));",
      "\nimport {restoreRawContractPartitionHistory} from './support/raw-contract-partition-history.js';\n"],
  ]){
    const source=read(file);assert.equal(source.split(after).length,2);assert.equal(source.split(insertion).length,2);
    assert.equal(source.replace(insertion,'').replace(after,before),previous(file));
  }
});

import {restoreToolingPartitionHistory} from './support/tooling-partition-history.js';

import {restoreCiRepairSourceHistory} from '../scripts/ci-repair-source-history.mjs';
