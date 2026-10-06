// Functions bound to constants: arrow functions, with an expression or a
// block body, and function expressions. Each is declared before the first
// top-level statement, so nothing calls it before it is initialised.

/** @typedef {{ $: 'nil' } | { $: 'cons', head: bigint, tail: List }} List */

const inc = (x) => x + 1;

const double = x => x * 2;

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
const average = (a, b) => (a + b) / 2;

const clamp = (low, value, high) => {
  if (value < low) return low;
  if (value > high) return high;
  return value;
};

const factorial = function (n) {
  if (n < 0n) throw new RangeError('negative');
  return n === 0n ? 1n : n * factorial(n - 1n);
};

const power = function power(base, n) {
  if (n < 0n) throw new RangeError('negative');
  return n === 0n ? 1n : base * power(base, n - 1n);
};

const length = (list) => {
  switch (list.$) {
    case 'nil':
      return 0n;
    case 'cons':
      return 1n + length(list.tail);
  }
};

const greet = (name) => `hello ${name}`;

export const answer = () => 42;

console.log(inc(41));
console.log(double(21));
console.log(average(1, 2));
console.log(clamp(0, 12, 10));
console.log(factorial(20n));
console.log(power(2n, 64n));
console.log(length({ $: 'cons', head: 1n, tail: { $: 'cons', head: 2n, tail: { $: 'nil' } } }));
console.log(greet('world'));
console.log(answer());
