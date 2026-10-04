// Mutually recursive functions: the Lean target writes each group as a
// mutual block of partial defs and the Rocq target as one ml_fix over the sum
// of the functions' parameter tuples, so every target runs them as the source
// does.

/**
 * @param {bigint} n
 * @returns {boolean}
 */
function isEven(n) {
  if (n === 0n) return true;
  return isOdd(n - 1n);
}

/**
 * @param {bigint} n
 * @returns {boolean}
 */
function isOdd(n) {
  if (n === 0n) return false;
  return isEven(n - 1n);
}

/**
 * Hofstadter's female sequence.
 * @param {bigint} n
 * @returns {bigint}
 */
function female(n) {
  if (n === 0n) return 1n;
  return n - male(female(n - 1n));
}

/**
 * Hofstadter's male sequence.
 * @param {bigint} n
 * @returns {bigint}
 */
function male(n) {
  if (n === 0n) return 0n;
  return n - female(male(n - 1n));
}

/**
 * Three functions taking turns to add to an accumulator.
 * @param {bigint} n
 * @param {bigint} acc
 * @returns {bigint}
 */
function first(n, acc) {
  if (n === 0n) return acc;
  return second(n - 1n, acc + 1n);
}

/**
 * @param {bigint} n
 * @param {bigint} acc
 * @returns {bigint}
 */
function second(n, acc) {
  if (n === 0n) return acc;
  return third(n - 1n, acc + 10n);
}

/**
 * @param {bigint} n
 * @param {bigint} acc
 * @returns {bigint}
 */
function third(n, acc) {
  if (n === 0n) return acc;
  return first(n - 1n, acc + 100n);
}

console.log(isEven(10n));
console.log(isOdd(7n));
console.log(female(20n));
console.log(male(20n));
console.log(first(10n, 0n));
