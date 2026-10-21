// Reproducible source capture for the observed tooling deadline repair.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

export function generateToolingPartitionHistory({check=false}={}) {
  const revision='a48efdd64201cefc205fd81703c8cd85777f515f',path='.github/workflows/js.yml';
  const {certificate,blob}=captureGitArtifactCertificate(revision,path);
  const before='        include:\n          - {group: grammar, partition: 1, shards: 3}\n';
  const rows=Array.from({length:4},(_,index)=>`          - {group: tooling, partition: ${index+1}, shards: 4}\n`).join('');
  const after=before.replace('        include:\n','        include:\n'+rows);
  const source=blob.toString('utf8');assert.equal(source.split(before).length,2);
  assert.equal([...source.matchAll(/\{group: tooling,/gu)].length,0);
  const policy={schemaVersion:1,revision,workflow:{path,certificate,source},group:'tooling',shards:4,timeoutMinutes:15,
    change:{before,after},
    boundary:'Four distinct required tooling cells replace the implicit single cell after its measured fifteen-minute cancellation. The existing modulo file partitioner runs every complete current test file exactly once. Every command, assertion, compiler flag, predicate, deadline and unrelated matrix cell remains unchanged; historical views invert only this exact source block. Full suites and native compilation remain CI-only.'};
  const file=new URL('../../parity/fixtures/tooling-partition-history.lino',import.meta.url);
  if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
  return policy;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateToolingPartitionHistory({check:process.argv.includes('--check')});
  console.log('Captured exact four-way tooling partition with unchanged fifteen-minute deadline.');
}
