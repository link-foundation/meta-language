// The existing record JSDoc branch's structural decisions. The common outer
// separator, actual source slices/trim and recursive type parser stay shared.
import { readRequiredNumberFact } from './fact-column-rules.js';

/** @param {number} first @param {number} last @returns {boolean} */
export function acceptRecordTypeBoundary(first, last) {
  return first === 123 && last === 125;
}

/** The original branch selects the first actual colon, including nested text.
 * @param {number[]} units @returns {number} */
export function readRecordTypeFieldColon(units) {
  for (let at = 0; at < units.length; at += 1) {
    if (readRequiredNumberFact(units, at) === 58) return at;
  }
  return -1;
}

/** @param {number} last @returns {boolean} */
export function acceptRecordTypeOptionalName(last) {
  return last === 63;
}
