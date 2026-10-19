import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {restoreNativeTestProfileHistory} from './support/native-test-profile-history.js';

test('actual native test optimization retains every target, logical scope, safety check and original deadline',()=>{
  const history=readProjectData(new URL('../../parity/naming/native-test-profile-source-history.lino',import.meta.url));
  for(const source of history.sources){
    assert.equal(Buffer.byteLength(source.sourceText),source.bytes);
    assert.equal(createHash('sha256').update(source.sourceText).digest('hex'),source.sha256);
  }
  const before='cargo test --no-fail-fast --all-features --tests --verbose -- $SUITE_FILTERS';
  const settings=['profile.test.package.meta-language.opt-level=3',
    'profile.test.package.meta-language.debug-assertions=true','profile.test.package.meta-language.overflow-checks=true'];
  const profile=settings.map(value=>"--config '"+value+"'").join(' ');
  const after=before.replace(' -- $SUITE_FILTERS',' '+profile+' -- $SUITE_FILTERS');
  assert.equal(history.before,before);assert.equal(history.after,after);assert.equal(history.profile,profile);
  for(const path of ['.github/workflows/rust.yml','rust/tests/unit/ci-cd/workflow_release.rs']){
    const original=history.sources.find(source=>source.file===path);assert.ok(original);
    const raw=readFileSync(new URL('../../'+path,import.meta.url),'utf8');
    const current=path.endsWith('.yml')?restoreWindowsLinkerLookupHistory(raw):raw;
    assert.equal(current,original.sourceText.replace(before,after),'only the explicit checked package profile changes');
    assert.equal(restoreNativeTestProfileHistory(current),original.sourceText);
  }
  for(const path of ['rust/scripts/simulate-fresh-merge.sh','js/scripts/issue-195-rust-suite-partitions.mjs']){
    const source=readFileSync(new URL('../../'+path,import.meta.url),'utf8');
    for(const setting of settings)assert.ok(source.includes(setting),'the same checked profile already exists in '+path);
  }
  const oldWorkflow=history.sources.find(source=>source.file==='.github/workflows/rust.yml').sourceText;
  const current=restoreWindowsLinkerLookupHistory(readFileSync(new URL('../../.github/workflows/rust.yml',import.meta.url),'utf8'));
  const oldJob=parseWorkflow(oldWorkflow).find(job=>job.id==='test'),job=parseWorkflow(current).find(job=>job.id==='test');
  assert.equal(job.header,oldJob.header,'all platforms, physical cells, prerequisites, conditions and original deadlines remain exact');
  assert.equal(job.steps.length,oldJob.steps.length);
  for(let at=0;at<job.steps.length;at++)if(job.steps[at].name!=='Run tests')assert.equal(job.steps[at].text,oldJob.steps[at].text);
  const step=job.steps.find(step=>step.name==='Run tests');
  assert.equal(step.run.trim(),'node ../scripts/with-cache-cleanup.mjs --event test -- '+after);
  const filters=[...new Set([...job.header.matchAll(/^\s+filters: ([^\n]+)/gmu)].map(match=>match[1]))];
  assert.equal(filters.length,5,'every original logical filter is exercised');
  for(const selected of filters){
    const child=spawnSync('bash',['-e','-c','node() { printf "%s\\0" "$@"; }\n'+step.run],
      {encoding:'utf8',env:{...process.env,SUITE_FILTERS:selected},timeout:5000,maxBuffer:65536});
    assert.equal(child.error,undefined);assert.equal(child.signal,null);assert.equal(child.status,0,child.stderr);
    assert.deepEqual(child.stdout.split('\0').filter(Boolean),[
      '../scripts/with-cache-cleanup.mjs','--event','test','--','cargo','test','--no-fail-fast','--all-features','--tests','--verbose',
      ...settings.flatMap(value=>['--config',value]),'--',...selected.trim().split(/\s+/u),
    ],'the actual shell passes all original arguments and only the three checked profile settings; no Cargo process runs');
  }
  assert.equal(history.captures.length,2);
  for(const capture of history.captures){
    assert.equal(capture.job.conclusion,'cancelled');
    assert.ok(capture.annotations.some(annotation=>annotation.message==='The job has exceeded the maximum execution time of 15m0s'));
    assert.ok(capture.diagnosticLines.some(line=>line.includes('unoptimized')));
  }
});

test('native test profile history refuses missing or weakened flags and exposes every unrelated workflow edit',()=>{
  const history=readProjectData(new URL('../../parity/naming/native-test-profile-source-history.lino',import.meta.url));
  const source=restoreWindowsLinkerLookupHistory(readFileSync(new URL('../../.github/workflows/rust.yml',import.meta.url),'utf8'));
  for(const [before,after]of [['opt-level=3','opt-level=0'],['debug-assertions=true','debug-assertions=false'],
    ['overflow-checks=true','overflow-checks=false'],['--all-features',''],['--no-fail-fast',''],['--tests --verbose','--verbose']]){
    assert.throws(()=>restoreNativeTestProfileHistory(source.replace(history.after,history.after.replace(before,after))));
  }
  assert.throws(()=>restoreNativeTestProfileHistory(source+'\n'+history.after));
  assert.throws(()=>restoreNativeTestProfileHistory(source.replace(history.after,history.before)));
  const original=history.sources.find(row=>row.file==='.github/workflows/rust.yml').sourceText;
  assert.equal(restoreNativeTestProfileHistory(source.replace('timeout-minutes: 15','timeout-minutes: 99')),
    original.replace('timeout-minutes: 15','timeout-minutes: 99'),'an unrelated gate edit survives the inverse and remains visible to original assertions');
});

import {restoreWindowsLinkerLookupHistory} from './support/windows-linker-lookup-history.js';
