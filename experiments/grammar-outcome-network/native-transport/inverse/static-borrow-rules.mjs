// Closed shared selection for caller-owned existing static string references.

/** @param {number} entries @param {number} bytes @returns {boolean} */
export function acceptOwnedOutcomeStaticBorrowResources(entries, bytes) {
  return Number.isFinite(entries) && Math.trunc(entries) === entries && entries >= 0 && entries <= 128
    && Number.isFinite(bytes) && Math.trunc(bytes) === bytes && bytes >= 0 && bytes <= 1048576;
}

/** @param {string} text @param {string[]} values @param {number} bytes @returns {number} */
export function findOwnedOutcomeStaticBorrowIndex(text, values, bytes) {
  if (!acceptOwnedOutcomeStaticBorrowResources(values.length, bytes)) return -3;
  let selected = -1;
  let index = 0;
  while (index < values.length) {
    let previous = 0;
    while (previous < index) {
      if (values[previous] === values[index]) return -2;
      previous += 1;
    }
    if (values[index] === text) selected = index;
    index += 1;
  }
  return selected;
}

/** @param {number} code @returns {string} */
export function readOwnedOutcomeStaticBorrowFailure(code) {
  if (code === -1) return 'no existing static reference matches the source value';
  if (code === -2) return 'duplicate static reference labels are ambiguous';
  if (code === -3) return 'static reference registry exceeds its resource limit';
  return 'invalid static reference selection';
}
