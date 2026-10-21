import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {captureGitArtifactCertificate} from '../scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {restoreRawPlanningWorkflowHistory} from '../scripts/raw-planning-workflow-history.mjs';
import {restoreGrammarTestPartitionHistory} from '../scripts/grammar-test-partition-history.mjs';

test('raw generation and complete replay retain every prior command and assertion in distinct required ten-minute jobs',()=>{
  const revision='76e27c44c5854a2cb547aea5cef06ea80adf729c',path='.github/workflows/js.yml';
  const original=captureGitArtifactCertificate(revision,path);
  verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path});
  const before=original.blob.toString('utf8'),actual=restoreGrammarTestPartitionHistory(readFileSync(new URL('../../'+path,import.meta.url),'utf8'),path);
  const matrix='          - group: source-model-raw\n            generated_slice: first\n';
  assert.equal(before.split(matrix).length,2);
  let expected=before.replace(matrix,matrix+'          - group: source-model-raw-generation\n            generated_slice: first\n');
  const names=['Check complete original link raw projection native generation','Run complete original link raw projection checks'];
  for(const name of names){
    const start=expected.indexOf('      - name: '+name+'\n');assert.ok(start>=0);
    const end=expected.indexOf('\n      - name:',start+1);assert.ok(end>start);
    const step=expected.slice(start,end);assert.equal(step.split("matrix.group == 'source-model-raw'").length,2);
    expected=expected.slice(0,start)+step.replace("matrix.group == 'source-model-raw'","matrix.group == 'source-model-raw-generation'")+expected.slice(end);
  }
  assert.equal(actual,expected,'only one required matrix cell and two complete scheduling predicates change');
  const old=parseWorkflow(before),current=parseWorkflow(actual);
  assert.equal(old.length,current.length);
  const job=current.find(row=>row.id==='package-checks'),previous=old.find(row=>row.id===job.id);
  assert.match(job.header,/^    timeout-minutes: 10$/mu);
  assert.match(job.header,/^      fail-fast: false$/mu);
  assert.equal(job.steps.length,previous.steps.length);
  assert.deepEqual(job.steps.map(row=>row.name),previous.steps.map(row=>row.name));
  const retained=job.steps.find(row=>row.name==='Run complete original link raw native oracle contracts');
  assert.equal(retained.text,previous.steps.find(row=>row.name===retained.name).text);
  for(const other of old.filter(row=>row.id!==job.id))assert.deepEqual(current.find(row=>row.id===other.id),other);
  assert.match(current.find(row=>row.id==='test').header,/package-checks/u);
});

test('historical workflow views authenticate the entire new schedule and refuse unknown current authentication',()=>{
  const path='.github/workflows/js.yml',source=readFileSync(new URL('../../'+path,import.meta.url),'utf8');
  const old=captureGitArtifactCertificate('76e27c44c5854a2cb547aea5cef06ea80adf729c',path).blob.toString('utf8');
  assert.equal(restoreRawPlanningWorkflowHistory(source,path),old);
  for(const legacyHint of ['.github/workflows/ci.yml','.github/workflows/rust.yml']){
    assert.equal(restoreRawPlanningWorkflowHistory(source,legacyHint),old,
      'legacy cross-workflow hints still require the exact complete JavaScript digest and certificate');
  }
  assert.equal(restoreRawPlanningWorkflowHistory(source,'unknown.js'),source);
  for(const changed of [source+'\n# unknown source\n',source.replace('timeout-minutes: 10','timeout-minutes: 11'),
    source.replace('source-model-raw-generation','unknown-generation')]){
    assert.notEqual(changed,source);
    assert.equal(restoreRawPlanningWorkflowHistory(changed,path),changed);
  }
});
