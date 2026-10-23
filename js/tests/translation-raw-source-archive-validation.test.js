import assert from 'node:assert/strict';
import test from 'node:test';
import {runOriginalLinkRawArchiveValidationJobs,validateOriginalLinkRawSourceArchives} from '../../experiments/original-link-complete-table-projection/raw-source-archive-validation.mjs';

test('all available archive workers start before fresh validation and neither phase publishes early',{timeout:5000},async()=>{
 const sources=['complete current source','complete consuming source','complete borrowed source'];
 const workers=sources.map(()=>Promise.withResolvers()),validation=Promise.withResolvers(),ready=Promise.withResolvers();
 const events=[];let published=false;
 const pending=runOriginalLinkRawArchiveValidationJobs(sources,owned=>{
  assert.deepEqual(owned,sources);assert.equal(Object.isFrozen(owned),true);
  assert.deepEqual(events,['start 0','start 1','start 2']);events.push('validate');ready.resolve();return validation.promise;
 },(source,index)=>{assert.equal(source,sources[index]);events.push('start '+index);return workers[index].promise;},3).then(result=>{published=true;return result;});
 await ready.promise;assert.equal(published,false);
 workers[2].resolve('borrowed receipt');workers[0].resolve('current receipt');workers[1].resolve('consuming receipt');
 await Promise.resolve();assert.equal(published,false,'archive completion cannot authorize unvalidated candidate text');
 validation.resolve('actual full fresh source validation');
 assert.deepEqual(await pending,{sources,receipts:['current receipt','consuming receipt','borrowed receipt'],validated:'actual full fresh source validation'});
});

test('successful fresh validation still waits for every original archive exit',{timeout:5000},async()=>{
 const gate=Promise.withResolvers(),validated=Promise.withResolvers();let published=false;
 const pending=runOriginalLinkRawArchiveValidationJobs(['first','second'],()=>{validated.resolve();return'checked';},(source,index)=>index===0?gate.promise:Promise.resolve(source),2).then(result=>{published=true;return result;});
 await validated.promise;await Promise.resolve();assert.equal(published,false);
 gate.resolve('first');assert.deepEqual((await pending).receipts,['first','second']);
});

test('caller changes cannot substitute source snapshots during worker start or validation',async()=>{
 const sources=['first','second','third'],original=[...sources],seen=[];
 const result=await runOriginalLinkRawArchiveValidationJobs(sources,owned=>{
  assert.deepEqual(owned,original);assert.notEqual(owned,sources);assert.throws(()=>{owned[0]='replacement';});return'checked';
 },(source,index)=>{seen.push([index,source]);sources.fill('caller replacement');sources.length=1;return Promise.resolve(source);},3);
 assert.deepEqual(seen,original.map((source,index)=>[index,source]));assert.deepEqual(result.sources,original);
});

test('every later replay settles and both archive and compiler failures remain explicit',async()=>{
 const attempted=[],finished=[],archiveError=new Error('original archive failure'),validationError=new Error('fresh compiler source differs');
 await assert.rejects(runOriginalLinkRawArchiveValidationJobs(['first','second','third'],()=>{throw validationError;},async(source,index)=>{
  attempted.push(index);await Promise.resolve();finished.push(index);if(index===0)throw archiveError;return source;
 },2),error=>{
  assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,2);
  assert.ok(error.errors[0] instanceof AggregateError);assert.deepEqual(error.errors[0].errors,[archiveError]);assert.equal(error.errors[1],validationError);
  assert.deepEqual(attempted,[0,1,2]);assert.deepEqual(finished,[0,1,2]);return true;
 });
});

test('synchronous constructor failures cannot strand the barrier or skip subsequent sources',async()=>{
 const attempted=[],first=new Error('constructor failure');let validations=0;
 await assert.rejects(runOriginalLinkRawArchiveValidationJobs(['first','second','third'],()=>{validations++;return'checked';},(source,index)=>{
  attempted.push(index);if(index===0)throw first;return Promise.resolve(source);
 },3),error=>error instanceof AggregateError&&error.errors[0] instanceof AggregateError&&error.errors[0].errors[0]===first);
 assert.deepEqual(attempted,[0,1,2]);assert.equal(validations,1);
});

test('invalid source domains and non-CI entry refuse before invoking any worker or validator',async()=>{
 let calls=0;const callback=()=>{calls++;return'forbidden';};
 for(const sources of [[],new Array(2),['a',undefined],['\ud800'],['a'.repeat(1024*1024+1)],['a','b','c','d']]){
  await assert.rejects(runOriginalLinkRawArchiveValidationJobs(sources,callback,callback,3));
 }
 for(const capacity of [0,4,1.5,NaN])await assert.rejects(runOriginalLinkRawArchiveValidationJobs(['a'],callback,callback,capacity));
 for(const environment of [{},{CI:'false'}])await assert.rejects(validateOriginalLinkRawSourceArchives(null,callback,environment),/require CI=true/u);
 assert.equal(calls,0);
});
