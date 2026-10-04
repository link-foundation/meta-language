/**
 * @param {bigint} n
 * @returns {bigint}
 */
function count(n) {
  if (n < 0n) throw new RangeError('count expects a natural number');
  return n === 0n ? 0n : count(n - 1n) + 1n;
}

console.log(`count 3 = ${count(3n)}`);
console.log(`count -1 = ${count(-1n)}`);
