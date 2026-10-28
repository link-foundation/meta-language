// Deferred actual machine execution and independent original source controls.
import assert from 'node:assert/strict';
import {createContext,runInContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {assertVersions,encodeValue} from './controls.mjs';
import {createOrdinaryHostContext,assertOrdinaryHostSnapshot} from './heap-controls.mjs';
import {beginJavaScriptCallMachine,stepJavaScriptCallMachine} from '../../js/src/translation/javascript-call-machine.js';
import {decodeJavaScriptCallProgram,encodeJavaScriptCallProgram,decodeJavaScriptOrdinaryHeap,encodeJavaScriptOrdinaryHeap,
 decodeJavaScriptSemanticValue,encodeJavaScriptSemanticValue,decodeJavaScriptCallMachine,encodeJavaScriptCallMachine,encodeJavaScriptCallMachineTransition} from './call-machine-codecs.mjs';
export {assertVersions};
export const machineFixture=readProjectData(new URL('../../parity/fixtures/javascript-call-machine.lino',import.meta.url));
function invocation(program,before,after){
 if(after.$!=='machineContinue')return null;
 const frame=before.frames.at(-1),fn=program.functions.find(fn=>fn.identity===frame.identity),instruction=fn.instructions[frame.position];
 const returns=frame.pending.$==='callIdle'&&(instruction===undefined||instruction.$==='return'),minimum=before.frames.length-(returns?1:0);
 if(after.machine.frames.length<=minimum)return null;
 assert.equal(after.machine.frames.length,minimum+1,'one actual function invocation per step');
 const child=after.machine.frames.at(-1),parent=after.machine.frames.at(-2);assert.ok(parent);
 let receiver=child.registers[0],pending=parent.pending;
 if(pending.$==='waitCall'||pending.$==='waitGet'||pending.$==='waitSet'){
  const owner=program.functions.find(fn=>fn.identity===parent.identity),instruction=owner.instructions[parent.position-1];
  receiver=parent.registers[instruction.receiver];
 }else if(pending.$==='waitKey')receiver=pending.state.receiver;
 else if(pending.$==='waitNumber'){assert.equal(pending.state.$,'numberProtocolState');receiver=pending.state.state.receiver;}
 else if(pending.$==='waitText'){assert.equal(pending.state.$,'textProtocolState');receiver=pending.state.state.receiver;}
 else if(pending.$==='waitDefinition')receiver=pending.state.conversion.receiver;
 else assert.fail('every child retains its actual suspended caller');
 return {callee:encodeJavaScriptSemanticValue({$:'object',identity:child.identity,callable:true}),receiver:encodeJavaScriptSemanticValue(receiver),arguments:child.arguments.map(encodeJavaScriptSemanticValue)};
}
export function observeCallMachine(row){
 const program=decodeJavaScriptCallProgram(row.program),heap=decodeJavaScriptOrdinaryHeap(row.heap),receiver=decodeJavaScriptSemanticValue(row.receiver),arguments_=row.arguments.map(decodeJavaScriptSemanticValue);
 assert.deepEqual(encodeJavaScriptCallProgram(program),row.program);assert.deepEqual(encodeJavaScriptOrdinaryHeap(heap),row.heap);
 let current=beginJavaScriptCallMachine(program,heap,BigInt(row.entry),receiver,arguments_);const trace=[encodeJavaScriptCallMachineTransition(current)],calls=[];
 for(let steps=0;current.$==='machineContinue';steps+=1){assert.ok(steps<2048,'complete finite control must not omit execution steps');const before=current.machine;current=stepJavaScriptCallMachine(program,before);const call=invocation(program,before,current);if(call)calls.push(call);trace.push(encodeJavaScriptCallMachineTransition(current));}
 const expected=row.expected;
 if(expected.kind==='resource'){
  assert.equal(current.$,'machineResourceLimit');assert.equal(current.reason,expected.reason);
  assert.equal(current.machine.frames.at(-1).position,expected.position);
  assert.deepEqual(encodeJavaScriptOrdinaryHeap(current.machine.heap),expected.heap??row.heap);
  if(expected.convertedValue){
   assert.equal(current.machine.frames.at(-1).pending.$,expected.pending??'waitNumber');
   assert.equal(current.machine.frames.at(-1).pending.state.$,expected.pending==='waitText'?'textPrimitiveValue':'numberPrimitiveValue');
   assert.deepEqual(encodeJavaScriptSemanticValue(current.machine.frames.at(-1).pending.state.value),expected.convertedValue);
   assert.deepEqual(encodeJavaScriptCallMachineTransition(stepJavaScriptCallMachine(program,current.machine)),trace.at(-1),'quota retry retains the completed primitive and never replays conversion calls');
  }else assert.deepEqual(trace.at(-1).machine,trace.at(-2).machine,'resource refusal keeps every actual frame, register, argument, handler and heap');
 }
 else if(expected.kind==='allocation'){assert.equal(current.$,'machineAllocateError');assert.equal(String(current.realm),expected.realm);assert.equal(current.errorClass,expected.errorClass);assert.equal(current.reason,expected.reason);}
 else{assert.equal(current.$,expected.kind==='throw'?'machineThrow':'machineCompleted');assert.deepEqual(encodeJavaScriptSemanticValue(current.value),expected.value);assert.deepEqual(encodeJavaScriptOrdinaryHeap(current.heap),expected.heap??row.heap);}
 const realms=new Map(row.program.realms.map(realm=>[realm,createContext()]));
 const sourceRealm=realms.get(row.program.functions.find(fn=>fn.identity===row.entry).realm);assert.ok(sourceRealm);
 let context;const implementations=new Map(row.callbacks.map(callback=>{
  const owner=row.program.functions.find(fn=>fn.identity===callback.identity);assert.ok(owner,'every original callback retains its actual executable realm owner');
  return [callback.identity,runInContext('('+callback.expression+')',realms.get(owner.realm))];
 }));
 context=createOrdinaryHostContext(heap,call=>{
  const factory=implementations.get(String(call.callee.identity));assert.ok(factory,'every original callback has independent code');
  const fn=factory(context);return Reflect.apply(fn,context.materialize(call.receiver),call.arguments.map(context.materialize));
 },new Map([[row.program.exoticKey.identity,runInContext('Symbol.toPrimitive',sourceRealm)]]));
 let result,failure,threw=false;try{result=runInContext('('+row.source+')',sourceRealm)(context,...arguments_.map(context.materialize));}catch(error){failure=error;threw=true;}
 const observedCalls=context.calls.map(call=>({callee:encodeValue(call.callee),receiver:encodeValue(call.receiver),arguments:call.arguments.map(encodeValue)}));
 assert.deepEqual(calls,observedCalls,'all original invocations retain order, callee, actual receiver and complete arguments');
 if(expected.kind==='allocation'){
  assert.equal(threw,true);assert.equal(failure.name,expected.errorClass);
  assert.equal(Object.getPrototypeOf(failure),runInContext(expected.errorClass,realms.get(expected.realm)).prototype,'the requested actual realm owns the intrinsic Error instance');
  assertOrdinaryHostSnapshot(heap,context);
 }
 else if(expected.kind==='throw'){assert.equal(threw,true);assert.deepEqual(encodeValue(context.semantic(failure)),expected.value);assertOrdinaryHostSnapshot(decodeJavaScriptOrdinaryHeap(expected.heap??row.heap),context);}
 else{assert.equal(threw,false);assert.deepEqual(encodeValue(context.semantic(result)),expected.kind==='resource'?expected.referenceValue:expected.value);assertOrdinaryHostSnapshot(decodeJavaScriptOrdinaryHeap(expected.heap??row.heap),context);}
 return {...row,trace,calls};
}
export function observeInvalidCallProgram(row){const program=decodeJavaScriptCallProgram(row.program),heap=decodeJavaScriptOrdinaryHeap(machineFixture.invalidHeap),actual=beginJavaScriptCallMachine(program,heap,10n,{$:'undefined'},[]);assert.deepEqual(actual,{$:'machineInvalid',heap,reason:row.reason});return {...row,result:encodeJavaScriptCallMachineTransition(actual)};}
export function observeInvalidCallMachine(row){
 const program=decodeJavaScriptCallProgram(row.program),machine=decodeJavaScriptCallMachine(row.machine),heap=decodeJavaScriptOrdinaryHeap(row.heap);
 assert.deepEqual(encodeJavaScriptCallProgram(program),row.program);assert.deepEqual(encodeJavaScriptCallMachine(machine),row.machine);assert.deepEqual(encodeJavaScriptOrdinaryHeap(machine.heap),row.heap);
 const actual=stepJavaScriptCallMachine(program,machine);assert.deepEqual(actual,{$:'machineInvalid',heap,reason:row.reason});
 return {...row,result:encodeJavaScriptCallMachineTransition(actual)};
}
