// Only an actual constructor operation in the selected parent grants a child
// Construct context. Older species states retain their separate adapter path.
/** @param {JavaScriptCallPending} pending @returns {JavaScriptConstructionState|undefined} */
export function readJavaScriptPendingConstruction(pending){
 if(pending.$==='waitConstruction')return pending.state;
 if(pending.$==='waitArrayCreation')return pending.construction;
 if(pending.$==='waitArrayTransform'||pending.$==='waitArraySlice'||pending.$==='waitArrayConcatenation')return pending.construction?.operation;
 return undefined;
}
