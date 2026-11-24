/** @param {string} model @returns {boolean} */
export function acceptRustRetainedSourceSignatureModel(model){return model==='Rust source function signature 2';}
/** @param {string} model @param {boolean} retain @returns {string} */
export function selectRustRetainedSourceSignatureModel(model,retain){return model!==''?model:retain?'Rust source function signature 2':'';}
/** Version one retains its original strict function-owner refusal.
 * @param {string} model @param {string} owner @param {string} type
 * @param {number} paths @param {number} children @param {number} parameters
 * @param {boolean} reference @returns {boolean} */
export function acceptRustRetainedFunctionOwner(model,owner,type,paths,children,parameters,reference){return acceptRustRetainedSourceSignatureModel(model)&&owner==='function'&&type==='function'&&paths===0&&Number.isInteger(parameters)&&parameters>=0&&parameters<=100000&&children===parameters+1&&!reference;}
