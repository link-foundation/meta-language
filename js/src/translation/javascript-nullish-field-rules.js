// Ordinary property reads throw on null/undefined. Optional chains choose
// their skipped branch separately; returning nullish is not that skip.
import {readJavaScriptOptionalChainForm} from './javascript-optional-chain-rules.js';
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptNullishFieldForm(kind,element){return readJavaScriptOptionalChainForm(kind,element);}
/** Version-qualified V8 NonObjectPropertyLoadWithProperty template.
 * @param {string} absence @param {string} field @returns {string} */
export function readJavaScriptNullishFieldMessage(absence,field){return "Cannot read properties of "+absence+" (reading '"+field+"')";}
/** @returns {string} */
export function readJavaScriptNullishFieldCapture(){return 'receiver';}
