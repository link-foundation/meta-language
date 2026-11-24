import {readRequiredBooleanFact} from './fact-column-rules.js';
/** @returns {number} */
export function readRustFunctionPointerParameterLimit(){return 1024;}
/** Actual source syntax guard for the additive owned-domain profile.
 * @param {string} value @param {string} raw @returns {boolean} */
export function acceptRustFunctionPointerSafeSource(value,raw){return value==='fn'&&raw==='fn';}
/** Owned-domain pointers remain Copy; their parameter/result values do not.
 * Roles must come from complete admitted direct source types, never from a
 * callable body's runtime IDs, loans, producer flags or guessed Copy facts.
 * @param {boolean} ordinary @param {string[]} parameters @param {string} result
 * @param {number} minor @returns {string} */
export function readRustFunctionPointerOwnedDomainError(ordinary,parameters,result,minor){
 if(!ordinary)return 'owned function pointers require the ordinary source profile';
 if(!Number.isInteger(minor)||minor<90||minor>=100)return 'owned function pointer source semantics are uncovered outside Rust >=1.90.0 <1.100.0';
 if(parameters.length>readRustFunctionPointerParameterLimit())return 'the complete pointer signature exceeds its source budget';
 for(let at=0;at<parameters.length;at+=1)if(parameters[at]!=='copy'&&parameters[at]!=='owned')return 'owned function pointer parameters need complete direct Copy or supported owned source types';
 if(result!=='copy'&&result!=='owned')return 'owned function pointer results need a complete direct Copy or supported owned source type';
 return parameters.includes('owned')||result==='owned'?'':'the owned function pointer profile needs an actual owned parameter or result';
}
/** The original Copy domain remains an independent unchanged oracle.
 * @param {boolean} ordinary @param {boolean[]} copies @param {boolean} resultCopy
 * @param {string[]} roles @param {string} resultRole @param {number} minor
 * @returns {string} */
export function readRustFunctionPointerDomainError(ordinary,copies,resultCopy,roles,resultRole,minor){
 const copyError=readRustFunctionPointerTypeError(ordinary,copies,resultCopy);if(copyError==='')return '';
 if(roles.length!==copies.length)return copyError;
 const ownedError=readRustFunctionPointerOwnedDomainError(ordinary,roles,resultRole,minor);
 return ownedError===''?'':roles.includes('owned')||resultRole==='owned'?ownedError:copyError;
}
/** @param {boolean} ordinary @param {boolean[]} copies @param {boolean} resultCopy
 * @param {string[]} roles @param {string} resultRole @param {number} minor
 * @param {boolean} header @returns {string} */
export function readRustFunctionPointerHeaderDomainError(ordinary,copies,resultCopy,roles,resultRole,minor,header){
 const actual=readRustFunctionPointerDomainError(ordinary,copies,resultCopy,roles,resultRole,minor);
 return actual===''?'':header?readRustFunctionPointerHeaderTypeError(ordinary,copies,resultCopy,header):actual;
}
/** @param {boolean} ordinary @param {boolean[]} parameters @param {boolean} result
 * @returns {string} */
export function readRustFunctionPointerTypeError(ordinary,parameters,result){
 if(!ordinary)return 'function pointers require their separate active source profile';
 if(parameters.length>readRustFunctionPointerParameterLimit())return 'the complete pointer signature exceeds its source budget';
 for(let index=0;index<parameters.length;index+=1)if(!readRequiredBooleanFact(parameters,index))return 'function pointer parameters require direct immutable Copy source types';
 return result?'':'function pointer results require direct immutable Copy source types';
}

/** Private header collection may await completed nominal/sum child origins.
 * The original domain checker remains the final admission oracle.
 * @param {boolean} ordinary @param {boolean[]} parameters @param {boolean} result
 * @param {boolean} header @returns {string} */
export function readRustFunctionPointerHeaderTypeError(ordinary,parameters,result,header){
 return header&&ordinary&&parameters.length<=readRustFunctionPointerParameterLimit()?'':readRustFunctionPointerTypeError(ordinary,parameters,result);
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
