// Exact 64-bit allocation capacity from four unsigned 16-bit words and the
// single 2^64 exhausted marker. Counts cover actual bounded batch requests.
/** @param {number} value @returns {boolean} */
export function acceptLinkAllocationWord(value) {
  return Number.isSafeInteger(value) && value >= 0 && value <= 65535;
}
/** @param {number} first @param {number} second @param {number} third @param {number} fourth @param {boolean} exhausted @param {number} count @returns {boolean} */
export function acceptLinkAllocationCapacity(first, second, third, fourth, exhausted, count) {
  if (!acceptLinkAllocationWord(first) || !acceptLinkAllocationWord(second)
      || !acceptLinkAllocationWord(third) || !acceptLinkAllocationWord(fourth)
      || !Number.isSafeInteger(count) || count < 0 || count > 4294967295) return false;
  const zero = first === 0 && second === 0 && third === 0 && fourth === 0;
  if (exhausted) return zero && count === 0;
  if (zero) return false;
  if (third < 65535 || fourth < 65535) return true;
  const available = (65535 - second) * 65536 + 65536 - first;
  return count <= available;
}
