// The existing record JSDoc branch's structural decisions. The common outer
// separator, actual source slices/trim and recursive type parser stay shared.
import { readTypeSeparatorPositions } from './type-syntax-rules.js';

/** @param {number} first @param {number} last @returns {boolean} */
export function acceptRecordTypeBoundary(first, last) {
  return first === 123 && last === 125;
}

/** Read the first unquoted separator of the field name and type.
 * @param {number[]} units @returns {number} */
export function readRecordTypeFieldColon(units) {
  return readTypeSeparatorPositions(units, 58)[0] ?? -1;
}

/** @param {number} last @returns {boolean} */
export function acceptRecordTypeOptionalName(last) {
  return last === 63;
}
