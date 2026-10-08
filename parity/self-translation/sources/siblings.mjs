// Items that call the other top-level items of their module.

/** @param {number} x @returns {number} */
function square(x) {
  return x * x;
}

/** @param {number} x @param {number} limit @returns {number} */
export function cappedSquare(x, limit) {
  const squared = square(x);
  return squared > limit ? limit : squared;
}

/** @param {number} x @returns {number} */
export function cappedTwice(x) {
  return halve(cappedSquare(x, 3)) * 4;
}

// Declared after its caller, as JavaScript hoists a function declaration.
const halve = (x) => x / 2;

/** @param {string} name @returns {boolean} */
export function greets(name) {
  return name === greeting();
}

function greeting() {
  return 'hello';
}

/** @param {number} x @returns {number} */
export function boxed(x) {
  return unbox(x);
}

function unbox(x) {
  return new Box(x).value;
}

class Box {
  constructor(value) {
    this.value = value;
  }
}
