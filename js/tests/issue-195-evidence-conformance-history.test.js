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
  const source=restoreRawContractPartitionHistory(restoreCiRepairSourceHistory('.github/workflows/js.yml',readFileSync(new URL('.github/workflows/js.yml',root),'utf8')));
  assert.equal(createHash('sha256').update(source).digest('hex'),fixture.change.currentSha256);
  const previous=restoreConformancePartitionHistory(source);
  assert.equal(previous,fixture.workflow.text);
  assert.equal(restoreConformancePartitionHistory(previous),previous);
  assert.equal(Buffer.byteLength(previous),fixture.workflow.bytes);
  assert.equal(createHash('sha256').update(previous).digest('hex'),fixture.workflow.sha256);
  assert.ok(Buffer.from(previous).equals(execFileSync('git',['show',fixture.head+':'+fixture.workflow.path],{cwd:root,maxBuffer:128*1024})));
});

test('historical conformance views refuse missing duplicate mixed and unsupported matrices and retain unrelated edits',()=>{
  const source=restoreCiRepairSourceHistory('.github/workflows/js.yml',readFileSync(new URL('.github/workflows/js.yml',root),'utf8')),{before,after}=fixture.change;
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
  const edits = new Map([
    ['js/tests/support/platform-test-deadline-history.js', {
      suffix: "\nimport {restoreConformancePartitionHistory} from './conformance-partition-history.js';\n"
        + "\nimport {restoreEvidenceTranslationSourceHistory} from '../../scripts/evidence-translation-source-history.mjs';\n"
        + "\nimport {restoreCurrentCiRepairSource} from '../../scripts/current-ci-repair-source-history.mjs';\n"
        + "import {restoreDependencyRefreshPublication} from '../../scripts/tooling-frozen-source-history.mjs';\n",
      lines: [
        ["  if(source.startsWith('name: Dependency Refresh\\n'))source=restoreDependencyRefreshPublication(source,'.github/workflows/dependency-refresh.yml');\n", ''],
        ["  source=restoreCurrentCiRepairSource(source,'.github/workflows/rust.yml');\n", ''],
        ["  source=restoreEvidenceTranslationSourceHistory('.github/workflows/ci.yml',source);\n", ''],
        ['  source=restoreConformancePartitionHistory(source);\n', ''],
      ],
    }],
    ['js/tests/issue-195-evidence-translation-feedback.test.js', {
      suffix: "\nimport {restoreConformancePartitionHistory} from './support/conformance-partition-history.js';\n"
        + "import {restoreTranslationWorkflowHistory} from './support/translation-workflow-history.js';\n"
        + "\nimport {restoreEvidenceTranslationSourceHistory} from '../scripts/evidence-translation-source-history.mjs';\n",
      lines: [
        ['  text=restoreConformancePartitionHistory(text);\n', ''],
        ["restoreEvidenceTranslationSourceHistory('.github/workflows/ci.yml',readFileSync(new URL('../../.github/workflows/ci.yml', import.meta.url), 'utf8'))", "readFileSync(new URL('../../.github/workflows/ci.yml', import.meta.url), 'utf8')"],
        ["restoreTranslationWorkflowHistory(readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8'))", "readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8')"],
      ],
    }],
  ]);
  for (const [file, { suffix, lines }] of edits) {
    const current = restoreCiRepairSourceHistory(file, readFileSync(new URL(file,root), 'utf8'));
    assert.ok(current.endsWith(suffix));
    let previous = current.slice(0, -suffix.length);
    for (const [successor, predecessor] of lines) {
      assert.equal(previous.split(successor).length, 2);
      previous = previous.replace(successor, predecessor);
    }
    assert.ok(Buffer.from(previous).equals(execFileSync('git', ['show', revision+':'+file], { cwd: root, maxBuffer: 128*1024 })));
  }
});

import {restoreRawContractPartitionHistory} from './support/raw-contract-partition-history.js';

import {restoreCiRepairSourceHistory} from '../scripts/ci-repair-source-history.mjs';
