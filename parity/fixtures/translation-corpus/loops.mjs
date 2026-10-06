// Loops over mutable variables: for, while and do…while, break and
// continue, return from inside a loop, and statements that assign a
// variable on several paths. Each target runs a loop as a function of the
// variables it uses, which Rust and JavaScript print as a loop again, so a
// long loop needs no deep stack, with the same results.

/** @typedef {{ $: 'nil' } | { $: 'cons', head: bigint, tail: List }} List */

function sumTo(n) {
  let total = 0n;
  for (let i = 1n; i <= n; i++) {
    total += i;
  }
  return total;
}

function fibonacci(n) {
  let a = 0n, b = 1n;
  for (let i = 0n; i < n; i += 1n) {
    const next = a + b;
    a = b;
    b = next;
  }
  return a;
}

function collatzSteps(n) {
  let steps = 0;
  while (n !== 1n) {
    if (n % 2n === 0n) n /= 2n;
    else n = 3n * n + 1n;
    steps++;
  }
  return steps;
}

function digitCount(n) {
  let count = 0n;
  do {
    count++;
    n /= 10n;
  } while (n > 0n);
  return count;
}

function smallestDivisor(n) {
  for (let d = 2n; d * d <= n; d++) {
    if (n % d === 0n) return d;
  }
  return n;
}

function oddSumUntil(limit, stop) {
  let total = 0n;
  let i = 0n;
  while (true) {
    i++;
    if (i > limit) break;
    if (i % 2n === 0n) continue;
    if (i === stop) break;
    total += i;
  }
  return total * 1000n + i;
}

function gcd(a, b) {
  while (b !== 0n) {
    const r = a % b;
    a = b;
    b = r;
  }
  return a;
}

/** @param {List} list */
function listSum(list) {
  let total = 0n;
  let rest = list;
  while (true) {
    switch (rest.$) {
      case 'nil':
        return total;
      case 'cons':
        total += rest.head;
        rest = rest.tail;
        break;
    }
  }
}

function table(n) {
  let total = 0n;
  for (let i = 1n; i <= n; i++) {
    for (let j = 1n; j <= n; j++) {
      if (j > i) break;
      total += i * j;
    }
  }
  return total;
}

function countdown(start) {
  let text = '';
  for (let i = start; i > 0; i -= 1) {
    text += `${i} `;
  }
  return text + 'liftoff';
}

function classify(n) {
  let small = 0n, large = 0n;
  for (let i = 0n; i < n; i++) {
    if (i < 3n) {
      small += i;
    } else {
      large += i;
      small -= 1n;
    }
  }
  return `${small}/${large}`;
}

function shadowed(x) {
  let y = 0n;
  {
    const x = 5n;
    y = x;
  }
  return x + y;
}

function grade(score) {
  let points = 0n;
  switch (score) {
    case 3n:
      points += 10n;
    case 2n:
      points += 5n;
      break;
    case 1n:
      points = 1n;
      break;
    default:
      points = -1n;
  }
  return points;
}

function firstSquareAbove(n) {
  let k = 0n;
  while (true) {
    if (k * k > n) return k;
    k++;
  }
}

console.log(`sumTo ${sumTo(100n)}`);
console.log(`fibonacci ${fibonacci(90n)}`);
console.log(`collatz ${collatzSteps(27n)}`);
console.log(`digits ${digitCount(0n)} ${digitCount(123456789n)}`);
console.log(`divisor ${smallestDivisor(91n)} ${smallestDivisor(97n)}`);
console.log(`odd ${oddSumUntil(20n, 13n)} ${oddSumUntil(9n, 100n)}`);
console.log(`gcd ${gcd(1071n, 462n)}`);
console.log(`list ${listSum({ $: 'cons', head: 4n, tail: { $: 'cons', head: 38n, tail: { $: 'nil' } } })}`);
console.log(`table ${table(6n)}`);
console.log(countdown(5));
console.log(`classify ${classify(6n)}`);
console.log(`shadowed ${shadowed(1n)}`);
console.log(`grade ${grade(3n)} ${grade(2n)} ${grade(1n)} ${grade(0n)}`);
console.log(`square ${firstSquareAbove(50n)}`);
console.log(`long ${sumTo(100000n)} ${collatzSteps(837799n)}`);
