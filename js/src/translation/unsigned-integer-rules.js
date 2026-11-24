// Mechanically copied complete source declaration; do not edit.
// Qualified copy pending complete equivalence admission; regenerate with copy-unsigned-integer-definition.mjs.
/** @param {number} value @returns {number} */
export function normalizeJavaScriptUnsignedInteger(value) {
  if (!Number.isFinite(value) || value === 0) return 0;
  const integral = value < 0 ? -Math.floor(-value) : Math.floor(value);
  const remainder = integral % 4294967296;
  if (remainder < 0) return remainder + 4294967296;
  // A negative exact multiple produces -0 under %, but ToUint32 produces +0.
  if (remainder === 0) return 0;
  return remainder;
}
