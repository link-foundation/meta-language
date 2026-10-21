// Normal certified capture of source adapters; current obligations/classifiers
// remain untouched. No Rust generation, compilation or execution is invoked.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {TOOLING_FROZEN_SOURCE_REVISION,TOOLING_FROZEN_HOSTS,CURRENT_CONVERSATION_SOURCE,CRITICAL_DELIVERY_SOURCE_HISTORY,
  toolingFrozenSourcePin,replayToolingFrozenHostChanges,readCriticalDeliveryFrozenSource,readCriticalDeliveryAppend,
  verifyToolingFrozenSourceHistory,restoreToolingFrozenHostSource} from './tooling-frozen-source-history.mjs';
const root=new URL('../../',import.meta.url),output='parity/self-translation/tooling-frozen-source-history.lino';
const helper='js/scripts/tooling-frozen-source-history.mjs';

export function generateToolingFrozenSourceHistory({check=false,apply=false,hostSuccessors=false}={}){
  assert.equal(check&&apply,false);
  if(hostSuccessors){
    assert.equal(apply,false,'host successors never overwrite source hosts');
    const revision='8f2f6c483ec32a3ffc17afec694463965dc7551d',predecessor='8bc0ef9725388c7927acf1854870bf46f5e365be';
    const sources=['js/tests/ci-workflow-structure.test.js','js/tests/test-groups.test.js','rust/Cargo.toml'].map(path=>{
      const captured=captureGitArtifactCertificate(revision,path),source=captured.blob.toString('utf8');
      assert.deepEqual(Buffer.from(source),captured.blob);assert.equal(readFileSync(new URL(path,root),'utf8'),source);
      const old=captureGitArtifactCertificate(predecessor,path),prior=old.blob.toString('utf8');assert.deepEqual(Buffer.from(prior),old.blob);
      let start=0;while(start<prior.length&&start<source.length&&prior[start]===source[start])start++;
      let suffix=0;while(suffix<prior.length-start&&suffix<source.length-start&&prior.at(-1-suffix)===source.at(-1-suffix))suffix++;
      const edit={start,removed:prior.slice(start,prior.length-suffix),inserted:source.slice(start,source.length-suffix)};
      return{path,source,...toolingFrozenSourcePin(source),certificate:captured.certificate,
        previous:{revision:predecessor,source:prior,...toolingFrozenSourcePin(prior),certificate:old.certificate},edit};
    });
    const packet={schemaVersion:1,model:'certified tooling host successors 1',revision,
      sources,
      boundary:'Complete current workflow and scheduling tests and the crate manifest are independently certified at their actual commit. Finite inverses retain every certified prior source byte; original historical guards keep their exact scope while current tests verify checkout provenance, measured complete-workload ownership and the included identifier policy. No arbitrary successor, changed assertion, execution observation or weaker gate is authorized.'};
    verifyToolingFrozenSourceHistory(packet,{hostSuccessors:true});
    const target=new URL('parity/self-translation/tooling-frozen-host-successors.lino',root);
    if(check)checkProjectData(target,packet);else writeProjectData(target,packet);
    return packet;
  }
  const captured=captureGitArtifactCertificate(TOOLING_FROZEN_SOURCE_REVISION,CURRENT_CONVERSATION_SOURCE.path);
  const source=captured.blob.toString('utf8');assert.deepEqual(Buffer.from(source),captured.blob);
  assert.equal(readFileSync(new URL(CURRENT_CONVERSATION_SOURCE.path,root),'utf8'),source);
  const previous=readCriticalDeliveryFrozenSource().source;
  const hosts=TOOLING_FROZEN_HOSTS.map(specification=>{
    const original=captureGitArtifactCertificate(TOOLING_FROZEN_SOURCE_REVISION,specification.path);
    const source=original.blob.toString('utf8');assert.deepEqual(Buffer.from(source),original.blob);
    const replay=replayToolingFrozenHostChanges(source,specification.changes);
    return{path:specification.path,source,...toolingFrozenSourcePin(source),certificate:original.certificate,
      current:toolingFrozenSourcePin(replay.source),edits:replay.edits};
  });
  const packet={schemaVersion:1,model:'composed CI tooling historical scope 1',revision:TOOLING_FROZEN_SOURCE_REVISION,
    coordinateUnit:'UTF-16 code units',criticalHistory:CRITICAL_DELIVERY_SOURCE_HISTORY,
    conversation:{path:CURRENT_CONVERSATION_SOURCE.path,source,...toolingFrozenSourcePin(source),certificate:captured.certificate,
      previous:{source:previous,...toolingFrozenSourcePin(previous)},append:readCriticalDeliveryAppend(previous,source)},hosts,
    boundary:'Existing certified test-registration history supplies the exact historical classifier; every actual current discovered file and registered addition is checked independently. One complete certified conversation successor adds obligations only at the old list closing delimiter; historical byte assertions retain their exact source view while the actual current manifest and every old/new row remain gated. Complete host source inverses retain every original input and assertion. This proves source/scope conservation, not implementation completion, behavioral success or native execution.'};
  const outputs=verifyToolingFrozenSourceHistory(packet);
  for(const row of outputs){
    const actual=readFileSync(new URL(row.path,root),'utf8');
    if(check)assert.equal(restoreToolingFrozenHostSource(actual,row.path),row.previous);else assert.ok(actual===row.previous||actual===row.current,'unknown complete host source is never replaced');
  }
  if(check)checkProjectData(new URL(output,root),packet);else writeProjectData(new URL(output,root),packet);
  const helperPath=new URL(helper,root),current=readFileSync(helperPath,'utf8'),pattern=/const policySha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(current.split(pattern).length,2);
  const expected=current.replace(pattern,"const policySha256='"+toolingFrozenSourcePin(readFileSync(new URL(output,root))).sha256+"';");
  if(check)assert.equal(current,expected);else writeFileSync(helperPath,expected);
  if(apply)for(const row of outputs)writeFileSync(new URL(row.path,root),row.current);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.equal(new Set(args).size,args.length);assert.ok(args.every(value=>['--check','--apply','--host-successors'].includes(value)));
  const packet=generateToolingFrozenSourceHistory({check:args.includes('--check'),apply:args.includes('--apply'),hostSuccessors:args.includes('--host-successors')});
  console.log(JSON.stringify({sourceHosts:(packet.hosts??packet.sources).map(row=>row.path),appendedRequirementIds:packet.conversation?.append.ids??[],nativeExecution:false}));
}
