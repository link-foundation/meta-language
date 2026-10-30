// Split compound punctuation only where an actual type closing angle is needed.
/** @param {string} token @returns {string[]} */
export function readRustTypeClosingTokenParts(token){return token==='>>'?['>','>']:token==='>='?['>','=']:token==='>>='?['>','>','=']:[];}

/** Only a reference type or pattern context may split the Boolean token.
 * @param {string} token @returns {string[]} */
export function readRustReferenceTokenParts(token){return token==='&&'?['&','&']:[];}
