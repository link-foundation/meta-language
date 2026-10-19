import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {CANCELLATION_INVESTIGATION_FIXTURE,CANCELLATION_INVESTIGATION_HEAD,checkWorkflowCancellationEvidence,classifyWorkflowCancellation} from '../scripts/ci-cancellation-investigation.mjs';
import {recordIssue195Observations} from './support/issue-195-observations.js';

const file=path=>new URL('../../'+path,import.meta.url),snapshot=readProjectData(file(CANCELLATION_INVESTIGATION_FIXTURE));
const source='parity/sources/ci-cancellation-investigation-2026-10-16.txt',sha256=value=>createHash('sha256').update(value).digest('hex');
const facts=()=>({run:structuredClone(snapshot.responses.run.data),job:structuredClone(snapshot.responses.job.data),checkRun:structuredClone(snapshot.responses.checkRun.data),annotations:structuredClone(snapshot.responses.annotations.data),workflow:structuredClone(snapshot.workflow)});
function sourceEdit(value,change){value.workflow.text=change(value.workflow.text);const bytes=Buffer.from(value.workflow.text,'utf8');value.workflow.bytes=bytes.length;value.workflow.sha256=sha256(bytes);value.workflow.blobSha=createHash('sha1').update(Buffer.from('blob '+bytes.length+'\0')).update(bytes).digest('hex');}
function refused(value){const result=classifyWorkflowCancellation(value);assert.equal(result.decision,'unknown');assert.equal(result.cause,null);assert.equal(result.cancelledBy,null);assert.ok(result.reasons.length>0);}

test('merged checkpoint annotations establish automatic concurrency cancellation while preserving attribution uncertainty',()=>{
 const capture=readProjectData(file('parity/fixtures/ci-concurrency-cancellation-2026-10-19.lino'));
 for(const response of Object.values(capture.responses)) {
  assert.ok(response.bytes<=262144);
  assert.equal(Buffer.byteLength(response.responseText),response.bytes);
  assert.equal(sha256(response.responseText),response.sha256);
  assert.deepEqual(JSON.parse(response.responseText),response.data);
 }
 const {mainRun,latePullRequestRun,mainJob,mainCheckRun,mainAnnotations}=Object.fromEntries(Object.entries(capture.responses).map(([key,row])=>[key,row.data]));
 assert.equal(mainRun.id,39221979958);assert.equal(mainRun.head_sha,'ce0101fb7d9f37a36652db04c4c3c86949d9e3c8');
 assert.equal(mainJob.id,117959180627);assert.equal(mainJob.run_id,mainRun.id);assert.equal(mainCheckRun.id,mainJob.id);
 assert.equal(mainJob.head_sha,mainRun.head_sha);assert.equal(mainCheckRun.head_sha,mainRun.head_sha);
 assert.equal(mainJob.check_run_url,mainCheckRun.url);assert.equal(mainCheckRun.output.annotations_count,mainAnnotations.length);
 for(const record of [mainRun,mainJob,mainCheckRun]){assert.equal(record.status,'completed');assert.equal(record.conclusion,'cancelled');}
 const messages=mainAnnotations.filter(row=>row.annotation_level==='failure').map(row=>row.message);
 assert.ok(messages.includes('Canceling since a higher priority waiting request for check-js-refs/heads/main-javascript-tests-grammar-native-1 exists'));
 assert.equal(latePullRequestRun.id,39222035006);assert.equal(latePullRequestRun.event,'pull_request');
 assert.ok(Date.parse(latePullRequestRun.created_at)>=Date.parse(mainJob.started_at));
 assert.ok(Date.parse(latePullRequestRun.created_at)<=Date.parse(mainJob.completed_at));
 assert.equal(capture.scope,'Actual GitHub cancellation annotations; the higher-priority run identity is not named by the annotation.');
});

test('the authentic cancellation capture and original instruction support the recorded automatic job deadline',()=>{
 const capture=readFileSync(file(CANCELLATION_INVESTIGATION_FIXTURE));assert.ok(capture.length<=1048576);assert.equal(capture.length,478834);assert.equal(sha256(capture),'be821ec6dd1e369e56d9fde72af42dcc4b5b257facff1214fd9b5b47dbe589c8');
 assert.equal(sha256(readFileSync(file(source))),'b72664a2ffee896d2843e75060f46ec1b73cd050db697ffea56b08fbad78c030');
 const instruction=readFileSync(file(source),'utf8'),reply=JSON.parse(instruction.slice(instruction.indexOf('\n')+1,instruction.lastIndexOf('\n')))[0];assert.equal(reply.answer,'No, investigate the workflow');assert.ok(reply.question.includes('38431880453'));
 const checked=checkWorkflowCancellationEvidence(snapshot);assert.deepEqual(checked.errors,[]);const result=checked.observation;assert.equal(result.decision,'supported');assert.equal(result.cause,'automatic job timeout');assert.equal(result.cancelledBy,null);assert.equal(result.runId,38431880453);assert.equal(result.jobId,115423871220);assert.equal(result.head,CANCELLATION_INVESTIGATION_HEAD);assert.equal(result.timeoutMinutes,10);assert.equal(result.elapsedSeconds,603);assert.equal(result.observedOverrunSeconds,3);assert.equal(result.annotation,'The job has exceeded the maximum execution time of 10m0s');
 assert.equal(snapshot.workflow.blobSha,'a7e91d0c01565d6f4a1a9e8cce7b307a62c4bca5');assert.equal(snapshot.workflow.bytes,36386);assert.equal(snapshot.workflow.sha256,'964b0860676ec424c2b5d32bba1bb3626be888f8a894f66be4347ba3258b14c3');assert.equal(result.headConfiguration.job,'test');assert.match(result.headConfiguration.scope,/historical head configuration only/u);assert.equal(result.effectiveMergeCheckout,null);
 // Normal observation writing is disabled without the evidence runner's env.
 for(const[fixtureId,fixtureFile]of[['conversation-contract:ciCancellationInvestigationRequest',source],['ci-cancellation-investigation',CANCELLATION_INVESTIGATION_FIXTURE]])recordIssue195Observations({requirementId:'I195-WORKFLOW-CANCELLED-CI-INVESTIGATION',runtime:'tooling',suffix:'behavior',fixtureId,fixtureFile,assertions:['cancelledWorkflowInvestigated','cancellationCauseSupportedByActualWorkflowEvidence'],testName:'the authentic cancellation capture and original instruction support the recorded automatic job deadline'});
});

test('actors and generic cancellation messages cannot establish a cancellation cause',()=>{
 refused({run:{actor:{login:'konard'},conclusion:'cancelled'}});
 for(const change of[value=>{value.annotations=[];},value=>{delete value.annotations;},value=>{value.annotations=value.annotations.filter(row=>!row.message.includes('maximum execution time'));},value=>{value.annotations[0].message='The operation was canceled.';},value=>{value.annotations[0].message='The job has exceeded the maximum execution time of 9m0s';},value=>{value.annotations[0].annotation_level='notice';},value=>{value.annotations[0].path='unrelated';},value=>{value.annotations.push({...value.annotations[0],message:'The job has exceeded the maximum execution time of 15m0s'});}]){const value=facts();change(value);refused(value);}
 const actors=facts();actors.run.actor={login:'unrelated actor'};actors.run.triggering_actor={login:'another actor'};assert.equal(classifyWorkflowCancellation(actors).cause,'automatic job timeout','actor metadata is not cancellation attribution');assert.equal(classifyWorkflowCancellation(actors).cancelledBy,null);
});

test('inconsistent run, job, check-run, head, URLs or annotation ownership remains unknown',()=>{
 const changes=[value=>{value.run.id++;},value=>{value.job.run_id++;},value=>{value.job.id++;},value=>{value.checkRun.id++;},value=>{value.run.head_sha='0'.repeat(40);},value=>{value.job.head_sha='0'.repeat(40);},value=>{value.checkRun.head_sha='0'.repeat(40);},value=>{value.workflow.revision='main';},value=>{value.job.check_run_url+='0';},value=>{value.checkRun.output.annotations_url+='0';},value=>{value.checkRun.check_suite.id++;},value=>{value.checkRun.name='another job';},value=>{value.job.conclusion='failure';},value=>{value.run.status='in_progress';},value=>{value.checkRun.output.annotations_count++;},value=>{delete value.workflow;}];
 for(const change of changes){const value=facts();change(value);refused(value);}
});

test('contradictory deadlines or timestamps cannot turn head source into effective merge configuration',()=>{
 const changes=[
  value=>{value.job.completed_at=value.job.started_at;value.checkRun.completed_at=value.job.started_at;},
  value=>{value.job.completed_at='2026-10-13T16:53:42Z';value.checkRun.completed_at=value.job.completed_at;},
  value=>{value.checkRun.started_at='2026-10-13T16:53:44Z';},value=>{value.run.updated_at='2026-10-13T17:03:45Z';},value=>{value.run.run_started_at='2026-10-13T17:00:00Z';},
  value=>{sourceEdit(value,text=>text.replace(/(  test:\n[\s\S]*?    timeout-minutes: )10/u,(_,prefix)=>prefix+'15'));},
  value=>{sourceEdit(value,text=>text.replace(/(  test:\n[\s\S]*?)    timeout-minutes: 10\n/u,'$1'));},
  value=>{value.workflow.effectiveMergeCheckout=value.run.head_sha;},value=>{value.workflow.scope='verified merge configuration';},
 ];for(const change of changes){const value=facts();change(value);refused(value);}
});

test('changed captured bytes, parsed records, provenance, or source blobs fail before causal classification',()=>{
 for(const key of Object.keys(snapshot.responses))for(const change of[
  row=>{row.responseText+='\n';},row=>{row.sha256='0'.repeat(64);},row=>{row.url+='0';},row=>{row.bytes++;},row=>{row.data={actor:{login:'konard'}};},
  row=>{row.responseText+='\n';row.bytes=Buffer.byteLength(row.responseText);row.sha256=sha256(row.responseText);},
 ]){const value=structuredClone(snapshot);change(value.responses[key]);const checked=checkWorkflowCancellationEvidence(value);assert.ok(checked.errors.length>0,key);assert.equal(checked.observation.decision,'unknown');assert.equal(checked.observation.cancelledBy,null);}
 for(const change of[value=>{delete value.responses.annotations;},value=>{value.workflow.text+='\n';},value=>{value.workflow.blobSha='0'.repeat(40);},value=>{value.capturedAt='unknown';},value=>{value.head='main';}]){const value=structuredClone(snapshot);change(value);const checked=checkWorkflowCancellationEvidence(value);assert.ok(checked.errors.length>0);assert.equal(checked.observation.decision,'unknown');}
});
