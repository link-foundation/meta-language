// Generated from the shared linked Rust cooked-string escape policy. Do not edit.
/** @param {number} unit @returns {boolean} */
export function acceptRustCookedStringEscapePrefix(unit) {
  return unit === 34 || unit === 39 || unit === 48 || unit === 92 || unit === 110 || unit === 114 || unit === 116 || unit === 117 || unit === 120;
}

/** @param {number} unit @returns {number} */
export function readRustAsciiEscapeDigit(unit) {
  if (unit % 1 !== 0) return -1;
  if (unit >= 48 && unit <= 57) return unit - 48 + 0;
  if (unit >= 65 && unit <= 70) return unit - 65 + 10;
  if (unit >= 97 && unit <= 102) return unit - 97 + 10;
  return -1;
}

/** @param {number} highUnit @param {number} lowUnit @returns {number} */
export function readRustAsciiEscapeValue(highUnit, lowUnit) {
  const high = readRustAsciiEscapeDigit(highUnit);
  const low = readRustAsciiEscapeDigit(lowUnit);
  if (high < 0 || high > 7 || low < 0) return -1;
  return high * 16 + low;
}
