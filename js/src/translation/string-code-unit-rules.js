// Read one UTF-16 code unit through operations already represented by the
// portable core. This complete function supplies the shared callable template.

/** @param {string} text @param {number} position @returns {number} */
export function readStringCodeUnit(text, position) {
  const index = Number.isNaN(position) ? 0 : Math.trunc(position);
  if (index < 0 || index >= text.length) return NaN;
  return text.slice(index, index + 1).codePointAt(0) ?? NaN;
}
