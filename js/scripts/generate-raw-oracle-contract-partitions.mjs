// Split only original contract registration and required CI scheduling.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {parseNative} from '../src/native-grammar-parser.js';
import {restoreCiTestPartitionWorkflow} from './ci-test-partition-history.mjs';
import {tokenize} from '../src/translation/lexer.js';

/** Derive complete original callbacks and exact reversible source changes. */
export function generateRawOracleContractPartitions({check=false}={}) {
  const root=new URL('../../',import.meta.url),revision='3a8cbfb2d87366702f7bc7a50182892d4ee1188a';
  const testPath='experiments/original-link-complete-table-projection/raw-native-observations.test.mjs';
  const original=captureGitArtifactCertificate(revision,testPath),source=original.blob.toString('utf8');
  const bytes=Buffer.from(source),tree=parseNative('native-javascript',source);assert.equal(tree.hasError,false);
  const nodes=[tree],tests=[];
  const gate=source.indexOf("if (process.env.CI === 'true') {");assert.ok(gate>0);
  for(let at=0;at<nodes.length;at+=1){
    const node=nodes[at];nodes.push(...node.children.map(row=>row.node));
    if(node.term!=='call_expression')continue;
    const fn=node.children.find(row=>row.field==='function')?.node;
    if(!fn||bytes.subarray(fn.start,fn.end).toString()!=='test')continue;
    const args=node.children.find(row=>row.field==='arguments')?.node;
    const title=args?.children.find(row=>row.node.term==='string')?.node;assert.ok(title);
    const literal=bytes.subarray(title.start,title.end).toString(),tokens=tokenize(literal,'JavaScript').tokens;
    const name=tokens[0].value;assert.equal(typeof name,'string');
    const partition=node.start<Buffer.byteLength(source.slice(0,gate))?'schema'
      :name==='CI executes the selected complete raw emission profile and persists only real observations'?'native':'plan';
    tests.push({name,partition,start:node.start,end:node.end,
      sha256:createHash('sha256').update(bytes.subarray(node.start,node.end)).digest('hex')});
  }
  tests.sort((first,second)=>first.start-second.start);
  assert.equal(tests.filter(row=>row.partition==='schema').length,11);
  assert.equal(tests.filter(row=>row.partition==='plan').length,7);
  assert.equal(tests.filter(row=>row.partition==='native').length,1);
  assert.equal(new Set(tests.map(row=>row.name)).size,19);
  const testsEdits=[
    {before:"import test from 'node:test';",after:"import registerRawContract from 'node:test';\nimport {createOriginalLinkRawContractPartition} from './raw-contract-partitions.mjs';\nconst rawContracts=createOriginalLinkRawContractPartition(registerRawContract);\nconst test=(...argumentsList)=>rawContracts.test(...argumentsList);"},
    {before:"if (process.env.CI === 'true') {",after:"if (process.env.CI === 'true' && rawContracts.runPlan) {"},
    {before:'\n}\n',after:'\n}\nrawContracts.finish();\n',last:true},
  ];
  const apply=(text,edits)=>{
    for(const edit of edits){
      if(edit.last){assert.ok(text.endsWith(edit.before));text=text.slice(0,-edit.before.length)+edit.after;}
      else {assert.equal(text.split(edit.before).length,2);text=text.replace(edit.before,()=>edit.after);}
    }return text;
  };
  const currentTest=apply(source,testsEdits);
  const workflowPath='.github/workflows/js.yml',workflow=captureGitArtifactCertificate(revision,workflowPath);
  const previousWorkflow=workflow.blob.toString('utf8');
  const name='      - name: Run complete original link raw native oracle contracts\n';
  const start=previousWorkflow.indexOf(name),end=previousWorkflow.indexOf('\n      - name:',start+1);assert.ok(start>0&&end>start);
  const step=previousWorkflow.slice(start,end);
  const flag="          ORIGINAL_LINK_RAW_SEPARATE_SUITES: '1'";assert.equal(step.split(flag).length,2);
  const plan=step.replace(flag,flag+"\n          ORIGINAL_LINK_RAW_CONTRACT_PARTITION: 'plan'");
  const schema=step.replace(name,'      - name: Run complete original link raw schema and consumer contracts\n')
    .replace("matrix.group == 'source-model-raw'","matrix.group == 'source-model-raw-schema'")
    .replace(flag,flag+"\n          ORIGINAL_LINK_RAW_CONTRACT_PARTITION: 'schema'");
  const cell='          - group: source-model-raw\n            generated_slice: first\n';
  const workflowEdits=[{before:cell,after:cell+'          - group: source-model-raw-schema\n            generated_slice: first\n'},
    {before:step,after:schema+'\n'+plan}];
  const currentWorkflow=apply(previousWorkflow,workflowEdits);
  const sources=[
    {path:testPath,source,certificate:original.certificate,edits:testsEdits,currentSource:currentTest},
    {path:workflowPath,source:previousWorkflow,certificate:workflow.certificate,edits:workflowEdits,currentSource:currentWorkflow},
  ].map(({currentSource,...row})=>{
    const file=new URL(row.path,root),actual=readFileSync(file,'utf8');
    if(check)assert.equal(row.path===workflowPath?restoreCiTestPartitionWorkflow(actual):actual,currentSource,row.path);
    else {assert.ok(actual===row.source||actual===currentSource,'unknown current source '+row.path);writeFileSync(file,currentSource);}
    return{...row,current:{bytes:Buffer.byteLength(currentSource),sha256:createHash('sha256').update(currentSource).digest('hex')}};
  });
  const fixture={schemaVersion:1,revision,testPath,tests,
    boundary:'Complete original test-call bodies, source order and content hashes. Eleven schema/consumer contracts and seven complete-plan contracts run exactly once across two required CI cells. The separately gated native contract retains its full default suite. No observation or skipped test is represented as evidence.'};
  const fixtureFile=new URL('parity/fixtures/raw-native-contract-partitions.lino',root);
  if(check)checkProjectData(fixtureFile,fixture);else writeProjectData(fixtureFile,fixture);
  const history={schemaVersion:1,revision,sources,
    boundary:'Complete original test and workflow byte inverses retain all callbacks, assertions, source archives, process/parse limits and deadlines. Exact source views authenticate historical guards only. Corrected CI timing and native execution remain unverified.'};
  const historyFile=new URL('parity/self-translation/raw-oracle-contract-partitions.lino',root);
  if(check)checkProjectData(historyFile,history);else writeProjectData(historyFile,history);
  const reader=new URL('./raw-oracle-contract-partition-history.mjs',import.meta.url),readerSource=readFileSync(reader,'utf8');
  const next=readerSource.replace(/const archiveHash='(?:pending|[a-f0-9]{64})';/u,
    "const archiveHash='"+createHash('sha256').update(readFileSync(historyFile)).digest('hex')+"';");
  if(check)assert.equal(readerSource,next);else writeFileSync(reader,next);
  return{fixture,history};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateRawOracleContractPartitions({check:process.argv.includes('--check')});
}
