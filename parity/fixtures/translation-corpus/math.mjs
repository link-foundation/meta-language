// The exactly specified part of Math and Number: abs, floor, ceil, trunc,
// round, sign, sqrt, max and min, the constants, and the Number predicates.
// Each is correctly rounded or exact, so every target computes the same
// binary64 value; -0, NaN and the infinities follow ECMAScript.

function hypot(a, b) {
  return Math.sqrt(a * a + b * b);
}

function digitSum(n) {
  let total = 0;
  while (n > 0) {
    total += n % 10;
    n = Math.floor(n / 10);
  }
  return total;
}

/**
 * @param {number[]} xs
 * @returns {number}
 */
function spread(xs) {
  return Math.max(...xs) - Math.min(...xs);
}

function clamp(x, low, high) {
  return Math.min(Math.max(x, low), high);
}

console.log(hypot(3, 4), hypot(1, 1));
console.log(digitSum(9875), digitSum(0));
console.log(spread([3, -1, 4, 1, 5]), spread([]), Math.max(1, ...[], 7, ...[2, 9]));
console.log(clamp(15, 0, 10), clamp(-3, 0, 10), clamp(NaN, 0, 10));
console.log(Math.abs(-2.5), Math.abs(-0), Math.abs(-Infinity));
console.log(Math.floor(2.7), Math.floor(-2.2), Math.floor(-0), Math.floor(-0.5), Math.floor(1e300));
console.log(Math.ceil(2.2), Math.ceil(-2.7), Math.ceil(-0.5), Math.ceil(0.5), Math.ceil(NaN));
console.log(Math.trunc(2.7), Math.trunc(-2.7), Math.trunc(-0.2), Math.trunc(4503599627370495.5));
console.log(Math.round(2.5), Math.round(-2.5), Math.round(-0.5), Math.round(0.49999999999999994), Math.round(-1e-320));
console.log(Math.round(4503599627370495.5), Math.round(-4503599627370495.5), Math.round(1.5), Math.round(-0.6));
console.log(Math.sign(-3), Math.sign(0.1), Math.sign(-0), Math.sign(0), Math.sign(NaN));
console.log(Math.sqrt(2), Math.sqrt(-1), Math.sqrt(-0), Math.sqrt(Infinity), Math.sqrt(1e-320));
console.log(Math.max(), Math.min(), Math.max(-0, 0), Math.min(0, -0), Math.max(NaN, 1), Math.min(3, 1, 2));
console.log(Math.PI, Math.E, Math.LN2, Math.LN10, Math.LOG2E, Math.LOG10E, Math.SQRT2, Math.SQRT1_2);
console.log(Number.MAX_SAFE_INTEGER, Number.MIN_SAFE_INTEGER, Number.EPSILON, Number.MAX_VALUE, Number.MIN_VALUE);
console.log(Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, Number.NaN);
console.log(Number.isInteger(5), Number.isInteger(5.5), Number.isInteger(Infinity), Number.isInteger(-0));
console.log(Number.isSafeInteger(9007199254740992), Number.isSafeInteger(9007199254740991), Number.isFinite(1 / 0), Number.isNaN(0 / 0));
console.log(isNaN(Math.sqrt(-4)), isFinite(Math.PI));
console.log(`floor ${Math.floor(-0.5)} round ${Math.round(-0.25)}`);
