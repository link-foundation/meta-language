// Functions that return nothing: a function that finishes without a return
// value, or leaves with `return;`, returns undefined, the unit value, and a
// call whose value a statement discards still runs, for the lines it prints.
import assert from 'node:assert/strict';

function hello(name) {
  console.log(`hello ${name}`);
}

function countTo(n) {
  for (let i = 1n; i <= n; i++) {
    if (i === 4n) {
      console.log('stop at 4');
      return;
    }
    console.log(`count ${i}`);
  }
  console.log('counted');
}

function report(label, value) {
  if (value < 0n) {
    console.log(`${label}: negative`);
    return;
  }
  console.log(`${label}: ${value}`);
}

function double(x) {
  console.log(`double ${x}`);
  return x * 2n;
}

function greetAll(n) {
  if (n === 0n) return;
  hello(`#${n}`);
  greetAll(n - 1n);
}

hello('world');
countTo(2n);
countTo(9n);
report('a', 5n);
report('b', -5n);
double(21n);
greetAll(3n);
const unit = hello('again');
let total = 0n;
for (let i = 0n; i < 3n; i++) {
  report('i', i);
  total += double(i);
}
console.log(`total ${total}`);
assert.strictEqual(double(4n), 8n);
