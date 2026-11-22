/** @typedef {{ $: 'selection', after: number, before: number,
 * first?: number|undefined, second?: number|undefined }} Selection */

/** @param {number} value @returns {number} */
function observeNumber(value) {
  console.log(value);
  return value;
}

/** @param {number} option @returns {Selection} */
function selectFields(option) {
  return {
    $: 'selection',
    before: observeNumber(1),
    ...(observeNumber(option) === 0 ? { first: observeNumber(2) } : {}),
    ...{ ...(observeNumber(4) === option ? { second: undefined } : {}) },
    after: observeNumber(9),
  };
}

/** @param {Selection} value @returns {boolean} */
function ownsFirst(value) {
  return Object.hasOwn(value, 'first');
}

/** @param {Selection} value @returns {boolean} */
function ownsSecond(value) {
  return Object.hasOwn(value, 'second');
}

const first = selectFields(0);
console.log(ownsFirst(first));
console.log(ownsSecond(first));
const second = selectFields(4);
console.log(ownsFirst(second));
console.log(ownsSecond(second));
