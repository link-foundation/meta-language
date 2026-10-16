// Record layout is supplied by the checked type. Host adapters render each
// actual child once and quote JavaScript property labels with the existing
// source-string adapter before applying these target syntax rules.
import { readRequiredTextFact } from './fact-column-rules.js';

/** @param {string[]} items @param {string} separator @returns {string} */
function joinRecordItems(items, separator) {
  let text = '';
  let index = 0;
  while (index < items.length) {
    if (index > 0) text += separator;
    text += readRequiredTextFact(items, index);
    index += 1;
  }
  return text;
}

/** @param {string} target @param {string[]} labels @param {string[]} items @returns {string} */
export function renderRecordValue(target, labels, items) {
  if (target === 'JavaScript') {
    const entries = [];
    let index = 0;
    while (index < items.length) {
      entries.push('[' + readRequiredTextFact(labels, index) + ']: ' + readRequiredTextFact(items, index));
      index += 1;
    }
    return '({' + joinRecordItems(entries, ', ') + '})';
  }
  if (target === 'Rust') return '(' + joinRecordItems(items, ', ') + (items.length === 1 ? ',' : '') + ')';
  if (target === 'Lean') return items.length > 0 ? '(' + joinRecordItems(items, ', ') + ')' : '()';
  if (target === 'Rocq') return items.length > 0 ? '(' + joinRecordItems(items, ', ') + ')' : 'tt';
  return '';
}

/** The index text is the host's exact existing checked integer spelling.
 * @param {string} target @param {string} object @param {string} field
 * @param {string} index @returns {string} */
export function renderRecordField(target, object, field, index) {
  if (target === 'JavaScript') return '(' + object + ')[' + field + ']';
  if (target === 'Rust') return '(' + object + ').' + index;
  return '';
}

/** @param {string} target @param {string} object @param {number} index
 * @param {number} count @returns {string} */
export function renderRecordProductField(target, object, index, count) {
  if (target === 'Lean') {
    if (count === 1) return object;
    let suffix = '';
    let position = 0;
    while (position < index) {
      suffix += '.2';
      position += 1;
    }
    return '(' + object + ')' + suffix + (index < count - 1 ? '.1' : '');
  }
  if (target === 'Rocq') {
    let value = object;
    let position = count - 1;
    while (position > index) {
      value = '(fst ' + value + ')';
      position -= 1;
    }
    return index === 0 ? value : '(snd ' + value + ')';
  }
  return '';
}

/** @param {string} target @param {string[]} fields @returns {string} */
export function renderRecordType(target, fields) {
  if (target === 'Lean') return fields.length > 0 ? '(' + joinRecordItems(fields, ' × ') + ')' : 'Unit';
  if (target === 'Rocq') return fields.length > 0 ? '(' + joinRecordItems(fields, ' * ') + ')%type' : 'unit';
  return '';
}
