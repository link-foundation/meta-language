/** @param {number} diagonal @param {number} deletion @param {number} insertion @returns {number} */
export function selectTextEdit(diagonal, deletion, insertion) {
  if (diagonal <= deletion && diagonal <= insertion) return 0;
  if (deletion <= insertion) return 1;
  return 2;
}

/** @param {number} value @returns {number} */
export function normalizeQuoteScalar(value) {
  if (value === 171 || value === 187 || value === 8220 || value === 8221
    || value === 8222 || value === 8223 || value === 8249 || value === 8250) return 34;
  if (value === 8216 || value === 8217 || value === 8218 || value === 8219) return 39;
  return value;
}

/** @param {number} value @returns {number} */
export function normalizeYoScalar(value) {
  if (value === 1025) return 1045;
  if (value === 1105) return 1077;
  return value;
}
