/**
 * @param {bigint} a
 * @param {bigint} b
 * @returns {bigint}
 */
function share(a, b) {
  return a / b;
}

console.log(`share 9 2 = ${share(9n, 2n)}`);
console.log(`share 9 0 = ${share(9n, 0n)}`);
