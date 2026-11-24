// All branches entering or leaving at the same boundary contribute to one change.
/** @param {number} previous @param {number} next @returns {number} */
export function exclusiveIntegerBoundary(previous, next) {
  if (previous === 1 && next !== 1) return -1;
  if (previous !== 1 && next === 1) return 1;
  return 0;
}
