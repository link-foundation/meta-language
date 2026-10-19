import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {mergeRustSuitePartitions} from '../scripts/issue-195-rust-suite-partitions.mjs';
import {restoreCiNodeRuntimeHistory} from './support/ci-node-runtime-history.js';

test('actual mixed Node and npm evidence retains exact partition gates and pins every original setup without changing scope',()=>{
  const history=readProjectData(new URL('../../parity/naming/ci-node-runtime-source-history.lino',import.meta.url));
  assert.equal(history.aggregate.run,39283021387);assert.equal(history.aggregate.jobs,222);
  assert.equal(history.aggregate.log.sha256,'144ab7d88b7c47cf2bbb30720bff165e2eaa691160accb96c338196e7361e4be');
  assert.ok(history.aggregate.diagnosticLines.some(line=>line.includes('toolchain versions differ between suite partitions')));
  assert.ok(history.aggregate.diagnosticLines.some(line=>line.includes('toolchain versions differ between native partitions')));
  for(const source of history.sources){
    assert.equal(Buffer.byteLength(source.sourceText),source.bytes);
    assert.equal(createHash('sha256').update(source.sourceText).digest('hex'),source.sha256);
  }
  assert.equal(history.nodeVersion,'24.22.0');assert.equal(history.npmVersion,'11.19.1');
  assert.equal(history.distribution.release.version,'v'+history.nodeVersion);
  assert.equal(history.distribution.release.npm,history.npmVersion);
  assert.equal(history.distribution.source,'https://nodejs.org/dist/index.json');
  assert.deepEqual(history.nodeVersionRange,{minimum:'24.22.0',maximum:'24.22.0',minimumInclusive:true,maximumInclusive:true});
  assert.deepEqual(history.npmVersionRange,{minimum:'11.19.1',maximum:'11.19.1',minimumInclusive:true,maximumInclusive:true});
  assert.equal(history.captures.length,4);
  for(const capture of history.captures){
    assert.equal(Buffer.byteLength(capture.sourceText),capture.bytes);
    assert.equal(createHash('sha256').update(capture.sourceText).digest('hex'),capture.sha256);
    assert.deepEqual(readStructuredDataLino(capture.sourceText),capture.record);
    assert.equal(capture.record.commit,history.aggregate.testedCommit);
  }
  for(const [first,second]of [[0,1],[2,3]]){
    const left=history.captures[first].record.toolchainVersions,right=history.captures[second].record.toolchainVersions;
    assert.deepEqual(Object.keys(left).filter(key=>left[key]!==right[key]).sort(),['node','npm']);
    assert.deepEqual(new Set([left.node,right.node]),new Set(['v24.21.0','v24.22.0']));
    assert.deepEqual(new Set([left.npm,right.npm]),new Set(['11.19.0','11.19.1']));
  }
  let pins=0;
  for(const owner of history.owners){
    const before=history.sources.find(row=>row.file===owner.file).sourceText;
    const current=restorePlatformTestDeadlineHistory(readFileSync(new URL('../../'+owner.file,import.meta.url),'utf8'));
    assert.equal(current,before.replace(/^(\s*node-version: )24$/gmu,(_match,prefix)=>prefix+'24.22.0'));
    assert.equal(restoreCiNodeRuntimeHistory(current),before);
    const oldJobs=parseWorkflow(before),jobs=parseWorkflow(current);assert.equal(jobs.length,oldJobs.length);
    for(let at=0;at<jobs.length;at++){
      assert.equal(jobs[at].header,oldJobs[at].header);
      assert.equal(jobs[at].steps.length,oldJobs[at].steps.length);
      for(let step=0;step<jobs[at].steps.length;step++)assert.equal(jobs[at].steps[step].text,
        oldJobs[at].steps[step].text.replace(/^(\s*node-version: )24$/gmu,(_match,prefix)=>prefix+'24.22.0'));
    }
    pins+=owner.count;
  }
  assert.equal(pins,25);
  for(const file of ['js/scripts/issue-195-rust-suite-partitions.mjs','js/scripts/issue-195-javascript-suite-partitions.mjs']){
    const original=history.sources.find(row=>row.file===file);
    assert.equal(readFileSync(new URL('../../'+file,import.meta.url),'utf8'),original.sourceText,'complete exact-version gates remain byte-identical');
  }
  const records=new Map(history.captures.slice(0,2).map(row=>[row.record.stage,row.record]));
  const result=mergeRustSuitePartitions({checkpoint:'pre-merge',commit:history.aggregate.testedCommit,records,
    root:'/home/runner/work/meta-language/meta-language',verifyPhysicalUnion:()=>{throw new Error('Source-only negative control has no actual complete native union');}});
  assert.match(result.get('rust-suite').error,/toolchain versions differ between native partitions/u);
});

test('runtime patch source inverse refuses missing mixed repeated foreign and unsupported selections while exposing unrelated edits',()=>{
  const history=readProjectData(new URL('../../parity/naming/ci-node-runtime-source-history.lino',import.meta.url));
  for(const owner of history.owners){
    const current=readFileSync(new URL('../../'+owner.file,import.meta.url),'utf8');
    for(const replacement of ['node-version: 24','node-version: 24.21.0','node-version: 24.22.1','node-version: 25','removed-version: 24.22.0']){
      assert.throws(()=>restoreCiNodeRuntimeHistory(current.replace('node-version: 24.22.0',replacement)));
    }
    assert.throws(()=>restoreCiNodeRuntimeHistory(current+'\n          node-version: 24.22.0\n'));
    assert.throws(()=>restoreCiNodeRuntimeHistory(current.replace(owner.header,'name: Foreign runtime')));
    const old=history.sources.find(row=>row.file===owner.file).sourceText;
    assert.equal(restoreCiNodeRuntimeHistory(current.replace('timeout-minutes:','changed-timeout-minutes:')),
      old.replace('timeout-minutes:','changed-timeout-minutes:'));
  }
});

test('runtime pin adapters preserve all complete preceding workflow guard modules and source assertions',()=>{
  const history=readProjectData(new URL('../../parity/naming/ci-node-runtime-source-history.lino',import.meta.url));
  const views=[
    {file:'js/tests/support/windows-linker-lookup-history.js',edits:[[
      "  assert.equal(typeof source,'string');",
      "  assert.equal(typeof source,'string');\n  if(source.includes('node-version: 24.22.0'))source=restoreCiNodeRuntimeHistory(source);",
    ]],addition:"\nimport {restoreCiNodeRuntimeHistory} from './ci-node-runtime-history.js';\n"},
    {file:'js/tests/support/generated-package-owner-history.js',edits:[[
      '  let original = source;',
      "  let original = source.includes('node-version: 24.22.0') ? restoreCiNodeRuntimeHistory(source) : source;",
    ]],addition:"\nimport {restoreCiNodeRuntimeHistory} from './ci-node-runtime-history.js';\n"},
    {file:'js/tests/issue-195-evidence-windows-linker-timeout.test.js',edits:[[
      "const current=readFileSync(new URL('../../'+file,import.meta.url),'utf8');",
      "const current=restoreCiNodeRuntimeHistory(readFileSync(new URL('../../'+file,import.meta.url),'utf8'));",
    ],[
      "const source=readFileSync(new URL('../../.github/workflows/rust.yml',import.meta.url),'utf8');",
      "const source=restoreCiNodeRuntimeHistory(readFileSync(new URL('../../.github/workflows/rust.yml',import.meta.url),'utf8'));",
    ]],addition:"\nimport {restoreCiNodeRuntimeHistory} from './support/ci-node-runtime-history.js';\n"},
    {file:'js/tests/issue-195-evidence-generated-owner-history.test.js',edits:[[
      "const current = readFileSync(new URL('../../' + evidence.path, import.meta.url), 'utf8');",
      "const current = restoreCiNodeRuntimeHistory(readFileSync(new URL('../../' + evidence.path, import.meta.url), 'utf8'));",
    ]],addition:"\nimport {restoreCiNodeRuntimeHistory} from './support/ci-node-runtime-history.js';\n"},
    {file:'js/tests/ci-workflow-structure.test.js',edits:[[
      "const workflow = (name) => name === 'js.yml' ? restoreGeneratedPackageOwnerHistory(read(`.github/workflows/${name}`)) : read(`.github/workflows/${name}`);",
      "const workflow = (name) => name === 'js.yml' ? restoreGeneratedPackageOwnerHistory(read(`.github/workflows/${name}`)) : restoreCiNodeRuntimeHistory(read(`.github/workflows/${name}`));",
    ]],addition:"\nimport {restoreCiNodeRuntimeHistory} from './support/ci-node-runtime-history.js';\n"},
  ];
  for(const view of views){
    const original=history.sources.find(row=>row.file===view.file);assert.ok(original);
    let expected=original.sourceText;
    for(const [before,after]of view.edits){assert.equal(expected.split(before).length,2);expected=expected.replace(before,after);}
    expected+=view.addition;
    const current=readFileSync(new URL('../../'+view.file,import.meta.url),'utf8');assert.equal(current,expected);
    let restored=current.slice(0,-view.addition.length);
    for(const [before,after]of [...view.edits].reverse()){assert.equal(restored.split(after).length,2);restored=restored.replace(after,before);}
    assert.equal(restored,original.sourceText,'every original assertion and source context reconstructs exactly');
  }
});

import {restorePlatformTestDeadlineHistory} from './support/platform-test-deadline-history.js';
