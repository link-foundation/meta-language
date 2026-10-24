// Primitive source comparison decisions; hosts supply actual typed operands.
/** @param {string} kind @returns {boolean} */
export function acceptJavaScriptComparisonPrimitive(kind){
 return kind==='float'||kind==='int'||kind==='nat'||kind==='bool'||kind==='string'||kind==='unit';
}
/** Existing homogeneous primitive operations already preserve their domain.
 * @param {string} left @param {string} right @returns {boolean} */
export function requireJavaScriptPrimitiveComparison(left,right){
 if(left==='option'||right==='option')return true;
 if(left===right&&left!=='unit')return false;
 return !((left==='int'||left==='nat')&&(right==='int'||right==='nat'));
}
/** @param {string} absence @returns {string} */
export function readJavaScriptComparisonAbsentNumber(absence){
 return absence==='null'?'0':absence==='undefined'?'NaN':'';
}
/** JavaScript Unit is the undefined value, including an effectful void call.
 * @param {string} kind @returns {string} */
export function readJavaScriptComparisonUnitAbsence(kind){return kind==='unit'?'undefined':'';}
/** Actual unknowns wait for call-site constraints before selecting defaults.
 * @param {string} left @param {string} right @param {boolean} force @returns {string} */
export function readJavaScriptComparisonInferenceStep(left,right,force){
 if(left==='var'||right==='var')return force?'unify':'defer';
 return (acceptJavaScriptComparisonPrimitive(left)||left==='option')&&(acceptJavaScriptComparisonPrimitive(right)||right==='option')?'independent':'unify';
}
/** @param {string} operation @param {string} left @param {string} right @returns {string} */
export function readJavaScriptPrimitiveComparisonForm(operation,left,right){
 if(operation==='eq'||operation==='ne')return 'strict';
 const a=left==='int'||left==='nat',b=right==='int'||right==='nat';
 if(left==='string'&&right==='string')return 'string';
 if(left==='string'&&b)return 'string-integer';
 if(a&&right==='string')return 'integer-string';
 if(a&&b)return 'integer';
 if(left==='float'&&right==='float')return 'number';
 if(left==='float'&&b)return 'number-integer';
 return a&&right==='float'?'integer-number':'coerce';
}
/** Unordered has code two and never satisfies an ordering relation.
 * @param {string} operation @param {boolean} reverse @returns {number[]} */
export function readJavaScriptComparisonAcceptedOrders(operation,reverse){
 if(operation==='eq')return [0];
 if(operation==='ne')return [-1,1,2];
 if(operation==='lt')return [reverse?1:-1];
 if(operation==='gt')return [reverse?-1:1];
 if(operation==='le')return [reverse?1:-1,0];
 return operation==='ge'?[reverse?-1:1,0]:[];
}
/** @param {string} symbol @returns {string} */
export function readJavaScriptLooseEqualityOperation(symbol){
 return symbol==='=='?'testJavaScriptPrimitiveLooseEquality':symbol==='!='?'testJavaScriptPrimitiveLooseInequality':'';
}
/** @param {string} operation @returns {boolean} */
export function acceptJavaScriptLooseEqualityOperation(operation){
 return operation==='testJavaScriptPrimitiveLooseEquality'||operation==='testJavaScriptPrimitiveLooseInequality';
}
/** @param {string} operation @returns {boolean} */
export function requireJavaScriptLooseEqualityNegation(operation){return operation==='testJavaScriptPrimitiveLooseInequality';}
/** Absence branches retain their null/undefined tags before this decision.
 * @param {string} left @param {string} right @returns {string} */
export function readJavaScriptPrimitiveLooseEqualityForm(left,right){
 if(left===right)return 'strict';
 const a=left==='int'||left==='nat',b=right==='int'||right==='nat';
 if(a&&b)return 'integer';
 if(left==='bool'||left==='string'&&right==='float')return 'left-number';
 if(right==='bool'||right==='string'&&left==='float')return 'right-number';
 if(left==='string'&&b)return 'string-integer';
 if(a&&right==='string')return 'integer-string';
 if(left==='float'&&b)return 'number-integer';
 return a&&right==='float'?'integer-number':'different';
}
