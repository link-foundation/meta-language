/** Immediate owned patterns cannot commit field moves before a fallible guard.
 * @param {boolean} owned @param {boolean} guarded @returns {string} */
export function readRustStandardSumGuardMode(owned,guarded){return owned&&guarded?'deferred guard':'ordinary';}
/** Or alternatives and whole-value aliases need their separate delayed move
 * agreement model; do not silently weaken the existing immediate-mode rule.
 * @param {number} alternatives @param {number} aliases @returns {string} */
export function readRustStandardSumDeferredGuardPatternError(alternatives,aliases){return alternatives!==1?'guarded owned sum alternatives need their delayed move agreement model':aliases!==0?'guarded owned sum whole-value aliases need their delayed payload move model':'';}
/** @param {boolean} deferred @param {boolean} owned @returns {string} */
export function readRustStandardSumGuardBindingDisposition(deferred,owned){return deferred?'readonly':owned?'transfer':'plain';}
