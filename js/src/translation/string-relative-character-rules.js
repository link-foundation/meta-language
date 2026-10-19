// Complete relative UTF-16 selection through existing portable operations.
// https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.at

/** @param {string} text @param {number} position @returns {string|undefined} */
export function readStringRelativeCharacter(text, position) {
  const relative = Number.isNaN(position) ? 0 : Math.trunc(position);
  const index = relative < 0 ? text.length + relative : relative;
  if (index < 0 || index >= text.length) return undefined;
  return text.slice(index, index + 1);
}
