/**
 * Estimates the model's support and discounts sparse evidence.
 * @param {number} distance
 * @param {number} maximumDistance
 * @param {number} letters
 * @returns {number}
 */
export function trigramConfidence(distance, maximumDistance, letters) {
  const support = letters < 8 ? letters / 8 : 1;
  return (1 - distance / maximumDistance) * support * support * support;
}

/**
 * @param {number} first
 * @param {number} second
 * @returns {boolean}
 */
export function minimumLanguageLead(first, second) {
  return second < first * 0.97;
}
