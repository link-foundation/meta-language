/** Immediate owned patterns cannot commit field moves before a fallible guard.
 * @param {boolean} owned @param {boolean} guarded @returns {string} */
export function readRustStandardSumGuardMode(owned,guarded){return owned&&guarded?'deferred guard':'ordinary';}
/** Or alternatives and whole-value aliases need their separate delayed move
 * agreement model; do not silently weaken the existing immediate-mode rule.
 * @param {number} alternatives @param {number} aliases @returns {string} */
export function readRustStandardSumDeferredGuardPatternError(alternatives,aliases){return alternatives!==1?'guarded owned sum alternatives need their delayed move agreement model':aliases!==0?'guarded owned sum whole-value aliases need their delayed payload move model':'';}
/** The first profile above remains a retained literal boundary. The actual
 * expanded reader validates all source bindings before committing any move.
 * @param {number} alternatives @param {number} aliases @returns {string} */
export function readRustStandardSumGuardAlternativePatternError(alternatives,aliases){return !Number.isInteger(alternatives)||alternatives<1||!Number.isInteger(aliases)||aliases<0?'guarded owned sum patterns need their actual finite alternative and alias counts':'';}
/** Facts are independent complete source type identities and root/loan records.
 * @param {boolean} complete @param {boolean} sameType @param {boolean} samePlace
 * @returns {string} */
export function readRustStandardSumGuardAlternativeBindingError(complete,sameType,samePlace){return !complete||!sameType?'guarded owned sum alternatives must bind the same complete actual source types':!samePlace?'guarded owned sum alternatives must retain the same actual root loans and lifetime provenance':'';}
/** A whole alias may own the value only when its subpatterns do not also bind
 * an owned field. Copy fields and wildcard inspection do not add a move.
 * @param {number} aliases @param {number} ownedBindings @returns {string} */
export function readRustStandardSumGuardAliasTransferError(aliases,ownedBindings){return aliases>0&&ownedBindings>1?'guarded owned sum whole-value aliases with owned field bindings need their partial-move agreement model':'';}
/** @param {boolean} deferred @param {boolean} owned @returns {string} */
export function readRustStandardSumGuardBindingDisposition(deferred,owned){return deferred?'readonly':owned?'transfer':'plain';}
