import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {restorePlatformTestDeadlineHistory} from './support/platform-test-deadline-history.js';

const history=readProjectData(new URL('../../parity/naming/platform-test-deadline-source-history.lino',import.meta.url));
const read=file=>restoreCiRepairSourceHistory(file,readFileSync(new URL('../../'+file,import.meta.url),'utf8'));

test('actual cold platform deadlines retain every complete test cell and checked compiler argument',()=>{
  for(const row of history.sources){
    assert.equal(Buffer.byteLength(row.sourceText),row.bytes);
    assert.equal(createHash('sha256').update(row.sourceText).digest('hex'),row.sha256);
  }
  assert.equal(history.captures.length,3);
  const windows=history.captures.find(row=>row.job===118270931507);
  assert.equal(windows.jobRecord.conclusion,'cancelled');
  assert.ok(windows.annotations.some(row=>row.message==='The job has exceeded the maximum execution time of 15m0s'));
  for(const capture of history.captures){
    assert.equal(capture.run,39309745847);
    assert.equal(capture.testedCommit,'8e153ff84594dd43513f322dad04e2bb4255a863');
    assert.ok(capture.diagnosticLines.some(line=>line.includes('Finished `test` profile')));
    assert.ok(capture.diagnosticLines.some(line=>/operation was cancel/iu.test(line)));
  }
  assert.equal(history.deadlineMinutes,30);
  for(const change of history.changes){
    const original=history.sources.find(row=>row.file===change.file).sourceText,current=read(change.file);
    assert.equal(current,original.replace(change.before,change.after),'only the complete recorded deadline block changes');
    assert.equal(restorePlatformTestDeadlineHistory(current),original);
  }
  const before=parseWorkflow(history.sources[0].sourceText),after=parseWorkflow(read(history.sources[0].file));
  assert.equal(after.length,before.length);
  for(let at=0;at<before.length;at++){
    const old=before[at],current=after[at];assert.equal(current.id,old.id);
    assert.deepEqual(current.steps,old.steps,'every test command, filter, conditional and compiler safety flag remains exact');
    if(old.id==='test'){
      assert.deepEqual([...current.header.matchAll(/^ {4}timeout-minutes: (.+)$/gmu)].map(row=>row[1]),['30']);
      assert.equal(current.header,old.header.replace(history.changes[0].before,history.changes[0].after));
    }else assert.equal(current.header,old.header,'all other jobs and deadlines remain exact');
  }
});

test('platform deadline source controls retain every preceding guard and expose partial repeated and unrelated edits',()=>{
  for(const change of history.changes){
    const source=read(change.file);
    assert.throws(()=>restorePlatformTestDeadlineHistory(source+'\n'+change.after));
    assert.throws(()=>restorePlatformTestDeadlineHistory(source+'\n'+change.before));
    assert.throws(()=>restorePlatformTestDeadlineHistory(source.replace(change.after,change.after.replace('30','31'))));
    assert.equal(restorePlatformTestDeadlineHistory(source+'\n// unrelated source edit\n'),
      history.sources.find(row=>row.file===change.file).sourceText+'\n// unrelated source edit\n');
  }
  const testImport="\nimport {restorePlatformTestDeadlineHistory} from './support/platform-test-deadline-history.js';\n";
  const views=[
    {file:'js/tests/support/ci-node-runtime-history.js',edits:[["  assert.equal(typeof source,'string');","  assert.equal(typeof source,'string');\n  source=restorePlatformTestDeadlineHistory(source);"]],addition:"\nimport {restorePlatformTestDeadlineHistory} from './platform-test-deadline-history.js';\n"},
    {file:'js/tests/support/native-test-profile-history.js',edits:[["  assert.equal(typeof source,'string');","  assert.equal(typeof source,'string');\n  source=restorePlatformTestDeadlineHistory(source);"]],addition:"\nimport {restorePlatformTestDeadlineHistory} from './platform-test-deadline-history.js';\n"},
    {file:'js/tests/issue-195-evidence-ci-node-runtime.test.js',edits:[["    const current=readFileSync(new URL('../../'+owner.file,import.meta.url),'utf8');","    const current=restorePlatformTestDeadlineHistory(readFileSync(new URL('../../'+owner.file,import.meta.url),'utf8'));"]],addition:testImport},
    {file:'js/tests/issue-195-evidence-native-test-profile.test.js',edits:[["    const current=path.endsWith('.yml')?restoreWindowsLinkerLookupHistory(raw):raw;","    const current=path.endsWith('.yml')?restoreWindowsLinkerLookupHistory(raw):restorePlatformTestDeadlineHistory(raw);"]],addition:testImport},
    {file:'js/tests/issue-195-evidence-resources.test.js',edits:[["const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');","const read = (relative) => restorePlatformTestDeadlineHistory(readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8'));"]],addition:testImport},
    {file:'js/tests/issue-195-rust-suite-partitions.test.js',edits:[
      ["  const workflow = readFileSync(path.join(root, '.github/workflows/rust.yml'), 'utf8');","  const workflow = restorePlatformTestDeadlineHistory(readFileSync(path.join(root, '.github/workflows/rust.yml'), 'utf8'));"],
      ["  const workflow=readFileSync(path.join(root,'.github/workflows/rust.yml'),'utf8');","  const workflow=restorePlatformTestDeadlineHistory(readFileSync(path.join(root,'.github/workflows/rust.yml'),'utf8'));"],
    ],addition:testImport},
  ];
  for(const view of views){
    const original=history.sources.find(row=>row.file===view.file).sourceText;
    let expected=original;
    for(const [before,after]of view.edits){assert.ok(expected.includes(before));expected=expected.replace(before,after);}
    expected+=view.addition;
    const current=read(view.file);assert.equal(current,expected);
    let restored=current.slice(0,-view.addition.length);
    for(const [before,after]of view.edits.toReversed()){assert.equal(restored.split(after).length,2);restored=restored.replace(after,before);}
    assert.equal(restored,original,'every original complete assertion and input remains reconstructible');
  }
});

import {restoreCiRepairSourceHistory} from '../scripts/ci-repair-source-history.mjs';
