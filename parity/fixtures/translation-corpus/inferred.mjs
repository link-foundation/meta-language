// Functions without JSDoc: their types are inferred from how the program
// uses them.

/** @typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint, right: Tree }} Tree */

function answer() {
  return 42;
}

function inc(x) {
  return x + 1;
}

function factorial(n) {
  if (n < 0n) throw new RangeError('negative');
  return n === 0n ? 1n : n * factorial(n - 1n);
}

function greet(name) {
  return 'hello ' + name;
}

function label(count) {
  return count + ' items';
}

function pick(flag, a, b) {
  return flag ? a : b;
}

function sum(tree) {
  switch (tree.$) {
    case 'leaf':
      return 0n;
    case 'node':
      return sum(tree.left) + tree.value + sum(tree.right);
  }
}

function single(value) {
  return { $: 'node', left: { $: 'leaf' }, value, right: { $: 'leaf' } };
}

const Geometry = {
  square(x) {
    return x * x;
  },
  hypotenuseSquared(a, b) {
    return Geometry.square(a) + Geometry.square(b);
  },
};

console.log(answer());
console.log(inc(41));
console.log(factorial(20n));
console.log(greet('world'));
console.log(label(3));
console.log(pick(true, 1.5, 2));
console.log(sum({ $: 'node', left: single(1n), value: 2n, right: single(3n) }));
console.log(Geometry.hypotenuseSquared(3, 4));
