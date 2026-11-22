/** @typedef {{$:'entry', value?: number}} Entry */

/** @returns {Entry} */
function missingEntry() {
  return { $: 'entry' };
}

/** @returns {Entry} */
function undefinedEntry() {
  return { $: 'entry', value: undefined };
}

/** @returns {Entry} */
function numericEntry() {
  return { $: 'entry', value: 7 };
}

/** @param {Entry} left @param {Entry} right @returns {Entry} */
function copyEntry(left, right) {
  return { ...left, ...right };
}

/** @param {Entry} item @returns {number} */
function readEntry(item) {
  return item.value ?? 0;
}

/** @param {Entry} item */
function printEntry(item) {
  console.log(Object.hasOwn(item, 'value'), readEntry(item));
}

printEntry(missingEntry());
printEntry(undefinedEntry());
printEntry(numericEntry());
printEntry(copyEntry(numericEntry(), missingEntry()));
printEntry(copyEntry(numericEntry(), undefinedEntry()));
