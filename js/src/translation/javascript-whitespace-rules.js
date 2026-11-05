// Shared ECMAScript WhiteSpace and LineTerminator unit predicate.
/** @param {number} unit @returns {boolean} */
export function acceptJavaScriptWhitespaceUnit(unit) {
  return unit >= 9 && unit <= 13 || unit === 32 || unit === 160 || unit === 5760 || unit >= 8192 && unit <= 8202 || unit === 8232 || unit === 8233 || unit === 8239 || unit === 8287 || unit === 12288 || unit === 65279;
}
