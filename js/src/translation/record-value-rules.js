// Portable record plans use original entry positions and UTF16 code units.
// Child checking, lexical identities and target syntax remain with their owners.
import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';

/** @returns {number[]} */
function refuseRequiredRecordName() {
  throw new RangeError('missing required record name row');
}

/** Read the actual selected row, without inventing a name for missing data.
 * @param {number[][]} names @param {number} index @returns {number[]} */
function readRequiredRecordName(names, index) {
  if (index < 0 || index % 1 !== 0 || index >= names.length) {
    throw new RangeError('invalid required record name position');
  }
  return names[index] ?? refuseRequiredRecordName();
}

/** @param {number} count @param {boolean} reverse @returns {number[]} */
export function readRecordValueOrder(count, reverse) {
  if (!Number.isSafeInteger(count) || count < 0) return [-1];
  const order = [];
  for (let index = 0; index < count; index += 1) order.push(reverse ? count - index - 1 : index);
  return order;
}

/** @param {number[]} name @returns {boolean} */
export function acceptRecordFieldName(name) {
  for (let index = 0; index < name.length; index += 1) {
    const unit = name[index];
    if (!Number.isInteger(unit) || unit < 0 || unit > 65535) return false;
  }
  return true;
}

/** Compare two admitted UTF16 vectors; callers validate their complete inputs.
 * @param {number[]} left @param {number[]} right @returns {number} */
export function compareRecordFieldNames(left, right) {
  const end = left.length < right.length ? left.length : right.length;
  for (let index = 0; index < end; index += 1) {
    if (readRequiredNumberFact(left, index) < readRequiredNumberFact(right, index)) return -1;
    if (readRequiredNumberFact(left, index) > readRequiredNumberFact(right, index)) return 1;
  }
  if (left.length < right.length) return -1;
  if (left.length > right.length) return 1;
  return 0;
}

/** @param {number[][]} fields @param {number[]} name @returns {number} */
export function readRecordFieldPosition(fields, name) {
  if (!acceptRecordFieldName(name)) return -1;
  for (let index = 0; index < fields.length; index += 1) {
    if (!acceptRecordFieldName(readRequiredRecordName(fields, index))) return -1;
  }
  for (let index = 0; index < fields.length; index += 1) {
    const comparison = compareRecordFieldNames(readRequiredRecordName(fields, index), name);
    if (!(comparison < 0) && !(comparison > 0)) return index;
  }
  return -1;
}

/** @param {number[][]} fields @returns {number[]} */
function readRecordSortedFieldPositions(fields) {
  for (let index = 0; index < fields.length; index += 1) {
    if (!acceptRecordFieldName(readRequiredRecordName(fields, index))) return [-1];
  }
  let sorted = readRecordValueOrder(fields.length, false);
  // Stable merge passes retain source order among equal names, so selecting
  // each group's final index preserves the existing last-property rule.
  for (let width = 1; width < sorted.length; width *= 2) {
    const merged = [];
    for (let start = 0; start < sorted.length; start += width * 2) {
      const middle = start + width < sorted.length ? start + width : sorted.length;
      const end = middle + width < sorted.length ? middle + width : sorted.length;
      let left = start;
      let right = middle;
      while (left < middle || right < end) {
        if (right >= end || (left < middle && compareRecordFieldNames(readRequiredRecordName(fields, readRequiredNumberFact(sorted, left)), readRequiredRecordName(fields, readRequiredNumberFact(sorted, right))) <= 0)) {
          merged.push(readRequiredNumberFact(sorted, left));
          left += 1;
        } else {
          merged.push(readRequiredNumberFact(sorted, right));
          right += 1;
        }
      }
    }
    sorted = merged;
  }
  return sorted;
}

/** @param {number[][]} fields @returns {number[]} */
export function readRecordFieldOrder(fields) {
  const sorted = readRecordSortedFieldPositions(fields);
  if (sorted.length > 0 && readRequiredNumberFact(sorted, 0) < 0) return [-1];
  const selected = [];
  for (let index = 0; index < sorted.length; index += 1) {
    if (index + 1 >= sorted.length || compareRecordFieldNames(readRequiredRecordName(fields, readRequiredNumberFact(sorted, index)), readRequiredRecordName(fields, readRequiredNumberFact(sorted, index + 1))) < 0) selected.push(readRequiredNumberFact(sorted, index));
  }
  return selected;
}

/** Map updates retain first insertion order and the last actual value.
 * @param {number[][]} fields @returns {number[]} */
export function readRecordFieldInsertionOrder(fields) {
  const sorted = readRecordSortedFieldPositions(fields);
  if (sorted.length > 0 && readRequiredNumberFact(sorted, 0) < 0) return [-1];
  const first = [];
  const last = [];
  let start = 0;
  for (let index = 0; index < sorted.length; index += 1) {
    if (index + 1 >= sorted.length || compareRecordFieldNames(readRequiredRecordName(fields, readRequiredNumberFact(sorted, index)), readRequiredRecordName(fields, readRequiredNumberFact(sorted, index + 1))) < 0) {
      first.push(readRequiredNumberFact(sorted, start));
      last.push(readRequiredNumberFact(sorted, index));
      start = index + 1;
    }
  }
  const selected = [];
  for (let index = 0; index < fields.length; index += 1) {
    let lower = 0;
    let upper = last.length;
    while (lower < upper) {
      const middle = Math.floor((lower + upper) / 2);
      const comparison = compareRecordFieldNames(readRequiredRecordName(fields, index), readRequiredRecordName(fields, readRequiredNumberFact(last, middle)));
      if (comparison < 0) upper = middle;
      else if (comparison > 0) lower = middle + 1;
      else {
        if (!(readRequiredNumberFact(first, middle) < index) && !(readRequiredNumberFact(first, middle) > index)) selected.push(readRequiredNumberFact(last, middle));
        lower = upper;
      }
    }
  }
  return selected;
}

/** @param {string} kind @returns {string} */
export function readRecordSpreadRole(kind) {
  if (kind === 'var') return 'open';
  if (kind === 'record') return 'fields';
  return 'refuse';
}

/** A single original constraint step, re-read after every recursive unify or
 * add-field operation so actual shared record mutations remain visible.
 * Actions: 0 unify, 1 add right, 2 add left, 3 missing right, 4 missing left,
 * 5 continue, 6 finish. The final two cells carry the next phase and index.
 * @param {number[][]} left @param {number[][]} right @param {boolean} leftOpen
 * @param {boolean} rightOpen @param {number} phase @param {number} index
 * @returns {number[]} */
export function readRecordTypeConstraintStep(left, right, leftOpen, rightOpen, phase, index) {
  if (phase < 1) {
    if (index >= left.length) return [5, -1, -1, 1, 0];
    const matching = readRecordFieldPosition(right, readRequiredRecordName(left, index));
    if (matching >= 0) return [0, index, matching, 0, index + 1];
    return [rightOpen ? 1 : 3, index, -1, 0, index + 1];
  }
  if (index >= right.length) return [6, -1, -1, 1, index];
  const matching = readRecordFieldPosition(left, readRequiredRecordName(right, index));
  if (matching >= 0) return [5, matching, index, 1, index + 1];
  return [leftOpen ? 2 : 4, -1, index, 1, index + 1];
}

/** @param {number} action @param {string} name @returns {string} */
export function readRecordTypeConstraintError(action, name) {
  return action === 3 || action === 4 ? 'record field ' + name + ' is missing' : '';
}

/** Preserve record-method diagnostics before checking actual argument values.
 * @param {string} kind @param {string} operation @param {number} parameters
 * @param {number} argumentsCount @returns {string} */
export function readRecordMethodError(kind, operation, parameters, argumentsCount) {
  if (kind !== 'function') return 'record has no method ' + operation;
  if (argumentsCount !== parameters) return '.' + operation + '() expects ' + parameters + ' arguments';
  return '';
}

/** Actions: 0 unify the actual field, 1 add to the actual open record, 2 leave
 * a missing closed field to the existing checked projection diagnostic.
 * @param {number[][]} fields @param {number[]} name @param {boolean} open @returns {number[]} */
export function readRecordFieldInferenceStep(fields, name, open) {
  const index = readRecordFieldPosition(fields, name);
  return index >= 0 ? [0, index] : [open ? 1 : 2, -1];
}

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} owner @param {string} name @returns {number} */
function recordTypeFactField(starts, counts, labels, targets, owner, name) {
  if (owner < 0) return -1;
  const start = readRequiredNumberFact(starts, owner);
  const end = start + readRequiredNumberFact(counts, owner);
  for (let edge = start; edge < end; edge += 1) {
    if (readRequiredTextFact(labels, edge) === name) return readRequiredNumberFact(targets, edge);
  }
  return -1;
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} owner @returns {string} */
function recordTypeFactKind(texts, starts, counts, labels, targets, owner) {
  const kind = recordTypeFactField(starts, counts, labels, targets, owner, 'kind');
  return kind < 0 ? '' : readRequiredTextFact(texts, kind);
}

/** @param {number[]} starts @param {number[]} counts @param {number[]} targets
 * @param {number} owner @returns {number[]} */
function recordTypeFactElements(starts, counts, targets, owner) {
  const values = [];
  if (owner < 0) return values;
  const start = readRequiredNumberFact(starts, owner);
  const end = start + readRequiredNumberFact(counts, owner);
  for (let edge = start; edge < end; edge += 1) values.push(readRequiredNumberFact(targets, edge));
  return values;
}

/** The target is an actual retained Record owner, rather than a spelling or
 * freshly reconstructed type. Variable bindings refer to the same fact IDs.
 * @param {string[]} texts @param {number[]} numbers @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number[]} bindings @param {number} root @param {number} owner
 * @returns {boolean} */
export function readRecordTypeOwnerPresence(texts, numbers, starts, counts, labels, targets, bindings, root, owner) {
  const pending = [root];
  const seen = [];
  for (let position = 0; position < pending.length; position += 1) {
    let node = readRequiredNumberFact(pending, position);
    let kind = recordTypeFactKind(texts, starts, counts, labels, targets, node);
    while (kind === 'var') {
      const id = recordTypeFactField(starts, counts, labels, targets, node, 'id');
      const variable = readRequiredNumberFact(numbers, id);
      if (!Number.isInteger(variable) || variable < 0 || variable >= bindings.length) break;
      const bound = readRequiredNumberFact(bindings, variable);
      const boundKind = recordTypeFactKind(texts, starts, counts, labels, targets, bound);
      if (boundKind === '') break;
      node = bound;
      kind = boundKind;
    }
    if (node === owner) return true;
    let visited = false;
    for (let index = 0; index < seen.length; index += 1) {
      if (readRequiredNumberFact(seen, index) === node) visited = true;
    }
    if (visited) continue;
    seen.push(node);
    if (kind === 'array' || kind === 'option' || kind === 'set' || kind === 'cell') {
      pending.push(recordTypeFactField(starts, counts, labels, targets, node, 'element'));
    } else if (kind === 'map') {
      pending.push(recordTypeFactField(starts, counts, labels, targets, node, 'key'));
      pending.push(recordTypeFactField(starts, counts, labels, targets, node, 'value'));
    } else if (kind === 'tuple' || kind === 'union' || kind === 'function') {
      const field = kind === 'tuple' ? 'elements' : kind === 'union' ? 'alternatives' : 'params';
      const sequence = recordTypeFactField(starts, counts, labels, targets, node, field);
      const children = recordTypeFactElements(starts, counts, targets, sequence);
      for (let index = 0; index < children.length; index += 1) pending.push(readRequiredNumberFact(children, index));
      if (kind === 'function') pending.push(recordTypeFactField(starts, counts, labels, targets, node, 'ret'));
    } else if (kind === 'record') {
      const sequence = recordTypeFactField(starts, counts, labels, targets, node, 'fields');
      const fields = recordTypeFactElements(starts, counts, targets, sequence);
      for (let index = 0; index < fields.length; index += 1) {
        pending.push(recordTypeFactField(starts, counts, labels, targets, readRequiredNumberFact(fields, index), 'type'));
      }
    }
  }
  return false;
}
