// console.log anywhere, not only in main: Rust and JavaScript print where the
// source prints; the Lean and Rocq targets thread the lines printed through
// every function that prints, directly or through a function it calls, so
// every target prints the same lines in the same order.
import assert from 'node:assert/strict';

function countdown(n) {
  if (n === 0n) return 0n;
  console.log(`tick ${n}`);
  return countdown(n - 1n) + n;
}

function loud(x) {
  console.log('loud');
  return x * 2n;
}

function isEven(n) {
  console.log(`even? ${n}`);
  if (n === 0n) return true;
  return isOdd(n - 1n);
}

function isOdd(n) {
  console.log(`odd? ${n}`);
  if (n === 0n) return false;
  return isEven(n - 1n);
}

function half(x) {
  console.log(`half of ${x}`);
  return x / 2;
}

function check(label, ok) {
  console.log(`${label}: ${ok}`);
  return ok;
}

for (let i = 0n; i < 3n; i++) console.log(i);
const total = countdown(3n);
console.log(total);
console.log(loud(loud(5n)));
let acc = 0n;
for (let i = 1n; i <= 4n; i++) {
  acc += loud(i);
}
console.log(acc);

console.log(isEven(3n));
const both = check('a', false) && check('b', true);
console.log(both);
const either = check('c', true) || check('d', true);
console.log(either);
console.log(half(half(9)));
assert(isOdd(1n));
assert.equal(isEven(2n), true);
const sum = half(3) + half(5);
console.log(sum);
