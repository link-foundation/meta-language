// Arrays the program reads and never mutates: literals, spreads, indexing,
// .length and for…of, in functions, at the top level and in data fields.
// An array is a Rust Vec, a Lean Array and a Rocq list; a read outside an
// array, undefined in JavaScript, aborts, which the translation assumes away.

/** @typedef {{ $: 'bag', items: number[] } | { $: 'nested', inner: Array<Bag> }} Bag */

function bag(items) {
  return { $: 'bag', items };
}

function weight(b) {
  let total = 0;
  if (b.$ === 'bag') {
    for (const x of b.items) total += x;
    return total;
  }
  for (const inner of b.inner) total += weight(inner);
  return total;
}

/**
 * @param {number[]} xs
 * @returns {number}
 */
function last(xs) {
  return xs[xs.length - 1];
}

function sum(values) {
  let s = 0n;
  for (const v of values) s += v;
  return s;
}

function range(n) {
  let out = [];
  let i = 0n;
  while (i < n) {
    out = [...out, i];
    i += 1n;
  }
  return out;
}

function at(values, i) {
  return values[i];
}

const xs = [1, 2, 3];
console.log(xs.length);
console.log(xs[0] + xs[2]);
const ys = [...xs, 4, ...xs];
let total = 0;
for (const y of ys) {
  total += y;
}
console.log(total);
console.log(last([1.5, 2.5, 3.25]));
console.log(sum([10n, 20n, 30n]));
console.log(at(['a', 'b'], 1));
const grid = [[1, 2], [3, 4, 5]];
let cells = 0;
for (const row of grid) {
  for (const cell of row) cells += cell;
}
console.log(cells);
console.log(grid[1].length);
const empty = [];
const more = [...empty, 7];
console.log(more[0]);
let joined = '';
for (let name of ['x', 'y']) {
  name = name + '!';
  joined = joined + name;
}
console.log(joined);
const r = range(4n);
console.log(r.length);
console.log(r[3]);
console.log(r[-0]);
console.log(sum(range(100n)));
console.log(weight(bag([1, 2, 3.5])));
console.log(weight({ $: 'nested', inner: [bag([1]), bag([]), { $: 'nested', inner: [bag([2, 3])] }] }));
