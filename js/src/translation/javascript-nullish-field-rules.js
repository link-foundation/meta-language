// Ordinary property reads throw on null/undefined. Optional chains choose
// their skipped branch separately; returning nullish is not that skip.
import {readJavaScriptOptionalChainForm} from './javascript-optional-chain-rules.js';
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptNullishFieldForm(kind,element){return readJavaScriptOptionalChainForm(kind,element);}
/** Version-qualified V8 NonObjectPropertyLoadWithProperty template.
 * @param {string} absence @param {string} field @returns {string} */
export function readJavaScriptNullishFieldMessage(absence,field){return readJavaScriptNullishPropertyMessagePrefix(absence,true)+field+readJavaScriptNullishPropertyMessageSuffix(true);}
/** @param {string} absence @param {boolean} named @returns {string} */
export function readJavaScriptNullishPropertyMessagePrefix(absence,named){return 'Cannot read properties of '+absence+(named?" (reading '":'');}
/** @param {boolean} named @returns {string} */
export function readJavaScriptNullishPropertyMessageSuffix(named){return named?"')":'';}
/** Side-effect-free V8 diagnostic descriptions, not ToPropertyKey. Function
 * source spelling, custom constructors, symbols and unknown objects retain
 * their own protocols. A key object's own toString method is never invoked.
 * @param {string} kind @param {boolean} ownsToString @param {boolean} ownsConstructor @returns {string} */
export function readJavaScriptNullishKeyDescription(kind,ownsToString,ownsConstructor){
 if(kind==='string'||kind==='float'||kind==='int'||kind==='nat'||kind==='bool'||kind==='unit'||kind==='option')return 'text';
 if(kind==='array'||kind==='tuple')return 'omitted';
 if(kind==='record')return ownsToString?'omitted':ownsConstructor?'unsupported':'object';
 if(kind==='map')return 'map';
 return kind==='set'?'set':'unsupported';
}
/** @returns {string} */
export function readJavaScriptNullishIndexKeyField(){return 'key';}
/** Object diagnostics require the actual literal/collection constructor;
 * structural parameter types alone do not prove their runtime prototype.
 * @param {string} form @param {string} kind @returns {boolean} */
export function acceptJavaScriptNullishKeyDescriptionOrigin(form,kind){
 if(form==='text')return true;
 if(form==='object')return kind==='record';
 if(form==='map')return kind==='map';
 if(form==='set')return kind==='set';
 return form==='omitted'&&(kind==='record'||kind==='array'||kind==='tuple');
}
/** @param {string} form @returns {string} */
export function readJavaScriptNullishKeyDescriptionText(form){
 if(form==='object')return '#<Object>';
 if(form==='map')return '#<Map>';
 return form==='set'?'#<Set>':'';
}
/** V8 x64/arm64 NoSideEffectsToString limits the *magnitude* to 100
 * 64-bit words. This source-engine profile is independent of the Rust target.
 * @returns {string} */
export function readJavaScriptBigIntegerPropertyDescriptionShift(){return '6400';}
/** @returns {string} */
export function readJavaScriptBigIntegerPropertyDescriptionOverflow(){return '<a very large BigInt>';}
/** @returns {string} */
export function readJavaScriptNullishFieldCapture(){return 'receiver';}
/** A live JavaScript Array slice allocates a different reference handle.
 * @param {string} language @param {string} kind @param {boolean} reference @returns {boolean} */
export function requireJavaScriptSliceReferenceCopy(language,kind,reference){return language==='JavaScript'&&kind==='array'&&reference;}
