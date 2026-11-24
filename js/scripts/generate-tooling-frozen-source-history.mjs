// Normal certified capture of source adapters; current obligations/classifiers
// remain untouched. No Rust generation, compilation or execution is invoked.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {restoreCrateFixturePackageSource} from './crate-fixture-package-source-history.mjs';
import {TOOLING_FROZEN_SOURCE_REVISION,TOOLING_FROZEN_HOSTS,CURRENT_CONVERSATION_SOURCE,CRITICAL_DELIVERY_SOURCE_HISTORY,
  toolingFrozenSourcePin,replayToolingFrozenHostChanges,readCriticalDeliveryFrozenSource,readCriticalDeliveryAppend,
  verifyToolingFrozenSourceHistory,restoreToolingFrozenHostSource,dependencyRefreshHistoryPath,
  dependencyRefreshHistoryFiles,readToolingFrozenSourceHistory,restoreDependencyRefreshPublication} from './tooling-frozen-source-history.mjs';
const root=new URL('../../',import.meta.url),output='parity/self-translation/tooling-frozen-source-history.lino';
const helper='js/scripts/tooling-frozen-source-history.mjs';

export function generateToolingFrozenSourceHistory({check=false,apply=false,hostSuccessors=false,dependencyRefresh=false}={}){
  assert.equal(check&&apply,false);
  if(dependencyRefresh){
    assert.ok(!apply&&!hostSuccessors);
    const revision='8f2f6c483ec32a3ffc17afec694463965dc7551d';
    const certifiedHosts=readToolingFrozenSourceHistory({hostSuccessors:true}).sources;
    const sources=dependencyRefreshHistoryFiles.map(file=>{
      const {blob,certificate}=captureGitArtifactCertificate(revision,file);
      const previous=blob.toString('utf8'), current=readFileSync(new URL(file,root),'utf8');
      assert.deepEqual(Buffer.from(previous),blob);
      let start=0;
      while(start<previous.length&&start<current.length&&previous[start]===current[start])start++;
      let end=previous.length, currentEnd=current.length;
      while(end>start&&currentEnd>start&&previous[end-1]===current[currentEnd-1]){end--;currentEnd--;}
      const edit={start,removed:previous.slice(start,end),inserted:current.slice(start,currentEnd)};
      assert.equal(previous.slice(0,start)+edit.inserted+previous.slice(end),current);
      assert.equal(current.slice(0,start)+edit.removed+current.slice(currentEnd),previous);
      const host=certifiedHosts.find(row=>row.path===file);
      if(host)assert.equal(previous,host.source);
      return{file,previous:toolingFrozenSourcePin(previous),current:toolingFrozenSourcePin(current),
        ...(host?{}:{certificate}),edit};
    });
    const packet={schemaVersion:1,revision,sources};
    const target=new URL(dependencyRefreshHistoryPath,root);
    if(check)checkProjectData(target,packet);else writeProjectData(target,packet);
    const path=new URL(helper,root),source=readFileSync(path,'utf8');
    const pattern=/const dependencyRefreshHistorySha256='(?:PENDING|[a-f0-9]{64})';/u;
    assert.equal(source.split(pattern).length,2);
    const expected=source.replace(pattern,"const dependencyRefreshHistorySha256='"+toolingFrozenSourcePin(readFileSync(target)).sha256+"';");
    if(check)assert.equal(source,expected);else writeFileSync(path,expected);
    return packet;
  }
  if(hostSuccessors){
    assert.equal(apply,false,'host successors never overwrite source hosts');
    const revision='8f2f6c483ec32a3ffc17afec694463965dc7551d',predecessor='8bc0ef9725388c7927acf1854870bf46f5e365be';
    const sources=['js/tests/ci-workflow-structure.test.js','js/tests/test-groups.test.js','rust/Cargo.toml'].map(path=>{
      const captured=captureGitArtifactCertificate(revision,path),source=captured.blob.toString('utf8');
      assert.deepEqual(Buffer.from(source),captured.blob);
      const current=restoreCrateFixturePackageSource(readFileSync(new URL(path,root),'utf8'),path);
      assert.equal(restoreDependencyRefreshPublication(current,path),source);
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
  const args=process.argv.slice(2);assert.equal(new Set(args).size,args.length);assert.ok(args.every(value=>['--check','--apply','--host-successors','--dependency-refresh'].includes(value)));
  const packet=generateToolingFrozenSourceHistory({check:args.includes('--check'),apply:args.includes('--apply'),hostSuccessors:args.includes('--host-successors'),dependencyRefresh:args.includes('--dependency-refresh')});
  console.log(JSON.stringify({sourceHosts:(packet.hosts??packet.sources).map(row=>row.path??row.file),appendedRequirementIds:packet.conversation?.append.ids??[],nativeExecution:false}));
}
