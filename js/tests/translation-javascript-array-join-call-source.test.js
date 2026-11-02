// Deferred source controls. Execute only after the full requirement draft.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileJavaScriptArrayJoinFunction,testJavaScriptArrayJoinOperation} from '../src/translation/javascript-array-join-call-source.js';
const object=identity=>({$:'object',identity,callable:false});
const roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)};
const compile=()=>compileJavaScriptArrayJoinFunction(7n,8n,roots);
const find=(code,operation)=>code.findIndex(row=>row.$==='primitive'&&row.operation===operation);

test('join requires actual primitive roots and boxes before reading length',()=>{
 assert.equal(testJavaScriptArrayJoinOperation('Array.join'),true);assert.equal(testJavaScriptArrayJoinOperation('Array.toString'),false);
 assert.equal(compileJavaScriptArrayJoinFunction(7n,8n,undefined),undefined);
 for(const field of ['objectPrototype','stringPrototype','numberPrototype','booleanPrototype','integerPrototype','symbolPrototype']){
  assert.equal(compileJavaScriptArrayJoinFunction(7n,8n,{...roots,[field]:{$:'null'}}),undefined);
  assert.equal(compileJavaScriptArrayJoinFunction(7n,8n,{...roots,[field]:{...roots[field],callable:true}}),undefined);
 }
 const fn=compile(),code=fn.instructions,wrapper=code.findIndex(row=>row.$==='primitiveWrapper'),length=code.findIndex(row=>row.$==='ordinaryGet');
 assert.equal(fn.receiverMode,'primitive intrinsic');assert.equal(code[0].operation,'require object coercible');assert.ok(wrapper<length);
 assert.deepEqual(code[wrapper],{$:'primitiveWrapper',destination:1,input:0,prototype:13});assert.equal(code[length].object,1);assert.equal(code[length].receiver,1);
 assert.ok(code.some(row=>row.$==='constant'&&row.value===roots.stringPrototype));
 const objectBranch=code.find(row=>row.$==='branch');assert.ok(objectBranch.yes>wrapper);
});

test('length is snapped before separator conversion even for empty joins',()=>{
 const code=compile().instructions,length=find(code,'read array like length'),separator=find(code,'convert primitive text'),enter=code.findIndex(row=>row.$==='arrayJoinEnter');
 assert.ok(length<separator&&separator<enter);assert.equal(code.filter(row=>row.$==='ordinaryGet'&&row.destination===2).length,1);
 const separatorBranch=code[separator-1];assert.equal(separatorBranch.$,'branch');assert.equal(code[separatorBranch.yes].$,'constant');assert.deepEqual(code[separatorBranch.yes].value,{$:'text',units:[44]});
 const empty=code[enter-1];assert.equal(empty.$,'branch');assert.equal(code[empty.yes].$,'return');assert.equal(empty.no,enter);
 assert.equal(code.some(row=>row.$==='primitive'&&row.operation==='construct Symbol descriptive text'),false);
});

test('join Gets every ascending index and skips ToString only for undefined and null',()=>{
 const code=compile().instructions,gets=code.filter(row=>row.$==='ordinaryGet');assert.equal(gets.length,2);
 assert.equal(code.some(row=>['ordinaryHas','iteratorProtocol','arrayAllocate','ordinarySet','argumentListCall','call'].includes(row.$)),false);
 const read=code.findIndex(row=>row.$==='ordinaryGet'&&row.destination===6);assert.equal(code[read-1].$,'propertyKey');assert.equal(code[read-1].input,4);assert.equal(code[read].receiver,1);
 const undefinedBranch=code[read+2],nullBranch=code[read+4];assert.equal(undefinedBranch.$,'branch');assert.equal(nullBranch.$,'branch');assert.equal(undefinedBranch.yes,nullBranch.yes);
 const advance=code[undefinedBranch.yes];assert.deepEqual(advance,{$:'numberBinary',destination:4,left:4,right:10,operator:'add'});
 assert.equal(code[nullBranch.no].operation,'convert primitive text');assert.equal(code[nullBranch.no].arguments[0],6);
 const back=code[undefinedBranch.yes+1];assert.equal(back.$,'jump');assert.deepEqual(code[back.target],{$:'numberBinary',destination:8,left:4,right:2,operator:'lessThan'});
});

test('cycle detection returns empty without releasing the enclosing owner token',()=>{
 const code=compile().instructions,enter=code.findIndex(row=>row.$==='arrayJoinEnter'),branch=code[enter+1],leave=code.findIndex(row=>row.$==='arrayJoinLeave');
 assert.equal(branch.condition,14);assert.equal(code[branch.no].$,'return');assert.equal(code[branch.no].source,5);assert.equal(branch.no,leave+1);
 assert.deepEqual(code[leave],{$:'arrayJoinLeave',receiver:1});
 for(const row of code)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(Number.isInteger(target)&&target>=0&&target<code.length);
});

import {beginJavaScriptCallMachine,runJavaScriptCallMachine} from '../src/translation/javascript-call-machine.js';
import {defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from '../src/translation/javascript-ordinary-heap.js';
const absent={$:'undefined'},number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
function fixture(){return {$:'ordinaryHeap',symbols:[99n],objects:[1n,2n,3n,4n,5n,6n,7n,9n,10n,11n].map(identity=>({$:'ordinaryObject',identity,prototype:{$:'null'},extensible:true,callable:identity===7n||identity===11n,properties:[]}))};}
function data(heap,identity,key,value){const next=defineJavaScriptOrdinaryProperty(heap,identity,text(key),createJavaScriptOrdinarySetDescriptor(value,true));assert.equal(next.success,true);return next.heap;}
function execute(heap,receiver,args=[],functions=[],identities=[]){
 const program={$:'callProgram',realms:[8n],exoticKey:{$:'symbol',identity:99n},functions:[compile(),...functions]},entry=beginJavaScriptCallMachine(program,heap,7n,receiver,args);
 assert.equal(entry.$,'machineContinue',entry.reason);return runJavaScriptCallMachine(program,entry.machine,4096,identities).transition;
}
test('portable join reads inherited values and nullish holes without skipping coordinates',()=>{
 let heap=fixture();heap={...heap,objects:heap.objects.map(row=>row.identity===9n?{...row,prototype:object(10n)}:row)};
 heap=data(heap,9n,'length',number(4));heap=data(heap,10n,'1',text('inherited'));heap=data(heap,9n,'2',{$:'null'});heap=data(heap,9n,'3',number(7));
 const result=execute(heap,object(9n));assert.equal(result.$,'machineCompleted');assert.deepEqual(result.value,text(',inherited,,7'));
});
test('portable join boxes primitive String with its actual indexed UTF16 slots',()=>{
 const result=execute(fixture(),text('\ud800A'),[text('|')],[],[12n]);assert.equal(result.$,'machineCompleted');assert.deepEqual(result.value,text('\ud800|A'));
});
test('separator conversion mutates the heap after the saved length and before index Gets',()=>{
 let heap=data(fixture(),9n,'length',number(2));heap=data(heap,9n,'0',text('a'));heap=data(heap,9n,'1',text('b'));heap=data(heap,10n,'toString',{$:'object',identity:11n,callable:true});
 const conversion={$:'callFunction',identity:11n,realm:8n,receiverMode:'strict',lexicalReceiver:absent,parameters:0,registers:6,captures:[],instructions:[{$:'constant',destination:1,value:object(9n)},{$:'constant',destination:2,value:text('length')},{$:'constant',destination:3,value:number(0)},{$:'ordinarySet',destination:4,object:1,key:2,value:3,receiver:1},{$:'constant',destination:2,value:text('1')},{$:'constant',destination:3,value:text('changed')},{$:'ordinarySet',destination:4,object:1,key:2,value:3,receiver:1},{$:'constant',destination:5,value:text('|')},{$:'return',source:5}]};
 const result=execute(heap,object(9n),[object(10n)],[conversion]);assert.equal(result.$,'machineCompleted');assert.deepEqual(result.value,text('a|changed'));
});
test('recursive generic receiver joins use the same active receiver guard',()=>{
 let heap=data(fixture(),9n,'length',number(2));heap=data(heap,9n,'0',object(9n));heap=data(heap,9n,'1',text('tail'));heap=data(heap,9n,'toString',{$:'object',identity:7n,callable:true});
 const first=execute(heap,object(9n));assert.equal(first.$,'machineCompleted');assert.deepEqual(first.value,text(',tail'));
 const second=execute(first.heap,object(9n));assert.equal(second.$,'machineCompleted');assert.deepEqual(second.value,text(',tail'));
});

import {stepJavaScriptCallMachine,readJavaScriptCallMachineError} from '../src/translation/javascript-call-machine.js';
test('join guard ownership cannot be removed from a suspended successful activation',()=>{
 const heap=data(fixture(),9n,'length',number(1)),program={$:'callProgram',realms:[8n],exoticKey:{$:'symbol',identity:99n},functions:[compile()]};let transition=beginJavaScriptCallMachine(program,heap,7n,object(9n),[]);
 for(let index=0;index<256&&transition.$==='machineContinue'&&transition.machine.frames.at(-1).activeJoin===undefined;index++)transition=stepJavaScriptCallMachine(program,transition.machine);
 assert.equal(transition.$,'machineContinue');const frame=transition.machine.frames.at(-1);assert.deepEqual(frame.activeJoin,object(9n));assert.equal(readJavaScriptCallMachineError(program,transition.machine),'');
 const {activeJoin:discarded,...forged}=frame;assert.notEqual(readJavaScriptCallMachineError(program,{...transition.machine,frames:[forged]}),'');
 const leave=compile().instructions.findIndex(row=>row.$==='arrayJoinLeave');assert.notEqual(readJavaScriptCallMachineError(program,{...transition.machine,frames:[{...frame,position:leave+1}]}),'');
});

test('different realm joins own independent guards for the same receiver',()=>{
 let heap=data(fixture(),9n,'length',number(2));heap=data(heap,9n,'0',object(9n));heap=data(heap,9n,'1',text('tail'));heap=data(heap,9n,'toString',{$:'object',identity:11n,callable:true});
 const program={$:'callProgram',realms:[8n,9n],exoticKey:{$:'symbol',identity:99n},functions:[compile(),compileJavaScriptArrayJoinFunction(11n,9n,roots)]};const entry=beginJavaScriptCallMachine(program,heap,7n,object(9n),[]),result=runJavaScriptCallMachine(program,entry.machine,4096,[]).transition;
 assert.equal(result.$,'machineCompleted',result.reason);assert.deepEqual(result.value,text(',tail,tail'));
});

test('selected V8 large length rejection precedes separator conversion',()=>{
 const heap=data(fixture(),9n,'length',number(4294967296)),result=execute(heap,object(9n),[{$:'symbol',identity:99n}]);assert.equal(result.$,'machineAllocateError');assert.equal(result.errorClass,'TypeError');assert.equal(result.reason,'Invalid array length');
 const code=compile().instructions;assert.ok(find(code,'require V8 array join length')<find(code,'convert primitive text'));
});
