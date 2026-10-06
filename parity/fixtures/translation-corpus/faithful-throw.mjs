/**
 * @param {bigint} n
 * @returns {bigint}
 */
function half(n) {
  console.log(`halving ${n}`);
  if (n % 2n !== 0n) throw new Error("an odd number has no half");
  return n / 2n;
}

console.log(`half 8 = ${half(8n)}`);
console.log(`half 7 = ${half(7n)}`);
console.log('never printed');
