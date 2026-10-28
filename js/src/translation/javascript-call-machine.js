import {readJavaScriptCallPrimitiveArityError,executeJavaScriptCallPrimitive} from './javascript-call-primitives.js';
import {testJavaScriptCallCodeCoordinate,findJavaScriptCallCodeFunction,readJavaScriptCallCodeRegisters,testJavaScriptCallStrictEqual} from './javascript-call-code.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,planJavaScriptOrdinaryGet,planJavaScriptOrdinarySet} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
import {beginJavaScriptPropertyDefinition,stepJavaScriptPropertyDefinition,resumeJavaScriptPropertyDefinition} from './javascript-property-definition.js';
import {beginJavaScriptPropertyKeyConversion,stepJavaScriptPropertyKeyConversion,resumeJavaScriptPropertyKeyConversion} from './javascript-property-key-conversion.js';

/** @typedef {{$:'callHandler',target:number,destination:number}} JavaScriptCallHandler */
/** @typedef {{$:'callIdle'} | {$:'waitCall',destination:number} |
 * {$:'waitGet',destination:number} | {$:'waitSet',destination:number} |
 * {$:'waitKey',destination:number,realm:bigint,state:JavaScriptPrimitiveConversionState} |
 * {$:'waitDefinition',destination:number,realm:bigint,state:JavaScriptPropertyDefinitionState} |
 * {$:'waitError',realm:bigint,errorClass:string,reason:string}} JavaScriptCallPending
 */
/** @typedef {{$:'callFrame',identity:bigint,position:number,registers:JavaScriptSemanticValue[],
 * arguments:JavaScriptSemanticValue[],handlers:JavaScriptCallHandler[],pending:JavaScriptCallPending}} JavaScriptCallFrame
 */
/** @typedef {{$:'callMachine',heap:JavaScriptOrdinaryHeap,frames:JavaScriptCallFrame[]}} JavaScriptCallMachine */
/** @typedef {{$:'machineContinue',machine:JavaScriptCallMachine} |
 * {$:'machineCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'machineThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'machineAllocateError',machine:JavaScriptCallMachine,realm:bigint,errorClass:string,reason:string} |
 * {$:'machineInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'machineResourceLimit',machine:JavaScriptCallMachine,reason:string}} JavaScriptCallMachineTransition
 */

/** @param {number} value @param {number} limit @returns {boolean} */
export function testJavaScriptCallCoordinate(value,limit){return testJavaScriptCallCodeCoordinate(value,limit);}
/** @param {JavaScriptCallProgram} program @param {bigint} identity @returns {number} */
export function findJavaScriptCallFunction(program,identity){return findJavaScriptCallCodeFunction(program,identity);}
/** @param {JavaScriptCallInstruction} instruction @returns {number[]} */
export function readJavaScriptCallRegisters(instruction){return readJavaScriptCallCodeRegisters(instruction);}

/** Validate actual code/identity facts, without executing any instruction.
 * @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @returns {string} */
export function readJavaScriptCallProgramError(program,heap){
  const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
  if(program.functions.length>4096||program.realms.length>4096)return 'call code directories exceed their explicit resource budget';
  let previous=0n;
  for(const realm of program.realms){if(realm<=previous||realm>18446744073709551615n)return 'call realms require ordered unique positive unsigned64 identities';previous=realm;}
  if(program.exoticKey.$!=='symbol')return 'call code requires the authoritative well-known primitive conversion Symbol';
  const keyError=readJavaScriptOrdinaryValueError(heap,program.exoticKey);if(keyError!=='')return keyError;
  previous=0n;let count=0;
  for(const fn of program.functions){
    if(fn.identity<=previous||fn.identity>18446744073709551615n)return 'call functions require ordered unique positive unsigned64 identities';previous=fn.identity;
    if(!program.realms.includes(fn.realm))return 'call functions require their actual declared source realm';
    const callable=readJavaScriptOrdinaryValueError(heap,{$:'object',identity:fn.identity,callable:true});if(callable!=='')return callable;
    if(fn.receiverMode!=='strict'&&fn.receiverMode!=='lexical')return 'call code requires explicit strict or lexical receiver binding';
    if(readJavaScriptOrdinaryValueError(heap,fn.lexicalReceiver)!=='')return 'lexical receivers belong to the latest global context';
    if(!testJavaScriptCallCoordinate(fn.registers,4097)||fn.registers===0||!testJavaScriptCallCoordinate(fn.parameters,fn.registers)||fn.parameters+fn.captures.length+1>fn.registers)return 'call function register and parameter coordinates must be complete';
    for(const capture of fn.captures)if(readJavaScriptOrdinaryValueError(heap,capture)!=='')return 'call captures belong to the latest global context';
    count+=fn.instructions.length;if(count>65536||fn.instructions.length===0)return 'call code exceeds its explicit instruction budget or has no entry';
    for(const instruction of fn.instructions){
      for(const register of readJavaScriptCallRegisters(instruction))if(!testJavaScriptCallCoordinate(register,fn.registers))return 'call instructions require actual in-range register coordinates';
      switch(instruction.$){
        case 'jump':if(!testJavaScriptCallCoordinate(instruction.target,fn.instructions.length))return 'call jumps require actual instruction targets';break;
        case 'branch':if(!testJavaScriptCallCoordinate(instruction.yes,fn.instructions.length)||!testJavaScriptCallCoordinate(instruction.no,fn.instructions.length))return 'call branches require both actual instruction targets';break;
        case 'pushHandler':if(!testJavaScriptCallCoordinate(instruction.target,fn.instructions.length))return 'call handlers require actual instruction targets';break;
        case 'argument':if(!testJavaScriptCallCoordinate(instruction.index,1048576))return 'call argument reads require exact bounded coordinates';break;
        case 'constant':if(readJavaScriptOrdinaryValueError(heap,instruction.value)!=='')return 'call constants belong to the latest global context';break;
        case 'primitive':{const error=readJavaScriptCallPrimitiveArityError(instruction.operation,instruction.arguments.length);if(error!=='')return error;break;}
        case 'numberBinary':if(!['add','subtract','multiply','divide','remainder','power','lessThan','equal'].includes(instruction.operator))return 'call number instructions require an actual registered operation';break;
        case 'propertyKey':if(!program.realms.includes(instruction.realm))return 'a property key operation retains its actual intrinsic realm';break;
        case 'propertyDefinition':if(!program.realms.includes(instruction.realm))return 'a property definition retains its actual intrinsic realm';break;
        default:break;
      }
    }
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallFrame[]} frames @param {JavaScriptCallFrame} frame @returns {JavaScriptCallMachine} */
export function replaceJavaScriptCallFrame(heap,frames,frame){return {$:'callMachine',heap,frames:frames.map((entry,index)=>index===frames.length-1?frame:entry)};}
/** @param {JavaScriptCallFrame} frame @param {number} destination @param {JavaScriptSemanticValue} value @returns {JavaScriptCallFrame} */
export function writeJavaScriptCallRegister(frame,destination,value){return {...frame,registers:frame.registers.map((entry,index)=>index===destination?value:entry),pending:{$:'callIdle'}};}
/** @param {JavaScriptCallFunction} fn @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptCallFrame} */
export function createJavaScriptCallFrame(fn,receiver,arguments_){
  let registers=[];
  for(let index=0;index<fn.registers;index+=1)registers.push({$:'undefined'});
  const bound=fn.receiverMode==='lexical'?fn.lexicalReceiver:receiver;
  registers=registers.map((entry,index)=>index===0?bound:entry);
  for(let index=0;index<fn.parameters;index+=1){const value=arguments_[index]??{$:'undefined'},slot=index+1;registers=registers.map((entry,position)=>position===slot?value:entry);}
  for(let index=0;index<fn.captures.length;index+=1){const value=fn.captures[index]??{$:'undefined'},slot=fn.parameters+1+index;registers=registers.map((entry,position)=>position===slot?value:entry);}
  return {$:'callFrame',identity:fn.identity,position:0,registers,arguments:arguments_,handlers:[],pending:{$:'callIdle'}};
}
/** Validate resumable machine state, never treat it as an execution certificate.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @returns {string} */
export function readJavaScriptCallMachineError(program,machine){
  const error=readJavaScriptCallProgramError(program,machine.heap);if(error!=='')return error;
  if(machine.frames.length===0||machine.frames.length>1024)return 'a call machine requires its complete bounded active stack';
  let slots=0,argumentSlots=0;
  for(let index=0;index<machine.frames.length;index+=1){
    const frame=machine.frames[index];if(frame===undefined)return 'call stacks must be dense';
    const fn=program.functions[findJavaScriptCallFunction(program,frame.identity)];if(fn===undefined)return 'every call frame retains its actual registered function';
    if(frame.registers.length!==fn.registers||!testJavaScriptCallCoordinate(frame.position,fn.instructions.length+1))return 'call frames retain their complete register and instruction coordinates';
    slots+=frame.registers.length;argumentSlots+=frame.arguments.length;if(slots>1048576||argumentSlots>1048576||frame.handlers.length>1024)return 'call frame storage exceeds its explicit resource budget';
    for(const value of frame.registers)if(readJavaScriptOrdinaryValueError(machine.heap,value)!=='')return 'call registers belong to the latest global context';
    for(const value of frame.arguments)if(readJavaScriptOrdinaryValueError(machine.heap,value)!=='')return 'call arguments belong to the latest global context';
    for(const handler of frame.handlers)if(!testJavaScriptCallCoordinate(handler.destination,fn.registers)||!testJavaScriptCallCoordinate(handler.target,fn.instructions.length))return 'call handlers retain their complete register and instruction coordinates';
    const pending=frame.pending;
    switch(pending.$){
      case 'waitCall':case 'waitGet':case 'waitSet':case 'waitKey':case 'waitDefinition':
        if(!testJavaScriptCallCoordinate(pending.destination,fn.registers))return 'pending calls retain their actual destination register';
        break;
      case 'callIdle':case 'waitError':break;
    }
    switch(pending.$){
      case 'waitError':if(!program.realms.includes(pending.realm))return 'pending Error allocation retains its actual intrinsic realm';break;
      case 'waitKey':if(!program.realms.includes(pending.realm))return 'pending key conversion retains its actual intrinsic realm';break;
      case 'waitDefinition':if(!program.realms.includes(pending.realm))return 'pending property definition retains its actual intrinsic realm';break;
      default:break;
    }
    if(index<machine.frames.length-1&&(pending.$==='callIdle'||pending.$==='waitError'))return 'a suspended parent frame retains its actual pending call';
  }
  return '';
}

/** @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function throwJavaScriptCallValue(machine,value){
  let frames=machine.frames;
  while(frames.length>0){
    const frame=frames[frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'call stacks must be dense'};
    const handler=frame.handlers[frame.handlers.length-1];
    if(handler!==undefined){const restored={...writeJavaScriptCallRegister(frame,handler.destination,value),position:handler.target,handlers:frame.handlers.slice(0,-1)};return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,restored)};}
    frames=frames.slice(0,-1);
  }
  return {$:'machineThrow',heap:machine.heap,value};
}
/** @param {JavaScriptCallMachine} machine @param {bigint} realm @param {string} errorClass @param {string} reason @returns {JavaScriptCallMachineTransition} */
export function requestJavaScriptCallError(machine,realm,errorClass,reason){
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error allocation requires its actual active frame'};
  const next=replaceJavaScriptCallFrame(machine.heap,machine.frames,{...frame,pending:{$:'waitError',realm,errorClass,reason}});
  return {$:'machineAllocateError',machine:next,realm,errorClass,reason};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_
 * @returns {JavaScriptCallMachineTransition} */
export function pushJavaScriptCall(program,machine,callee,receiver,arguments_){
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a nested call needs its actual suspended parent'};
  const owner=program.functions[findJavaScriptCallFunction(program,frame.identity)];if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a call frame needs its registered source owner'};
  if(callee.$!=='object')return requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected callee is not callable');
  if(!callee.callable)return requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected callee is not callable');
  const fn=program.functions[findJavaScriptCallFunction(program,callee.identity)];if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'the actual callable needs its registered executable definition'};
  let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;}
  if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'call stack storage exceeds the selected execution resource budget'};
  return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([createJavaScriptCallFrame(fn,receiver,arguments_)])}};
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptPropertyKeyConversionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallKey(program,machine,destination,realm,transition){
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'key conversion needs its actual active frame'};
  switch(transition.$){
    case 'propertyKeyContinuation':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...frame,pending:{$:'waitKey',destination,realm,state:transition.state}})};
    case 'propertyKeyCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...frame,pending:{$:'waitKey',destination,realm,state:transition.state}}),transition.callee,transition.receiver,transition.arguments);
    case 'propertyKeyCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.key))};
    case 'propertyKeyThrow':return throwJavaScriptCallValue(machine,transition.value);
    case 'propertyKeyIntrinsicError':return requestJavaScriptCallError(machine,realm,transition.errorClass,transition.reason);
    case 'propertyKeyInvalid':return {$:'machineInvalid',heap:machine.heap,reason:transition.reason};
  }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptPropertyDefinitionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallDefinition(program,machine,destination,realm,transition){
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'property definition needs its actual active frame'};
  switch(transition.$){
    case 'definitionContinuation':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...frame,pending:{$:'waitDefinition',destination,realm,state:transition.state}})};
    case 'definitionCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...frame,pending:{$:'waitDefinition',destination,realm,state:transition.state}}),transition.callee,transition.receiver,transition.arguments);
    case 'definitionCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
    case 'definitionThrow':return throwJavaScriptCallValue(machine,transition.value);
    case 'definitionIntrinsicError':return requestJavaScriptCallError(machine,realm,transition.errorClass,transition.reason);
    case 'definitionInvalid':return {$:'machineInvalid',heap:machine.heap,reason:transition.reason};
  }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function returnJavaScriptCallValue(program,machine,value){
  const frames=machine.frames.slice(0,-1),parent=frames[frames.length-1];
  if(parent===undefined)return {$:'machineCompleted',heap:machine.heap,value};
  const current={$:'callMachine',heap:machine.heap,frames},pending=parent.pending;
  switch(pending.$){
    case 'waitCall':case 'waitGet':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,writeJavaScriptCallRegister(parent,pending.destination,value))};
    case 'waitSet':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,writeJavaScriptCallRegister(parent,pending.destination,{$:'boolean',value:true}))};
    case 'waitKey':return acceptJavaScriptCallKey(program,current,pending.destination,pending.realm,resumeJavaScriptPropertyKeyConversion(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitDefinition':return acceptJavaScriptCallDefinition(program,current,pending.destination,pending.realm,resumeJavaScriptPropertyDefinition(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'callIdle':case 'waitError':return {$:'machineInvalid',heap:machine.heap,reason:'a returning call needs its actual suspended parent continuation'};
  }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction
 * @returns {JavaScriptCallMachineTransition} */
export function executeJavaScriptOrdinaryCallProperty(program,machine,frame,instruction){
  const next={...frame,position:frame.position+1},registers=frame.registers;
  switch(instruction.$){
    case 'ordinaryGet':{
      const object=registers[instruction.object]??{$:'undefined'};if(object.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary Get instructions require their actual Object operand'};
      const plan=planJavaScriptOrdinaryGet(machine.heap,object.identity,registers[instruction.key]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'});
      if(plan.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:plan.error};
      if(plan.action==='value')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,plan.value))};
      if(plan.action==='call')return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitGet',destination:instruction.destination}}),plan.callee,plan.receiver,plan.arguments);
      return {$:'machineInvalid',heap:machine.heap,reason:'ordinary Get needs its actual value or pending getter'};
    }
    case 'ordinarySet':{
      const object=registers[instruction.object]??{$:'undefined'};if(object.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary Set instructions require their actual Object operand'};
      const plan=planJavaScriptOrdinarySet(machine.heap,object.identity,registers[instruction.key]??{$:'undefined'},registers[instruction.value]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'});
      if(plan.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:plan.error};
      if(plan.action==='call')return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitSet',destination:instruction.destination}}),plan.callee,plan.receiver,plan.arguments);
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(plan.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:plan.success}))};
    }
    case 'requireWrite':{
      const success=registers[instruction.success]??{$:'undefined'},owner=program.functions[findJavaScriptCallFunction(program,frame.identity)];
      if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a strict write guard needs its actual registered source owner'};
      if(success.$!=='boolean')return {$:'machineInvalid',heap:machine.heap,reason:'a strict write guard requires its actual ordinary Set result'};
      const current=replaceJavaScriptCallFrame(machine.heap,machine.frames,next);
      return success.value?{$:'machineContinue',machine:current}:requestJavaScriptCallError(current,owner.realm,'TypeError','the ordinary assignment was rejected');
    }
    default:return {$:'machineInvalid',heap:machine.heap,reason:'an ordinary property executor needs its actual selected instruction'};
  }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction @returns {JavaScriptCallMachineTransition} */
export function executeJavaScriptCallPrimitiveInstruction(program,machine,frame,instruction){
 if(instruction.$!=='primitive')return {$:'machineInvalid',heap:machine.heap,reason:'a primitive executor requires its actual selected instruction'};
 const result=executeJavaScriptCallPrimitive(instruction.operation,instruction.arguments.map(index=>frame.registers[index]??{$:'undefined'}));
 switch(result.$){
  case 'primitiveValue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister({...frame,position:frame.position+1},instruction.destination,result.value))};
  case 'primitiveInvalid':return {$:'machineInvalid',heap:machine.heap,reason:result.reason};
  case 'primitiveResourceLimit':return {$:'machineResourceLimit',machine,reason:result.reason};
  case 'primitiveError':{
   const owner=program.functions[findJavaScriptCallFunction(program,frame.identity)];
   if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a primitive failure needs its actual registered source owner'};
   return requestJavaScriptCallError(replaceJavaScriptCallFrame(machine.heap,machine.frames,{...frame,position:frame.position+1}),owner.realm,result.errorClass,result.reason);
  }
 }
}

/** One instruction/continuation per step. Resource limits are explicit model
 * outcomes; no timeout is passed off as a language-level exception.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @returns {JavaScriptCallMachineTransition} */
export function stepJavaScriptCallMachine(program,machine){
  const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a call step needs its actual active frame'};
  const pending=frame.pending;
  if(pending.$==='waitKey')return acceptJavaScriptCallKey(program,machine,pending.destination,pending.realm,stepJavaScriptPropertyKeyConversion(machine.heap,pending.state));
  if(pending.$==='waitDefinition')return acceptJavaScriptCallDefinition(program,machine,pending.destination,pending.realm,stepJavaScriptPropertyDefinition(machine.heap,pending.state));
  if(pending.$!=='callIdle')return {$:'machineInvalid',heap:machine.heap,reason:'a pending call or allocation needs its actual completion'};
  const fn=program.functions[findJavaScriptCallFunction(program,frame.identity)];if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a call step needs its actual source function'};
  const instruction=fn.instructions[frame.position];
  if(instruction===undefined)return returnJavaScriptCallValue(program,machine,{$:'undefined'});
  const next={...frame,position:frame.position+1},current=replaceJavaScriptCallFrame(machine.heap,machine.frames,next),registers=frame.registers;
  switch(instruction.$){
    case 'constant':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,instruction.value))};
    case 'copy':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,registers[instruction.source]??{$:'undefined'}))};
    case 'argument':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,frame.arguments[instruction.index]??{$:'undefined'}))};
    case 'jump':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,position:instruction.target})};
    case 'branch':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,position:convertJavaScriptSemanticBoolean(registers[instruction.condition]??{$:'undefined'})?instruction.yes:instruction.no})};
    case 'return':return returnJavaScriptCallValue(program,current,registers[instruction.source]??{$:'undefined'});
    case 'throw':return throwJavaScriptCallValue(current,registers[instruction.source]??{$:'undefined'});
    case 'pushHandler':{
      if(frame.handlers.length>=1024)return {$:'machineResourceLimit',machine,reason:'handler storage exceeds the selected execution resource budget'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.concat([{$:'callHandler',target:instruction.target,destination:instruction.destination}])})};
    }
    case 'popHandler':{
      if(frame.handlers.length===0)return {$:'machineInvalid',heap:machine.heap,reason:'a handler pop requires its actual active handler'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.slice(0,-1)})};
    }
    case 'call':{
      const arguments_=instruction.arguments.map(index=>registers[index]??{$:'undefined'});
      return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'},arguments_);
    }
    case 'strictEqual':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:testJavaScriptCallStrictEqual(registers[instruction.left]??{$:'undefined'},registers[instruction.right]??{$:'undefined'})}))};
    case 'sameValue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:testJavaScriptSemanticSameValue(registers[instruction.left]??{$:'undefined'},registers[instruction.right]??{$:'undefined'})}))};
    case 'numberBinary':{
      const left=registers[instruction.left]??{$:'undefined'},right=registers[instruction.right]??{$:'undefined'};
      if(left.$!=='number')return {$:'machineInvalid',heap:machine.heap,reason:'typed number instructions require two actual Number operands'};
      if(right.$!=='number')return {$:'machineInvalid',heap:machine.heap,reason:'typed number instructions require two actual Number operands'};
      const op=instruction.operator;
      if(op==='lessThan'||op==='equal')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:op==='lessThan'?left.value<right.value:left.value===right.value}))};
      const value=op==='add'?left.value+right.value:op==='subtract'?left.value-right.value:op==='multiply'?left.value*right.value:op==='divide'?left.value/right.value:op==='remainder'?left.value%right.value:left.value**right.value;
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'number',value}))};
    }
    case 'primitive':return executeJavaScriptCallPrimitiveInstruction(program,machine,frame,instruction);
    case 'ordinaryGet':case 'ordinarySet':case 'requireWrite':return executeJavaScriptOrdinaryCallProperty(program,machine,frame,instruction);
    case 'propertyKey':return acceptJavaScriptCallKey(program,current,instruction.destination,instruction.realm,beginJavaScriptPropertyKeyConversion(machine.heap,registers[instruction.input]??{$:'undefined'},program.exoticKey));
    case 'propertyDefinition':return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,beginJavaScriptPropertyDefinition(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},registers[instruction.attributes]??{$:'undefined'},program.exoticKey));
  }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptCallMachineTransition} */
export function beginJavaScriptCallMachine(program,heap,identity,receiver,arguments_){
  const error=readJavaScriptCallProgramError(program,heap);if(error!=='')return {$:'machineInvalid',heap,reason:error};
  const fn=program.functions[findJavaScriptCallFunction(program,identity)];if(fn===undefined)return {$:'machineInvalid',heap,reason:'the entry callable needs its actual executable definition'};
  if(arguments_.length>1048576)return {$:'machineInvalid',heap,reason:'entry arguments exceed the explicit call storage budget'};
  const receiverError=readJavaScriptOrdinaryValueError(heap,receiver);if(receiverError!=='')return {$:'machineInvalid',heap,reason:receiverError};
  for(const value of arguments_)if(readJavaScriptOrdinaryValueError(heap,value)!=='')return {$:'machineInvalid',heap,reason:'entry arguments belong to the actual global context'};
  return {$:'machineContinue',machine:{$:'callMachine',heap,frames:[createJavaScriptCallFrame(fn,receiver,arguments_)]}};
}
/** The allocator must create the actual Error in the requested realm and
 * extend the same global heap. Its authority is external, not inferred from
 * an Object name/prototype or membership in this ordinary directory.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} realm @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallErrorAllocation(program,machine,realm,value){
  const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption needs its actual active frame'};
  if(frame.pending.$!=='waitError')return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption requires its actual pending allocation'};
  if(frame.pending.realm!==realm)return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption retains its actual requested realm'};
  const valueError=readJavaScriptOrdinaryValueError(machine.heap,value);if(valueError!=='')return {$:'machineInvalid',heap:machine.heap,reason:valueError};
  if(value.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'an intrinsic Error allocation supplies its actual noncallable Object'};
  if(value.callable)return {$:'machineInvalid',heap:machine.heap,reason:'an intrinsic Error allocation supplies its actual noncallable Object'};
  return throwJavaScriptCallValue(machine,value);
}
