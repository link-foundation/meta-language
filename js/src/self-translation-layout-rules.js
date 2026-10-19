/** @param {boolean} carriage @param {boolean} leading @param {boolean} trailing @param {boolean} gaps @returns {boolean} */
export function requireSelfTranslationLayoutEnvelope(carriage, leading, trailing, gaps) {
  return carriage || leading || trailing || gaps;
}
