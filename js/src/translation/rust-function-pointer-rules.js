import {readRequiredBooleanFact} from './fact-column-rules.js';
/** @param {boolean} ordinary @param {boolean[]} parameters @param {boolean} result
 * @returns {string} */
export function readRustFunctionPointerTypeError(ordinary,parameters,result){
 if(!ordinary)return 'function pointers require their separate active source profile';
 if(parameters.length>1024)return 'the complete pointer signature exceeds its source budget';
 for(let index=0;index<parameters.length;index+=1)if(!readRequiredBooleanFact(parameters,index))return 'function pointer parameters require direct immutable Copy source types';
 return result?'':'function pointer results require direct immutable Copy source types';
}
/** Equal callable signatures do not supply a noncapturing-closure witness.
 * @param {boolean} pointer @param {string} actual @param {boolean} closure
 * @param {number} captures @param {boolean} proven @returns {string} */
export function readRustFunctionPointerCoercionError(pointer,actual,closure,captures,proven){
 if(!pointer)return '';
 if(actual!=='function'||!proven)return 'a pointer argument needs its actual callable source proof';
 return closure&&captures!==0?'capturing closures cannot coerce to function pointers':'';
}
/** @param {string} kind @returns {string} */
export function readRustFunctionPointerObservationError(kind){return kind==='function'?'callable identity, address and numeric casts require their actual source representation':'';}
