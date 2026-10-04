// console.log with several arguments prints util.format of them: a literal
// first string reads its %s, %d, %i, %c and %% directives, each taking the
// next argument; a directive with no argument left stays as written, and the
// arguments left over follow after spaces, strings as they are and other
// values as the console shows them.
function average(values, count) {
  const mean = values / count;
  console.log('mean of', count, 'values:', mean);
  return mean;
}

function tally(n) {
  let sum = 0n;
  for (let i = 1n; i <= n; i++) {
    sum += i;
    console.log('step %d: sum %s', i, sum);
  }
  return sum;
}

const zero = -0;
console.log('total', tally(4n), 'done', true);
console.log(zero, 'x%sy', 2n);
console.log('%s=%d', 'answer', 42n);
console.log('%i%% of %s', 30n, 'the whole', 'and more', 7n);
console.log('%s %s %s', 'one');
console.log('%x stays, %s', 'unknown');
console.log('%c styled', 'color: red');
console.log('%d and %d', zero, 1.5);
console.log('', average(7, 2));
console.log(1.5, 2n, false);
