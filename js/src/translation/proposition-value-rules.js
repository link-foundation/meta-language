// A Boolean value of an existing checked proposition. No new equality,
// quantifier sampling or target-specific comparison semantics are introduced.
/** @param {string} kind @returns {string} */
export function readPropositionValueForm(kind){if(kind==='bool')return 'value';if(kind==='not')return 'negation';if(kind==='and'||kind==='or'||kind==='implies')return kind;if(kind==='forall')return 'finite quantification';return acceptPropositionValueComparison(kind)?'comparison':'';}
/** @param {string} operation @param {boolean} truth @returns {string} */
export function readPropositionValueConditionalBranch(operation,truth){if(operation==='and')return truth?'right':'false';if(operation==='or')return truth?'true':'right';return operation==='implies'?(truth?'right':'true'):'';}
/** @returns {string} */
export function readPropositionValueQuantifierJoin(){return 'and';}
/** @param {string} operation @returns {boolean} */
export function acceptPropositionValueComparison(operation){return operation==='eq'||operation==='ne'||operation==='lt'||operation==='le'||operation==='gt'||operation==='ge';}
/** Only these existing executable domains enumerate every value exactly.
 * @param {string} kind @returns {number[]} */
export function planPropositionValueQuantifier(kind){return kind==='bool'?[0,1]:kind==='unit'?[0]:[];}
/** @returns {number} */
export function readPropositionValueExpansionLimit(){return 256;}
/** @param {string} kind @returns {string} */
export function readPropositionValueDomainError(kind){return planPropositionValueQuantifier(kind).length?'':'a proposition value requires an exhaustive Bool or Unit quantifier domain';}
/** Existing formal equality is decided, rather than replaced with numeric,
 * display-string or object-identity equality.
 * @param {string} equality @param {boolean} negate @returns {string} */
export function renderRocqPropositionEqualityValue(equality,negate){const decision='(if (ltac:(decide equality) : {'+equality+'} + {~ ('+equality+')}) then true else false)';return negate?'(negb '+decision+')':decision;}
