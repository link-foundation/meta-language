// Portable-core conformance project: control flow, recursion, data types,
// modules, effects, and proofs in one JavaScript program. Integers are
// BigInt values, so arithmetic is exact; a function whose first statements
// reject negative arguments takes natural numbers.
import assert from 'node:assert/strict';

/**
 * @typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint, right: Tree }} Tree
 */

const Arith = {
  /**
   * @param {bigint} n
   * @returns {bigint}
   */
  fact(n) {
    if (n < 0n) throw new RangeError('fact expects a natural number');
    if (n === 0n) return 1n;
    return n * Arith.fact(n - 1n);
  },

  /**
   * @param {bigint} n
   * @returns {bigint}
   */
  sumTo(n) {
    if (n < 0n) throw new RangeError('sumTo expects a natural number');
    return n === 0n ? 0n : Arith.sumTo(n - 1n) + n;
  },

  /**
   * @param {bigint} n
   * @returns {bigint}
   */
  fib(n) {
    if (n < 0n) throw new RangeError('fib expects a natural number');
    switch (n) {
      case 0n:
        return 0n;
      case 1n:
        return 1n;
      default:
        return Arith.fib(n - 1n) + Arith.fib(n - 2n);
    }
  },

  /**
   * @param {bigint} a
   * @param {bigint} b
   * @returns {bigint}
   */
  monus(a, b) {
    return a > b ? a - b : 0n;
  },

  /**
   * Division rounding towards negative infinity.
   * @param {bigint} x
   * @returns {bigint}
   */
  halve(x) {
    const rest = x % 2n;
    return rest < 0n ? (x - rest) / 2n - 1n : x / 2n;
  },

  /**
   * The remainder that is never negative; by zero it is the dividend.
   * @param {bigint} x
   * @param {bigint} y
   * @returns {bigint}
   */
  remainder(x, y) {
    if (y === 0n) return x;
    const rest = x % y;
    if (rest >= 0n) return rest;
    return y > 0n ? rest + y : rest - y;
  },
};

const Tree = {
  /**
   * @param {Tree} t
   * @returns {bigint}
   */
  size(t) {
    switch (t.$) {
      case 'leaf':
        return 0n;
      case 'node':
        return Tree.size(t.left) + 1n + Tree.size(t.right);
    }
  },

  /**
   * @param {Tree} t
   * @returns {bigint}
   */
  total(t) {
    switch (t.$) {
      case 'leaf':
        return 0n;
      case 'node':
        return Tree.total(t.left) + t.value + Tree.total(t.right);
    }
  },

  /**
   * @param {Tree} t
   * @returns {Tree}
   */
  mirror(t) {
    switch (t.$) {
      case 'leaf':
        return t;
      case 'node':
        return { $: 'node', left: Tree.mirror(t.right), value: t.value, right: Tree.mirror(t.left) };
    }
  },

  /**
   * @param {Tree} t
   * @param {bigint} x
   * @returns {Tree}
   */
  insert(t, x) {
    switch (t.$) {
      case 'leaf':
        return { $: 'node', left: t, value: x, right: t };
      case 'node': {
        const { left, value, right } = t;
        if (x < value) return { $: 'node', left: Tree.insert(left, x), value, right };
        return { $: 'node', left, value, right: Tree.insert(right, x) };
      }
    }
  },
};

/**
 * @param {bigint} n
 * @returns {string}
 */
function classify(n) {
  if (n < 0n) return 'negative';
  if (n === 0n) return 'zero';
  return 'positive';
}

/**
 * @param {bigint} n
 * @returns {string}
 */
function describe(n) {
  if (n < 0n) throw new RangeError('describe expects a natural number');
  const doubled = n + n;
  const label = classify(doubled - 10n);
  return `${n} doubled is ${doubled} (${label})`;
}

/** @returns {Tree} */
function sample() {
  return Tree.insert(Tree.insert(Tree.insert(Tree.insert({ $: 'leaf' }, 5n), 2n), 8n), 3n);
}

assert.equal(Arith.fact(5n), 120n);
console.log(`fact 20 = ${Arith.fact(20n)}`);
console.log(`sumTo 100 = ${Arith.sumTo(100n)}`);
console.log(`fib 25 = ${Arith.fib(25n)}`);
console.log(`monus 3 5 = ${Arith.monus(3n, 5n)}`);
console.log(`halve -7 = ${Arith.halve(-7n)}`);
console.log(`remainder -7 3 = ${Arith.remainder(-7n, 3n)}`);
console.log(`remainder 7 0 = ${Arith.remainder(7n, 0n)}`);
const t = sample();
console.log(`size = ${Tree.size(t)}, total = ${Tree.total(t)}`);
console.log(`mirrored total = ${Tree.total(Tree.mirror(t))}`);
console.log(describe(3n));
console.log(describe(7n));
console.log(classify(-4n));
