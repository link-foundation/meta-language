// The draft consumes exact IEEE-754 binary64 bit patterns. Sign, exponent,
// significand and source text models are portable rules, not host formatters.
// Only reinterpreting a runtime float as its bits remains a host boundary.
import {
  acceptBinaryDoublePrecisionFormatModel, renderBinaryDoublePrecisionSpecial,
  renderFiniteBinaryDoublePrecision,
} from './issue-199-decimal-number-rules.mjs';

/** @param {bigint} bits @param {string} model @returns {string} */
export function renderBinaryDoublePrecisionBits(bits, model) {
  if (bits < 0n || bits > 18446744073709551615n || !acceptBinaryDoublePrecisionFormatModel(model)) return '';
  const negative = bits >= 9223372036854775808n;
  const payload = bits % 9223372036854775808n;
  const fraction = payload % 4503599627370496n;
  const biasedExponent = payload / 4503599627370496n;
  if (biasedExponent === 2047n) {
    return renderBinaryDoublePrecisionSpecial(model, fraction === 0n ? 'infinity' : 'not-a-number', negative);
  }
  const mantissa = biasedExponent === 0n ? fraction : fraction + 4503599627370496n;
  const exponent = biasedExponent === 0n ? -1074n : biasedExponent - 1075n;
  return renderFiniteBinaryDoublePrecision(mantissa, exponent, negative, model);
}

/** @param {bigint} bits @returns {string} */
export function renderJavaScriptNumberBits(bits) {
  return renderBinaryDoublePrecisionBits(bits, 'ecmascript-number-text');
}

/** @param {bigint} bits @returns {string} */
export function renderNodeConsoleNumberBits(bits) {
  return renderBinaryDoublePrecisionBits(bits, 'node-console-number-text');
}

/** @param {bigint} bits @returns {string} */
export function renderRustDefaultNumberBits(bits) {
  return renderBinaryDoublePrecisionBits(bits, 'rust-default-binary-double-precision-text');
}
