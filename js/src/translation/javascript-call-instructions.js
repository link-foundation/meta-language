import {readJavaScriptDescriptorOperands} from './javascript-descriptor-operands.js';
import {readJavaScriptFunctionText} from './javascript-function-text.js';
import {beginJavaScriptFunctionBind} from './javascript-function-binding.js';
import {executeJavaScriptSymbolRegistryOperation} from './javascript-symbol-registry.js';
import {executeJavaScriptWeakCollectionOperation} from './javascript-weak-collections.js';
import {readJavaScriptWeakCollectionOperationKind} from './javascript-weak-collection-rules.js';
import {beginJavaScriptArrayConcatenation} from './javascript-array-concatenation.js';
import {beginJavaScriptArraySlice} from './javascript-array-slice.js';
import {beginJavaScriptArrayCreation} from './javascript-array-creation.js';
import {beginJavaScriptConstruction} from './javascript-construction.js';
import {beginJavaScriptObjectFromEntries} from './javascript-object-from-entries.js';
import {beginJavaScriptPrimitiveWrapper} from './javascript-primitive-wrappers.js';
import {beginJavaScriptCollectionForEach} from './javascript-collection-for-each.js';
import {beginJavaScriptPropertyDefinitions} from './javascript-property-definitions.js';
import {beginJavaScriptPropertyReflection} from './javascript-property-reflection.js';
import {beginJavaScriptInstanceOf} from './javascript-instance-of.js';
import {beginJavaScriptObjectObservation} from './javascript-object-integrity.js';
import {beginJavaScriptObjectEnumeration} from './javascript-own-enumeration.js';
import {constructJavaScriptPropertyFunctionName} from './javascript-function-names.js';
import {findJavaScriptCallArgumentList,countJavaScriptCallArgumentListValues,openJavaScriptCallArgumentList,appendJavaScriptCallArgument} from './javascript-call-argument-lists.js';
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
import {beginJavaScriptArgumentRest} from './javascript-argument-rest.js';
import {findJavaScriptIteratorRegion,removeJavaScriptIteratorRegion} from './javascript-iterator-regions.js';
import {beginJavaScriptSynchronousIterator,beginJavaScriptSynchronousIteratorStep,beginJavaScriptSynchronousIteratorClose} from './javascript-iterator-protocol.js';
import {beginJavaScriptStringIteratorResult} from './javascript-string-iteration.js';
import {beginJavaScriptArraySpread} from './javascript-array-spread.js';
import {beginJavaScriptArrayIteratorNext} from './javascript-array-iteration.js';
import {beginJavaScriptArrayTransform} from './javascript-array-transform.js';
import {beginJavaScriptArraySearch} from './javascript-array-search.js';
import {beginJavaScriptArrayCallback} from './javascript-array-callbacks.js';
import {beginJavaScriptObjectAssignment,beginJavaScriptObjectSpread} from './javascript-object-spread.js';
import {readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {readJavaScriptCollectionIteratorResultPrototypesError,beginJavaScriptCollectionIteratorResult} from './javascript-collection-iterator-results.js';
import {testJavaScriptCallStrictEqual} from './javascript-call-code.js';
import {findJavaScriptOrdinaryObject,createJavaScriptOrdinarySetDescriptor,setJavaScriptOrdinaryPrototype} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
import {beginJavaScriptPropertyDefinition,completeJavaScriptPropertyDefinitionWithSymbol} from './javascript-property-definition.js';
import {beginJavaScriptPropertyKeyConversion} from './javascript-property-key-conversion.js';
import {readJavaScriptErrorAllocationDirectory} from './javascript-error-construction-rules.js';
import {executeJavaScriptPrimitiveMethod} from './javascript-primitive-methods.js';
import {executeJavaScriptCollectionOperation,copyJavaScriptCollection} from './javascript-collections.js';
import {testJavaScriptArrayValue,testJavaScriptArrayIntrinsicOperation} from './javascript-arrays.js';
import {replaceJavaScriptCallFrame,writeJavaScriptCallRegister,replaceJavaScriptCallPending,advanceJavaScriptCallFrame,throwJavaScriptCallValue,unwindJavaScriptCallCompletion,requestJavaScriptCallError,pushJavaScriptCall,acceptJavaScriptCallFunctionBind,finishJavaScriptCallFunctionName,acceptJavaScriptCallKey,acceptJavaScriptCallDefinition,returnJavaScriptCallValue,executeJavaScriptOrdinaryCallProperty,executeJavaScriptCallPrimitiveInstruction,pushJavaScriptErrorConstructor,pushJavaScriptOrdinaryClassConstructor,pushJavaScriptCollectionConstructor,acceptJavaScriptCollectionCompletion,requestJavaScriptErrorValue,acceptJavaScriptCollectionIteratorResultCompletion,acceptJavaScriptCallObjectSpread,acceptJavaScriptCallArrayCallback,acceptJavaScriptCallArraySearch,acceptJavaScriptCallArrayConcatenation,acceptJavaScriptCallArraySlice,acceptJavaScriptCallArrayTransform,acceptJavaScriptCallArrayIteration,acceptJavaScriptCallArraySpread,acceptJavaScriptCallCompletion,readJavaScriptCallIteratorCloseCompletion,writeJavaScriptCallIteratorResult,acceptJavaScriptCallIteratorProtocol,acceptJavaScriptCallArgumentRest,acceptJavaScriptCallObjectEnumeration,acceptJavaScriptCallObjectIntegrity,acceptJavaScriptCallInstanceOf,acceptJavaScriptCallPropertyReflection,acceptJavaScriptCallPropertyDefinitions,acceptJavaScriptCallCollectionForEach,acceptJavaScriptCallPrimitiveWrapper,acceptJavaScriptCallObjectFromEntries,acceptJavaScriptCallConstructorCheck,acceptJavaScriptCallConstruction,acceptJavaScriptCallArrayCreation} from './javascript-call-machine.js';

/** @param {JavaScriptCallMachine} machine @param {JavaScriptCallMachineTransition} transition
 * @returns {JavaScriptCallMachineTransition} */
function retainJavaScriptCallResourceLimitMachine(machine,transition){
  if(transition.$==='machineResourceLimit')return {$:'machineResourceLimit',machine,reason:transition.reason};
  return transition;
}

/** @param {JavaScriptCallMachine} machine @param {JavaScriptCallFrame} next
 * @param {JavaScriptCallArgumentListsUpdate} update
 * @returns {JavaScriptCallMachineTransition} */
function acceptJavaScriptCallArgumentListsUpdate(machine,next,update){
  switch(update.$){
    case 'argumentListsInvalid':return {$:'machineInvalid',heap:machine.heap,reason:update.reason};
    case 'argumentListsResourceLimit':return {$:'machineResourceLimit',machine,reason:update.reason};
    case 'argumentListsUpdated':{
      const updatedLists=update.lists;
      let count=0;
      for(let position=0;position<machine.frames.length;position+=1){
        const active=machine.frames[position];
        count+=active.arguments.length;
        const activeLists=active.argumentLists;
        if(position===machine.frames.length-1)count+=countJavaScriptCallArgumentListValues(updatedLists);
        else if(activeLists!==undefined)count+=countJavaScriptCallArgumentListValues(activeLists);
      }
      if(count>1048576)return {$:'machineResourceLimit',machine,reason:'argument Lists share the complete actual stack argument storage budget'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,argumentLists:update.lists})};
    }
  }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @returns {JavaScriptCallMachineTransition} */
export function executeJavaScriptCallInstruction(program,machine,frame){
  const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a call step needs its actual source function'};
  const instruction=fn.instructions[frame.position];
  if(instruction===undefined)return returnJavaScriptCallValue(program,machine,{$:'undefined'});
  const next=advanceJavaScriptCallFrame(frame),current=replaceJavaScriptCallFrame(machine.heap,machine.frames,next),registers=frame.registers;
  switch(instruction.$){
    case 'functionBind':return acceptJavaScriptCallFunctionBind(program,current,instruction.destination,fn.realm,beginJavaScriptFunctionBind(machine.heap,registers[0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments.slice(1)));
    case 'functionToString':{const input=registers[0]??{$:'undefined'},result=readJavaScriptFunctionText(program,machine.heap,input);if(result.$==='functionTextValue')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,result.value))};if(result.$==='functionTextError')return requestJavaScriptCallError(current,fn.realm,result.errorClass,result.reason);if(result.$==='functionTextInvalid')return {$:'machineInvalid',heap:machine.heap,reason:result.reason};const retained=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(next,{$:'waitFunctionText',destination:instruction.destination,realm:fn.realm,input,kind:result.kind}));return {$:'machineNeedFunctionText',machine:retained,input,kind:result.kind,realm:fn.realm};}
    case 'boundFunctionCall':{
      const bound=machine.heap.objects[findJavaScriptOrdinaryObject(machine.heap.objects,frame.identity)];if(bound===undefined||bound.$!=='boundFunctionObject')return {$:'machineInvalid',heap:machine.heap,reason:'a bound call retains its actual internal-slot owner'};
      if(bound.boundArguments.length+frame.arguments.length>1048576)return {$:'machineResourceLimit',machine,reason:'bound and supplied arguments exceed the shared call List budget'};
      const nextMachine=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(next,{$:'waitCall',destination:instruction.destination}));
      return retainJavaScriptCallResourceLimitMachine(machine,pushJavaScriptCall(program,nextMachine,bound.boundTarget,bound.boundThis,bound.boundArguments.concat(frame.arguments)));
    }
    case 'constant':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,instruction.value))};
    case 'copy':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,registers[instruction.source]??{$:'undefined'}))};
    case 'argument':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,frame.arguments[instruction.index]??{$:'undefined'}))};
    case 'argumentCount':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'number',value:frame.arguments.length}))};
    case 'argumentAt':{
      const index=registers[instruction.index]??{$:'undefined'};if(index.$!=='number'||!Number.isSafeInteger(index.value)||Object.is(index.value,-0)||index.value<0||index.value>=1048576)return {$:'machineInvalid',heap:machine.heap,reason:'dynamic argument reads retain their actual bounded Call List coordinate'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,frame.arguments[index.value]??{$:'undefined'}))};
    }
    case 'jump':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,position:instruction.target})};
    case 'branch':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,position:convertJavaScriptSemanticBoolean(registers[instruction.condition]??{$:'undefined'})?instruction.yes:instruction.no})};
    case 'return':return returnJavaScriptCallValue(program,current,registers[instruction.source]??{$:'undefined'});
    case 'throw':return throwJavaScriptCallValue(current,registers[instruction.source]??{$:'undefined'});
    case 'pushFinally':{
      if(frame.handlers.length>=1024)return {$:'machineResourceLimit',machine,reason:'finalizer storage exceeds its actual handler budget'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.concat([{$:'callFinallyHandler',target:instruction.target}])})};
    }
    case 'completeJump':return acceptJavaScriptCallCompletion(program,unwindJavaScriptCallCompletion(current,{$:'callCompletion',kind:'jump',value:{$:'undefined'},target:instruction.target,depth:instruction.depth}));
    case 'finishFinally':{
      const handler=frame.handlers[frame.handlers.length-1];if(handler===undefined||handler.$!=='callDeferredCompletion')return {$:'machineInvalid',heap:machine.heap,reason:'finalizer exit retains its actual deferred completion'};
      const resumed=replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.slice(0,-1)});return acceptJavaScriptCallCompletion(program,unwindJavaScriptCallCompletion(resumed,handler.completion));
    }
    case 'pushHandler':{
      if(frame.handlers.length>=1024)return {$:'machineResourceLimit',machine,reason:'handler storage exceeds the selected execution resource budget'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.concat([{$:'callHandler',target:instruction.target,destination:instruction.destination}])})};
    }
    case 'popHandler':{
      if(frame.handlers[frame.handlers.length-1]?.$!=='callHandler')return {$:'machineInvalid',heap:machine.heap,reason:'a handler pop requires its actual active catch handler'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.slice(0,-1)})};
    }
    case 'argumentListOpen':return acceptJavaScriptCallArgumentListsUpdate(machine,next,openJavaScriptCallArgumentList(machine.heap,frame.argumentLists??[],instruction.slot,frame.handlers.length));
    case 'argumentListAppend':return acceptJavaScriptCallArgumentListsUpdate(machine,next,appendJavaScriptCallArgument(machine.heap,frame.argumentLists??[],instruction.slot,frame.handlers.length,registers[instruction.value]??{$:'undefined'}));
    case 'argumentListCall':{
      const lists=frame.argumentLists??[],at=findJavaScriptCallArgumentList(lists,instruction.slot),list=lists[at];if(list===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a variadic Call consumes its actual opened argument List exactly once'};
      const caller=replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,argumentLists:lists.filter((row,index)=>index!==at),pending:{$:'waitCall',destination:instruction.destination}}),callee=registers[instruction.callee]??{$:'undefined'},receiver=registers[instruction.receiver]??{$:'undefined'};
      if(instruction.mode==='construct'){const directory=program.constructorDirectory;if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'variadic construction requires actual constructor authority'};return acceptJavaScriptCallConstruction(program,caller,instruction.destination,fn.realm,beginJavaScriptConstruction(program,machine.heap,directory,callee,receiver,list.values),list.values);}
      const transition=instruction.mode==='collection'?pushJavaScriptCollectionConstructor(program,caller,callee,list.values):instruction.mode==='error class'?pushJavaScriptErrorConstructor(program,caller,callee,list.values):instruction.mode==='ordinary class'?pushJavaScriptOrdinaryClassConstructor(program,caller,callee,receiver,list.values):pushJavaScriptCall(program,caller,callee,receiver,list.values);
      return retainJavaScriptCallResourceLimitMachine(machine,transition);
    }
    case 'call':{
      const arguments_=instruction.arguments.map(index=>registers[index]??{$:'undefined'});
      return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'},arguments_);
    }
    case 'constructErrorClass':return pushJavaScriptErrorConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'callClassConstructor':return pushJavaScriptOrdinaryClassConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'constructCollection':return pushJavaScriptCollectionConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'argumentRest':return acceptJavaScriptCallArgumentRest(current,instruction.destination,instruction.realm,beginJavaScriptArgumentRest(machine.heap,frame.arguments,instruction.from,registers[instruction.prototype]??{$:'undefined'}));
    case 'iteratorProtocol':{
      const regions=frame.iterators??[],region=regions[findJavaScriptIteratorRegion(regions,instruction.slot)],pending={$:'iteratorOperation',destination:instruction.destination,done:instruction.done,slot:instruction.slot,operation:instruction.operation,realm:instruction.realm};
      if(instruction.operation==='acquire'){
        if(region!==undefined)return {$:'machineInvalid',heap:machine.heap,reason:'iterator opening cannot overwrite an active source region'};if(regions.length>=128)return {$:'machineResourceLimit',machine,reason:'iterator opening exceeds its actual frame region budget'};
        return acceptJavaScriptCallIteratorProtocol(program,current,pending,beginJavaScriptSynchronousIterator(machine.heap,registers[instruction.input]??{$:'undefined'},registers[instruction.iteratorKey]??{$:'undefined'}));
      }
      if(region===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'iterator operation requires its actual opened source region'};
      if(instruction.operation==='step')return acceptJavaScriptCallIteratorProtocol(program,current,pending,beginJavaScriptSynchronousIteratorStep(machine.heap,region.record,instruction.readValue??true));
      if(region.record.done){const removed=removeJavaScriptIteratorRegion(machine.heap,regions,instruction.slot);if(removed.$!=='iteratorRegionsUpdated')return {$:'machineInvalid',heap:machine.heap,reason:'exhausted iterator release retains its actual region'};return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallIteratorResult({...next,iterators:removed.regions},instruction.destination,instruction.done,{$:'undefined'},true))};}
      return acceptJavaScriptCallIteratorProtocol(program,current,pending,beginJavaScriptSynchronousIteratorClose(machine.heap,region.record,readJavaScriptCallIteratorCloseCompletion(frame)));
    }
    case 'collectionIteratorResult':{
      if(instruction.kind==='String')return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,instruction.destination,fn.realm,beginJavaScriptStringIteratorResult(machine.heap,registers[0]??{$:'undefined'},registers[instruction.arrayPrototype]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'}));
      if(instruction.kind==='Array'){
        const error=readJavaScriptCollectionIteratorResultPrototypesError(machine.heap,registers[instruction.arrayPrototype]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'});if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
        return acceptJavaScriptCallArrayIteration(program,machine,beginJavaScriptArrayIteratorNext(machine.heap,registers[0]??{$:'undefined'},program.exoticKey));
      }
      return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,instruction.destination,fn.realm,beginJavaScriptCollectionIteratorResult(machine.heap,registers[0]??{$:'undefined'},instruction.kind,registers[instruction.arrayPrototype]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'}));
    }
    case 'collectionOperation':{
      const receiver=registers[0]??{$:'undefined'};
      if(readJavaScriptWeakCollectionOperationKind(instruction.operation)!=='')return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,executeJavaScriptWeakCollectionOperation(machine.heap,receiver,instruction.operation,frame.arguments));
      return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,executeJavaScriptCollectionOperation(machine.heap,receiver,instruction.operation,frame.arguments));
    }
    case 'primitiveMethod':{
     if(testJavaScriptArrayIntrinsicOperation(instruction.operation)){
      const result=testJavaScriptArrayValue(machine.heap,frame.arguments[0]??{$:'undefined'});if(result.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:result.error};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:result.array}))};
     }
     const completion=executeJavaScriptPrimitiveMethod(registers[0]??{$:'undefined'},instruction.operation,frame.arguments);
     switch(completion.$){
      case 'primitiveValue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,completion.value))};
      case 'primitiveInvalid':return {$:'machineInvalid',heap:machine.heap,reason:completion.reason};
      case 'primitiveError':return requestJavaScriptCallError(machine,fn.realm,completion.errorClass,completion.reason);
      case 'primitiveResourceLimit':return {$:'machineResourceLimit',machine,reason:completion.reason};
     }
    }
    case 'collectionCopy':return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,copyJavaScriptCollection(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},registers[instruction.adder]??{$:'undefined'},registers[instruction.iteratorKey]??{$:'undefined'},registers[instruction.iteratorMethod]??{$:'undefined'},registers[instruction.iteratorPrototype]??{$:'undefined'},registers[instruction.nextMethod]??{$:'undefined'}));
    case 'constructErrorValue':return requestJavaScriptErrorValue(program,machine,frame,instruction);
    case 'readDerivedReceiver':{const receiver=registers[0]??{$:'undefined'};if(receiver.$==='undefined')return requestJavaScriptCallError(current,fn.realm,'ReferenceError','derived this requires its actual completed super construction');return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,receiver))};}
    case 'invokeErrorIntrinsic':return {$:'machineInvokeErrorIntrinsic',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitErrorIntrinsic',destination:instruction.destination}}),realm:fn.realm,callee:{$:'object',identity:fn.identity,callable:true},receiver:registers[0]??{$:'undefined'},arguments:frame.arguments};
    case 'invokeHostCapability':return {$:'machineInvokeHostCapability',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitHostCapability',destination:instruction.destination,beforeDirectory:readJavaScriptErrorAllocationDirectory(machine.heap)}}),realm:fn.realm,callee:{$:'object',identity:fn.identity,callable:true},receiver:registers[0]??{$:'undefined'},arguments:frame.arguments};
    case 'strictEqual':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:testJavaScriptCallStrictEqual(registers[instruction.left]??{$:'undefined'},registers[instruction.right]??{$:'undefined'})}))};
    case 'sameValue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:testJavaScriptSemanticSameValue(registers[instruction.left]??{$:'undefined'},registers[instruction.right]??{$:'undefined'})}))};
    case 'numberBinary':{
      const left=registers[instruction.left]??{$:'undefined'},right=registers[instruction.right]??{$:'undefined'};
      if(left.$!=='number')return {$:'machineInvalid',heap:machine.heap,reason:'typed number instructions require two actual Number operands'};
      if(right.$!=='number')return {$:'machineInvalid',heap:machine.heap,reason:'typed number instructions require two actual Number operands'};
      const op=instruction.operator,first=left.value,second=right.value;
      if(op==='lessThan'||op==='equal')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:op==='lessThan'?first<second:first===second}))};
      const value=op==='add'?first+second:op==='subtract'?first-second:op==='multiply'?first*second:op==='divide'?first/second:op==='remainder'?first%second:first**second;
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'number',value}))};
    }
    case 'primitive':return executeJavaScriptCallPrimitiveInstruction(program,machine,frame,instruction);
    case 'closureAllocate':case 'ordinaryAllocate':case 'arrayAllocate':case 'collectionAllocate':case 'collectionIteratorAllocate':case 'ordinaryGet':case 'ordinarySet':case 'ordinaryDelete':case 'ordinaryHas':case 'requireWrite':return executeJavaScriptOrdinaryCallProperty(program,machine,frame,instruction);
    case 'propertyKey':return acceptJavaScriptCallKey(program,current,instruction.destination,instruction.realm,beginJavaScriptPropertyKeyConversion(machine.heap,registers[instruction.input]??{$:'undefined'},program.exoticKey));
    case 'propertyDefinition':return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,beginJavaScriptPropertyDefinition(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},registers[instruction.attributes]??{$:'undefined'},program.exoticKey,instruction.operation));
    case 'ordinaryDefine':{const decoded=readJavaScriptDescriptorOperands(instruction.fields.map(coordinate=>registers[coordinate]??{$:'undefined'}));return decoded.error!==''?{$:'machineInvalid',heap:machine.heap,reason:decoded.error}:acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,completeJavaScriptPropertyDefinitionWithSymbol(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},decoded.descriptor,program.exoticKey,'Reflect.defineProperty'));}
    case 'arrayConcatenation':return acceptJavaScriptCallArrayConcatenation(program,current,instruction.destination,fn.realm,beginJavaScriptArrayConcatenation(machine.heap,instruction.species,registers[instruction.input??0]??{$:'undefined'},frame.arguments,registers[instruction.spreadableKey]??{$:'undefined'},program.exoticKey));
    case 'arraySlice':return acceptJavaScriptCallArraySlice(program,current,instruction.destination,fn.realm,beginJavaScriptArraySlice(machine.heap,instruction.species,registers[instruction.input??0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'},frame.arguments.length>1,program.exoticKey));
    case 'arrayTransform':return acceptJavaScriptCallArrayTransform(program,current,instruction.destination,fn.realm,beginJavaScriptArrayTransform(machine.heap,instruction.species,instruction.operation,registers[instruction.input??0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'},program.exoticKey));
    case 'arraySearch':{const at=instruction.operation==='Array.at';return acceptJavaScriptCallArraySearch(program,current,instruction.destination,fn.realm,beginJavaScriptArraySearch(machine.heap,instruction.operation,registers[instruction.input??0]??{$:'undefined'},at?{$:'undefined'}:frame.arguments[0]??{$:'undefined'},frame.arguments[at?0:1]??{$:'undefined'},frame.arguments.length>=(at?1:2),program.exoticKey));}
    case 'collectionForEach':return acceptJavaScriptCallCollectionForEach(program,current,instruction.destination,fn.realm,beginJavaScriptCollectionForEach(machine.heap,instruction.operation,registers[0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'}));
    case 'arrayCallback':return acceptJavaScriptCallArrayCallback(program,current,instruction.destination,fn.realm,beginJavaScriptArrayCallback(machine.heap,instruction.operation,registers[instruction.input??0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'},program.exoticKey,frame.arguments.length>=2));
    case 'arraySpread':{
      const index=registers[instruction.index]??{$:'undefined'};if(index.$!=='number')return {$:'machineInvalid',heap:machine.heap,reason:'Array spread index retains its actual Number coordinate'};
      return acceptJavaScriptCallArraySpread(program,current,instruction.destination,instruction.realm,beginJavaScriptArraySpread(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},index.value,registers[instruction.iteratorKey]??{$:'undefined'}));
    }
    case 'objectSpread':return acceptJavaScriptCallObjectSpread(program,current,instruction.destination,instruction.realm,instruction.operation==='assignment'?beginJavaScriptObjectAssignment(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},program.exoticKey):beginJavaScriptObjectSpread(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},(instruction.excluded??[]).map(index=>registers[index]??{$:'undefined'})));
    case 'objectEnumeration':return acceptJavaScriptCallObjectEnumeration(program,current,instruction.destination,fn.realm,beginJavaScriptObjectEnumeration(machine.heap,instruction.operation,frame.arguments[0]??{$:'undefined'},registers[instruction.prototype]??{$:'undefined'}));
    case 'propertyDefinitions':return acceptJavaScriptCallPropertyDefinitions(program,current,instruction.destination,fn.realm,beginJavaScriptPropertyDefinitions(machine.heap,instruction.operation,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},program.exoticKey));
    case 'propertyReflection':return acceptJavaScriptCallPropertyReflection(current,instruction.destination,fn.realm,beginJavaScriptPropertyReflection(machine.heap,instruction.operation,registers[instruction.input]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'},registers[instruction.arrayPrototype]??{$:'undefined'}));
    case 'instanceOf':return acceptJavaScriptCallInstanceOf(program,current,instruction.destination,fn.realm,beginJavaScriptInstanceOf(machine.heap,instruction.operation,registers[instruction.left]??{$:'undefined'},registers[instruction.right]??{$:'undefined'},registers[instruction.key]??{$:'undefined'}));
    case 'objectObservation':return acceptJavaScriptCallObjectIntegrity(current,instruction.destination,fn.realm,beginJavaScriptObjectObservation(machine.heap,instruction.operation,registers[instruction.input]??{$:'undefined'},registers[instruction.other]??{$:'undefined'}));
    case 'arrayJoinEnter':{
     const receiver=registers[instruction.receiver]??{$:'undefined'};if(receiver.$!=='object'||frame.activeJoin!==undefined)return {$:'machineInvalid',heap:machine.heap,reason:'join entry retains an actual Object and fresh activation'};
     const recursive=machine.frames.some(parent=>parent.activeJoin!==undefined&&readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity)?.realm===fn.realm&&testJavaScriptSemanticSameValue(parent.activeJoin,receiver)),joinedFrame=recursive?next:{...next,activeJoin:receiver};
     return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,current.frames,writeJavaScriptCallRegister(joinedFrame,instruction.destination,{$:'boolean',value:!recursive}))};
    }
    case 'arrayJoinLeave':{
     const receiver=registers[instruction.receiver]??{$:'undefined'};if(frame.activeJoin===undefined||!testJavaScriptSemanticSameValue(frame.activeJoin,receiver))return {$:'machineInvalid',heap:machine.heap,reason:'join leave retains its actually owned receiver guard'};
     const {activeJoin:released,...releasedFrame}=next;return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,current.frames,releasedFrame)};
    }
    case 'arrayCreation':return acceptJavaScriptCallArrayCreation(program,current,instruction.destination,fn.realm,beginJavaScriptArrayCreation(machine.heap,instruction.operation,registers[0]??{$:'undefined'},frame.arguments,registers[instruction.prototype]??{$:'undefined'},registers[instruction.iteratorKey]??{$:'undefined'},program.exoticKey));
    case 'readNewTarget':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,frame.newTarget??{$:'undefined'}))};
    case 'constructorCheck':return acceptJavaScriptCallConstructorCheck(program,current,instruction.destination,registers[instruction.input]??{$:'undefined'},instruction.required);
    case 'construct':{const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'construction requires actual constructor authority'}:acceptJavaScriptCallConstruction(program,current,instruction.destination,fn.realm,beginJavaScriptConstruction(program,machine.heap,directory,registers[instruction.callee]??{$:'undefined'},registers[instruction.newTarget]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'})));}
    case 'objectFromEntries':return acceptJavaScriptCallObjectFromEntries(program,current,instruction.destination,fn.realm,beginJavaScriptObjectFromEntries(machine.heap,registers[instruction.input]??{$:'undefined'},registers[instruction.prototype]??{$:'undefined'},registers[instruction.iteratorKey]??{$:'undefined'},program.exoticKey));
    case 'primitiveWrapper':return acceptJavaScriptCallPrimitiveWrapper(current,instruction.destination,beginJavaScriptPrimitiveWrapper(machine.heap,registers[instruction.input]??{$:'undefined'},registers[instruction.prototype]??{$:'undefined'}));
    case 'symbolRegistry':{
      const result=executeJavaScriptSymbolRegistryOperation(machine.heap,instruction.operation,registers[instruction.input]??{$:'undefined'});
      if(result.$==='symbolRegistryValue')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(result.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,result.value))};
      if(result.$==='symbolRegistryAllocate')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(result.heap,machine.frames,{...next,pending:{$:'waitSymbolAllocation',destination:instruction.destination,description:result.key,registryKey:result.key}})};
      if(result.$==='symbolRegistryError')return requestJavaScriptCallError(machine,fn.realm,result.errorClass,result.reason);
      if(result.$==='symbolRegistryResourceLimit')return {$:'machineResourceLimit',machine,reason:result.reason};
      return {$:'machineInvalid',heap:machine.heap,reason:result.reason};
    }
    case 'symbolAllocate':{
      const description=registers[instruction.input]??{$:'undefined'};if(!['undefined','text','sourceText'].includes(description.$))return {$:'machineInvalid',heap:machine.heap,reason:'Symbol allocation requires its actual completed optional text description'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitSymbolAllocation',destination:instruction.destination,description}})};
    }
    case 'symbolDescription':{
      const symbol=registers[instruction.input]??{$:'undefined'};if(symbol.$!=='symbol')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol description instructions require their actual primitive Symbol input'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitSymbolDescription',destination:instruction.destination,symbol}})};
    }
    case 'functionName':{
      const object=registers[instruction.object]??{$:'undefined'},key=registers[instruction.key]??{$:'undefined'},named=constructJavaScriptPropertyFunctionName(key,instruction.prefix);
      const target=object.$==='object'?machine.heap.objects[findJavaScriptOrdinaryObject(machine.heap.objects,object.identity)]:undefined;if(object.$!=='object'||!object.callable||target?.$!=='closureObject')return {$:'machineInvalid',heap:machine.heap,reason:'function naming requires its actual allocated runtime callable'};
      if(named.$==='functionNameInvalid')return {$:'machineInvalid',heap:machine.heap,reason:named.reason};
      if(named.$==='functionNameResourceLimit')return {$:'machineResourceLimit',machine,reason:named.reason};
      if(named.$==='functionNameSymbolDescription')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitFunctionNameSymbol',destination:instruction.destination,object,key,prefix:instruction.prefix,realm:instruction.realm}})};
      return finishJavaScriptCallFunctionName(program,current,instruction.destination,object,instruction.realm,named.value);
    }
    case 'literalAccessor':{
      const descriptor=createJavaScriptPropertyDescriptor('accessor',true,true),value=registers[instruction.value]??{$:'undefined'};
      // An absent opposite field preserves an existing accessor's counterpart;
      // conversion from a data property supplies undefined through the kernel.
      if(instruction.kind==='get'){descriptor.getter=value;descriptor.setterPresent=false;}else{descriptor.setter=value;descriptor.getterPresent=false;}
      return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,completeJavaScriptPropertyDefinitionWithSymbol(machine.heap,registers[instruction.object]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},descriptor,program.exoticKey));
    }
    case 'literalPrototype':{
      const object=registers[instruction.object]??{$:'undefined'},value=registers[instruction.value]??{$:'undefined'};
      if(object.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'literal prototype installation requires its actual allocated Object'};
      if(value.$!=='object'&&value.$!=='null')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'undefined'}))};
      const installed=setJavaScriptOrdinaryPrototype(machine.heap,object.identity,value);
      if(installed.error!=='')return {$:'machineInvalid',heap:installed.heap,reason:installed.error};
      if(!installed.success)return {$:'machineInvalid',heap:installed.heap,reason:'a fresh literal Object must admit its acyclic actual prototype'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(installed.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'undefined'}))};
    }
    case 'ordinaryDataProperty':return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,completeJavaScriptPropertyDefinitionWithSymbol(machine.heap,registers[instruction.object]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},createJavaScriptOrdinarySetDescriptor(registers[instruction.value]??{$:'undefined'},true),program.exoticKey));
  }
}
