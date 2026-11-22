/** @param {number} value @returns {boolean} */
export function documentScalarIsWhitespace(value) {
  return value === 9 || value === 10 || value === 12 || value === 13 || value === 32;
}

/** @param {number} value @returns {boolean} */
export function documentScalarIsValid(value) {
  return value > 0 && value <= 1114111 && (value < 55296 || value > 57343);
}
