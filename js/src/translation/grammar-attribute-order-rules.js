// Closed decisions for ordinary JavaScript own-key order. Hosts supply UTF-16
// code units and retain insertion order between non-index attribute names.
/** @param {number[]} units @returns {number} */
export function readGrammarAttributeIntegerKey(units) {
  if (units.length < 1 || units.length > 10
    || (units.length > 1 && units[0] === 48)) return -1;
  let value = 0;
  for (let index = 0; index < units.length; index += 1) {
    const unit = units[index];
    if (!Number.isInteger(unit) || unit < 48 || unit > 57) return -1;
    value = value * 10 + unit - 48;
  }
  return value <= 4294967294 ? value : -1;
}

/** @param {number} left @param {number} right @returns {number} */
export function compareGrammarAttributeIntegerKeys(left, right) {
  if (left < 0) return right < 0 ? 0 : 1;
  if (right < 0) return -1;
  return left < right ? -1 : left > right ? 1 : 0;
}
