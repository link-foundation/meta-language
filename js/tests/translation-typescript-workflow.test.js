import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync,mkdtempSync,mkdirSync,writeFileSync,realpathSync,rmSync,existsSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {partitionTests,partitionGroupFiles,testFiles} from '../scripts/test-groups.mjs';
import {generateTranslationWorkflowHistory} from '../scripts/generate-translation-workflow-history.mjs';
import {restoreTranslationWorkflowHistory} from './support/translation-workflow-history.js';
import {captureTranslationTestFeedbackPartition,mergeTranslationTestFeedbackPartitions} from '../scripts/translation-test-feedback.mjs';
import {readProjectData,writeProjectData} from '../scripts/project-data.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));
const read=file=>readFileSync(new URL('../../'+file,import.meta.url),'utf8');

test('six bounded translation partitions retain every previous file once and preserve all original workflow bytes',()=>{
  const history=generateTranslationWorkflowHistory({check:true});
  assert.equal(history.workflows.length,3);
  for(const row of history.workflows){
    const current=read(row.file);
    assert.equal(restoreTranslationWorkflowHistory(current),row.sourceText);
    assert.equal(restoreTranslationWorkflowHistory(current+'\n# changed\n'),current+'\n# changed\n');
  }
  const workflow=read('.github/workflows/js.yml');
  const tests=parseWorkflow(workflow).find(job=>job.id==='tests');
  assert.match(tests.header,/timeout-minutes: 15/u);assert.match(tests.header,/fail-fast: false/u);
  const entries=[...tests.header.matchAll(/\{group: translation, partition: ([0-9]+), shards: ([0-9]+)\}/gu)]
    .map(([,index,count])=>[Number(index),Number(count)]);
  assert.deepEqual(entries,[[1,6],[2,6],[3,6],[4,6],[5,6],[6,6]]);
  assert.ok(workflow.includes('--partitions "${{ matrix.shards }}"'));
  assert.ok(workflow.includes('pattern: translation-test-feedback-${{ github.sha }}-*-of-6'));
  assert.match(workflow,/merge \\\n\s+--partitions 6/u);
  const files=partitionTests(testFiles()).groups.translation;
  const assigned=entries.flatMap(([index,count])=>partitionGroupFiles(files,index+'/'+count));
  assert.equal(new Set(assigned).size,assigned.length);assert.deepEqual(assigned.toSorted(),files.toSorted());
  const originalFiles=execFileSync('git',['ls-tree','-r','--name-only',history.revision,'js/tests'],
    {cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}).split('\n')
    .filter(file=>/^js\/tests\/[^/]+\.test\.js$/u.test(file)).map(file=>file.slice('js/tests/'.length));
  for(const file of partitionTests(originalFiles).groups.translation)assert.ok(files.includes(file),file);
});

test('six feedback partitions merge exact scope and bytes while missing foreign and tampered cohorts refuse',(t)=>{
  const root=realpathSync(mkdtempSync(join(tmpdir(),'meta-language-six-feedback-')));
  t.after(()=>rmSync(root,{recursive:true}));
  mkdirSync(join(root,'js/tests'),{recursive:true});
  for(let index=0;index<6;index++)writeFileSync(join(root,'js/tests/translation-'+index+'.test.js'),'');
  const payload='issue-195-artifacts/compiler-feedback/source.lino';
  mkdirSync(join(root,'issue-195-artifacts/compiler-feedback'),{recursive:true});
  writeFileSync(join(root,payload),'(retained compiler bytes)\n');
  const commit='a'.repeat(40),inputDirectory=join(root,'parts'),directories=[];
  const capture=index=>captureTranslationTestFeedbackPartition({root,commit,partitions:6,
    shard:(index+1)+'/6',outputDirectory:join(inputDirectory,'translation-test-feedback-'+commit+'-'+(index+1)+'-of-6')});
  for(let index=0;index<6;index++){
    const receipt=capture(index);assert.equal(receipt.files.length,1);
    directories.push(join(inputDirectory,'translation-test-feedback-'+commit+'-'+(index+1)+'-of-6'));
  }
  const arguments_={root,commit,inputDirectory,partitions:6,outputDirectory:join(root,'merged')};
  const merged=mergeTranslationTestFeedbackPartitions(arguments_);
  assert.equal(merged.partitions,6);assert.deepEqual(merged.files,[payload]);
  assert.equal(readFileSync(join(root,'merged',payload),'utf8'),'(retained compiler bytes)\n');
  const rejected={...arguments_,outputDirectory:join(root,'rejected')};
  assert.throws(()=>mergeTranslationTestFeedbackPartitions({...rejected,partitions:3}),/exactly once/u);
  rmSync(directories[3],{recursive:true});
  assert.throws(()=>mergeTranslationTestFeedbackPartitions(rejected),/exactly once/u);
  capture(3);
  const receiptFile=join(directories[2],'translation-test-feedback-partition.lino');
  const receipt=readProjectData(receiptFile);receipt.shard='3/3';writeProjectData(receiptFile,receipt);
  assert.throws(()=>mergeTranslationTestFeedbackPartitions(rejected),/provenance/u);
  assert.equal(existsSync(rejected.outputDirectory),false);
});

test('TypeScript native execution follows JavaScript and every Rust partition retains its actual observations',()=>{
  const ci=parseWorkflow(read('.github/workflows/ci.yml'));
  const native=ci.find(job=>job.id==='native-rust'),javascript=ci.find(job=>job.id==='native-javascript');
  assert.match(native.header,/needs: \[runtime-parity, native-javascript\]/u);
  assert.match(native.header,/needs\.native-javascript\.result == 'success'/u);
  assert.match(javascript.header,/needs: \[js, runtime-parity\]/u);
  const step=native.steps.find(value=>value.name==='Validate TypeScript graph Rust execution');
  assert.ok(step.afterFailure);assert.match(step.run,/run-native\.mjs --results-dir issue-195-results/u);
  assert.ok(native.steps.indexOf(step)>native.steps.findIndex(value=>value.name==='Validate the emitted translations with the native toolchain'));
  assert.ok(native.steps.indexOf(step)<native.steps.findIndex(value=>value.name==='Upload the stage outputs'));
  const retain=parseWorkflow(read('.github/workflows/rust.yml')).flatMap(job=>job.steps)
    .filter(value=>value.name==='Retain TypeScript graph native observations');
  assert.equal(retain.length,3);
  for(const upload of retain){
    assert.ok(upload.afterFailure);
    assert.match(upload.text,/path: issue-195-artifacts\/typescript-typed-graph-native\//u);
    assert.match(upload.text,/if-no-files-found: error/u);
  }
});
