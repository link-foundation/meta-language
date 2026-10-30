import {beginJavaScriptPrimitiveConversion,stepJavaScriptPrimitiveConversion,resumeJavaScriptPrimitiveConversion} from './javascript-primitive-conversion.js';

/** @typedef {{$:'propertyKeyContinuation',state:JavaScriptPrimitiveConversionState} |
 * {$:'propertyKeyCall',state:JavaScriptPrimitiveConversionState,callee:JavaScriptSemanticValue,
 * receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'propertyKeyCompleted',key:JavaScriptSemanticValue} |
 * {$:'propertyKeyThrow',value:JavaScriptSemanticValue} |
 * {$:'propertyKeyIntrinsicError',errorClass:string,reason:string} |
 * {$:'propertyKeyInvalid',reason:string}} JavaScriptPropertyKeyConversionTransition
 */

/** Only primitive domains that need textual conversion arrive here. Actual
 * Number/BigInt formatting uses the source String operation and its existing
 * translated runtime, never a hand-transcribed native formatter.
 * @param {JavaScriptSemanticValue} value @returns {string} */
export function readJavaScriptPrimitivePropertyKeySpelling(value){
  switch(value.$){
    case 'undefined':return 'undefined';
    case 'null':return 'null';
    case 'boolean':return value.value?'true':'false';
    case 'number':return String(value.value);
    case 'integer':return String(value.value);
    case 'text':throw new TypeError('semantic text keys retain their exact units');
    case 'symbol':throw new TypeError('semantic Symbol keys retain their exact identity');
    case 'object':throw new TypeError('a property key needs its completed actual primitive conversion');
  }
}

/** Complete ToPropertyKey after the actual ToPrimitive(string) operation.
 * Symbols retain identity; text retains its original units and every other
 * primitive uses the normative source String operation. Continuations keep
 * their actual state/callee/receiver/arguments and exact abrupt values.
 * @param {JavaScriptPrimitiveConversionTransition} transition
 * @returns {JavaScriptPropertyKeyConversionTransition} */
export function finishJavaScriptPropertyKeyConversion(transition){
  switch(transition.$){
    case 'primitiveContinuation':return {$:'propertyKeyContinuation',state:transition.state};
    case 'primitiveCall':return {$:'propertyKeyCall',state:transition.state,callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
    case 'primitiveThrow':return {$:'propertyKeyThrow',value:transition.value};
    case 'primitiveIntrinsicError':return {$:'propertyKeyIntrinsicError',errorClass:transition.errorClass,reason:transition.reason};
    case 'primitiveInvalid':return {$:'propertyKeyInvalid',reason:transition.reason};
    case 'primitiveCompleted':{
      const value=transition.value;
      if(value.$==='text'||value.$==='symbol')return {$:'propertyKeyCompleted',key:value};
      if(value.$==='object')return {$:'propertyKeyInvalid',reason:'a completed primitive conversion cannot contain an Object'};
      const spelling=readJavaScriptPrimitivePropertyKeySpelling(value);
      if(spelling.length>1048576)return {$:'propertyKeyInvalid',reason:'property key text exceeds the explicit UTF16 unit budget'};
      const units=[];
      for(let index=0;index<spelling.length;index+=1)units.push(spelling.charCodeAt(index));
      return {$:'propertyKeyCompleted',key:{$:'text',units}};
    }
  }
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input
 * @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptPropertyKeyConversionTransition} */
export function beginJavaScriptPropertyKeyConversion(heap,input,exoticKey){
  return finishJavaScriptPropertyKeyConversion(beginJavaScriptPrimitiveConversion(heap,input,exoticKey,'string'));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state
 * @returns {JavaScriptPropertyKeyConversionTransition} */
export function stepJavaScriptPropertyKeyConversion(heap,state){
  if(state.hint!=='string')return {$:'propertyKeyInvalid',reason:'a property key continuation retains its actual string hint'};
  return finishJavaScriptPropertyKeyConversion(stepJavaScriptPrimitiveConversion(heap,state));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptPropertyKeyConversionTransition} */
export function resumeJavaScriptPropertyKeyConversion(heap,state,completion){
  if(state.hint!=='string')return {$:'propertyKeyInvalid',reason:'a property key continuation retains its actual string hint'};
  return finishJavaScriptPropertyKeyConversion(resumeJavaScriptPrimitiveConversion(heap,state,completion));
}
