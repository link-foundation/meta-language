// Async functions whose every call is awaited where it is made: nothing runs
// concurrently, so each await is the ordinary call of its function and the
// program prints the same lines, in the same order, in every target.

/** @typedef {{ $: 'nil' } | { $: 'cons', head: bigint, tail: List }} List */

/**
 * @param {bigint} n
 * @returns {Promise<bigint>}
 */
async function fetchSquare(n) {
  return n * n;
}

async function sumSquares(list) {
  switch (list.$) {
    case 'nil':
      return 0n;
    case 'cons':
      return (await fetchSquare(list.head)) + (await sumSquares(list.tail));
  }
}

const describe = async (name, value) => name + ' = ' + String(value);

// An async function may return another's call: its Promise is adopted.
async function total(list) {
  return sumSquares(list);
}

export async function average(a, b) {
  const sum = await add(a, b);
  return sum / 2;
}

async function add(a, b) {
  return a + b;
}

const list = { $: 'cons', head: 3n, tail: { $: 'cons', head: 4n, tail: { $: 'nil' } } };
console.log(await fetchSquare(12n));
console.log(await describe('sum of squares', await total(list)));
console.log(await average(1, 2));
console.log(await 7n);
