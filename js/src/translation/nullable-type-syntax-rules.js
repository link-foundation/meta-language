// The original composite type reader's ordered alternative and nullable-prefix
// decisions. Source slicing and recursive child type reading stay with hosts.
import { readRequiredTextFact } from './fact-column-rules.js';

/** Actions: 0 unchanged, 1 optional alternative, 2 ordered union,
 * 3 named absence, 4 nullable prefix. Absence: 1 null, 2 undefined, 3 both.
 * @param {string[]} alternatives @param {number} first @returns {number[]} */
export function readNullableTypeSyntax(alternatives, first) {
  if (alternatives.length > 1) {
    let missing = 0;
    let values = 0;
    let value = -1;
    let absence = 0;
    for (let at = 0; at < alternatives.length; at += 1) {
      const part = readRequiredTextFact(alternatives, at);
      if (part === 'null' || part === 'undefined') {
        missing += 1;
        absence = part === 'null' ? 1 : 2;
      } else {
        values += 1;
        value = at;
      }
    }
    if (missing > 0 && values === 1) return [1, value, missing > 1 ? 3 : absence];
    return [2, 0, 0];
  }
  const name = readRequiredTextFact(alternatives, 0);
  if (name === 'null') return [3, 0, 1];
  if (name === 'undefined') return [3, 0, 2];
  if (first === 63) return [4, 0, 1];
  return [0, 0, 0];
}

/** @param {number} code @returns {string} */
export function readNullableTypeAbsence(code) {
  if (code === 1) return 'null';
  if (code === 2) return 'undefined';
  if (code === 3) return 'both';
  return '';
}

/** @param {string} target @returns {string} */
export function readNullableTypeTargetRefusal(target) {
  return 'no ' + target + ' type for absence';
}
