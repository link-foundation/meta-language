// Expressions too long for one Rust line, which the layout breaks one item
// per line the way rustfmt would (issue #217).
export const KEYWORDS = ['abstract', 'arguments', 'await', 'boolean', 'break', 'byte', 'case', 'catch', 'char', 'class', 'const', 'continue'];

/** @param {number} first @param {number} second @param {number} third @param {number} fourth @returns {number} */
export function weightedSum(first, second, third, fourth) {
  return (first * 1000 + second * 100 + third * 10 + fourth) * (first + second + third + fourth + 1);
}
