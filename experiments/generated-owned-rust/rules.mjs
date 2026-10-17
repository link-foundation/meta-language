// Generated from the private linked whole owned Rust policy. Do not edit.
/** @param {string} model @returns {boolean} */
export function acceptGeneratedOwnedRustModel(model){return model === "closed generated owned Rust module 1";}
/** @param {boolean} standard @param {boolean} string @param {boolean} receiver @param {number} arity @returns {boolean} */
export function acceptGeneratedOwnedRustClone(standard,string,receiver,arity){return standard&&string&&receiver&&arity === 0;}
/** @param {number} value @param {boolean} negativeZero @returns {boolean} */
export function acceptGeneratedOwnedRustCodeUnit(value,negativeZero){return !negativeZero&&value >= 0&&value <= 65535&&value % 1 === 0;}
/** @param {number} value @returns {boolean} */
export function acceptGeneratedOwnedRustLiteralCodePoint(value){return value >= 32 || value === 9 || value === 10 || value === 13;}
/** @param {boolean} comparison @param {boolean} parenthesized @returns {boolean} */
export function acceptGeneratedOwnedRustEqualityGrouping(comparison,parenthesized){return !comparison||parenthesized;}
/** @param {boolean} line @param {number} third @param {number} fourth @returns {boolean} */
export function acceptGeneratedOwnedRustOrdinaryComment(line,third,fourth){return third !== 33 && (line ? third !== 47 || fourth === 47 : third !== 42 || fourth === 42 || fourth === 47);}
