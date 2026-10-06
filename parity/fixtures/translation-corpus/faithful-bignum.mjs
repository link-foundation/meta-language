/**
 * @param {bigint} n
 * @returns {bigint}
 */
function fact(n) {
  return n === 0n ? 1n : n * fact(n - 1n);
}

console.log(`fact 21 = ${fact(21n)}`);
console.log(`fact 30 = ${fact(30n)}`);
console.log(`minus = ${3n - 5n}`);
