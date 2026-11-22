// The portable source closure has no host snapshot publication capability.
// All heaps in that closure pass through the complete structural validator.
/** @param {JavaScriptOrdinaryHeap} _heap @returns {boolean} */
export function testJavaScriptValidatedHeapSnapshot(_heap) {
  return false;
}
