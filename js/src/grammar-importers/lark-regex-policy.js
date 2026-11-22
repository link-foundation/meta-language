/** @param {string} flag @returns {boolean} */
export function acceptLarkRegexFlag(flag) {
  return flag === 'i' || flag === 's';
}

/** @param {string} character @returns {boolean} */
export function acceptLarkRegexQuantifier(character) {
  return character === '+' || character === '*' || character === '?';
}

/** @param {string} character @returns {boolean} */
export function acceptLarkRegexMetaCharacter(character) {
  return character === '(' || character === ')' || character === '|' || character === '^'
    || character === '$' || character === '{' || character === '}' || character === ']'
    || character === '+' || character === '*' || character === '?';
}

/** @param {number} point @returns {boolean} */
export function acceptLarkRegexLiteralEscape(point) {
  return point < 128 && (point < 48 || point > 57)
    && (point < 65 || point > 90) && (point < 97 || point > 122);
}
