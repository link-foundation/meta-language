import assert from "node:assert/strict";
/**
 * @param {number} x
 * @returns {number}
 */
function half(x) {
  return x / 2;
}
console.log(42);
console.log(6 * 7);
console.log(0.1 + 0.2);
console.log(-0);
console.log(`${-0}`);
console.log(half(5) % 2);
console.log(-7 % 3);
console.log(1 / 0);
console.log(-(1 / 0));
console.log(0 / 0);
console.log(1e21 + 1);
console.log("x" + 1.5);
console.log(5e-324);
console.log(half(3) < 2);
console.log(NaN === NaN);
assert.strictEqual(NaN, 0 / 0);
assert.notStrictEqual(0, -0);
assert(0 === -0);
