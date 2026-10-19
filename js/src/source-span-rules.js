// Generated from the shared source-span offset policy.
/** @param {number} value @param {boolean} negativeZero @returns {boolean} */
export function acceptSourceSpanOffset(value, negativeZero) { return !negativeZero && value >= 0 && value <= 9007199254740991 && value % 1 === 0; }
/** @param {number} start @param {number} end @returns {boolean} */
export function acceptSourceSpanInterval(start, end) { return start <= end; }
/** Actual token offsets have no negative-zero spelling.
 * @param {number} start @param {number} prefixEnd @param {number} consumedEnd @returns {boolean} */
export function acceptCompleteDeclarationSourceInterval(start,prefixEnd,consumedEnd){
 return acceptSourceSpanOffset(start,false) && acceptSourceSpanOffset(prefixEnd,false)
  && acceptSourceSpanOffset(consumedEnd,false) && acceptSourceSpanInterval(start,prefixEnd)
  && acceptSourceSpanInterval(prefixEnd,consumedEnd);
}
