// Shared source decisions for standard owned sums.
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

/** Standard prelude/type/variant spellings after actual import resolution. A
 * catalog adapter must still reject local owners and namespace shadows.
 * @param {string} path @returns {string} */
export function readRustStandardSumSourcePathOwner(path){
 if(['Option','std::option::Option','core::option::Option','None','Some','Option::None','Option::Some','std::option::Option::None','std::option::Option::Some','core::option::Option::None','core::option::Option::Some'].includes(path))return 'std::option::Option';
 return ['Result','std::result::Result','core::result::Result','Ok','Err','Result::Ok','Result::Err','std::result::Result::Ok','std::result::Result::Err','core::result::Result::Ok','core::result::Result::Err'].includes(path)?'std::result::Result':'';
}

/** @param {string} path @returns {string} */
export function readRustStandardSumSourcePathConstructor(path){
 if(['None','Option::None','std::option::Option::None','core::option::Option::None'].includes(path))return 'None';
 if(['Some','Option::Some','std::option::Option::Some','core::option::Option::Some'].includes(path))return 'Some';
 if(['Ok','Result::Ok','std::result::Result::Ok','core::result::Result::Ok'].includes(path))return 'Ok';
 return ['Err','Result::Err','std::result::Result::Err','core::result::Result::Err'].includes(path)?'Err':'';
}

/** @returns {number} */
export function readRustStandardSumSourceInstantiationLimit(){return 256;}
/** @returns {number} */
export function readRustStandardSumSourceDepthLimit(){return 16;}

/** Concrete datatype names never consume expression/completion temporaries.
 * @param {number} ordinal @param {number} collision @returns {string} */
export function readRustStandardSumSourceSpecializationName(ordinal,collision){
 if(!Number.isInteger(ordinal)||ordinal<0||ordinal>=256||!Number.isInteger(collision)||collision<0||collision>=4096)return '';
 return 'ml_standard_sum_value'+String(ordinal)+'_'+String(collision);
}

/** Header inventory is private syntax context, never admitted Copy evidence.
 * @param {boolean} header @param {boolean} complete @param {boolean} copies
 * @returns {string} */
export function readRustStandardSumSourcePhase(header,complete,copies){
 return header?'provisional':complete&&copies?'admitted':'refuse';
}

/** Complete concrete child depths include saved aliases and callable/products.
 * @param {number[]} children @param {boolean} sum @returns {number} */
export function readRustStandardSumSourceDepth(children,sum){
 let depth=0;
 for(const child of children){if(!Number.isInteger(child)||child<0||child>16)return -1;if(child>depth)depth=child;}
 const result=depth+(sum?1:0);return result<=16?result:-1;
}

/** Charge complete source metadata before copying it. Units are UTF16 code
 * units, not encoded bytes. These finite bounds are independent of type depth.
 * @param {number} nodes @param {number} units @param {number} additionalNodes
 * @param {number} additionalUnits @returns {string} */
export function readRustStandardSumSourceMaterializationError(nodes,units,additionalNodes,additionalUnits){
 for(const value of [nodes,units,additionalNodes,additionalUnits])if(!Number.isInteger(value)||value<0)return 'invalid standard sum source materialization count';
 return nodes>1048576||units>8388608||additionalNodes>1048576-nodes||additionalUnits>8388608-units?'the complete standard sum source metadata exceeds its materialization budget':'';
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

/** Only this constructor's one actual payload determines every type argument.
 * Other variants require an explicit or contextual complete sum type.
 * @param {string} owner @param {string} constructor @returns {boolean} */
export function acceptRustStandardSumSourceInferredConstructor(owner,constructor){
 return owner==='std::option::Option'&&constructor==='Some';
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
