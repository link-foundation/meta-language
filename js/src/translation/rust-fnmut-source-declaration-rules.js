/** @param {string} model @param {number} minor @returns {string} */
export function readRustFnMutSourceModelError(model,minor){return model!=='Rust closed source FnMut specializations 1'?'select the actual closed source FnMut specialization model':!Number.isInteger(minor)||minor<90||minor>=100?'closed source FnMut specialization is uncovered outside Rust >=1.90 <1.100':'';}
/** @param {number} tokens @param {number} parameters @param {number} generics
 * @param {number} callbacks @returns {string} */
export function readRustFnMutSourceDeclarationBudgetError(tokens,parameters,generics,callbacks){return !Number.isSafeInteger(tokens)||tokens<0||tokens>1000000||!Number.isSafeInteger(parameters)||parameters<0||parameters>256||!Number.isSafeInteger(generics)||generics<0||generics>32||callbacks!==1?'a complete FnMut source declaration needs one actual callback and bounded tokens, parameters and concrete generic arguments':'';}
