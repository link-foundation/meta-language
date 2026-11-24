/** @param {string} model @param {number} minor @returns {string} */
export function readRustFnMutSourceModelError(model,minor){return model!=='Rust closed source FnMut specializations 1'?'select the actual closed source FnMut specialization model':!Number.isInteger(minor)||minor<90||minor>=100?'closed source FnMut specialization is uncovered outside Rust >=1.90 <1.100':'';}
/** @param {number} tokens @param {number} parameters @param {number} generics
 * @param {number} callbacks @returns {string} */
export function readRustFnMutSourceDeclarationBudgetError(tokens,parameters,generics,callbacks){return !Number.isSafeInteger(tokens)||tokens<0||tokens>1000000||!Number.isSafeInteger(parameters)||parameters<0||parameters>256||!Number.isSafeInteger(generics)||generics<0||generics>32||callbacks!==1?'a complete FnMut source declaration needs one actual callback and bounded tokens, parameters and concrete generic arguments':'';}

/** @param {string} owner @param {number} index @returns {boolean} */
export function selectRustFnMutOpaqueErrorPosition(owner, index) {
  return owner === 'std::result::Result' && index === 1;
}

/** @param {number} scopes @param {number} depth @param {boolean} startsArgument @returns {string} */
export function readRustFnMutOpaqueBinderError(scopes, depth, startsArgument) {
  return scopes === 1 && depth === 1 && startsArgument ? '' : 'an unconstrained concrete type binder is admitted only as a bare standard Result error position';
}

/** @param {string} kind @param {string} value @returns {string} */
export function readRustFnMutOpaqueBodyTokenError(kind, value) {
  return kind === 'identifier' && value === 'Err' || kind === 'macro' && value === 'vec' ? 'explicit generic error extraction/construction and inferred vector aggregation need their separate universal source interpretation' : '';
}

/** @param {string} operation @returns {boolean} */
export function acceptRustFnMutOpaqueObservation(operation) {
  return operation === 'is_ok' || operation === 'is_err' || operation === 'is_some' || operation === 'is_none';
}
