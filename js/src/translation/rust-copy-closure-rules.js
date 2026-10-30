import {readRequiredBooleanFact,readRequiredTextFact} from './fact-column-rules.js';
/** Actual syntax roles only; a wildcard consumes a value without a binding.
 * @param {string} role @returns {string} */
export function readRustCopyClosureParameterForm(role){return role==='_'?'wildcard':role==='('?'product':role==='binding'?'binding':'unsupported';}
/** A distinct ordinary immutable source profile, with no inferred ownership.
 * @param {boolean} ordinary @param {string[]} names @param {boolean[]} copies
 * @param {boolean} resultWritten @param {boolean} resultCopy @returns {string} */
export function readRustCopyClosureHeaderError(ordinary,names,copies,resultWritten,resultCopy){
 const error=readRustCopyClosureParameterHeaderError(ordinary,names,copies);if(error)return error;
 if(!resultWritten)return 'the closure result requires its actual explicit type';
 return resultCopy?'':'the closure result requires a direct immutable Copy source type';
}
/** @param {boolean} collecting @param {boolean[]} copies @param {number} assigned @returns {string} */
export function readRustCopyClosureCaptureError(collecting,copies,assigned){
 if(assigned!==0)return 'mutable closure captures require their shared ownership interpretation';
 if(!collecting)for(let index=0;index<copies.length;index+=1)if(!readRequiredBooleanFact(copies,index))return 'closure captures require proven direct immutable Copy source values';
 return '';
}
/** @param {string} kind @returns {boolean} */
export function acceptRustCopyClosureInvocation(kind){return kind==='function';}
/** Anonymous closure identity never follows from equal callable signatures.
 * @param {boolean} first @param {boolean} second @param {boolean} same
 * @returns {string} */
export function readRustCopyClosureJoinError(first,second,same){return (first||second)&&!same?'distinct anonymous closure types require an actual source coercion':'';}

/** Header checks do not assert that an unresolved result type is known.
 * @param {boolean} ordinary @param {string[]} names @param {boolean[]} copies
 * @returns {string} */
export function readRustCopyClosureParameterHeaderError(ordinary,names,copies){
 if(!ordinary)return 'closure ownership requires its separate active source profile';
 if(names.length!==copies.length||names.length>1024)return 'the complete typed closure header exceeds its source budget';
 for(let index=0;index<names.length;index+=1){if(!readRequiredBooleanFact(copies,index))return 'closure parameters require direct immutable Copy source types';for(let before=0;before<index;before+=1)if(readRequiredTextFact(names,before)===readRequiredTextFact(names,index))return 'closure parameters require distinct actual bindings';}
 return '';
}
/** @param {boolean} requested @param {boolean} written @param {boolean} known
 * @returns {boolean} */
export function testRustCopyClosureDeferredResult(requested,written,known){return requested&&!written&&!known;}
/** Admission is for actual partial source syntax, never a complete value type.
 * @param {string} kind @param {number} parameters @param {number} layers
 * @param {boolean} identity @returns {string} */
export function readRustCopyClosureDeferredSourceError(kind,parameters,layers,identity){return kind==='lambda'&&parameters===0&&layers===0&&identity?'':'a deferred fallback needs its actual zero-argument closure source and identity';}
/** @param {string} kind @param {boolean} pending @param {boolean} known
 * @param {boolean} copy @returns {string} */
export function readRustCopyClosureDeferredResultError(kind,pending,known,copy){return kind==='lambda'&&pending&&known&&copy?'':'a deferred closure needs its actual complete immutable Copy result context';}

/** Only source positions that inherit the result's type are traversed.
 * @param {string} kind @param {string} operation @returns {string} */
export function readRustCopyClosureResultContextRole(kind,operation){return kind==='let'?'binding':kind==='name'?'name':kind==='if'?'branches':kind==='match'?'rows':kind==='unary'&&operation==='neg'?'operand':kind==='binary'&&['add','sub','mul','div','rem'].includes(operation)?'operands':'leaf';}
/** Unsuffixed integer syntax cannot silently acquire a float or Boolean type.
 * @param {string} kind @param {boolean} negative @param {boolean} typed
 * @param {string} result @param {boolean} signed @returns {string} */
export function readRustCopyClosureLiteralResultError(kind,negative,typed,result,signed){if(negative&&result==='fixed'&&!signed)return 'an unsigned Rust result cannot use unary negation';return kind==='num'&&!typed&&result!=='fixed'?'an unsuffixed Rust integer literal needs an actual integer result context':'';}

/** Lexical scope frames supply actual binder identity; spelling alone is not global identity.
 * @param {string[]} path @param {string} name @returns {boolean} */
export function testRustCopyClosureResultBinding(path,name){return path.length===1&&path[0]===name;}

/** Reference-pattern tokens retain their actual immutable layer count.
 * @param {string} token @returns {number} */
export function readRustCopyClosureReferencePatternDepth(token){return token==='&'?1:token==='&&'?2:0;}
/** Copy scalar destructuring retains the callable parameter's full signature.
 * @param {number} depth @param {number} layers @param {boolean} references
 * @param {boolean} parameter @returns {string} */
export function readRustCopyClosureReferencePatternError(depth,layers,references,parameter){
 if(depth===0)return '';
 return Number.isInteger(depth)&&depth>=1&&depth<=256&&Number.isInteger(layers)&&layers>=depth&&layers<=256&&references&&parameter?'':'a closure reference pattern needs its actual immutable Copy scalar reference layers';
}

/** Complete actual source bindings include names introduced by product patterns.
 * @param {string[]} names @returns {string} */
export function readRustCopyClosurePatternBindingError(names){
 if(names.length>1024)return 'the complete typed closure header exceeds its source budget';
 for(let index=0;index<names.length;index+=1)for(let before=0;before<index;before+=1)if(readRequiredTextFact(names,before)===readRequiredTextFact(names,index))return 'closure parameters require distinct actual bindings';
 return '';
}
