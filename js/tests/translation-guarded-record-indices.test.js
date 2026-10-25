// Deferred actual-source branch scope and complete shared-policy controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {readJavaScriptOwnPropertyBranchEnvironments,forgetJavaScriptOwnPropertyGuard,checkJavaScriptGuardedRecordIndex} from '../src/translation/javascript-guarded-record-indices.js';
import {readJavaScriptOwnPropertyGuardArm,acceptJavaScriptOwnPropertyGuardBindings,acceptJavaScriptGuardedRecordIndex,readJavaScriptGuardedRecordProjectionPositions} from '../src/translation/javascript-own-property-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-guarded-record-indices.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('actual branch profiles retain all operation and binder guards',()=>{
 assert.equal(readJavaScriptOwnPropertyGuardArm('math','testJavaScriptOwnedProperty',false),1);assert.equal(readJavaScriptOwnPropertyGuardArm('math','testJavaScriptOwnedProperty',true),-1);assert.equal(readJavaScriptOwnPropertyGuardArm('app','testJavaScriptOwnedProperty',false),0);
 assert.equal(acceptJavaScriptOwnPropertyGuardBindings(2,false,true,true,'record','string',false),true);
 for(const count of [0,1,3])assert.equal(acceptJavaScriptOwnPropertyGuardBindings(count,false,true,true,'record','string',false),false);
 assert.equal(acceptJavaScriptOwnPropertyGuardBindings(2,false,true,true,'record','string',true),false);assert.equal(acceptJavaScriptOwnPropertyGuardBindings(2,false,true,true,'record','cell',false),false);
 assert.equal(acceptJavaScriptGuardedRecordIndex('record',false,'string',false),false);assert.deepEqual(readJavaScriptGuardedRecordProjectionPositions(3),[2,1,0]);
});
test('rebinding either actual source operand forgets a fact without editing the parent scope',()=>{
 const type={kind:'record',fields:[{name:'value',type:{kind:'float'}}]},env=new Map([['table',type],['key',{kind:'string'}]]);
 const name=value=>({k:'name',path:[value]}),condition={k:'math',op:'testJavaScriptOwnedProperty',args:[{value:name('table'),spread:false},{value:name('key'),spread:false}]};
 const selected=readJavaScriptOwnPropertyBranchEnvironments(condition,env);assert.notEqual(selected.then,env);assert.equal(selected.else,env);
 for(const rebound of ['table','key']){const copied=new Map(selected.then);forgetJavaScriptOwnPropertyGuard(copied,rebound);assert.throws(()=>checkJavaScriptGuardedRecordIndex({expr:()=>({k:'var',name:'key',type:{kind:'string'}})},{k:'index',object:name('table'),index:name('key')},{k:'var',name:'table',type},copied,[],undefined),error=>error.reason.startsWith('dynamic record index:'));}
});
