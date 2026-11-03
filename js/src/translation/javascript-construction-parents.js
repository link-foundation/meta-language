// Only an actual constructor operation in the selected parent grants a child
// Construct context. Older species states retain their separate adapter path.
import {readJavaScriptActiveConstruction} from './javascript-construction.js';
/** @param {JavaScriptCallPending} pending @returns {JavaScriptConstructionState|undefined} */
export function readJavaScriptPendingConstruction(pending){
 if(pending.$==='waitConstruction')return readJavaScriptActiveConstruction(pending.state);
 if(pending.$==='waitArrayCreation')return pending.construction===undefined?undefined:readJavaScriptActiveConstruction(pending.construction);
 if(pending.$==='waitArrayTransform'||pending.$==='waitArraySlice'||pending.$==='waitArrayConcatenation')return pending.construction?.operation===undefined?undefined:readJavaScriptActiveConstruction(pending.construction.operation);
 return undefined;
}
