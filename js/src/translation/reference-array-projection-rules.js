// A checked projection can borrow a shared array only when nothing between
// receiver evaluation and the scalar read can mutate it. Other reads retain
// their original payload snapshot and every existing runtime/error contract.

/** @param {string} kind @param {string} type @param {boolean[]} children @returns {boolean} */
export function acceptReferenceArrayProjectionIndex(kind, type, children) {
  if (type !== 'float' && type !== 'int' && type !== 'nat') return false;
  if (kind === 'var' || kind === 'lit') return children.length === 0;
  if (kind === 'unary' || kind === 'cast') {
    if (children.length !== 1) return false;
  } else if (kind === 'binary') {
    if (children.length !== 2) return false;
  } else if (kind === 'math') {
    if (children.length === 0) return false;
  } else return false;
  for (let index = 0; index < children.length; index += 1) if (children[index] !== true) return false;
  return true;
}
/** @param {string} operation @param {string} operand @param {string} kind @param {boolean} reference @param {string} result @param {boolean} index @returns {string} */
export function readReferenceArrayProjection(operation, operand, kind, reference, result, index) {
  if (operand !== 'referenceRead' || kind !== 'array' || !reference) return '';
  if (operation === 'length') return result === 'float' || result === 'int' || result === 'nat' ? operation : '';
  return index && (operation === 'index' || operation === 'optionalIndex') ? operation : '';
}
/** @param {string} language @param {string} operand @param {string} kind @param {boolean} reference @returns {boolean} */
export function acceptReferenceArraySourceOperands(language, operand, kind, reference) {
  return language === 'JavaScript' && operand === 'referenceRead' && kind === 'array' && reference;
}
/** @param {number} initial @param {string[]} operands @returns {number} */
export function readReferenceArrayProjectionSuffix(initial, operands) {
  let suffix = initial;
  let occupied = true;
  while (occupied) {
    occupied = false;
    for (let index = 0; index < operands.length; index += 1) {
      const text = operands[index] ?? '';
      if (text.includes('ml_reference_array' + suffix) || text.includes('ml_reference_position' + suffix)
          || text.includes('ml_reference_projection' + suffix)) occupied = true;
    }
    if (occupied) suffix += 1;
  }
  return suffix;
}
/** @param {string} operation @param {string} receiver @param {string} position @param {string} index @param {string} result @param {string} suffix @returns {string} */
export function renderReferenceArrayProjection(operation, receiver, position, index, result, suffix) {
  const owner = 'ml_reference_array' + suffix, key = 'ml_reference_position' + suffix;
  const value = 'ml_reference_projection' + suffix;
  const opening = '{ let ' + owner + ' = ' + receiver + '; ';
  if (operation === 'length') {
    const length = result === 'float' ? owner + '.borrow().len() as f64'
      : 'crate::ml::Big::from_u128(' + owner + '.borrow().len() as u128)';
    return opening + 'let ' + value + ' = ' + length + '; ' + value + ' }';
  }
  if (operation !== 'index' && operation !== 'optionalIndex') return '';
  const converted = index === 'float' ? 'crate::ml_array::number_index(' + key + ')' : key + '.to_index()';
  const method = operation === 'index' ? 'at' : 'optional_at';
  return opening + 'let ' + key + ' = ' + position + '; let ' + value + ' = crate::ml_array::' + method
    + '(&*' + owner + '.borrow(), ' + converted + '); ' + value + ' }';
}
