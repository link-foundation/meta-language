import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {restoreConformancePartitionHistory} from './support/conformance-partition-history.js';

const root=new URL('../../',import.meta.url);
const fixture=readProjectData(new URL('parity/fixtures/javascript-conformance-deadline-2026-10-20.lino',root));

test('historical conformance selection restores every preceding workflow byte under the original deadline',()=>{
  const source=readFileSync(new URL('.github/workflows/js.yml',root),'utf8');
  assert.equal(createHash('sha256').update(source).digest('hex'),fixture.change.currentSha256);
  const previous=restoreConformancePartitionHistory(source);
  assert.equal(previous,fixture.workflow.text);
  assert.equal(restoreConformancePartitionHistory(previous),previous);
  assert.equal(Buffer.byteLength(previous),fixture.workflow.bytes);
  assert.equal(createHash('sha256').update(previous).digest('hex'),fixture.workflow.sha256);
  assert.ok(Buffer.from(previous).equals(execFileSync('git',['show',fixture.head+':'+fixture.workflow.path],{cwd:root,maxBuffer:128*1024})));
});

test('historical conformance views refuse missing duplicate mixed and unsupported matrices and retain unrelated edits',()=>{
  const source=readFileSync(new URL('.github/workflows/js.yml',root),'utf8'),{before,after}=fixture.change;
  for(const replacement of ['',after+after,before+after,after.replace('shards: 4','shards: 5'),after.replace('partition: 3','partition: 2')]){
    assert.notEqual(replacement,after);
    assert.throws(()=>restoreConformancePartitionHistory(source.replace(after,replacement)),assert.AssertionError);
  }
  for(const [before,after]of [['timeout-minutes:','changed-timeout-minutes:'],['fail-fast: false','fail-fast: true']]){
    const changed=source.replace(before,after);assert.notEqual(changed,source);
    assert.equal(restoreConformancePartitionHistory(changed),fixture.workflow.text.replace(before,after));
  }
  assert.equal(restoreConformancePartitionHistory(source+'\n# unrelated\n'),fixture.workflow.text+'\n# unrelated\n');
  assert.equal(restoreConformancePartitionHistory('name: Other\n'),'name: Other\n');
  assert.throws(()=>restoreConformancePartitionHistory(null),assert.AssertionError);
});

test('conformance adapters retain both complete original guard modules and every preceding assertion',()=>{
  const revision='9ac3038132953e6a220bba6f06a3e6147d39836f';
  for(const [file,addition,marker,call]of [
    ['js/tests/support/platform-test-deadline-history.js',"\nimport {restoreConformancePartitionHistory} from './conformance-partition-history.js';\n","  assert.equal(typeof source,'string');",'  source=restoreConformancePartitionHistory(source);'],
    ['js/tests/issue-195-evidence-translation-feedback.test.js',"\nimport {restoreConformancePartitionHistory} from './support/conformance-partition-history.js';\n",'function assertInitialTranslationShards(text) {','  text=restoreConformancePartitionHistory(text);'],
  ]){
    const current=readFileSync(new URL(file,root),'utf8');assert.ok(current.endsWith(addition));
    const changed=marker+'\n'+call;assert.equal(current.split(changed).length,2);
    const previous=current.slice(0,-addition.length).replace(changed,marker);
    assert.ok(Buffer.from(previous).equals(execFileSync('git',['show',revision+':'+file],{cwd:root,maxBuffer:128*1024})));
  }
});
