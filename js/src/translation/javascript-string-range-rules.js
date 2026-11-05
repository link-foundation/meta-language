// Complete checked-operator plans; hosts supply actual captured numeric operands.
/** @param {string} value @returns {string[]} */
function planStringRangeInteger(value){return [value,value,'eq',value,'trunc','0','if'];}
/** @param {string} value @returns {string[]} */
function planStringRangeNonnegative(value){return planStringRangeInteger(value).concat(['0','max']);}
/** @returns {string[]} */
function planStringRangeRelativeStart(){
 return planStringRangeInteger('start').concat(['0','lt','length'],planStringRangeInteger('start'),['add','0','max'],planStringRangeInteger('start'),['length','min','if']);
}
/** The two final stack values are the complete slice window. NaN becomes zero
 * before substring endpoint ordering; substr uses a relative start and length.
 * @param {string} operation @returns {string[]} */
export function planJavaScriptStringRangeWindow(operation){
 if(operation==='slice')return ['start','end'];
 if(operation==='substring')return planStringRangeNonnegative('start').concat(planStringRangeNonnegative('end'),['min'],planStringRangeNonnegative('start'),planStringRangeNonnegative('end'),['max']);
 if(operation==='substr')return planStringRangeRelativeStart().concat(planStringRangeRelativeStart(),planStringRangeNonnegative('end'),['add']);
 return [];
}
/** @param {string} operation @returns {boolean} */
export function acceptJavaScriptStringRangeOperation(operation){return operation==='slice'||operation==='substring'||operation==='substr';}
