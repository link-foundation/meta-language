// Generated from the shared linked Rust cooked-string escape policy. Do not edit.
/** @param {number} unit @returns {boolean} */
export function acceptRustCookedStringEscapePrefix(unit) {
  return unit === 34 || unit === 39 || unit === 48 || unit === 92 || unit === 110 || unit === 114 || unit === 116 || unit === 117 || unit === 120;
}

/** @param {number} unit @returns {number} */
export function readRustAsciiEscapeDigit(unit) {
  if (unit === 48) return 0;
  if (unit === 49) return 1;
  if (unit === 50) return 2;
  if (unit === 51) return 3;
  if (unit === 52) return 4;
  if (unit === 53) return 5;
  if (unit === 54) return 6;
  if (unit === 55) return 7;
  if (unit === 56) return 8;
  if (unit === 57) return 9;
  if (unit === 65 || unit === 97) return 10;
  if (unit === 66 || unit === 98) return 11;
  if (unit === 67 || unit === 99) return 12;
  if (unit === 68 || unit === 100) return 13;
  if (unit === 69 || unit === 101) return 14;
  if (unit === 70 || unit === 102) return 15;
  return -1;
}

/** @param {number} highUnit @param {number} lowUnit @returns {number} */
export function readRustAsciiEscapeValue(highUnit, lowUnit) {
  const high = readRustAsciiEscapeDigit(highUnit);
  const low = readRustAsciiEscapeDigit(lowUnit);
  if (high < 0 || high > 7 || low < 0) return -1;
  return high * 16 + low;
}
