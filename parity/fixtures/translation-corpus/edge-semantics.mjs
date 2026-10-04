// JavaScript edge semantics the portable core must preserve: truncating
// BigInt division and remainder, string concatenation with numbers and
// booleans, console.log's BigInt rendering, radix literals, escapes, nested
// templates, a tag test ahead of a switch on the same value, and asserts that
// combine conditions or compare structures.
import assert from 'node:assert/strict';

/** @typedef {{ $: 'nil' } | { $: 'cons', head: bigint, tail: List }} List */

/**
 * @param {bigint} n
 * @returns {bigint}
 */
function pred(n) {
  if (n < 0n) throw new RangeError('pred expects a natural number');
  return n - 1n;
}

/**
 * @param {bigint} a
 * @param {bigint} b
 * @returns {bigint}
 */
function quot(a, b) {
  return a / b;
}

/**
 * @param {bigint} a
 * @param {bigint} b
 * @returns {bigint}
 */
function rem(a, b) {
  return a % b;
}

/**
 * @param {bigint} n
 * @returns {bigint}
 */
function negate(n) {
  if (n < 0n) throw new RangeError('negate expects a natural number');
  return -n;
}

/**
 * @param {bigint} n
 * @returns {string}
 */
function label(n) {
  return 'n=' + n + ', positive=' + (n > 0n) + '!';
}

/**
 * @param {boolean} b
 * @returns {string}
 */
function yesNo(b) {
  switch (b) {
    case true:
      return 'yes';
    default:
      return 'no';
  }
}

/**
 * @param {bigint} n
 * @returns {List}
 */
function countdown(n) {
  if (n < 0n) throw new RangeError('countdown expects a natural number');
  if (n === 0n) return { $: 'nil' };
  return { $: 'cons', head: n, tail: countdown(n - 1n) };
}

/**
 * @param {List} xs
 * @returns {bigint}
 */
function sum(xs) {
  switch (xs.$) {
    case 'nil':
      return 0n;
    default:
      return xs.head + sum(xs.tail);
  }
}

/**
 * @param {List} xs
 * @returns {string}
 */
function render(xs) {
  if (xs.$ === 'nil') return '';
  switch (xs.$) {
    case 'cons': {
      const { head, tail } = xs;
      const rest = render(tail);
      return rest === '' ? `${head}` : `${head},${rest}`;
    }
    default:
      return '?';
  }
}

const big = 12345678901234567890n * 98765432109876543210n;
assert.equal(pred(1n), 0n);
assert(sum(countdown(4n)) === 10n && yesNo(true) === 'yes');
assert.deepStrictEqual(countdown(1n), { $: 'cons', head: 1n, tail: { $: 'nil' } });
console.log(String(pred(0n)));
console.log(`${quot(-7n, 2n)} ${rem(-7n, 2n)} ${rem(7n, -2n)} ${quot(7n, -2n)}`);
console.log(negate(3n).toString());
console.log(label(5n));
console.log(label(-5n));
console.log(yesNo(1n > 2n));
console.log(big);
console.log(0x1fn);
console.log(true);
console.log('quote " backslash \\ tab\tend');
console.log(`nested ${`inner ${1n + 2n}`} done`);
console.log();
console.log(render(countdown(5n)));
console.log(`${sum(countdown(100n))}`);
