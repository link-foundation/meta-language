import assert from 'node:assert/strict';
/** @param {boolean} x @returns {boolean} */
function loud(x) { console.log('loud'); return x; }
assert(loud(true) && true);
console.log('done');
