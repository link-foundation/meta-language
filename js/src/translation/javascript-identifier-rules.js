// Identifier escapes contribute one scalar independently. Unlike string
// escapes, two surrogate escapes cannot combine into an identifier scalar.
/** @param {number[]} units @param {number} start @returns {number[]} */
export function decodeJavaScriptIdentifierEscape(units, start) {
  if (!Number.isSafeInteger(start) || start < 0 || start + 2 > units.length
      || units[start] !== 92 || units[start + 1] !== 117) return [-1, Math.min(units.length, start + 2)];
  const braced = units[start + 2] === 123;
  let index = start + (braced ? 3 : 2), digits = 0, scalar = 0;
  while (index < units.length && (braced || digits < 4)) {
    const unit = units[index] ?? -1;
    let digit = -1;
    if (unit >= 48 && unit <= 57) digit = unit - 48;
    else if (unit >= 65 && unit <= 70) digit = unit - 55;
    else if (unit >= 97 && unit <= 102) digit = unit - 87;
    if (digit < 0) break;
    scalar = scalar * 16 + digit;
    digits += 1;
    index += 1;
    if (scalar > 1114111) return [-1, index];
  }
  if (braced) {
    if (digits === 0 || units[index] !== 125) return [-1, Math.min(units.length, index + 1)];
    index += 1;
  } else if (digits !== 4) return [-1, Math.min(units.length, index + 1)];
  if (scalar >= 55296 && scalar <= 57343) return [-1, index];
  return [index, scalar];
}
/** @param {string} name @returns {boolean} */
export function acceptJavaScriptReservedIdentifier(name) {
  return ['await', 'break', 'case', 'catch', 'class', 'const', 'continue', 'debugger', 'default', 'delete', 'do',
    'else', 'enum', 'export', 'extends', 'false', 'finally', 'for', 'function', 'if', 'import', 'in', 'instanceof',
    'new', 'null', 'return', 'super', 'switch', 'this', 'throw', 'true', 'try', 'typeof', 'var', 'void', 'while',
    'with', 'yield', 'let', 'static', 'implements', 'interface', 'package', 'private', 'protected', 'public'].includes(name);
}
/** @param {string} name @param {string} raw @returns {boolean} */
export function acceptJavaScriptSourceKeywordToken(name, raw) {
  return !raw.includes('\\') || !(acceptJavaScriptReservedIdentifier(name)
    || ['as', 'async', 'from', 'get', 'meta', 'of', 'set', 'target'].includes(name));
}
/** @param {string} kind @param {string} name @param {string} raw @returns {string} */
export function readJavaScriptSourceTokenGoalValue(kind, name, raw) {
  return kind === 'identifier' && !acceptJavaScriptSourceKeywordToken(name, raw) ? '' : name;
}
/** @param {string} name @param {string} raw @param {boolean} binding @returns {string} */
export function readJavaScriptEscapedIdentifierError(name, raw, binding) {
  if (!raw.includes('\\')) return '';
  if (acceptJavaScriptReservedIdentifier(name)) return 'escaped JavaScript identifier is reserved: ' + name;
  return binding && (name === 'arguments' || name === 'eval') ? 'invalid strict JavaScript binding identifier: ' + name : '';
}
/** @param {string} context @returns {boolean} */
export function acceptJavaScriptBindingIdentifierContext(context) {
  return ['function', 'parameter', 'closure parameter', 'array binding', 'namespace', 'const', 'let', 'for…of'].includes(context);
}
/** @param {boolean} identifier @param {number} following @returns {boolean} */
export function acceptJavaScriptNumberIdentifierBoundary(identifier, following) {
  return !identifier && following !== 92 && !(following >= 48 && following <= 57);
}
