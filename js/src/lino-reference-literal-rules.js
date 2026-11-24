// The Links Notation versioned reference boundary consumes a tilde, one or
// more decimal digits, an opening brace, and the payload through its closing
// brace. Hosts supply one code unit at a time and use the upstream decoder.
// State 3 marks a reserved prefix, state 4 a complete token, and -1 a mismatch.
/** @param {number} state @param {number} unit @returns {number} */
export function advanceReferenceLiteral(state, unit) {
  if (state === 0) return unit === 126 ? 1 : -1; // ~
  const digit = unit >= 48 && unit <= 57;
  if (state === 1) return digit ? 2 : -1;
  if (state === 2) {
    if (digit) return 2;
    return unit === 123 ? 3 : -1; // {
  }
  if (unit === 125) return 4; // }
  if (unit === 32 || unit === 9 || unit === 13 || unit === 10 || unit === 40 || unit === 41) return -1;
  return 3;
}
