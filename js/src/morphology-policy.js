/** @param {number} integer @returns {number} */
export function russianNumberAgreementKind(integer) {
  const ones = integer % 10;
  const tens = integer % 100;
  if (ones === 1 && tens !== 11) return 0;
  if (ones >= 2 && ones <= 4 && (tens < 12 || tens > 14)) return 1;
  return 2;
}
