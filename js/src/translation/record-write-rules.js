// Qualified state operations for existing own fields of closed shared records.
// Source assignments are statements; their discarded result is explicit Unit.

/** @param {string} operator @returns {boolean} */
export function acceptRecordFieldAssignmentOperator(operator) {
  return operator === '=' || operator === '+=' || operator === '-=' || operator === '*='
    || operator === '/=' || operator === '%=' || operator === '**=' || operator === '++' || operator === '--';
}
/** @param {string} operator @param {string} field @returns {string} */
export function readRecordFieldWriteIdentity(operator, field) {
  return acceptRecordFieldAssignmentOperator(operator) ? 'writeRecordField' + operator + ':' + field : '';
}
/** @param {string} operation @returns {string} */
export function readRecordFieldWriteOperator(operation) {
  const operators = ['=', '+=', '-=', '*=', '/=', '%=', '**=', '++', '--'];
  for (const operator of operators) if (operation.startsWith('writeRecordField' + operator + ':')) return operator;
  return '';
}
/** @param {string} operation @returns {string} */
export function readRecordFieldWriteName(operation) {
  const operator = readRecordFieldWriteOperator(operation);
  return operator === '' ? '' : operation.slice(17 + operator.length);
}
/** @param {string} operator @returns {number} */
export function readRecordFieldWriteArgumentCount(operator) {
  return operator === '++' || operator === '--' ? 0 : 1;
}
/** Conservative text reservation also protects nested generated operands.
 * @param {number} initial @param {string} receiver @param {string[]} arguments_
 * @returns {number} */
export function readRecordFieldWriteSuffix(initial, receiver, arguments_) {
  const text = receiver + ' ' + arguments_.join(' ');
  let suffix = initial;
  while (text.includes('ml_field_owner' + suffix) || text.includes('ml_field_value' + suffix)
    || text.includes('ml_field_previous' + suffix)) suffix += 1;
  return suffix;
}
/** @param {string} expression @param {string} operation @param {string} kind
 * @param {boolean} reference @returns {boolean} */
export function acceptLinkedRecordFieldWrite(expression, operation, kind, reference) {
  return expression === 'method' && kind === 'record' && reference && readRecordFieldWriteOperator(operation) !== '';
}
/** @param {string} expression @param {string} kind @param {boolean} ordered @returns {string} */
export function readLinkedOrderedRecordSnapshotOperation(expression, kind, ordered) {
  return expression === 'referenceRead' && kind === 'record' && ordered ? 'readOrderedRecordSnapshot' : '';
}
/** @param {string} operator @param {string} kind @param {boolean} reference
 * @param {boolean} open @param {boolean} field @param {string} valueKind @returns {string} */
export function readRecordFieldWriteRefusal(operator, kind, reference, open, field, valueKind) {
  if (!acceptRecordFieldAssignmentOperator(operator)) return 'this assignment operator needs its complete state operation';
  if (kind !== 'record' || !reference || open) return 'a field write needs a closed shared record with explicit own fields';
  if (!field) return 'creating a field needs a dynamic property and descriptor representation';
  if (operator !== '=' && valueKind !== 'float') return 'this compound field assignment needs its actual coercion and value domain';
  return '';
}
/** Additional JavaScript domains normalize through the ordinary checked binary operation.
 * The original refusal remains the contract of the direct Number write renderer.
 * @param {string} language @param {string} operator @param {string} kind @returns {string} */
export function readRecordCompoundBinaryOperation(language, operator, kind) {
  if (language !== 'JavaScript' || (kind !== 'int' && kind !== 'string')) return '';
  if (kind === 'string') return operator === '+=' ? 'plus' : '';
  return readRecordNumericCompoundBinaryOperation(operator);
}
/** @param {string} operator @returns {string} */
function readRecordNumericCompoundBinaryOperation(operator) {
  if (operator === '+=' || operator === '++') return 'plus';
  if (operator === '-=' || operator === '--') return 'sub';
  if (operator === '*=') return 'mul';
  if (operator === '/=') return 'div';
  if (operator === '%=') return 'rem';
  if (operator === '**=') return 'exponentiation';
  return '';
}
/** @param {string} language @param {string} operator @param {string} kind
 * @param {string} absence @returns {string} */
export function readOptionalRecordCompoundBinaryOperation(language, operator, kind, absence) {
  if (language !== 'JavaScript' || absence !== 'undefined' && absence !== 'both') return '';
  if (kind === 'float') return readRecordNumericCompoundBinaryOperation(operator);
  if (kind === 'string') return readRecordCompoundBinaryOperation(language, operator, kind);
  return '';
}
/** @param {string} language @param {string} operator @param {string} kind
 * @param {boolean} reference @param {boolean} open @param {boolean} field
 * @param {string} valueKind @returns {string} */
export function readRecordFieldSourceWriteRefusal(language, operator, kind, reference, open, field, valueKind) {
  const operation = readRecordCompoundBinaryOperation(language, operator, valueKind);
  if (operation !== '') {
    if (kind !== 'record' || !reference || open) return 'a field write needs a closed shared record with explicit own fields';
    if (!field) return 'creating a field needs a dynamic property and descriptor representation';
    return '';
  }
  return readRecordFieldWriteRefusal(operator, kind, reference, open, field, valueKind);
}
/** @param {string} target @param {string} operator @param {string} receiver
 * @param {string} field @param {string} index @param {string[]} arguments_
 * @param {string} suffix @returns {string} */
export function renderRecordFieldWrite(target, operator, receiver, field, index, arguments_, suffix) {
  const owner = 'ml_field_owner' + suffix, value = 'ml_field_value' + suffix, previous = 'ml_field_previous' + suffix;
  const unary = operator === '++' || operator === '--';
  if (target === 'JavaScript') return '(void ((' + receiver + ')[' + field + ']'
    + operator + (unary ? '' : '(' + (arguments_[0] ?? '') + ')') + '))';
  if (target !== 'Rust') return '';
  let output = '{ let ' + owner + ' = ' + receiver + '; ';
  if (operator !== '=') output += 'let ' + previous + ' = ' + owner + '.borrow().' + index + '; ';
  if (!unary) output += 'let ' + value + ' = ' + (arguments_[0] ?? '') + '; ';
  const result = operator === '=' ? value : operator === '++' ? previous + ' + 1.0'
    : operator === '--' ? previous + ' - 1.0' : operator === '**='
      ? 'if ' + value + ' == 0.0 { 1.0 } else if ' + previous + '.is_nan() || ' + value
        + '.is_nan() || (' + previous + '.abs() == 1.0 && ' + value + '.is_infinite()) { f64::NAN } else { ' + previous + '.powf(' + value + ') }'
    : previous + ' ' + operator.slice(0, 1) + ' ' + value;
  return output + owner + '.borrow_mut().' + index + ' = ' + result + '; () }';
}
/** Numeric keys are inserted ahead of greater numeric keys and every normal
 * String key. A new String key is appended; overwrites retain their positions.
 * @param {number[]} numericKeys @param {number} field @returns {number[]} */
export function readOptionalRecordFieldOrderFollowers(numericKeys, field) {
  const selected = numericKeys[field] ?? -1;
  const followers = [];
  if (selected < 0) return followers;
  for (let index = 0; index < numericKeys.length; index += 1) {
    const key = numericKeys[index] ?? -1;
    if (key < 0 || key > selected) followers.push(index);
  }
  return followers;
}

/** @param {string} receiver @param {string} index @param {string} argument
 * @param {string} suffix @param {boolean} ordered @param {string[]} followers @returns {string} */
export function renderOptionalRustRecordFieldWrite(receiver, index, argument, suffix, ordered, followers) {
  const owner = 'ml_field_owner' + suffix, value = 'ml_field_value' + suffix;
  const capture = '{ let ' + owner + ' = ' + receiver + '; let ' + value + ' = ' + argument + '; ';
  const storage = 'let mut ' + owner + ' = ' + owner + '.borrow_mut(); ';
  if (!ordered) return capture + storage + owner + '.' + index + ' = Some((' + value + ',)); () }';
  const field = owner + '.0.' + index, order = owner + '.1';
  const position = followers.length === 0 ? order + '.len()'
    : order + '.iter().position(|ml_ordered_field| [' + followers.join(', ') + '].contains(ml_ordered_field)).unwrap_or(' + order + '.len())';
  const next = 'ml_field_previous' + suffix;
  return capture + storage + 'if ' + field + '.is_none() { let ' + next + ' = ' + position
    + '; ' + order + '.insert(' + next + ', ' + index + '.0); } ' + field + ' = Some((' + value + ',)); () }';
}
/** @returns {string} */
export function readRecordFieldWriteStatement() {
  return 'An own field write evaluates its actual shared receiver once, reads a compound operand before the right-hand expression, evaluates that expression once, then writes the same field allocation after releasing every earlier storage borrow; its statement result is Unit.';
}
