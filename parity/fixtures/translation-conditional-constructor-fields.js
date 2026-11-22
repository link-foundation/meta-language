/** @typedef {{ $: 'flag', payload: boolean } | { $: 'measured', payload: number } | { $: 'empty' }} Reading */

/** @param {number} value @returns {number} */
function observeNumber(value) {
  console.log(value);
  return value;
}

/** @param {string} tag @param {boolean} value @returns {number} */
function observeFlag(tag, value) {
  console.log(tag);
  return observeNumber(value ? 1 : 0);
}

/** @param {Reading} reading @returns {number} */
function inspect(reading) {
  return reading.$ === 'measured' ? observeNumber(reading.payload)
    : reading.$ !== 'flag' ? observeNumber(-1) : observeFlag(reading.$, reading.payload);
}

console.log(inspect({ $: 'measured', payload: 7 }));
console.log(inspect({ $: 'flag', payload: true }));
console.log(inspect({ $: 'empty' }));
