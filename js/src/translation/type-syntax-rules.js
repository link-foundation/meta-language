// The existing outer-level JSDoc separator scan, over mechanically projected
// UTF16 units. Slice/trim and recursive child parsing stay with host adapters.
import { readRequiredNumberFact } from './fact-column-rules.js';

/** @param {number[]} units @param {number} separator @returns {number[]} */
export function readTypeSeparatorPositions(units, separator) {
  let depth = 0;
  const positions = [];
  let at = 0;
  while (at < units.length) {
    const unit = readRequiredNumberFact(units, at);
    if (unit === 123 || unit === 91 || unit === 60 || unit === 40) depth += 1;
    const previous = at > 0 ? readRequiredNumberFact(units, at - 1) : -1;
    if ((unit === 125 || unit === 93 || unit === 62 || unit === 41) && !(unit === 62 && previous === 61)) depth -= 1;
    if (depth === 0 && unit === separator) positions.push(at);
    at += 1;
  }
  return positions;
}

/** @param {string} field @returns {boolean} */
export function acceptUnionTypeField(field) {
  return field === 'kind' || field === 'alternatives';
}

/** @param {number} count @returns {boolean} */
export function acceptUnionTypeCount(count) {
  return Number.isSafeInteger(count) && count >= 2;
}

/** @param {string} target @returns {string} */
export function readUnionTargetRefusal(target) {
  if (target === 'Rust') return 'no Rust type for union';
  if (target === 'Lean') return 'no Lean type for union';
  if (target === 'Rocq') return 'no Rocq type for union';
  return '';
}
