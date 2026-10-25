// Deferred actual-source branch scope and complete shared-policy controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {readJavaScriptOwnPropertyBranchEnvironments,forgetJavaScriptOwnPropertyGuard,checkJavaScriptGuardedRecordIndex,createJavaScriptOwnKeyCallbackEnvironment,installJavaScriptOwnKeyCallbackBindings} from '../src/translation/javascript-guarded-record-indices.js';
import {readJavaScriptOwnPropertyGuardArm,acceptJavaScriptOwnPropertyGuardBindings,acceptJavaScriptGuardedRecordIndex,readJavaScriptGuardedRecordProjectionPositions,readJavaScriptOwnKeyCallbackPosition,readJavaScriptOwnKeyCallbackReceiverPosition,acceptJavaScriptOwnKeyCallbackSource,acceptJavaScriptOwnKeyCallbackBinding} from '../src/translation/javascript-own-property-rules.js';
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
test('own-key membership follows complete shared callback roles and source guards',()=>{
 for(const operation of ['map','filter','some','every','find','findIndex','flatMap','forEach']){
  assert.equal(readJavaScriptOwnKeyCallbackPosition(operation),0);
  assert.equal(readJavaScriptOwnKeyCallbackReceiverPosition(operation),2);
 }
 assert.equal(readJavaScriptOwnKeyCallbackPosition('reduce'),1);
 assert.equal(readJavaScriptOwnKeyCallbackReceiverPosition('reduce'),3);
 assert.equal(readJavaScriptOwnKeyCallbackPosition('sort'),-1);
 assert.equal(readJavaScriptOwnKeyCallbackReceiverPosition('sort'),-1);
 assert.equal(acceptJavaScriptOwnKeyCallbackSource('math','enumerateJavaScriptObjectKeys',1,false,true,'record',false,'lambda'),true);
 for(const [expression,operation,count,spread,local,owner,open,callback] of [
  ['app','enumerateJavaScriptObjectKeys',1,false,true,'record',false,'lambda'],
  ['math','enumerateJavaScriptObjectValues',1,false,true,'record',false,'lambda'],
  ['math','enumerateJavaScriptObjectKeys',2,false,true,'record',false,'lambda'],
  ['math','enumerateJavaScriptObjectKeys',1,true,true,'record',false,'lambda'],
  ['math','enumerateJavaScriptObjectKeys',1,false,false,'record',false,'lambda'],
  ['math','enumerateJavaScriptObjectKeys',1,false,true,'record',true,'lambda'],
  ['math','enumerateJavaScriptObjectKeys',1,false,true,'record',false,'name'],
 ])assert.equal(acceptJavaScriptOwnKeyCallbackSource(expression,operation,count,spread,local,owner,open,callback),false);
 assert.equal(acceptJavaScriptOwnKeyCallbackBinding(false,false,'string'),true);
 for(const args of [[true,false,'string'],[false,true,'string'],[false,false,'cell']])assert.equal(acceptJavaScriptOwnKeyCallbackBinding(...args),false);
});
test('only the actual callback node installs a fact for its actual immutable key binder',()=>{
 const type={kind:'record',fields:[{name:'value',type:{kind:'float'}}]},text={kind:'string'},number={kind:'float'};
 const env=new Map([['table',type]]),name=value=>({k:'name',path:[value]});
 const receiver={k:'math',op:'enumerateJavaScriptObjectKeys',args:[{value:name('table'),spread:false}]};
 const callback={k:'lambda'},parameters=[{name:'key',type:text}];let ordinal=0;
 const checker={expr:node=>({k:'var',name:node.path[0],type:text}),freshName:prefix=>prefix+ordinal++};
 const read=scope=>checkJavaScriptGuardedRecordIndex(checker,{k:'index',object:name('table'),index:name('key')},{k:'var',name:'table',type},scope,[],undefined);
 const selected=createJavaScriptOwnKeyCallbackEnvironment(receiver,callback,'map',env);
 const install=(node,params)=>{const scope=new Map(selected);installJavaScriptOwnKeyCallbackBindings(node,params,scope);return scope;};
 assert.equal(read(install(callback,parameters)).k,'let');
 for(const scope of [env,install({...callback},parameters),install(callback,[]),install(callback,[...parameters,{name:'table',type:number}]),install(callback,[...parameters,{name:'index',type:number},{name:'keys',type:{kind:'array',element:text}}])])assert.throws(()=>read(scope),error=>error.reason.startsWith('dynamic record index:'));
 const reduce=createJavaScriptOwnKeyCallbackEnvironment(receiver,callback,'reduce',env);
 installJavaScriptOwnKeyCallbackBindings(callback,[{name:'total',type:number},...parameters,{name:'index',type:number}],reduce);
 assert.equal(read(reduce).k,'let');
 for(const rebound of ['table','key']){const scope=install(callback,parameters);forgetJavaScriptOwnPropertyGuard(scope,rebound);assert.throws(()=>read(scope),error=>error.reason.startsWith('dynamic record index:'));}
});
