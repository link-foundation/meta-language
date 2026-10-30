// Dense standard arrays with explicit shared identity. Sparse slots and custom
// descriptors remain separate from this represented array storage model.

/** @param {string} operation @returns {string} */
export function readArrayStateMethodResult(operation) {
  return operation === 'splice' ? 'removed array' : operation === 'fill' || operation === 'copyWithin' ? 'receiver' : '';
}
/** @param {string} expression @param {string} operation @param {string} kind @param {boolean} reference @returns {boolean} */
export function acceptLinkedArrayStateMethod(expression, operation, kind, reference) {
  return expression === 'method' && kind === 'array' && reference && readArrayStateMethodResult(operation) !== '';
}
/** @param {string} operation @returns {number} */
export function readArrayStateMethodRequiredArguments(operation) { return operation === 'fill' ? 1 : 0; }
/** @param {string} operation @param {number} position @returns {string} */
export function readArrayStateMethodArgumentRole(operation, position) {
  if (operation === 'splice') return position < 2 ? 'number index' : 'element';
  if (operation === 'fill') return position === 0 ? 'element' : position === 1 ? 'number index' : position === 2 ? 'end index' : '';
  if (operation === 'copyWithin') return position < 2 ? 'number index' : position === 2 ? 'end index' : '';
  return '';
}
/** @param {string} kind @param {string} element @param {string} absence @returns {string} */
export function readArrayStateMethodArgumentProfile(kind, element, absence) {
  if (kind === 'float' || kind === 'bool' || kind === 'unit') return kind;
  return kind === 'option' && (element === 'float' || element === 'bool' || element === 'unit')
    && (absence === 'undefined' || absence === 'null' || absence === 'both') ? 'option ' + absence + ' ' + element : '';
}
/** Actual operands are bound before any storage borrow or numeric conversion.
 * @param {number} initial @param {string} receiver @param {string[]} arguments_ @returns {number} */
export function readArrayStateMethodSuffix(initial, receiver, arguments_) {
  const text = receiver + ' ' + arguments_.join(' ');
  let suffix = initial;
  while (text.includes('ml_array_owner' + suffix) || text.includes('ml_array_argument' + suffix + '_')) suffix += 1;
  return suffix;
}
/** @param {string} profile @param {string} value @param {string} undefinedValue @returns {string} */
export function renderArrayStateNumericValue(profile, value, undefinedValue) {
  if (profile === 'float') return value;
  if (profile === 'bool') return 'if ' + value + ' { 1f64 } else { 0f64 }';
  if (profile === 'unit') return undefinedValue;
  const parts = profile.split(' '), absence = parts[1] ?? '', element = parts[2] ?? '';
  const present = element === 'float' ? 'ml_index_present' : element === 'bool' ? 'if ml_index_present { 1f64 } else { 0f64 }' : 'f64::NAN';
  if (absence === 'both') return 'match ' + value + ' { crate::MlOptional::Value(ml_index_present) => ' + present
    + ', crate::MlOptional::Null => 0f64, crate::MlOptional::Undefined => ' + undefinedValue + ' }';
  return 'match ' + value + ' { Some(ml_index_present) => ' + present + ', None => ' + (absence === 'null' ? '0f64' : undefinedValue) + ' }';
}
/** @param {string} value @param {string} length @returns {string} */
export function renderArrayStateRelativeIndex(value, length) {
  return '{ let ml_relative_value: f64 = ' + value + '; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; '
    + 'let ml_bound = ' + length + ' as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }';
}
/** @param {string} operation @returns {string} */
export function readArrayStateMethodIdentity(operation) {
  return operation === 'splice' ? 'replace-array-elements' : operation === 'fill' ? 'fill-array-elements' : operation === 'copyWithin' ? 'copy-array-elements-within' : '';
}
/** @returns {string} */
export function readArrayStateMethodStatement() {
  return 'Dense shared array range mutations evaluate the actual receiver and every argument once in order, distinguish missing and undefined parameters, normalize Number indices without narrowing before bounds, retain the same receiver identity and original element references, and copy overlapping ranges in the required direction; splice returns a fresh removed array.';
}
/** @param {string} target @param {string} operation @param {string} receiver
 * @param {string[]} arguments_ @param {string[]} profiles @param {string[]} types
 * @param {string} suffix @returns {string} */
export function renderArrayStateMethodExpression(target, operation, receiver, arguments_, profiles, types, suffix) {
  if (readArrayStateMethodResult(operation) === '') return '';
  if (target === 'JavaScript') return '(' + receiver + ').' + operation + '(' + arguments_.join(', ') + ')';
  if (target !== 'Rust') return '';
  const owner = 'ml_array_owner' + suffix;
  let output = '{ let ' + owner + ' = ' + receiver + '; ';
  for (let index = 0; index < arguments_.length; index += 1) output += 'let ml_array_argument' + suffix + '_' + index + ': ' + (types[index] ?? '') + ' = ' + (arguments_[index] ?? '') + '; ';
  const names = [];
  for (let index = 0; index < arguments_.length; index += 1) names.push('ml_array_argument' + suffix + '_' + index);
  output += 'let ml_array_length = ' + owner + '.borrow().len(); ';
  if (operation === 'splice') {
    const begin = arguments_.length === 0 ? '0f64' : renderArrayStateNumericValue(profiles[0] ?? '', names[0] ?? '', 'f64::NAN');
    output += 'let ml_array_start = ' + renderArrayStateRelativeIndex(begin, 'ml_array_length') + '; ';
    const count = arguments_.length === 0 ? '0usize' : arguments_.length === 1 ? 'ml_array_length - ml_array_start'
      : '{ let ml_delete_value: f64 = ' + renderArrayStateNumericValue(profiles[1] ?? '', names[1] ?? '', 'f64::NAN')
        + '; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }';
    const inserted = [];
    for (let index = 2; index < arguments_.length; index += 1) inserted.push(names[index] ?? '');
    return output + 'let ml_array_count = ' + count + '; let ml_array_inserted = vec![' + inserted.join(', ') + ']; '
      + 'let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); '
      + 'let ml_removed: Vec<_> = ' + owner + '.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); '
      + 'std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) }';
  }
  const first = operation === 'fill' ? 1 : 0;
  const startValue = arguments_.length <= first ? '0f64' : renderArrayStateNumericValue(profiles[first] ?? '', names[first] ?? '', 'f64::NAN');
  output += 'let ml_array_start = ' + renderArrayStateRelativeIndex(startValue, 'ml_array_length') + '; ';
  const endValue = arguments_.length <= 2 ? 'ml_array_length as f64' : renderArrayStateNumericValue(profiles[2] ?? '', names[2] ?? '', 'ml_array_length as f64');
  const end = 'let ml_array_end = ' + renderArrayStateRelativeIndex(endValue, 'ml_array_length') + '; ';
  if (operation === 'fill') return output + end + '{ let mut ml_array_values = ' + owner + '.borrow_mut(); '
    + 'if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(' + (names[0] ?? '') + '); } } ' + owner + ' }';
  const fromValue = arguments_.length <= 1 ? '0f64' : renderArrayStateNumericValue(profiles[1] ?? '', names[1] ?? '', 'f64::NAN');
  return output + 'let ml_array_from = ' + renderArrayStateRelativeIndex(fromValue, 'ml_array_length') + '; '
    + end
    + 'let ml_array_count = ml_array_end.saturating_sub(ml_array_from).min(ml_array_length - ml_array_start); '
    + '{ let mut ml_array_values = ' + owner + '.borrow_mut(); if ml_array_from < ml_array_start && ml_array_start < ml_array_from + ml_array_count { '
    + 'for ml_array_index in (0..ml_array_count).rev() { ml_array_values[ml_array_start + ml_array_index] = ml_array_values[ml_array_from + ml_array_index].clone(); } '
    + '} else { for ml_array_index in 0..ml_array_count { ml_array_values[ml_array_start + ml_array_index] = ml_array_values[ml_array_from + ml_array_index].clone(); } } } ' + owner + ' }';
}
