/** @param {boolean[]} left @param {boolean[]} right @returns {number} */
export function findGrammarDistinguishingObservation(left, right) {
  for (let index = 0; index < Math.min(left.length, right.length); index += 1) {
    if (left[index] !== right[index]) return index;
  }
  return -1;
}
