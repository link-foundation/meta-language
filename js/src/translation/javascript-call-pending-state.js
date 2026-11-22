// Validate suspended operations against their actual frame, instruction and heap.
import {readJavaScriptDescriptorOperands} from './javascript-descriptor-operands.js';
import {readJavaScriptFunctionText} from './javascript-function-text.js';
import {readJavaScriptFunctionBindStateError} from './javascript-function-binding.js';
import {readJavaScriptArrayConcatenationStateError} from './javascript-array-concatenation.js';
import {readJavaScriptArraySliceStateError} from './javascript-array-slice.js';
import {stepJavaScriptArrayCreation,readJavaScriptArrayCreationStateError} from './javascript-array-creation.js';
import {readJavaScriptConstructionStateError} from './javascript-construction.js';
import {readJavaScriptObjectFromEntriesStateError} from './javascript-object-from-entries.js';
import {readJavaScriptPrimitiveWrapperStateError} from './javascript-primitive-wrappers.js';
import {readJavaScriptCollectionForEachStateError} from './javascript-collection-for-each.js';
import {readJavaScriptPropertyDefinitionsStateError} from './javascript-property-definitions.js';
import {readJavaScriptPropertyReflectionStateError} from './javascript-property-reflection.js';
import {readJavaScriptInstanceOfStateError} from './javascript-instance-of.js';
import {readJavaScriptObjectIntegrityStateError} from './javascript-object-integrity.js';
import {readJavaScriptObjectEnumerationStateError} from './javascript-own-enumeration.js';
import {testJavaScriptPropertyDescriptorsSameValue} from './javascript-property-descriptors.js';
import {readJavaScriptArgumentRestStateError} from './javascript-argument-rest.js';
import {findJavaScriptIteratorRegion} from './javascript-iterator-regions.js';
import {readJavaScriptIteratorProtocolStateError} from './javascript-iterator-protocol.js';
import {readJavaScriptArraySpreadStateError} from './javascript-array-spread.js';
import {readJavaScriptArrayIteratorStateError} from './javascript-array-iteration.js';
import {readJavaScriptSpeciesConstructionStateError} from './javascript-species-construction.js';
import {readJavaScriptKnownArrayConstructorFact,classifyJavaScriptArraySpeciesConstructor} from './javascript-array-species.js';
import {readJavaScriptArrayTransformStateError} from './javascript-array-transform.js';
import {readJavaScriptArraySearchStateError} from './javascript-array-search.js';
import {readJavaScriptArrayCallbackStateError} from './javascript-array-callbacks.js';
import {readJavaScriptObjectSpreadStateError} from './javascript-object-spread.js';
import {readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {readJavaScriptArrayAssignmentStateError} from './javascript-array-assignment.js';
import {readJavaScriptCollectionIteratorResultStateError} from './javascript-collection-iterator-results.js';
import {testJavaScriptCollectionIteratorIntrinsicMode} from './javascript-collection-iterator-call-source.js';
import {readJavaScriptOrdinaryValueError} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptPropertyDefinitionStateError} from './javascript-property-definition.js';
import {readJavaScriptSourceExceptionClass} from './source-exception-rules.js';
import {testJavaScriptErrorIntrinsicMode,readJavaScriptErrorConstructionOperandsError,readJavaScriptErrorPriorDirectoryError} from './javascript-error-construction-rules.js';
import {testJavaScriptHostCapabilityMode} from './javascript-host-capability-rules.js';
import {testJavaScriptPrimitiveIntrinsicMode} from './javascript-collection-call-source.js';
import {readJavaScriptNumberCallSourceOperation,readJavaScriptTextCallSourceOperation,readJavaScriptNumberCallStateError,readJavaScriptTextCallStateError,readJavaScriptCallIteratorCloseCompletion,readJavaScriptCallConstructorFailure} from './javascript-call-machine.js';

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @param {JavaScriptCallPending} pending @returns {string} */
export function readJavaScriptCallPendingError(program,heap,frame,fn,suspended,pending){
    switch(pending.$){
      case 'waitObjectPrototype':
      case 'waitPropertyDefinitions':
      case 'waitPropertyReflection':
      case 'waitInstanceOf':
      case 'waitObjectIntegrity':
      case 'waitObjectEnumeration':
      case 'waitObjectFromEntries':
      case 'waitObjectSpread':
        return readJavaScriptCallPendingObjectError(program,heap,frame,fn,suspended,pending);
      case 'waitArrayCreation':
      case 'waitConstructorCheck':
      case 'waitConstruction':
      case 'waitFunctionBind':
      case 'waitFunctionText':
      case 'waitPrimitiveWrapper':
        return readJavaScriptCallPendingConstructionError(program,heap,frame,fn,suspended,pending);
      case 'waitError':
      case 'waitSymbolAllocation':
      case 'waitSymbolDescription':
      case 'waitFunctionNameSymbol':
      case 'waitSourceError':
      case 'waitErrorValue':
      case 'waitErrorIntrinsic':
      case 'waitHostCapability':
        return readJavaScriptCallPendingIntrinsicError(program,heap,frame,fn,suspended,pending);
      case 'waitArrayConcatenation':
      case 'waitArraySlice':
      case 'waitArrayTransform':
        return readJavaScriptCallPendingArraySpeciesError(program,heap,frame,fn,suspended,pending);
      case 'waitArraySearch':
      case 'waitCollectionForEach':
      case 'waitArrayCallback':
      case 'waitArrayAssignment':
      case 'waitArgumentRest':
        return readJavaScriptCallPendingArrayCallbackError(program,heap,frame,fn,suspended,pending);
      case 'waitIteratorProtocol':
      case 'waitArraySpread':
      case 'waitArrayIteratorNext':
      case 'waitCollectionIteratorResult':
        return readJavaScriptCallPendingIteratorError(program,heap,frame,fn,suspended,pending);
      case 'waitKey':
      case 'waitDefinition':
      case 'waitNumber':
      case 'waitText':
        return readJavaScriptCallPendingConversionError(program,heap,frame,fn,suspended,pending);
      default:return '';
    }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @param {JavaScriptCallPending} pending @returns {string} */
function readJavaScriptCallPendingObjectError(program,heap,frame,fn,suspended,pending){
    switch(pending.$){
      case 'waitObjectPrototype':{
        const invalidInstruction='primitive prototype metadata retains its actual source input operation and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='objectObservation')return invalidInstruction;
        if(instruction.operation!=='Object.getPrototypeOf'||instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.input)||pending.input.$==='object'||pending.input.$==='null'||pending.input.$==='undefined'||suspended)return invalidInstruction;
        const invalid=readJavaScriptOrdinaryValueError(heap,pending.input);if(invalid!=='')return invalid;return '';
      }
      case 'waitPropertyDefinitions':{
        const invalidInstruction='bulk definitions retain actual source target attributes Symbol and selected child';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1],state=pending.state,selected=state.operation==='Object.create'?state.prototype:state.target;
        if(instruction===undefined||instruction.$!=='propertyDefinitions')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.operation!==state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.target]??{$:'undefined'},selected)||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},state.input)||!testJavaScriptSemanticSameValue(program.exoticKey,state.exoticKey)||suspended&&state.call===undefined)return invalidInstruction;
        const error=readJavaScriptPropertyDefinitionsStateError(heap,state);if(error!=='')return error;return '';
      }
      case 'waitPropertyReflection':{
        const invalidInstruction='property reflection retains actual source input key realm prototypes and no invented child call';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1],state=pending.state;
        if(instruction===undefined||instruction.$!=='propertyReflection')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.operation!==state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},state.input)||!testJavaScriptSemanticSameValue(frame.registers[instruction.key]??{$:'undefined'},state.key)||!testJavaScriptSemanticSameValue(frame.registers[instruction.objectPrototype]??{$:'undefined'},state.objectPrototype)||!testJavaScriptSemanticSameValue(frame.registers[instruction.arrayPrototype]??{$:'undefined'},state.arrayPrototype)||suspended)return invalidInstruction;
        const invalid=readJavaScriptPropertyReflectionStateError(heap,state);if(invalid!=='')return invalid;return '';
      }
      case 'waitInstanceOf':{
        const invalidInstruction='instanceof retains its actual selected source operands Symbol operation and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1],state=pending.state;
        if(instruction===undefined||instruction.$!=='instanceOf')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.operation!==state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.left]??{$:'undefined'},state.left)||!testJavaScriptSemanticSameValue(frame.registers[instruction.right]??{$:'undefined'},state.right)||!testJavaScriptSemanticSameValue(frame.registers[instruction.key]??{$:'undefined'},state.key)||suspended&&state.call===undefined)return invalidInstruction;
        const invalid=readJavaScriptInstanceOfStateError(heap,state);if(invalid!=='')return invalid;return '';
      }
      case 'waitObjectIntegrity':{
        const invalidInstruction='integrity changes retain actual source operation input and no invented child call';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='objectObservation')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.operation!==pending.state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.state.input)||suspended)return invalidInstruction;
        const invalid=readJavaScriptObjectIntegrityStateError(heap,pending.state);if(invalid!=='')return invalid;return '';
      }
      case 'waitObjectEnumeration':{
        const invalidInstruction='Object enumeration retains its actual source operation input and realm Array prototype';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='objectEnumeration')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.operation!==pending.state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.arguments[0]??{$:'undefined'},pending.state.input)||!testJavaScriptSemanticSameValue(frame.registers[instruction.prototype]??{$:'undefined'},pending.state.prototype))return invalidInstruction;
        const invalid=readJavaScriptObjectEnumerationStateError(heap,pending.state);if(invalid!=='')return invalid;if(suspended&&pending.state.phase!=='getter')return 'Object enumeration child retains its actual getter phase';return '';
      }
      case 'waitObjectFromEntries':{
        const invalidInstruction='Object.fromEntries retains its actual source input realm prototype and well-known Symbols';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='objectFromEntries')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.state.input)||!testJavaScriptSemanticSameValue(frame.registers[instruction.prototype]??{$:'undefined'},pending.state.prototype)||!testJavaScriptSemanticSameValue(frame.registers[instruction.iteratorKey]??{$:'undefined'},pending.state.iteratorKey)||!testJavaScriptSemanticSameValue(program.exoticKey,pending.state.exoticKey))return invalidInstruction;
        const invalid=readJavaScriptObjectFromEntriesStateError(heap,pending.state);if(invalid!=='')return invalid;
        if(suspended&&(pending.state.call===undefined||pending.error!==undefined))return 'Object.fromEntries child retains its actual selected call rather than an allocation request';
        const setup=pending.error;
        if(setup!==undefined&&!testJavaScriptPendingCallConstructorFailure(program,heap,pending.state.call?.callee??{$:'undefined'},setup))return 'Object.fromEntries selected-call failure retains its actual callee constructor kind Error realm and reason';
        return '';
      }
      case 'waitObjectSpread':{
        const invalidInstruction='object spread retains its actual source instruction, destination and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='objectSpread')return invalidInstruction;
        if(instruction.realm!==pending.realm||instruction.destination!==pending.destination||!program.realms.includes(pending.realm))return invalidInstruction;
        const exoticKey=pending.state.exoticKey;
        if(pending.state.operation!==instruction.operation||instruction.operation==='assignment'&&(exoticKey===undefined||!testJavaScriptSemanticSameValue(exoticKey,program.exoticKey)))return 'object copy retains its actual CreateDataProperty or Set operation and Symbol';
        const error=readJavaScriptObjectSpreadStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.target,frame.registers[instruction.target]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.input,frame.registers[instruction.input]??{$:'undefined'}))return 'object spread cannot substitute its actual target or source input';
        const excluded=instruction.excluded??[],excludedValues=pending.state.excluded;if(excluded.length!==excludedValues.length)return 'object copy retains every actual excluded-key operand';for(let at=0;at<excluded.length;at++)if(!testJavaScriptSemanticSameValue(excludedValues[at]??{$:'undefined'},frame.registers[excluded[at]??-1]??{$:'undefined'}))return 'object copy cannot substitute an excluded key';
        if(suspended&&pending.state.phase!=='getter'&&pending.state.call===undefined)return 'object copy retains only actual selected getters setters and numeric callbacks';
        return '';
      }
      default:return '';
    }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @param {JavaScriptCallPending} pending @returns {string} */
function readJavaScriptCallPendingConstructionError(program,heap,frame,fn,suspended,pending){
    switch(pending.$){
      case 'waitArrayCreation':{
        const invalidInstruction='Array creation retains its actual receiver arguments roots Symbols and source operation';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='arrayCreation')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm||instruction.operation!==pending.state.operation||!testJavaScriptSemanticSameValue(frame.registers[0]??{$:'undefined'},pending.state.receiver)||!testJavaScriptSemanticSameValue(frame.registers[instruction.prototype]??{$:'undefined'},pending.state.arrayPrototype)||!testJavaScriptSemanticSameValue(frame.registers[instruction.iteratorKey]??{$:'undefined'},pending.state.iteratorKey)||!testJavaScriptSemanticSameValue(program.exoticKey,pending.state.exoticKey)||frame.arguments.length!==pending.state.arguments.length||frame.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,pending.state.arguments[index]??{$:'undefined'})))return invalidInstruction;
        const invalid=readJavaScriptArrayCreationStateError(heap,pending.state);if(invalid!=='')return invalid;
        if(pending.construction!==undefined||pending.fallback!==undefined){const request=stepJavaScriptArrayCreation(heap,pending.state);if(request.$!=='arrayCreationConstruct'||pending.error!==undefined||pending.construction!==undefined&&pending.fallback!==undefined)return 'Array creation retains only its actual selected construction phase';
         const construction=pending.construction;
         if(construction!==undefined){const directory=program.constructorDirectory;if(directory===undefined)return 'Array creation construction retains actual constructor authority';const invalid=readJavaScriptConstructionStateError(program,heap,directory,construction);if(invalid!=='')return invalid;if(!testJavaScriptSemanticSameValue(construction.target,request.constructor)||!testJavaScriptSemanticSameValue(construction.newTarget,request.constructor)||construction.arguments.length!==request.arguments.length||construction.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,request.arguments[index]??{$:'undefined'})))return 'Array creation retains its actual constructor and complete arguments';}
         else{const fallback=pending.fallback;if(fallback===undefined||fallback.$!=='arrayCreationArrayFallback'||fallback.length!==request.fallback.length||!testJavaScriptSemanticSameValue(fallback.prototype,request.fallback.prototype)||suspended)return 'Array creation fallback retains its actual ArrayCreate length and prototype';const directory=program.constructorDirectory;if(directory===undefined)return 'Array creation fallback requires actual constructor authority';const classified=classifyJavaScriptArraySpeciesConstructor(heap,directory,request.constructor);if(classified.$==='arrayConstructorUnknown'||readJavaScriptKnownArrayConstructorFact(classified)?.constructable===true)return 'Array creation fallback cannot replace an actual constructor';}
        }
        if(suspended&&(pending.construction?.call===undefined&&pending.state.call===undefined||pending.error!==undefined))return 'an Array creation child retains only its actual selected call';
        const setup=pending.error;
        if(setup!==undefined&&(pending.construction!==undefined||pending.fallback!==undefined||!testJavaScriptPendingCallConstructorFailure(program,heap,pending.state.call?.callee??{$:'undefined'},setup)))return 'Array creation setup failure retains its actual selected callee Error realm and reason';
        return '';
      }
      case 'waitConstructorCheck':{
        const invalidInstruction='constructor checks retain actual selected input predicate mode and instruction';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='constructorCheck')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.required!==pending.required||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.input)||suspended)return invalidInstruction;return '';
      }
      case 'waitConstruction':{
        const invalidInstruction='construction retains every actual held ordered argument';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1],directory=program.constructorDirectory;
        if(directory===undefined||instruction===undefined||pending.realm!==fn.realm)return invalidInstruction;
        switch(instruction.$){
          case 'construct':
            if(instruction.destination!==pending.destination||!testJavaScriptSemanticSameValue(frame.registers[instruction.callee]??{$:'undefined'},pending.state.target)||!testJavaScriptSemanticSameValue(frame.registers[instruction.newTarget]??{$:'undefined'},pending.state.newTarget))return invalidInstruction;
            if(instruction.arguments.length!==pending.state.arguments.length||instruction.arguments.some((coordinate,index)=>!testJavaScriptSemanticSameValue(frame.registers[coordinate]??{$:'undefined'},pending.state.arguments[index]??{$:'undefined'})))return invalidInstruction;
            break;
          case 'argumentListCall':
            if(instruction.mode!=='construct'||instruction.destination!==pending.destination||!testJavaScriptSemanticSameValue(frame.registers[instruction.callee]??{$:'undefined'},pending.state.target)||!testJavaScriptSemanticSameValue(frame.registers[instruction.receiver]??{$:'undefined'},pending.state.newTarget))return invalidInstruction;
            if(pending.heldArguments===undefined||pending.heldArguments.length!==pending.state.arguments.length||pending.heldArguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,pending.state.arguments[index]??{$:'undefined'})))return 'variadic construction retains its consumed immutable argument List proof';
            break;
          default:return invalidInstruction;
        }
        const invalid=readJavaScriptConstructionStateError(program,heap,directory,pending.state);if(invalid!=='')return invalid;
        if(suspended&&pending.state.call===undefined)return 'a construction child retains its actual selected getter conversion or body call';return '';
      }
      case 'waitFunctionBind':{
        const invalidInstruction='binding retains its actual intrinsic target receiver and complete argument prefix';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='functionBind')return invalidInstruction;
        if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(pending.state.target,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.boundThis,frame.arguments[0]??{$:'undefined'})||pending.state.arguments.length!==Math.max(0,frame.arguments.length-1)||pending.state.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,frame.arguments[index+1]??{$:'undefined'})))return invalidInstruction;
        const error=readJavaScriptFunctionBindStateError(heap,pending.state);if(error!=='')return error;if(suspended&&pending.state.call===undefined)return 'binding child frames retain their actual selected metadata getter';return '';
      }
      case 'waitFunctionText':{
        const invalidInstruction='function text retains its actual intrinsic receiver realm source category and external metadata request';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1],request=readJavaScriptFunctionText(program,heap,pending.input);if(instruction===undefined||instruction.$!=='functionToString')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptSemanticSameValue(pending.input,frame.registers[0]??{$:'undefined'})||request.$!=='functionTextNeedAuthority'||request.kind!==pending.kind||suspended)return invalidInstruction;return '';
      }
      case 'waitPrimitiveWrapper':{
        const invalidInstruction='primitive wrappers retain their actual selected input prototype and allocation instruction';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='primitiveWrapper')return invalidInstruction;
        if(instruction.destination!==pending.destination||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.state.input)||!testJavaScriptSemanticSameValue(frame.registers[instruction.prototype]??{$:'undefined'},pending.state.prototype)||suspended)return invalidInstruction;
        const invalid=readJavaScriptPrimitiveWrapperStateError(heap,pending.state);if(invalid!=='')return invalid;return '';
      }
      default:return '';
    }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @param {JavaScriptCallPending} pending @returns {string} */
function readJavaScriptCallPendingIntrinsicError(program,heap,frame,fn,suspended,pending){
    switch(pending.$){
      case 'waitError':if(!program.realms.includes(pending.realm))return 'pending Error allocation retains its actual intrinsic realm';return '';
      case 'waitSymbolAllocation':{
        const invalidInstruction='Symbol allocation retains its actual converted optional description and selected instruction';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||!['undefined','text','sourceText'].includes(pending.description.$)||readJavaScriptOrdinaryValueError(heap,pending.description)!==''||suspended)return invalidInstruction;
        switch(instruction.$){
          case 'symbolRegistry':{
            const registryKey=pending.registryKey;
            if(instruction.operation!=='for'||registryKey===undefined||!['text','sourceText'].includes(pending.description.$)||!testJavaScriptSemanticSameValue(registryKey,pending.description))return invalidInstruction;
            if(instruction.destination!==pending.destination||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.description))return invalidInstruction;
            return '';
          }
          case 'symbolAllocate':
            if(pending.registryKey!==undefined||instruction.destination!==pending.destination||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.description))return invalidInstruction;
            return '';
          default:return invalidInstruction;
        }
      }
      case 'waitSymbolDescription':{
        const invalidInstruction='Symbol description metadata retains its actual selected Symbol input and instruction';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='symbolDescription')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.symbol.$!=='symbol'||readJavaScriptOrdinaryValueError(heap,pending.symbol)!==''||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.symbol)||suspended)return invalidInstruction;return '';
      }
      case 'waitFunctionNameSymbol':{
        const invalidInstruction='Symbol name metadata retains its actual selected function property key prefix and instruction';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='functionName')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.realm!==pending.realm||instruction.prefix!==pending.prefix||pending.key.$!=='symbol'||pending.object.$!=='object'||!pending.object.callable||!testJavaScriptSemanticSameValue(frame.registers[instruction.object]??{$:'undefined'},pending.object)||!testJavaScriptSemanticSameValue(frame.registers[instruction.key]??{$:'undefined'},pending.key))return invalidInstruction;
        const invalid=readJavaScriptOrdinaryValueError(heap,pending.key)||readJavaScriptOrdinaryValueError(heap,pending.object);if(invalid!=='')return invalid;return '';
      }
      case 'waitSourceError':{
        if(!program.realms.includes(pending.realm)||fn.realm!==pending.realm)return 'pending source Error allocation retains its actual source realm';
        if((pending.message.$!=='text'&&pending.message.$!=='sourceText')||readJavaScriptOrdinaryValueError(heap,pending.message)!==''||typeof pending.messagePresent!=='boolean')return 'pending source Error retains exact UTF16 message and Boolean presence';
        const invalidInstruction='pending source Error retains its actual source instruction and arguments';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined)return invalidInstruction;
        switch(instruction.$){
          case 'primitive':{
            if(readJavaScriptSourceExceptionClass(instruction.operation)!==pending.errorClass||pending.errorClass===''||instruction.arguments.length!==2)return 'pending source Error retains its actual source instruction and arguments';
            const message=frame.registers[instruction.arguments[0]?? -1]??{$:'undefined'},present=frame.registers[instruction.arguments[1]?? -1]??{$:'undefined'};
            if(!testJavaScriptSemanticSameValue(message,pending.message))return 'pending source Error retains its actual source instruction and arguments';
            if(present.$!=='boolean')return 'pending source Error retains its actual source instruction and arguments';
            if(present.value!==pending.messagePresent)return 'pending source Error retains its actual source instruction and arguments';
            break;
          }
          default:return 'pending source Error retains its actual source instruction and arguments';
        }
        return '';
      }
      case 'waitErrorValue':{
        const invalidInstruction='Error value resumption retains its actual constructor instruction';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='constructErrorValue')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.realm!==pending.realm||instruction.errorClass!==pending.errorClass||instruction.bindReceiver!==pending.bindReceiver)return 'Error value resumption retains its actual destination, realm and constructor role';
        const constructor=frame.registers[instruction.constructor]??{$:'undefined'},newTarget=frame.registers[instruction.newTarget]??{$:'undefined'},prototype=frame.registers[instruction.prototype]??{$:'undefined'},message=frame.registers[instruction.message]??{$:'undefined'},present=frame.registers[instruction.present]??{$:'undefined'};
        const error=readJavaScriptErrorConstructionOperandsError(heap,constructor,newTarget,prototype,message,present);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(constructor,pending.constructor)||!testJavaScriptSemanticSameValue(newTarget,pending.newTarget)||!testJavaScriptSemanticSameValue(prototype,pending.prototype)||!testJavaScriptSemanticSameValue(message,pending.message))return 'an Error allocation cannot substitute its actual constructor operands';
        if(present.$!=='boolean'||present.value!==pending.messagePresent)return 'an Error allocation retains actual source presence';
        const prior=readJavaScriptErrorPriorDirectoryError(heap,pending.beforeDirectory);if(prior!=='')return prior;
        return '';
      }
      case 'waitErrorIntrinsic':{
        const invalidInstruction='an external Error callback retains its actual registered source instruction';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='invokeErrorIntrinsic')return invalidInstruction;
        if(!testJavaScriptErrorIntrinsicMode(fn.receiverMode)||instruction.destination!==pending.destination)return invalidInstruction;
        return '';
      }
      case 'waitHostCapability':{
        const invalidInstruction='an external capability retains its actual qualified source instruction';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='invokeHostCapability')return invalidInstruction;
        if(!testJavaScriptHostCapabilityMode(fn.receiverMode)||instruction.destination!==pending.destination)return invalidInstruction;
        const prior=readJavaScriptErrorPriorDirectoryError(heap,pending.beforeDirectory);if(prior!=='')return prior;
        return '';
      }
      default:return '';
    }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @param {JavaScriptCallPending} pending @returns {string} */
function readJavaScriptCallPendingArraySpeciesError(program,heap,frame,fn,suspended,pending){
    switch(pending.$){
      case 'waitArrayConcatenation':{
        const invalidInstruction='Array concatenation retains actual source instruction destination and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='arrayConcatenation')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm)return invalidInstruction;
        const species=instruction.species;
        if(species===undefined)return 'Array concatenation retains its actual species authority';
        const invalid=readJavaScriptArrayConcatenationStateError(heap,species,pending.state);if(invalid!=='')return invalid;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.spreadableKey,frame.registers[instruction.spreadableKey]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey)||pending.state.arguments.length!==frame.arguments.length||pending.state.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,frame.arguments[index]??{$:'undefined'})))return 'Array concatenation retains original receiver arguments and Symbols';
        const construction=pending.construction;
        if(construction!==undefined){const selected=Object.hasOwn(pending.state,'constructor')?pending.state.constructor:undefined;if(pending.state.phase!=='construct'||selected===undefined||!testJavaScriptSemanticSameValue(construction.constructor,selected)||construction.length!==0)return 'Array concatenation retains its actual zero-length species construction';const invalid=readJavaScriptSpeciesConstructionStateError(program,heap,species,construction);if(invalid!=='')return invalid;}
        if(suspended&&(construction!==undefined?construction.call===undefined:pending.state.call===undefined))return 'Array concatenation child retains its actual selected callback';return '';
      }
      case 'waitArraySlice':{
        const invalidInstruction='Array slice retains actual source instruction destination and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='arraySlice')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm)return invalidInstruction;
        const species=instruction.species;
        if(species===undefined)return 'Array slice retains its actual species authority';
        const invalid=readJavaScriptArraySliceStateError(heap,species,pending.state);if(invalid!=='')return invalid;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.start,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.end,frame.arguments[1]??{$:'undefined'})||pending.state.endPresent!==(frame.arguments.length>1)||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array slice retains original receiver bounds argument presence and Symbol';
        const construction=pending.construction;
        if(construction!==undefined){const selected=Object.hasOwn(pending.state,'constructor')?pending.state.constructor:undefined;if(pending.state.phase!=='construct'||selected===undefined||!testJavaScriptSemanticSameValue(construction.constructor,selected)||construction.length!==pending.state.count)return 'Array slice retains its actual selected species construction';const invalid=readJavaScriptSpeciesConstructionStateError(program,heap,species,construction);if(invalid!=='')return invalid;}
        if(suspended&&(construction!==undefined?construction.call===undefined:pending.state.call===undefined))return 'Array slice child retains its actual selected callback';return '';
      }
      case 'waitArrayTransform':{
        const invalidInstruction='Array transform continuation retains its actual source operation, destination and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='arrayTransform')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm||pending.state.operation!==instruction.operation)return invalidInstruction;
        const species=instruction.species;
        if(species===undefined)return 'Array transform retains its actual species authority';
        const error=readJavaScriptArrayTransformStateError(heap,species,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.callback,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.thisArgument,frame.arguments[1]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array transforms cannot substitute actual source, callback, thisArg or conversion Symbol';
        const construction=pending.construction;
        if(construction!==undefined){
          const selected=Object.hasOwn(pending.state,'constructor')?pending.state.constructor:undefined;
          if(pending.state.phase!=='construct'||selected===undefined||pending.state.constructorFact===undefined||!testJavaScriptSemanticSameValue(construction.constructor,selected)||construction.length!==(pending.state.operation==='Array.map'?pending.state.length:0))return 'nested species construction retains its actual transform selection';
          const error=readJavaScriptSpeciesConstructionStateError(program,heap,species,construction);if(error!=='')return error;
        }
        if(suspended&&(construction!==undefined?construction.call===undefined:pending.state.call===undefined))return 'Array transform child frames retain their actual selected call';return '';
      }
      default:return '';
    }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @param {JavaScriptCallPending} pending @returns {string} */
function readJavaScriptCallPendingArrayCallbackError(program,heap,frame,fn,suspended,pending){
    switch(pending.$){
      case 'waitArraySearch':{
        const invalidInstruction='Array search continuation retains its actual source operation, destination and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='arraySearch')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm||pending.state.operation!==instruction.operation)return invalidInstruction;
        const error=readJavaScriptArraySearchStateError(heap,pending.state);if(error!=='')return error;
        const at=instruction.operation==='Array.at',search=at?{$:'undefined'}:frame.arguments[0]??{$:'undefined'},from=frame.arguments[at?0:1]??{$:'undefined'},present=frame.arguments.length>=(at?1:2);
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.searchElement,search)||!testJavaScriptSemanticSameValue(pending.state.fromIndex,from)||pending.state.fromPresent!==present||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array search cannot substitute its actual receiver, search value, index argument, presence or Symbol';
        if(suspended&&pending.state.call===undefined)return 'only a selected Array search getter or conversion can retain a child frame';
        return '';
      }
      case 'waitCollectionForEach':{
        const invalidInstruction='collection callback continuation retains its actual operation destination and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='collectionForEach')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm||instruction.operation!==pending.state.operation)return invalidInstruction;
        const error=readJavaScriptCollectionForEachStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.callback,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.thisArgument,frame.arguments[1]??{$:'undefined'}))return 'collection callbacks retain their actual receiver callback and thisArgument';
        if(suspended&&pending.state.call===undefined)return 'a collection callback child retains its actual selected call';return '';
      }
      case 'waitArrayCallback':{
        const invalidInstruction='Array callback continuation retains its actual source operation, destination and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='arrayCallback')return invalidInstruction;
        if(instruction.destination!==pending.destination||pending.realm!==fn.realm||pending.state.operation!==instruction.operation)return invalidInstruction;
        const error=readJavaScriptArrayCallbackStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.callback,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.thisArgument,frame.arguments[1]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey)||pending.state.initialPresent!==(frame.arguments.length>=2))return 'Array callbacks cannot substitute their receiver, callback, thisArg or well-known Symbol';
        if(suspended&&pending.state.call===undefined)return 'only an actual Array getter, conversion or callback can retain a child frame';
        return '';
      }
      case 'waitArrayAssignment':{
        const invalidInstruction='Array assignment retains its actual source Set instruction, destination and realm';
        if(!(frame.position>0))return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='ordinarySet')return invalidInstruction;
        if(fn.realm!==pending.realm||instruction.destination!==pending.destination)return invalidInstruction;
        const error=readJavaScriptArrayAssignmentStateError(heap,pending.state);if(error!=='')return error;
        const receiver=frame.registers[instruction.receiver]??{$:'undefined'},definition=pending.state.definition;
        if(receiver.$!=='object'||receiver.identity!==definition.identity||!testJavaScriptSemanticSameValue(definition.key,frame.registers[instruction.key]??{$:'undefined'})||!testJavaScriptSemanticSameValue(definition.descriptor.value,frame.registers[instruction.value]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array assignment cannot substitute its receiver, key, value or well-known Symbol';
        const descriptor=definition.descriptor;
        if(!descriptor.valuePresent||descriptor.writablePresent||descriptor.enumerablePresent||descriptor.configurablePresent||descriptor.getterPresent||descriptor.setterPresent)return 'Array assignment retains its actual value-only ordinary Set descriptor';
        if(suspended&&pending.state.$!=='arrayAssignmentNumber')return 'only a pending Array numeric callback can have a suspended child call';
        return '';
      }
      case 'waitArgumentRest':{
        const invalidInstruction='argument rest retains its actual source instruction and prototype';
        if(frame.position===0)return invalidInstruction;
        const instruction=fn.instructions[frame.position-1];if(instruction===undefined||instruction.$!=='argumentRest')return invalidInstruction;
        if(instruction.destination!==pending.destination||instruction.realm!==pending.realm||instruction.from!==pending.state.from||!testJavaScriptSemanticSameValue(pending.state.prototype,frame.registers[instruction.prototype]??{$:'undefined'}))return invalidInstruction;
        const error=readJavaScriptArgumentRestStateError(heap,frame.arguments,pending.state);if(error!=='')return error;if(suspended)return 'argument copying cannot hide a selected source callback';return '';
      }
      default:return '';
    }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @param {JavaScriptCallPending} pending @returns {string} */
function readJavaScriptCallPendingIteratorError(program,heap,frame,fn,suspended,pending){
    switch(pending.$){
      case 'waitIteratorProtocol':{
        if(frame.position===0)return 'iterator work retains its actual source instruction and region';
        const state=pending.state,instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='iteratorProtocol')return 'iterator work retains its actual source instruction and region';
        if(instruction.destination!==pending.destination||instruction.done!==pending.done||instruction.slot!==pending.slot||instruction.realm!==pending.realm||instruction.operation!==pending.operation||state.operation!==pending.operation)return 'iterator work retains its actual source instruction and region';
        const error=readJavaScriptIteratorProtocolStateError(heap,state);if(error!=='')return error;if(pending.operation==='step'&&(state.readValue??true)!==(instruction.readValue??true))return 'iterator advancement retains its actual value-versus-elision choice';
        if(pending.operation==='acquire'){
          if(!testJavaScriptSemanticSameValue(state.input,frame.registers[instruction.input]??{$:'undefined'})||!testJavaScriptSemanticSameValue(state.key,frame.registers[instruction.iteratorKey]??{$:'undefined'}))return 'iterator acquisition retains its held source operand and key';
        }else{
          const regions=frame.iterators??[],region=regions[findJavaScriptIteratorRegion(regions,pending.slot)];if(region===undefined||!testJavaScriptSemanticSameValue(region.record.iterator,state.record.iterator)||!testJavaScriptSemanticSameValue(region.record.nextMethod,state.record.nextMethod)||region.record.done!==state.record.done)return 'iterator continuation retains its actual cached region record';
          if(pending.operation==='close'){const completion=readJavaScriptCallIteratorCloseCompletion(frame);if(completion.kind!==state.completion.kind||!testJavaScriptSemanticSameValue(completion.value,state.completion.value))return 'iterator cleanup retains the actual deferred completion';}
        }
        if(suspended&&state.call===undefined)return 'iterator child retains its actual selected getter or method';return '';
      }
      case 'waitArraySpread':{
        if(frame.position===0)return 'Array spread retains its actual source instruction, destination and realm';
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined||instruction.$!=='arraySpread')return 'Array spread retains its actual source instruction, destination and realm';
        if(instruction.destination!==pending.destination||instruction.realm!==pending.realm||pending.realm!==fn.realm)return 'Array spread retains its actual source instruction, destination and realm';
        const error=readJavaScriptArraySpreadStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(frame.registers[instruction.target]??{$:'undefined'},pending.state.target)||
           !testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.state.input)||
           !testJavaScriptSemanticSameValue(frame.registers[instruction.iteratorKey]??{$:'undefined'},pending.state.iteratorKey))return 'Array spread retains its actual target, source and iterator key';
        const index=frame.registers[instruction.index]??{$:'undefined'};if(index.$!=='number'||!Number.isSafeInteger(index.value)||index.value<0||pending.state.index<index.value)return 'Array spread retains its original exact accumulation coordinate';
        if(suspended&&pending.state.protocol?.call===undefined)return 'Array spread child retains its actual selected iterator call';return '';
      }
      case 'waitArrayIteratorNext':{
        const instruction=fn.instructions[frame.position];
        if(!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode)||fn.realm!==pending.realm||instruction===undefined||instruction.$!=='collectionIteratorResult'||instruction.kind!=='Array'||instruction.destination!==pending.destination)return 'Array next continuation retains its actual instruction, operation and realm';
        const error=readJavaScriptArrayIteratorStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array next retains its actual receiver and conversion Symbol';
        if(suspended&&pending.state.call===undefined)return 'Array next child retains its actual selected getter or conversion';return '';
      }
      case 'waitCollectionIteratorResult':{
        const instruction=fn.instructions[frame.position];
        if(!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode)||fn.realm!==pending.realm||instruction===undefined||instruction.$!=='collectionIteratorResult'||instruction.destination!==pending.destination)return 'iterator result continuation retains its actual source instruction, destination and realm';
        const error=readJavaScriptCollectionIteratorResultStateError(heap,pending.state);if(error!=='')return error;
        if(pending.state.collectionKind!==instruction.kind||!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[0]??{$:'undefined'}))return 'iterator result continuation cannot replace its actual receiver or operation brand';
        if(!testJavaScriptSemanticSameValue(pending.state.arrayPrototype,frame.registers[instruction.arrayPrototype]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.objectPrototype,frame.registers[instruction.objectPrototype]??{$:'undefined'}))return 'iterator result continuation cannot substitute its actual realm prototypes';
        if(suspended)return 'iterator result allocation cannot hide a pending source child call';
        return '';
      }
      default:return '';
    }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @param {JavaScriptCallPending} pending @returns {string} */
function readJavaScriptCallPendingConversionError(program,heap,frame,fn,suspended,pending){
    switch(pending.$){
      case 'waitKey':if(!program.realms.includes(pending.realm))return 'pending key conversion retains its actual intrinsic realm';return '';
      case 'waitDefinition':{
        if(!program.realms.includes(pending.realm))return 'pending property definition retains its actual intrinsic realm';const invalid=readJavaScriptPropertyDefinitionStateError(heap,pending.state);if(invalid!=='')return invalid;
        const invalidTarget='property definition retains its actual selected source target';
        const invalidOperation='property definition retains its actual source result policy destination and realm';
        if(!(frame.position>0))return invalidTarget;
        const instruction=fn.instructions[frame.position-1];
        if(instruction===undefined)return invalidTarget;
        const operation=pending.state.operation??'Object.defineProperty';
        switch(instruction.$){
          case 'ordinaryDefine':{
            const target=frame.registers[instruction.target];
            if(target===undefined||!testJavaScriptSemanticSameValue(target,pending.state.target))return invalidTarget;
            if(operation!=='Reflect.defineProperty'||instruction.destination!==pending.destination||instruction.realm!==pending.realm)return invalidOperation;
            const operands=readJavaScriptDescriptorOperands(instruction.fields.map(coordinate=>frame.registers[coordinate]??{$:'undefined'}));
            const wrongDescriptor='internal definitions retain their actual key and complete preconverted descriptor operands';
            const state=pending.state;
            const key=frame.registers[instruction.key]??{$:'undefined'};
            if(operands.error!=='')return wrongDescriptor;
            switch(state.$){
              case 'propertyDefinitionArray':
                if(!testJavaScriptSemanticSameValue(key,state.key)||!testJavaScriptPropertyDescriptorsSameValue(state.assignment.definition.descriptor,operands.descriptor))return wrongDescriptor;
                break;
              case 'propertyDefinitionReady':
                if(!testJavaScriptSemanticSameValue(key,state.key)||!testJavaScriptPropertyDescriptorsSameValue(state.descriptor,operands.descriptor))return wrongDescriptor;
                break;
              default:return wrongDescriptor;
            }
            break;
          }
          case 'propertyDefinition':{
            const target=frame.registers[instruction.target];
            if(target===undefined||!testJavaScriptSemanticSameValue(target,pending.state.target))return invalidTarget;
            if(operation!==(instruction.operation??'Object.defineProperty')||instruction.destination!==pending.destination||instruction.realm!==pending.realm)return invalidOperation;
            break;
          }
          case 'ordinaryDataProperty':
          case 'literalAccessor':{
            const target=frame.registers[instruction.object];
            if(target===undefined||!testJavaScriptSemanticSameValue(target,pending.state.target))return invalidTarget;
            if(operation!=='Object.defineProperty'||instruction.destination!==pending.destination||instruction.realm!==pending.realm)return invalidOperation;
            break;
          }
          default:return invalidTarget;
        }
        const state=pending.state;
        if(suspended&&(state.$==='propertyDefinitionReady'||state.operation==='Reflect.defineProperty'&&state.call===undefined))return 'a definition suspension retains its actual selected child rather than a completed descriptor';
        const wrongConversion='property definition retains its actual primitive conversion authority';
        switch(state.$){
          case 'propertyDefinitionArray':
            if(!testJavaScriptSemanticSameValue(state.assignment.exoticKey,program.exoticKey))return 'Array definition retains its actual primitive conversion Symbol';
            if(suspended&&state.assignment.$!=='arrayAssignmentNumber')return 'Array definition child retains its actual numeric callback phase';
            break;
          case 'propertyDefinitionKey':
            if(!testJavaScriptSemanticSameValue(state.conversion.exoticKey,program.exoticKey))return wrongConversion;
            break;
          case 'propertyDefinitionDescriptor':{
            const exoticKey=state.exoticKey;
            if(exoticKey!==undefined&&!testJavaScriptSemanticSameValue(exoticKey,program.exoticKey))return wrongConversion;
            break;
          }
          default:break;
        }
        return '';
      }
      case 'waitNumber':{
        if(!program.realms.includes(pending.realm))return 'pending number conversion retains its actual intrinsic realm';
        const numberState=pending.state;
        const stateError=readJavaScriptNumberCallStateError(heap,numberState);if(stateError!=='')return stateError;
        if(readJavaScriptNumberCallSourceOperation(fn,frame,pending.destination,pending.realm,program.exoticKey,numberState)==='')return 'pending number conversion retains its actual source instruction, receiver, Symbol and realm';
        if(suspended){
          switch(numberState.$){
            case 'numberPrimitiveValue':return 'a completed numeric primitive cannot have a suspended child call';
            case 'numberProtocolState':break;
          }
        }
        return '';
      }
      case 'waitText':{
        if(!program.realms.includes(pending.realm))return 'pending text conversion retains its actual intrinsic realm';
        const textState=pending.state;
        const stateError=readJavaScriptTextCallStateError(heap,textState);if(stateError!=='')return stateError;
        if(readJavaScriptTextCallSourceOperation(fn,frame,pending.destination,pending.realm,program.exoticKey,textState)==='')return 'pending text conversion retains its actual source instruction, receiver, Symbol and realm';
        if(suspended){
          switch(textState.$){
            case 'textPrimitiveValue':return 'a completed string primitive cannot have a suspended child call';
            case 'textProtocolState':break;
          }
        }
        return '';
      }
      default:return '';
    }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptObjectFromEntriesErrorRequest} retained @returns {boolean} */
function testJavaScriptPendingCallConstructorFailure(program,heap,callee,retained){
 if(callee.$!=='object'||retained.$!=='objectFromEntriesErrorRequest')return false;
 const owner=readJavaScriptExecutableCallFunction(program,heap,callee.identity);
 if(owner===undefined)return false;
 const actual=readJavaScriptCallConstructorFailure(heap,owner);
 if(actual===undefined)return false;
 return retained.realm===actual.realm&&retained.errorClass===actual.errorClass&&retained.reason===actual.reason;
}
