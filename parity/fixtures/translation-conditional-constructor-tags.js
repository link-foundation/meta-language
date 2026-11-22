/** @typedef {{ $: 'first', before: number, after: number }
 * | { $: 'second', before: number, after: number }
 * | { $: 'third', before: number, after: number }} Selection */

/** @param {number} value @returns {number} */
function observeNumber(value) {
  console.log(value);
  return value;
}

/** @param {number} option @returns {Selection} */
function selectConstructor(option) {
  return {
    before: observeNumber(1),
    $: observeNumber(option) === 0 ? 'first' : observeNumber(option + 1) === 3 ? 'third' : 'second',
    after: observeNumber(9),
  };
}

console.log(selectConstructor(0).$);
console.log(selectConstructor(1).$);
console.log(selectConstructor(2).$);
