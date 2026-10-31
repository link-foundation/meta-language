import {readRequiredBooleanFact} from './fact-column-rules.js';
/** @param {boolean} ordinary @param {boolean[]} parameters @param {boolean} result
 * @returns {string} */
export function readRustFunctionPointerTypeError(ordinary,parameters,result){
 if(!ordinary)return 'function pointers require their separate active source profile';
 if(parameters.length>1024)return 'the complete pointer signature exceeds its source budget';
 for(let index=0;index<parameters.length;index+=1)if(!readRequiredBooleanFact(parameters,index))return 'function pointer parameters require direct immutable Copy source types';
 return result?'':'function pointer results require direct immutable Copy source types';
}

/** Private header collection may await completed nominal/sum child origins.
 * The original domain checker remains the final admission oracle.
 * @param {boolean} ordinary @param {boolean[]} parameters @param {boolean} result
 * @param {boolean} header @returns {string} */
export function readRustFunctionPointerHeaderTypeError(ordinary,parameters,result,header){
 return header&&ordinary&&parameters.length<=1024?'':readRustFunctionPointerTypeError(ordinary,parameters,result);
}

/** Safe fn pointers are Copy independently of the signature's value domains.
 * This leaf is available only to the actual parsed private header inventory.
 * @param {string} kind @param {number} layers @param {boolean} header
 * @returns {boolean} */
export function acceptRustFunctionPointerHeaderCopy(kind,layers,header){
 return kind==='function'&&layers===0&&header;
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
