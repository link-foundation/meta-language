// Finite historical investigation. External text is archival provenance;
// decisions inspect its automatically parsed linked records and workflow source.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {readProjectData,serializeProjectData} from './project-data.mjs';

export const CANCELLATION_INVESTIGATION_FIXTURE='parity/fixtures/ci-cancellation-investigation.lino';
export const CANCELLATION_INVESTIGATION_HEAD='c9122d45aa25199925720dd09d406db1909a5ba7';
const repository='link-foundation/meta-language',api='https://api.github.com/repos/'+repository;
const runId=38431880453,jobId=115423871220,workflowPath='.github/workflows/js.yml';
const endpoints={run:`actions/runs/${runId}`,job:`actions/jobs/${jobId}`,checkRun:`check-runs/${jobId}`,annotations:`check-runs/${jobId}/annotations`,workflow:`contents/${workflowPath}?ref=${CANCELLATION_INVESTIGATION_HEAD}`};
const pinnedResponseHashes={
 run:'e95712faedf1367e7b05283f391a71254e84610b3f5bcd9e092a8e2e313889be',
 job:'a40e5d9bafabe5187e7c7d4192e4678d29fd74a5dac66694b88c21d0c594fdf8',
 checkRun:'40bccc22dcb3df1a6528876b890671435f44295bed89de3ec5f0ec4ecdba0513',
 annotations:'1a237fc697234d8d1e91b7dec2dac348e910da29b8f49e325de70c9310ff3aef',
 workflow:'dc1ac74ea873980f60e25ab999b75ea92ba10cfb6fc238f3ac002fee51e56d9a',
};
const maximumResponseBytes=262144,maximumFixtureBytes=1048576;
const sha256=value=>createHash('sha256').update(value).digest('hex');
const fixtureFile=new URL('../../'+CANCELLATION_INVESTIGATION_FIXTURE,import.meta.url);
const unknown=reason=>({decision:'unknown',cause:null,cancelledBy:null,reasons:[reason]});
function instant(value){assert.match(value,/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$/u);const result=Date.parse(value);assert.ok(Number.isFinite(result));assert.equal(new Date(result).toISOString().replace('.000Z','Z'),value);return result;}
function workflowSource(data){
 assert.equal(data.path,workflowPath);assert.equal(data.encoding,'base64');assert.equal(data.html_url,`https://github.com/${repository}/blob/${CANCELLATION_INVESTIGATION_HEAD}/${workflowPath}`);
 const compact=data.content.replace(/\s/gu,''),bytes=Buffer.from(compact,'base64');assert.equal(bytes.toString('base64'),compact);assert.equal(bytes.length,data.size);assert.ok(bytes.length<=maximumResponseBytes);
 const text=new TextDecoder('utf-8',{fatal:true}).decode(bytes);assert.equal(createHash('sha1').update(Buffer.from('blob '+bytes.length+'\0')).update(bytes).digest('hex'),data.sha);
 assert.equal(data.git_url,api+'/git/blobs/'+data.sha);
 return{revision:CANCELLATION_INVESTIGATION_HEAD,path:workflowPath,blobSha:data.sha,bytes:bytes.length,sha256:sha256(bytes),text,scope:'historical head configuration only; effective merge checkout unverified',effectiveMergeCheckout:null};
}
/** A consistent annotation establishes the recorded cause; actors establish none. */
export function classifyWorkflowCancellation({run,job,checkRun,annotations,workflow}={}){
 try{
  assert.ok(run&&job&&checkRun&&workflow&&Array.isArray(annotations),'complete run/job/check-run/annotation/config evidence is required');assert.ok(annotations.length>0&&annotations.length<=100);
  assert.equal(run.id,runId);assert.equal(job.id,jobId);assert.equal(job.run_id,run.id);assert.equal(checkRun.id,job.id);
  assert.equal(run.head_sha,CANCELLATION_INVESTIGATION_HEAD);assert.equal(job.head_sha,run.head_sha);assert.equal(checkRun.head_sha,run.head_sha);assert.equal(workflow.revision,run.head_sha);assert.equal(workflow.path,workflowPath);
  assert.equal(run.html_url,`https://github.com/${repository}/actions/runs/${runId}`);assert.equal(job.html_url,run.html_url+'/job/'+jobId);assert.equal(checkRun.details_url,job.html_url);assert.equal(job.check_run_url,api+'/check-runs/'+jobId);assert.equal(checkRun.output.annotations_url,job.check_run_url+'/annotations');assert.equal(checkRun.check_suite.id,run.check_suite_id);
  for(const value of[run,job,checkRun]){assert.equal(value.status,'completed');assert.equal(value.conclusion,'cancelled');}assert.equal(job.name,checkRun.name);assert.equal(checkRun.output.annotations_count,annotations.length);
  assert.equal(workflow.scope,'historical head configuration only; effective merge checkout unverified');assert.equal(workflow.effectiveMergeCheckout,null);assert.equal(sha256(Buffer.from(workflow.text,'utf8')),workflow.sha256);assert.equal(Buffer.byteLength(workflow.text),workflow.bytes);assert.equal(createHash('sha1').update(Buffer.from('blob '+workflow.bytes+'\0')).update(workflow.text).digest('hex'),workflow.blobSha);
  const jobs=parseWorkflow(workflow.text),selected=jobs.filter(row=>row.id==='test');assert.equal(selected.length,1,'the actual historical package job must be unambiguous');
  const names=[...selected[0].header.matchAll(/^ {4}name:\s*(.+)$/gmu)],timeouts=[...selected[0].header.matchAll(/^ {4}timeout-minutes:\s*(\d+)\s*$/gmu)],workflowNames=[...workflow.text.matchAll(/^name:\s*(.+)$/gmu)];assert.equal(names.length,1);assert.equal(timeouts.length,1);assert.equal(workflowNames.length,1);
  assert.equal(job.name,workflowNames[0][1]+' / '+names[0][1]);const timeoutMinutes=Number(timeouts[0][1]);assert.equal(timeoutMinutes,10,'this finite investigation concerns the recorded ten-minute package deadline');
  const deadlines=annotations.filter(row=>/maximum execution time/u.test(row.message??''));assert.equal(deadlines.length,1,'one actual timeout annotation is required');assert.equal(deadlines[0].message,'The job has exceeded the maximum execution time of 10m0s');assert.equal(deadlines[0].annotation_level,'failure');assert.equal(deadlines[0].path,'.github');
  assert.equal(job.started_at,checkRun.started_at);assert.equal(job.completed_at,checkRun.completed_at);const start=instant(job.started_at),end=instant(job.completed_at),runStart=instant(run.run_started_at),runEnd=instant(run.updated_at),elapsedSeconds=(end-start)/1000;assert.ok(runStart<=start&&start<=end&&end<=runEnd,'run and job timestamps must agree');assert.ok(elapsedSeconds>=timeoutMinutes*60,'the job must have reached the annotated deadline');
  return{decision:'supported',cause:'automatic job timeout',cancelledBy:null,runId,jobId,head:run.head_sha,timeoutMinutes,elapsedSeconds,observedOverrunSeconds:elapsedSeconds-timeoutMinutes*60,annotation:deadlines[0].message,headConfiguration:{path:workflow.path,revision:workflow.revision,blobSha:workflow.blobSha,sha256:workflow.sha256,job:'test',scope:workflow.scope},effectiveMergeCheckout:null};
 }catch(error){return unknown(error.message);}
}
/** Authentic frozen bytes are checked before any causal classification. */
export function checkWorkflowCancellationEvidence(snapshot){
 const errors=[];try{
  assert.equal(snapshot.version,1);assert.equal(snapshot.model,'cancelled CI run 38431880453 investigation 1');assert.equal(snapshot.repository,repository);assert.equal(snapshot.head,CANCELLATION_INVESTIGATION_HEAD);assert.ok(Number.isFinite(Date.parse(snapshot.capturedAt)));
  assert.deepEqual(Object.keys(snapshot.responses),Object.keys(endpoints));
  for(const[key,endpoint]of Object.entries(endpoints)){
   const response=snapshot.responses[key];assert.equal(response.url,api+'/'+endpoint);assert.equal(typeof response.responseText,'string');assert.ok(response.responseText.isWellFormed());const bytes=Buffer.from(response.responseText,'utf8');assert.ok(bytes.length<=maximumResponseBytes);assert.equal(response.bytes,bytes.length);assert.equal(response.sha256,sha256(bytes));assert.equal(response.sha256,pinnedResponseHashes[key],'response differs from the authentic frozen capture: '+key);assert.ok(Number.isFinite(Date.parse(response.capturedAt)));assert.deepEqual(JSON.parse(response.responseText),response.data,'linked records must equal the captured response bytes');
  }
  assert.deepEqual(snapshot.workflow,workflowSource(snapshot.responses.workflow.data));
 }catch(error){errors.push(error.message);}
 const observation=errors.length?unknown('captured evidence failed byte/provenance validation'):classifyWorkflowCancellation({run:snapshot.responses?.run?.data,job:snapshot.responses?.job?.data,checkRun:snapshot.responses?.checkRun?.data,annotations:snapshot.responses?.annotations?.data,workflow:snapshot.workflow});
 return{errors,observation};
}
/** Only an explicit maintainer action calls gh; PR checks read the frozen file. */
export function captureWorkflowCancellationEvidence(){
 const responses={};for(const[key,endpoint]of Object.entries(endpoints)){
  const responseText=execFileSync('gh',['api','repos/'+repository+'/'+endpoint],{encoding:'utf8',maxBuffer:maximumResponseBytes,timeout:30000}),bytes=Buffer.from(responseText,'utf8');responses[key]={url:api+'/'+endpoint,capturedAt:new Date().toISOString(),bytes:bytes.length,sha256:sha256(bytes),responseText,data:JSON.parse(responseText)};
 }
 return{version:1,model:'cancelled CI run 38431880453 investigation 1',repository,head:CANCELLATION_INVESTIGATION_HEAD,capturedAt:new Date().toISOString(),responses,workflow:workflowSource(responses.workflow.data)};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 if(process.argv.includes('--capture')){
  const snapshot=captureWorkflowCancellationEvidence(),text=serializeProjectData(snapshot,fileURLToPath(fixtureFile)),bytes=Buffer.byteLength(text);assert.ok(bytes<=maximumFixtureBytes,'the authentic capture exceeds the one-MiB fixture limit');writeFileSync(fixtureFile,text);console.log(JSON.stringify({fixture:CANCELLATION_INVESTIGATION_FIXTURE,bytes,sha256:sha256(text),responseHashes:Object.fromEntries(Object.entries(snapshot.responses).map(([key,value])=>[key,value.sha256])),workflow:{blobSha:snapshot.workflow.blobSha,sha256:snapshot.workflow.sha256,bytes:snapshot.workflow.bytes}}));
 }else{const checked=checkWorkflowCancellationEvidence(readProjectData(fixtureFile));assert.deepEqual(checked.errors,[]);assert.equal(checked.observation.decision,'supported');console.log(JSON.stringify(checked.observation));}
}
