/**
 * @param {boolean} previous
 * @param {boolean} following
 * @returns {boolean}
 */
export function insideGrammarSearchWord(previous, following) {
  return previous && following;
}

/**
 * @param {boolean} advances
 * @param {boolean} acceptedBoundary
 * @returns {boolean}
 */
export function acceptGrammarSearchEnd(advances, acceptedBoundary) {
  return advances && acceptedBoundary;
}

/**
 * @param {boolean} overlaps
 * @param {boolean} overlapping
 * @returns {boolean}
 */
export function skipGrammarSearchStart(overlaps, overlapping) {
  return overlaps && !overlapping;
}
