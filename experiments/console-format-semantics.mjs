// Node's util.format behaviour for the console.log forms the translator accepts.
console.log('a', 1n, -0, 1.5, true, 'b');
console.log(-0, 'x%sy', 2n);
console.log('100%%');
console.log('100%%', 1n);
console.log('%s=%d', 'x', 5n);
console.log('%s %s', -0, 3n);
console.log('%d', -0);
console.log('%i', -7n);
console.log('%s', 'only', 'extra', 4n);
console.log('%s %s %s', 'a');
console.log('%x %s', 'y');
console.log('%c red', 'color: red');
console.log('trailing %', 1n);
console.log('%%s', 1n);
console.log('%f', -0, '%s');
console.log('', 1n);
console.log('%s', true, false);
