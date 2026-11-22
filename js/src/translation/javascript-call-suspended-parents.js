import {readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {findJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {testJavaScriptOrdinaryClassConstructorMode} from './javascript-ordinary-class-rules.js';
import {readJavaScriptSelectedCallFrameError} from './javascript-call-machine.js';

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} parent @param {JavaScriptCallFunction} fn
 * @param {JavaScriptCallFrame} frame @returns {string} */
export function readJavaScriptSelectedParentCallError(program,heap,parent,fn,frame){
 const pending=parent.pending;
 switch(pending.$){
  case 'waitDefinition':{
   const call=pending.state.call;
   return call===undefined?'':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitPropertyDefinitions':{
   const call=pending.state.call;
   return call===undefined?'bulk definitions retain their actual selected child':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitInstanceOf':{
   const call=pending.state.call;
   return call===undefined?'instanceof child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitIteratorProtocol':{
   const call=pending.state.call;
   return call===undefined?'iterator region child retains its actual selected callback':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitArraySpread':{
   const call=pending.state.protocol?.call;
   return call===undefined?'Array spread child retains its actual selected iterator callback':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitFunctionBind':{
   const call=pending.state.call;
   return call===undefined?'a binding child retains its actual metadata getter':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitObjectFromEntries':{
   const call=pending.state.call;
   return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitCollectionForEach':{
   const call=pending.state.call;
   return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitArrayCallback':{
   const call=pending.state.call;
   return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitArraySearch':{
   const call=pending.state.call;
   return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitArrayIteratorNext':{
   const call=pending.state.call;
   return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitArrayTransform':{
   const construction=pending.construction;
   if(construction!==undefined){
    const call=construction.call;
    return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
   }
   const call=pending.state.call;
   return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitArraySlice':{
   const construction=pending.construction;
   if(construction!==undefined){
    const call=construction.call;
    return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
   }
   const call=pending.state.call;
   return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitArrayConcatenation':{
   const construction=pending.construction;
   if(construction!==undefined){
    const call=construction.call;
    return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
   }
   const call=pending.state.call;
   return call===undefined?'an Array operation child retains its actual selected call':readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
  }
  case 'waitObjectEnumeration':{
   const state=pending.state;
   if(state.phase!=='getter')return 'Object enumeration child retains its actual selected getter';
   return readJavaScriptSelectedCallFrameError(fn,frame,state.callee,state.input,[]);
  }
  case 'waitArrayCreation':{
   const construction=pending.construction?.call,callback=pending.state.call;
   if(construction===undefined)return callback===undefined?'an Array creation child retains its actual selected callback or constructor body':readJavaScriptSelectedCallFrameError(fn,frame,callback.callee,callback.receiver,callback.arguments);
   const invalid=readJavaScriptSelectedCallFrameError(fn,frame,construction.callee,construction.receiver,construction.arguments);if(invalid!=='')return invalid;
   return construction.constructing&&!testJavaScriptSemanticSameValue(frame.newTarget??{$:'undefined'},construction.newTarget)?'an Array creation constructor retains its actual newTarget':'';
  }
  case 'waitConstruction':{
   const call=pending.state.call;if(call===undefined)return 'a construction child retains its actual selected call';
   const invalid=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(invalid!=='')return invalid;
   return call.constructing&&!testJavaScriptSemanticSameValue(frame.newTarget??{$:'undefined'},call.newTarget)?'a construction body retains its actual newTarget':'';
  }
  case 'waitCall':{
   const owner=readJavaScriptExecutableCallFunction(program,heap,parent.identity),instruction=owner?.instructions[parent.position-1];
   if(instruction?.$!=='boundFunctionCall')return '';
   const bound=heap.objects[findJavaScriptOrdinaryObject(heap.objects,parent.identity)];
   if(bound===undefined||bound.$!=='boundFunctionObject'||bound.boundArguments.length+parent.arguments.length>1048576)return 'a bound call child retains its actual complete stored and supplied argument Lists';
   return readJavaScriptSelectedCallFrameError(fn,frame,bound.boundTarget,bound.boundThis,bound.boundArguments.concat(parent.arguments));
  }
  case 'waitObjectSpread':{
   const state=pending.state,call=state.call;
   if(call!==undefined)return readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);
   const callee=state.callee,receiver=fn.receiverMode==='lexical'?fn.lexicalReceiver:state.input;
   if(state.phase!=='getter'||callee.$!=='object'||callee.identity!==frame.identity||frame.arguments.length!==0||!testJavaScriptSemanticSameValue(frame.registers[0]??{$:'undefined'},receiver))return 'an object spread child retains its actual getter, receiver and empty arguments';
   return '';
  }
  default:return '';
 }
}

/** @param {JavaScriptCallFrame} parent @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {JavaScriptCallInstruction} instruction @param {number} destination @returns {string} */
export function readJavaScriptSuspendedConstructorFrameError(parent,frame,fn,instruction,destination){
  switch(instruction.$){
    case 'callClassConstructor':return readJavaScriptConstructorFrameOperandsError(parent,frame,fn,destination,instruction.destination,instruction.callee,true,instruction.receiver,instruction.arguments);
    case 'constructCollection':return readJavaScriptConstructorFrameOperandsError(parent,frame,fn,destination,instruction.destination,instruction.callee,false,-1,instruction.arguments);
    case 'argumentListCall':
      if(instruction.mode==='ordinary class')return readJavaScriptConstructorFrameOperandsError(parent,frame,fn,destination,instruction.destination,instruction.callee,true,instruction.receiver,undefined);
      if(instruction.mode==='collection')return readJavaScriptConstructorFrameOperandsError(parent,frame,fn,destination,instruction.destination,instruction.callee,false,-1,undefined);
      return 'a constructor frame retains its actual suspended source new operation';
    default:return 'a constructor frame retains its actual suspended source new operation';
  }
}

/** @param {JavaScriptCallFrame} parent @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {number} destination @param {number} actualDestination @param {number} calleeRegister
 * @param {boolean} ordinary @param {number} receiverRegister @param {number[]|undefined} argumentRegisters @returns {string} */
function readJavaScriptConstructorFrameOperandsError(parent,frame,fn,destination,actualDestination,calleeRegister,ordinary,receiverRegister,argumentRegisters){
  if(actualDestination!==destination)return 'a constructor frame retains its actual suspended source new operation';
  if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)!==ordinary)return 'a constructor frame retains its actual class or standard collection operation';
  const callee=parent.registers[calleeRegister]??{$:'undefined'},receiver=ordinary?(parent.registers[receiverRegister]??{$:'undefined'}):{$:'undefined'};
  if(callee.$!=='object'||callee.identity!==frame.identity||!testJavaScriptSemanticSameValue(receiver,frame.registers[0]??{$:'undefined'}))return 'an ordinary class frame retains its actual new callee and receiver';
  if(argumentRegisters!==undefined){
    if(argumentRegisters.length!==frame.arguments.length)return 'an ordinary class frame retains its actual argument list';
    for(let at=0;at<argumentRegisters.length;at++)if(!testJavaScriptSemanticSameValue(parent.registers[argumentRegisters[at]??-1]??{$:'undefined'},frame.arguments[at]??{$:'undefined'}))return 'ordinary class frames cannot substitute their actual ordered constructor arguments';
  }
  return '';
}
