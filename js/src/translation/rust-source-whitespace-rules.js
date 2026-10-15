// Generated from the linked Rust source whitespace policy. Do not edit.
import { acceptRustNumberDigitBoundary } from './rust-number-rules.js';
/** @param {number} unit @returns {boolean} */
export function acceptRustSourceWhitespace(unit) {
  return unit === 9 || unit === 10 || unit === 11 || unit === 12 || unit === 13 || unit === 32 || unit === 133 || unit === 8206 || unit === 8207 || unit === 8232 || unit === 8233;
}
/** @param {number} unit @param {boolean} firstPosition @returns {boolean} */
export function acceptRustSourceByteOrderMark(unit, firstPosition) {
  return firstPosition && unit === 65279;
}
/** @param {boolean} floating @param {boolean} trailingSeparator @param {string} suffix @param {boolean} continuedSuffix @param {number} followingCodeUnit @returns {boolean} */
export function acceptRustSourceNumberDigitBoundary(floating, trailingSeparator, suffix, continuedSuffix, followingCodeUnit) {
  return acceptRustNumberDigitBoundary(floating, trailingSeparator, suffix, continuedSuffix, followingCodeUnit)
    || (acceptRustSourceWhitespace(followingCodeUnit)
      && acceptRustNumberDigitBoundary(floating, trailingSeparator, suffix, continuedSuffix, 32));
}
