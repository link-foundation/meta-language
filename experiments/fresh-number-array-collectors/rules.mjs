// Generated from the complete linked collector grammar. Do not edit.
/** @param {string} name @returns {boolean} */
export function acceptFreshNumberArrayCollectorBinding(name) {
  return name !== "" && name !== "break" && name !== "case" && name !== "catch" && name !== "class" && name !== "const" && name !== "continue" && name !== "debugger" && name !== "default" && name !== "delete" && name !== "do" && name !== "else" && name !== "enum" && name !== "export" && name !== "extends" && name !== "false" && name !== "finally" && name !== "for" && name !== "function" && name !== "if" && name !== "import" && name !== "in" && name !== "instanceof" && name !== "new" && name !== "null" && name !== "return" && name !== "super" && name !== "switch" && name !== "this" && name !== "throw" && name !== "true" && name !== "try" && name !== "typeof" && name !== "var" && name !== "void" && name !== "while" && name !== "with";
}
/** @param {string} name @returns {boolean} */
export function acceptFreshNumberArrayCollectorFunctionName(name) {
  return acceptFreshNumberArrayCollectorBinding(name) && name !== "undefined" && name !== "NaN" && name !== "Infinity";
}
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
