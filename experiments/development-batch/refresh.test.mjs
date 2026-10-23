import assert from 'node:assert/strict';
import test from 'node:test';
import {readDevelopmentAuthoringPlan,developmentAuthoringEnvironment,runDevelopmentAuthoringPhase} from './refresh.mjs';

test('authoring phases refuse changed commands before invoking any process',()=>{
 for(const change of [plan=>{plan.phases[1].stages.reverse();},plan=>{plan.phases[2].stages.reverse();},plan=>{plan.phases[3].stages.reverse();},plan=>{plan.phases.pop();},plan=>{plan.phases[1].stages[0].script='cargo';},plan=>{plan.phases[0].stages[0].args.push('--native');}]){
  const plan=readDevelopmentAuthoringPlan();change(plan);let calls=0;
  assert.throws(()=>runDevelopmentAuthoringPhase(plan,'history',{execute:()=>{calls++;return{status:0,signal:null};}}));
  assert.equal(calls,0);
 }
 assert.throws(()=>runDevelopmentAuthoringPhase(readDevelopmentAuthoringPlan(),'unknown'),/unknown authoring phase/u);
});

test('final verification checks live sources first and preserves every read-only command and resource bound',()=>{
 const commands=[];
 for(const check of [false,true]){
  const calls=[];
  runDevelopmentAuthoringPhase(readDevelopmentAuthoringPlan(),'verification',{check,execute:(command,args,options)=>{
   calls.push({command,args,options});return{status:0,signal:null};
  }});
  assert.equal(calls.length,17);
  assert.deepEqual(calls[0].args.slice(4),['js/scripts/check-issue-195-sources.mjs','--online']);
  assert.equal(calls[2].args[4],'js/scripts/check-issue-195-acceptance.mjs');
  assert.deepEqual(calls.at(-1).args.slice(4),['experiments/grammar-outcome-network/native-transport/inverse/generate-relative-character-self-translation.mjs','--check']);
  assert.ok(calls.every(call=>call.command===process.execPath&&call.options.timeout===80000&&call.options.killSignal==='SIGKILL'));
  assert.ok(calls.every(call=>call.args.includes('--max-old-space-size=352')));
  assert.ok(calls.every(call=>!call.args.some(arg=>['--refresh-manifest','--refresh-declarations','--update','--run'].includes(arg))));
  commands.push(calls.map(call=>call.args));
 }
 assert.deepEqual(commands[0],commands[1],'verification already selects complete check-only arguments');
});

test('authoring failure retains its receipt and stops dependent processes',()=>{
 const calls=[],receipts=[];
 assert.throws(()=>runDevelopmentAuthoringPhase(readDevelopmentAuthoringPlan(),'history',{execute:(command,args,options)=>{
  calls.push({command,args,options});return{status:calls.length===2?7:0,signal:null};
 },record:rows=>receipts.push(rows)}),/author failed/u);
 assert.equal(calls.length,2);assert.equal(receipts.length,2);
 assert.equal(receipts[0].length,1);assert.equal(receipts[1].length,2);
 assert.equal(receipts[1][1].exitCode,7);
 assert.ok(calls.every(call=>call.command===process.execPath&&call.options.timeout===80000&&call.options.killSignal==='SIGKILL'));
 assert.ok(calls.every(call=>call.args.includes('--max-old-space-size=352')));
});

test('authoring clears inherited target directories and native test switches',()=>{
 const original={PATH:'/retained/tools',CARGO_TARGET_DIR:'/unrelated/build',CARGO_BUILD_TARGET_DIR:'/other/build',NODE_OPTIONS:'--max-old-space-size=8192',CI:'true',TRANSLATION_CHECK_RUST:'1',SELF_TRANSLATION_CHECK_RUST:'1'};
 const selected=developmentAuthoringEnvironment(original);
 for(const key of ['CARGO_TARGET_DIR','CARGO_BUILD_TARGET_DIR','NODE_OPTIONS','CI'])assert.equal(Object.hasOwn(selected,key),false);
 assert.equal(selected.PATH,original.PATH);assert.equal(selected.TRANSLATION_CHECK_RUST,'0');assert.equal(selected.SELF_TRANSLATION_CHECK_RUST,'0');
 assert.equal(original.CARGO_TARGET_DIR,'/unrelated/build');
 const calls=[];
 runDevelopmentAuthoringPhase(readDevelopmentAuthoringPlan(),'naming',{check:true,environment:original,execute:(command,args,options)=>{calls.push({command,args,options});return{status:0,signal:null};}});
 assert.equal(calls.length,2);assert.ok(calls[0].args.includes('js/scripts/build-concept-records.mjs'));assert.equal(calls[0].args.at(-1),'--check');assert.deepEqual(calls[1].args.slice(-2),['--additional-inventories','--check']);
 for(const call of calls)assert.deepEqual(call.options.env,selected);
});
