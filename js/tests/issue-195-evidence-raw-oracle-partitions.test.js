import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {generateRawOracleContractPartitions} from '../scripts/generate-raw-oracle-contract-partitions.mjs';
import {restoreRawOracleContractPartitionHistory} from '../scripts/raw-oracle-contract-partition-history.mjs';
import {createOriginalLinkRawContractPartition} from '../../experiments/original-link-complete-table-projection/raw-contract-partitions.mjs';
import {restoreCiTestPartitionWorkflow} from '../scripts/ci-test-partition-history.mjs';
import {parseNative} from '../src/native-grammar-parser.js';
import {tokenize} from '../src/translation/lexer.js';

const environment=profile=>({CI:'true',ORIGINAL_LINK_RAW_NATIVE:'0',
  ORIGINAL_LINK_RAW_SEPARATE_SUITES:'1',ORIGINAL_LINK_RAW_CONTRACT_PARTITION:profile});

test('both required raw partitions register every original callback exactly once without executing another partition',()=>{
  const {fixture}=generateRawOracleContractPartitions({check:true});
  const rows=fixture.tests.filter(row=>row.partition!=='native'),executed=[];
  for(const profile of ['schema','plan']){
    const registered=[],runner=createOriginalLinkRawContractPartition((name,callback)=>{
      registered.push(name);callback();return name;
    },environment(profile));
    assert.equal(runner.runPlan,profile==='plan');
    const encountered=profile==='schema'?rows.filter(row=>row.partition==='schema'):rows;
    for(const row of encountered){
      runner.test(row.name,()=>executed.push(row.name));
    }
    runner.finish();assert.deepEqual(registered,rows.filter(row=>row.partition===profile).map(row=>row.name));
  }
  assert.equal(executed.length,18);assert.equal(new Set(executed).size,18);
  assert.deepEqual(executed,rows.map(row=>row.name));
  const register=()=>1,all=createOriginalLinkRawContractPartition(register,{});
  assert.equal(all.test,register);assert.equal(all.runPlan,true);all.finish();
});

test('unknown profiles missing registrations duplicates and native admission in injected partitions refuse',()=>{
  const {fixture}=generateRawOracleContractPartitions({check:true});
  const name=fixture.tests[0].name;
  for(const value of [environment('unknown'),{...environment('plan'),CI:'false'},
    {...environment('plan'),ORIGINAL_LINK_RAW_NATIVE:'1'},
    {...environment('plan'),ORIGINAL_LINK_RAW_SEPARATE_SUITES:'0'}]){
    assert.throws(()=>createOriginalLinkRawContractPartition(()=>{},value));
  }
  const runner=createOriginalLinkRawContractPartition(()=>{},environment('schema'));
  assert.throws(()=>runner.finish());assert.throws(()=>runner.test('unknown',()=>{}));
  runner.test(name,()=>{});assert.throws(()=>runner.test(name,()=>{}));
  assert.throws(()=>runner.test(fixture.tests.find(row=>row.partition==='native').name,()=>{}));
});

test('the entire current test source retains every original call body hash and a complete certified inverse',()=>{
  const {fixture,history}=generateRawOracleContractPartitions({check:true});
  const source=readFileSync(new URL('../../'+fixture.testPath,import.meta.url),'utf8'),bytes=Buffer.from(source);
  const tree=parseNative('native-javascript',source);assert.equal(tree.hasError,false);
  const nodes=[tree],actual=[];
  for(let at=0;at<nodes.length;at+=1){
    const node=nodes[at];nodes.push(...node.children.map(row=>row.node));
    if(node.term!=='call_expression')continue;
    const fn=node.children.find(row=>row.field==='function')?.node;
    if(!fn||bytes.subarray(fn.start,fn.end).toString()!=='test')continue;
    const args=node.children.find(row=>row.field==='arguments')?.node;
    const name=args.children.find(row=>row.node.term==='string')?.node;assert.ok(name);
    actual.push({name:tokenize(bytes.subarray(name.start,name.end).toString(),'JavaScript').tokens[0].value,
      start:node.start,sha256:createHash('sha256').update(bytes.subarray(node.start,node.end)).digest('hex')});
  }
  actual.sort((first,second)=>first.start-second.start);
  assert.deepEqual(actual.map(({name,sha256})=>({name,sha256})),fixture.tests.map(({name,sha256})=>({name,sha256})));
  const original=history.sources.find(row=>row.path===fixture.testPath).source;
  assert.equal(restoreRawOracleContractPartitionHistory(source,fixture.testPath),original);
  assert.equal(restoreRawOracleContractPartitionHistory(source+'\n# changed\n',fixture.testPath),source+'\n# changed\n');
});

test('two required raw contract cells preserve complete commands limits and the preceding whole workflow',()=>{
  const {history}=generateRawOracleContractPartitions({check:true}),path='.github/workflows/js.yml';
  // The complete certified predecessor owns these original scheduling assertions.
  // Current physical partitions have independent conservation controls.
  const currentSource=restoreCiTestPartitionWorkflow(readFileSync(new URL('../../'+path,import.meta.url),'utf8'));
  const previousSource=history.sources.find(row=>row.path===path).source;
  assert.equal(restoreRawOracleContractPartitionHistory(currentSource,path),previousSource);
  const previous=parseWorkflow(previousSource),current=parseWorkflow(currentSource);
  assert.equal(current.length,previous.length);
  const before=previous.find(row=>row.id==='package-checks'),after=current.find(row=>row.id==='package-checks');
  assert.equal(after.steps.length,before.steps.length+1);
  const old=before.steps.find(row=>row.name==='Run complete original link raw native oracle contracts');
  const plan=after.steps.find(row=>row.name===old.name);
  const schema=after.steps.find(row=>row.name==='Run complete original link raw schema and consumer contracts');
  assert.equal(plan.text.replace("\n          ORIGINAL_LINK_RAW_CONTRACT_PARTITION: 'plan'",''),old.text);
  assert.equal(schema.text.replace("\n          ORIGINAL_LINK_RAW_CONTRACT_PARTITION: 'schema'",'')
    .replace('Run complete original link raw schema and consumer contracts',old.name)
    .replace("matrix.group == 'source-model-raw-schema'","matrix.group == 'source-model-raw'"),old.text);
  assert.match(after.header,/^    timeout-minutes: 10$/mu);assert.match(after.header,/fail-fast: false/u);
  assert.equal(after.header.replace('          - group: source-model-raw-schema\n            generated_slice: first\n',''),before.header);
  for(const oldJob of previous){
    const actual=current.find(row=>row.id===oldJob.id);
    if(oldJob.id!=='package-checks')assert.deepEqual(actual,oldJob);
    else for(const step of oldJob.steps.filter(row=>row.name!==old.name))assert.equal(actual.steps.find(row=>row.name===step.name).text,step.text);
  }
  for(const value of [currentSource+'\n# changed\n',currentSource.replace('timeout-minutes: 10','timeout-minutes: 11')]){
    assert.equal(restoreRawOracleContractPartitionHistory(value,path),value);
  }
});
