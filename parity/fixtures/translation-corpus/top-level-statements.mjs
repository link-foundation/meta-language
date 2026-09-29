// Top-level variables, assignments, ifs and loops.
let total = 0n;
for (let i = 1n; i <= 100n; i++) {
  total += i;
}
console.log(total);

let a = 0n;
let b = 1n;
let steps = 0n;
while (b < 1000n) {
  const next = a + b;
  a = b;
  b = next;
  steps++;
}
console.log(a);
console.log(b);
console.log(steps);

const limit = 50n;
let count = 0n;
let n = 27n;
do {
  n = n % 2n === 0n ? n / 2n : 3n * n + 1n;
  count += 1n;
  if (count >= limit) break;
} while (n !== 1n);
console.log(count);

let label = 'small';
if (total > 1000n) {
  label = 'large';
}
console.log(label);
total = total * 2n;
console.log(total);
{
  let total = 5n;
  steps = total + steps;
}
console.log(steps);
let x = 0.5;
for (let k = 0n; k < 3n; k++) x = x * 2;
console.log(x);
let lo = 1n;
let hi = 2n;
if (lo < hi) {
  lo = hi;
  hi = hi * 3n;
}
console.log(lo + hi);
