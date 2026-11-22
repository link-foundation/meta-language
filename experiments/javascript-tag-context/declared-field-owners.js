/** @typedef {{ $: 'shared', value: number } | { $: 'empty' }} Input */
/** @typedef {{ $: 'shared', value: boolean } | { $: 'empty' }} Output */
/** @typedef {{ payload: Input }} RecordCarrier */
/** @typedef {{ $: 'available', payload: Output } | { $: 'unavailable' }} TaggedCarrier */

/** @param {RecordCarrier} carrier @returns {number} */
function readRecordCarrier(carrier) {
  const payload = carrier.payload;
  if (payload.$ === 'shared') return payload.value;
  return -1;
}

/** @param {TaggedCarrier} carrier @returns {boolean} */
function readTaggedCarrier(carrier) {
  if (carrier.$ === 'available') {
    const payload = carrier.payload;
    if (payload.$ === 'shared') return payload.value;
  }
  return false;
}

/** @param {number} value @returns {RecordCarrier} */
function makeRecordCarrier(value) {
  return { payload: { $: 'shared', value } };
}

/** @param {boolean} value @returns {TaggedCarrier} */
function makeTaggedCarrier(value) {
  return { $: 'available', payload: { $: 'shared', value } };
}

console.log(readRecordCarrier(makeRecordCarrier(7)));
console.log(readRecordCarrier({ payload: { $: 'empty' } }));
console.log(readTaggedCarrier(makeTaggedCarrier(true)));
console.log(readTaggedCarrier({ $: 'unavailable' }));
