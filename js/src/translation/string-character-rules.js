// Complete UTF-16 character selection through existing portable operations.
// https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.charat

/** @param {string} text @param {number} position @returns {string} */
export function readStringCharacter(text, position) {
  const index = Number.isNaN(position) ? 0 : Math.trunc(position);
  if (index < 0 || index >= text.length) return '';
  return text.slice(index, index + 1);
}
