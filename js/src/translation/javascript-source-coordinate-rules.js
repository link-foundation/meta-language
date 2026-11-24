/** @param {number} value @param {number} limit @returns {boolean} */
export function testJavaScriptSourceCoordinate(value, limit) {
  return Number.isSafeInteger(value) && value >= 0 && value < limit && (value !== 0 || 1 / value > 0);
}
