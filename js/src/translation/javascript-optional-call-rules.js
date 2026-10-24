// Complete source selections for lexical optional callable application.
/** @returns {string} */
export function readJavaScriptOptionalCallOperation(){return 'invokeJavaScriptOptionalCallable';}
/** A property reference must keep its original this binding and chain extent.
 * @param {string} kind @param {number} pathLength @returns {boolean} */
export function acceptJavaScriptOptionalCallHead(kind,pathLength){
 return kind==='name'&&pathLength>=1||kind==='functionValue'||kind==='lambda'||kind==='app'||kind==='let'||kind==='if'||kind==='math'||kind==='none'||kind==='unit';
}
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptOptionalCallForm(kind,element){
 if(kind==='function')return 'direct';
 if(kind==='unit'||kind==='option'&&element==='unit')return 'absent';
 return kind==='option'&&element==='function'?'guarded':'unsupported';
}
/** @param {string} kind @returns {string} */
export function readJavaScriptOptionalCallResultForm(kind){return kind==='unit'?'undefined':kind==='option'?'flatten':'present';}
/** @returns {string} */
export function readJavaScriptOptionalCallField(){return 'callable';}
/** Chained results need a separate skipped-chain/returned-value distinction.
 * @param {string} suffix @returns {boolean} */
export function requireJavaScriptOptionalCallChain(suffix){return suffix==='.'||suffix==='?.'||suffix==='['||suffix==='(';}
