// Complete dense array selection through existing portable operations.
// https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.at

/** @param {number[]} values @param {number} position @returns {number|undefined} */
export function readArrayRelativeElement(values, position) {
  const relative = Number.isNaN(position) ? 0 : Math.trunc(position);
  const index = relative < 0 ? values.length + relative : relative;
  if (index < 0 || index >= values.length) return undefined;
  return values[index];
}
