import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {javascriptSuiteTestGroups,javascriptSuitePartition,javascriptSuiteCommandArguments,mergeJavaScriptSuitePartitions} from '../scripts/issue-195-javascript-suite-partitions.mjs';
import {stageFiles} from '../scripts/issue-195-evidence-stages.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {restoreEvidenceTranslationSourceHistory} from '../scripts/evidence-translation-source-history.mjs';
import {generateEvidenceTranslationHistory} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-evidence-translation-history.mjs';
import {restoreNativeCompatibilitySourceHistory} from '../scripts/native-compatibility-source-history.mjs';
import {restorePhysicalReportWorkflow} from '../scripts/ci-test-partition-history.mjs';
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

function evidenceTranslationPartitionRecords() {
  const groups=javascriptSuiteTestGroups(),commit='a'.repeat(40),checkpoint='pre-merge',records=new Map();
  for(const group of Object.keys(groups))for(let index=1;index<=(group==='translation'?6:1);index++){
    const selection=javascriptSuitePartition(group,index+'/'+(group==='translation'?6:1),groups);
    records.set(selection.name,{schemaVersion:1,issue:195,stage:selection.name,commit,checkpoint,javascriptPartition:selection,
      toolchainVersions:{node:'v24.22.0'},groups:{'suite:javascript':{
        commands:[['node',...javascriptSuiteCommandArguments(group,selection.shard)].join(' ')],
        artifacts:['logs/'+selection.name+'.log'],failureLogs:[]}},error:null});
  }
  return{groups,commit,checkpoint,records};
}

test('six actual evidence translation scopes preserve every original workflow byte outside their matrix block',()=>{
  const [sources,fixtures]=generateEvidenceTranslationHistory({check:true});
  assert.equal(sources.sources.length,8);assert.equal(fixtures.sources.length,1);
  for(const row of [...sources.sources,...fixtures.sources]){
    assert.deepEqual(pin(restoreNativeCompatibilitySourceHistory(row.file,readFileSync(new URL(row.file,root),'utf8'))),row.current);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
    const current=readFileSync(new URL(row.file,root),'utf8');
    assert.equal(restoreEvidenceTranslationSourceHistory(row.file,current),row.sourceText);
    const unknown=current+'\n# unknown current source\n';assert.equal(restoreEvidenceTranslationSourceHistory(row.file,unknown),unknown);
  }
  const row=fixtures.sources[0],physical=readFileSync(new URL(row.file,root),'utf8'),actual=restorePhysicalReportWorkflow(physical,row.file);
  const before='          - group: translation\n            shard: 1/1\n            partition: 1-of-1\n';
  const after=Array.from({length:6},(_,index)=>'          - group: translation\n            shard: '+(index+1)+'/6\n            partition: '+(index+1)+'-of-6\n').join('');
  assert.equal(actual.split(after).length,2);assert.equal(actual.includes(before),false);
  assert.equal(actual.replace(after,before),row.sourceText,'all original jobs, steps, tools, native flags, gates and artifact paths stay exact');
  assert.equal(restoreEvidenceTranslationSourceHistory(row.file,physical),row.sourceText);
  assert.equal(restoreEvidenceTranslationSourceHistory(row.file,row.sourceText),row.sourceText);
  const changed=physical+'\n# changed input\n';assert.equal(restoreEvidenceTranslationSourceHistory(row.file,changed),changed);
  const job=parseWorkflow(physical).find(job=>job.id==='javascript-suite-independent');assert.ok(job);
  assert.match(job.header,/timeout-minutes: 60/u);assert.match(job.header,/fail-fast: false/u);
  const scopes=[...job.header.matchAll(/- group: translation\n\s+shard: ([1-6]\/6)\n\s+partition: ([1-6]-of-6)/gu)];
  assert.equal(scopes.length,6);assert.deepEqual(scopes.map(row=>row[1]),['1/6','2/6','3/6','4/6','5/6','6/6']);
  assert.deepEqual(scopes.map(row=>row[2]),['1-of-6','2-of-6','3-of-6','4-of-6','5-of-6','6-of-6']);
});

test('six complete evidence translation partitions form the exact current file union and retain distinct physical paths',()=>{
  const value=evidenceTranslationPartitionRecords(),selections=[...value.records.values()].filter(row=>row.javascriptPartition.group==='translation').map(row=>row.javascriptPartition);
  const files=selections.flatMap(row=>row.files),paths=selections.flatMap(row=>Object.values(stageFiles(row.name)));
  assert.deepEqual([...files].sort(),[...value.groups.translation].sort());assert.equal(new Set(files).size,files.length);
  assert.equal(new Set(paths).size,12);assert.ok(selections.every(row=>row.files.length>0));
  const result=mergeJavaScriptSuitePartitions({...value,testGroups:value.groups}).get('javascript-suite');
  assert.equal(result.error,null);assert.equal(result.groups['suite:javascript'].commands.length,value.records.size);
  assert.equal(result.groups['suite:javascript'].artifacts.length,value.records.size);
});

test('incomplete failed altered or mixed evidence translation cohorts cannot supply a logical suite',()=>{
  for(const mutate of [
    value=>value.records.delete('javascript-suite-translation-6-of-6'),
    value=>value.records.get('javascript-suite-translation-2-of-6').javascriptPartition.files.pop(),
    value=>value.records.get('javascript-suite-translation-2-of-6').javascriptPartition.files.push('foreign.test.js'),
    value=>{value.records.get('javascript-suite-translation-2-of-6').commit='b'.repeat(40);},
    value=>{value.records.get('javascript-suite-translation-2-of-6').error='native translation test failed';},
    value=>{value.records.get('javascript-suite-translation-2-of-6').groups['suite:javascript'].commands[0]='node scripts/test-groups.mjs --run translation';},
    value=>{value.records.get('javascript-suite-translation-2-of-6').toolchainVersions.node='v24.0.0';},
    value=>{value.records.get('javascript-suite-translation-2-of-6').groups['suite:javascript'].artifacts=[];},
    value=>{const selection=javascriptSuitePartition('translation','1/3',value.groups);const row=structuredClone(value.records.get('javascript-suite-translation-1-of-6'));row.stage=selection.name;row.javascriptPartition=selection;value.records.set(selection.name,row);},
  ]){
    const value=evidenceTranslationPartitionRecords();mutate(value);
    const result=mergeJavaScriptSuitePartitions({...value,testGroups:value.groups}).get('javascript-suite');
    assert.equal(typeof result.error,'string');assert.ok(result.error.length>0);assert.deepEqual(result.groups,{});
  }
});
