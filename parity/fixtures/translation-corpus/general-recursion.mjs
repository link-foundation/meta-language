// Recursion with no structurally decreasing argument: the Lean target writes
// these as partial defs and the Rocq target as ml_fix over each function's
// one-step unfolding, so every target runs them as the source does.

/**
 * Steps of the Collatz sequence from n down to 1.
 * @param {bigint} n
 * @returns {bigint}
 */
function collatz(n) {
  if (n <= 1n) return 0n;
  if (n % 2n === 0n) return 1n + collatz(n / 2n);
  return 1n + collatz(3n * n + 1n);
}

/**
 * The sum of n, n + 3, n + 6, … below limit, accumulated in acc.
 * @param {bigint} n
 * @param {bigint} limit
 * @param {bigint} acc
 * @returns {bigint}
 */
function climb(n, limit, acc) {
  if (n >= limit) return acc;
  return climb(n + 3n, limit, acc + n);
}

/**
 * @param {bigint} a
 * @param {bigint} b
 * @returns {bigint}
 */
function gcd(a, b) {
  if (b === 0n) return a;
  return gcd(b, a % b);
}

/**
 * How many halvings take x below 1.
 * @param {number} x
 * @returns {number}
 */
function halvings(x) {
  if (x < 1) return 0;
  return 1 + halvings(x / 2);
}

/**
 * Whether n reaches 1 within the given steps of the Collatz sequence.
 * @param {bigint} n
 * @param {bigint} steps
 * @returns {boolean}
 */
function settles(n, steps) {
  if (n === 1n) return true;
  if (steps === 0n) return false;
  return settles(n % 2n === 0n ? n / 2n : 3n * n + 1n, steps - 1n);
}

console.log(collatz(27n));
console.log(climb(0n, 3000n, 0n));
console.log(gcd(1071n, 462n));
console.log(halvings(1000));
console.log(settles(27n, 111n));
console.log(settles(27n, 110n));
