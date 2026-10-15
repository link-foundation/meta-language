// Generated from the linked Rust source whitespace policy. Do not edit.
/** @param {number} unit @returns {boolean} */
export function acceptRustSourceWhitespace(unit) {
  return unit === 9 || unit === 10 || unit === 11 || unit === 12 || unit === 13 || unit === 32 || unit === 133 || unit === 8206 || unit === 8207 || unit === 8232 || unit === 8233;
}
/** @param {number} unit @param {boolean} firstPosition @returns {boolean} */
export function acceptRustSourceByteOrderMark(unit, firstPosition) {
  return firstPosition && unit === 65279;
}
