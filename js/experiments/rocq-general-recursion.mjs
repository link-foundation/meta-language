/** @param {bigint} n @returns {bigint} */
function collatz(n) {
  if (n <= 1n) return 0n;
  if (n % 2n === 0n) return 1n + collatz(n / 2n);
  return 1n + collatz(3n * n + 1n);
}
/** @param {bigint} n @param {bigint} limit @param {bigint} acc @returns {bigint} */
function climb(n, limit, acc) {
  if (n >= limit) return acc;
  return climb(n + 3n, limit, acc + n);
}
console.log(collatz(27n));
console.log(climb(0n, 3000n, 0n));
