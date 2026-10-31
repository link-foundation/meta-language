// Source draft for standard owned sums. Runtime adapters are not integrated yet.
// Callers must establish resolved definition ownership from the actual catalog,
// direct immutable Copy payload origins and complete checked type identities.
// These scalar projections neither create nor replace any certificate.
// Instantiate the existing named datatype/constructor/match algebra. In
// particular, types.option is unsuitable: it collapses nested optional values.

/** A resolved standard definition, never an unresolved caller spelling.
 * @param {string} path @param {boolean} standardDefinition
 * @param {number} layers @returns {string} */
export function readRustStandardSumSourceOwner(path,standardDefinition,layers){
 if(!standardDefinition||layers!==0)return '';
 return path==='std::option::Option'||path==='core::option::Option'?'std::option::Option':path==='std::result::Result'||path==='core::result::Result'?'std::result::Result':'';
}

/** @param {string} owner @returns {number} */
export function readRustStandardSumSourceArgumentCount(owner){
 return owner==='std::option::Option'?1:owner==='std::result::Result'?2:-1;
}

/** The argument identities come from complete checked origin structures, with
 * separate entries even when their meaning agrees. A nested sum remains an
 * actual child datatype; this function performs no identity construction.
 * @param {string} owner @param {string[]} identities
 * @param {boolean[]} directCopies @returns {boolean} */
export function acceptRustStandardSumSourceArguments(owner,identities,directCopies){
 const count=readRustStandardSumSourceArgumentCount(owner);
 if(count<0||identities.length!==count||directCopies.length!==count)return false;
 for(let at=0;at<count;at+=1)if((identities[at]??'')===''||directCopies[at]!==true)return false;
 return true;
}

/** Exact original constructor order for the existing datatype declaration.
 * Missing/extra constructors must not be hidden by an optional-value encoding.
 * @param {string} owner @returns {string[]} */
export function readRustStandardSumSourceConstructors(owner){
 return owner==='std::option::Option'?['None','Some']:owner==='std::result::Result'?['Ok','Err']:[];
}

/** Zero-based actual type argument for the sole field; -1 is a nullary
 * constructor and -2 refuses. The adapter retains actual constructor ownership
 * separately, before passing its selected local variant name here.
 * @param {string} owner @param {string} constructor @returns {number} */
export function readRustStandardSumSourceFieldArgument(owner,constructor){
 if(owner==='std::option::Option')return constructor==='None'?-1:constructor==='Some'?0:-2;
 if(owner==='std::result::Result')return constructor==='Ok'?0:constructor==='Err'?1:-2;
 return -2;
}

/** @param {string} owner @param {string} constructor @returns {number} */
export function readRustStandardSumSourceConstructorArity(owner,constructor){
 const argument=readRustStandardSumSourceFieldArgument(owner,constructor);
 return argument===-2?-1:argument===-1?0:1;
}

/** Narrow question-mark context: an ordinary enclosing callable, matching
 * resolved sum families, direct Copy operand fields, and identical complete
 * error identities for Result. Option payload/result types may differ.
 * Identity strings must be the checker's collision-free structural identities,
 * never names, widths, source snippets or fabricated truth-valued proofs.
 * Custom Try/FromResidual, conversion errors and indirect values stay refused.
 * @param {string} operandOwner @param {string} returnOwner
 * @param {boolean} ordinaryCallable @param {boolean} directCopyOperand
 * @param {string} operandError @param {string} returnError @returns {boolean} */
export function acceptRustStandardSumQuestionContext(operandOwner,returnOwner,ordinaryCallable,directCopyOperand,operandError,returnError){
 if(!ordinaryCallable||!directCopyOperand||operandOwner!==returnOwner)return false;
 if(operandOwner==='std::option::Option')return operandError===''&&returnError==='';
 return operandOwner==='std::result::Result'&&operandError!==''&&operandError===returnError;
}

/** These are private completion-tree dispositions, not semantic IR operations.
 * The adapter must first admit the complete question context above, capture
 * the operand once, and use actual ctor/match terms with both branches. A
 * return disposition exits the nearest callable through its existing return
 * packet; it cannot become an ordinary expression-level conditional value.
 * @param {string} owner @param {string} constructor @returns {string} */
export function readRustStandardSumQuestionBranch(owner,constructor){
 if(readRustStandardSumSourceConstructorArity(owner,constructor)<0)return '';
 if(owner==='std::option::Option')return constructor==='Some'?'continue field':'return absent';
 return constructor==='Ok'?'continue field':'return error field';
}
