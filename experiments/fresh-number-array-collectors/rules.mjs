// Generated from the complete linked collector grammar. Do not edit.
/** @param {number[]} tokens @param {number[]} pattern @returns {boolean} */
export function acceptFreshNumberArrayCollectorPattern(tokens, pattern) {
  if (tokens.length !== pattern.length || tokens.length > 256) return false;
  let fn = -1;
  let input = -1;
  let collector = -1;
  let index = -1;
  for (let position = 0; position < tokens.length; position += 1) {
    const actual = tokens[position] ?? -99;
    const expected = pattern[position] ?? -99;
    if (!Number.isSafeInteger(actual) || actual < 0 || actual >= 1256) return false;
    if (expected >= 0) {
      if (actual !== expected) return false;
    } else {
      if (actual < 1000) return false;
      if (expected === -1) { if (fn < 0) fn = actual; else if (actual !== fn) return false; }
      else if (expected === -2) { if (input < 0) input = actual; else if (actual !== input) return false; }
      else if (expected === -3) { if (collector < 0) collector = actual; else if (actual !== collector) return false; }
      else if (expected === -4) { if (index < 0) index = actual; else if (actual !== index) return false; }
      else return false;
    }
  }
  return fn >= 1000 && input >= 1000
    && collector >= 1000 && index >= 1000
    && fn !== input && fn !== collector && fn !== index
    && input !== collector && input !== index && collector !== index;
}
