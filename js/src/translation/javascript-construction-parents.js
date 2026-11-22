// Only an actual constructor operation in the selected parent grants a child
// Construct context. Older species states retain their separate adapter path.
import {readJavaScriptActiveConstruction} from './javascript-construction.js';
/** @param {JavaScriptCallPending} pending @returns {JavaScriptConstructionState|undefined} */
export function readJavaScriptPendingConstruction(pending){
 switch(pending.$){
  case 'waitConstruction':return readJavaScriptActiveConstruction(pending.state);
  case 'waitArrayCreation':{
   const construction=pending.construction;
   return construction===undefined?undefined:readJavaScriptActiveConstruction(construction);
  }
  case 'waitArrayTransform':
  case 'waitArraySlice':
  case 'waitArrayConcatenation':{
   const operation=pending.construction?.operation;
   return operation===undefined?undefined:readJavaScriptActiveConstruction(operation);
  }
  default:return undefined;
 }
}
