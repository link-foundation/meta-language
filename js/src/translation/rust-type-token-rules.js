// Split compound punctuation only where an actual type closing angle is needed.
/** @param {string} token @returns {string[]} */
export function readRustTypeClosingTokenParts(token){return token==='>>'?['>','>']:token==='>='?['>','=']:token==='>>='?['>','>','=']:[];}
