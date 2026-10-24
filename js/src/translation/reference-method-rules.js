// Shared method admission and target construction. Operation identities refer
// to the existing shared reference-operations.lino definitions.

/** @param {string} kind @param {string} operation @returns {string} */
export function readReferenceMethodResult(kind, operation) {
  if (kind === 'array') {
    if (operation === 'push' || operation === 'unshift') return 'length';
    if (operation === 'pop' || operation === 'shift') return 'optional element';
    if (operation === 'reverse') return 'receiver';
  }
  if (kind === 'set' && operation === 'add' || kind === 'map' && operation === 'set') return 'receiver';
  if (kind === 'set' || kind === 'map') {
    if (operation === 'has' || operation === 'delete') return 'boolean';
    if (operation === 'clear') return 'unit';
    if (kind === 'map' && operation === 'get') return 'optional value';
  }
  return '';
}

/** An additive checked-wire profile; the original operation union stays exact.
 * @param {string} expression @param {string} operation @param {string} receiver
 * @param {boolean} reference @returns {boolean}
 */
export function acceptLinkedReferenceMethodOperation(expression, operation, receiver, reference) {
  return expression === 'method' && reference && readReferenceMethodResult(receiver, operation) !== '';
}

/** @param {string} kind @param {string} operation @returns {string} */
export function readReferenceMethodRefusal(kind, operation) {
  if ((kind === 'map' || kind === 'set') && (operation === 'keys' || operation === 'values' || operation === 'entries')) {
    return 'live collection iteration needs ordered entries and tombstones rather than a snapshot';
  }
  return '';
}

/** @param {string} expression @param {string} operation @returns {number} */
export function readReferenceCallbackEffect(expression, operation) {
  if (expression === 'method' && (operation === 'splice' || operation === 'fill' || operation === 'copyWithin')) return 1;
  if (expression === 'method' && operation.startsWith('writeRecordField')) return 1;
  if (expression === 'cellWrite' || expression === 'referenceWrite') return 1;
  if (expression === 'method' && operation !== 'has' && operation !== 'get'
    && (readReferenceMethodResult('array', operation) !== '' || readReferenceMethodResult('map', operation) !== ''
      || readReferenceMethodResult('set', operation) !== '')) return 1;
  if (expression === 'call' || expression === 'functionRef') return 2;
  if (expression === 'apply') return 3;
  return 0;
}

/** @param {string} operation @param {number} count @returns {boolean} */
export function acceptSnapshotCallbackParameters(operation, count) {
  return count < (operation === 'reduce' ? 4 : 3);
}

/** Scoped/control operands are absent from the flat codec operand inventory.
 * @param {string} expression @param {string} ordinary @returns {string}
 */
export function readReferenceCallbackOperandFields(expression, ordinary) {
  if (expression === 'lambda') return 'body';
  if (expression === 'let') return 'value body';
  if (expression === 'match') return 'scrutinee cases';
  if (expression === 'outCons') return 'head tail';
  return ordinary;
}

/** @param {string} kind @param {string} operation @returns {number} */
export function readReferenceMethodRequiredArguments(kind, operation) {
  if (kind === 'map' && operation === 'set') return 2;
  if (operation === 'add' || operation === 'get' || operation === 'has' || operation === 'delete') return 1;
  return 0;
}

/** Unused extra arguments are still evaluated in their original order.
 * @param {string} kind @param {string} operation @param {number} position @returns {string}
 */
export function readReferenceMethodArgumentRole(kind, operation, position) {
  if (kind === 'array' && (operation === 'push' || operation === 'unshift')) return 'element';
  if ((kind === 'set' || kind === 'map') && position === 0 && operation !== 'clear') return 'key';
  if (kind === 'map' && operation === 'set' && position === 1) return 'value';
  return '';
}

/** @param {string} kind @returns {boolean} */
export function acceptReferenceMethodKeyKind(kind) {
  return kind === 'bool' || kind === 'float' || kind === 'string' || kind === 'int' || kind === 'nat' || kind === 'fixed';
}

/** Actual linked identities of operations with shared mutation semantics.
 * @param {string} kind @param {string} operation @returns {string}
 */
export function readReferenceMethodIdentity(kind, operation) {
  if (kind === 'array') {
    if (operation === 'push') return 'append-array-elements';
    if (operation === 'pop') return 'remove-final-array-element';
    if (operation === 'shift') return 'remove-initial-array-element';
    if (operation === 'unshift') return 'prepend-array-elements';
    if (operation === 'reverse') return 'reverse-array-elements';
  }
  if (kind === 'set') {
    if (operation === 'add') return 'add-set-member';
    if (operation === 'delete') return 'remove-set-member';
    if (operation === 'clear') return 'clear-set-members';
    if (operation === 'has') return 'read-set-membership';
  }
  if (kind === 'map') {
    if (operation === 'set') return 'set-map-entry';
    if (operation === 'delete') return 'remove-map-entry';
    if (operation === 'clear') return 'clear-map-entries';
    if (operation === 'get') return 'read-map-entry';
    if (operation === 'has') return 'read-map-membership';
  }
  return '';
}

/** @returns {string} */
export function readReferenceMethodStatement() {
  return 'Shared collection methods evaluate their actual receiver and every argument once in source order; each declared mutation changes the same allocation, preserving insertion order, SameValueZero primitive keys and the declared JavaScript result.';
}

/** @param {string} target @param {string} kind @param {string} operation
 * @param {string} receiver @param {string[]} arguments_ @param {boolean} floatKey
 * @param {string} suffix @returns {string}
 */
export function renderReferenceMethodExpression(target, kind, operation, receiver, arguments_, floatKey, suffix) {
  if (readReferenceMethodResult(kind, operation) === '') return '';
  if (target === 'JavaScript') return '(' + receiver + ').' + operation + '(' + arguments_.join(', ') + ')';
  if (target !== 'Rust') return '';
  const owner = 'ml_method_owner' + suffix, values = 'ml_method_values' + suffix;
  let prefix = '{ let ' + owner + ' = ' + receiver + '; ';
  for (let index = 0; index < arguments_.length; index += 1) prefix += 'let ml_method_argument' + suffix + '_' + index + ' = ' + (arguments_[index] ?? '') + '; ';
  const first = 'ml_method_argument' + suffix + '_0', second = 'ml_method_argument' + suffix + '_1';
  if (kind === 'array') {
    if (operation === 'push' || operation === 'unshift') {
      const names = [];
      for (let index = 0; index < arguments_.length; index += 1) names.push('ml_method_argument' + suffix + '_' + index);
      const changes = operation === 'push' ? values + '.extend(ml_inserted);' : values + '.splice(0..0, ml_inserted);';
      return prefix + 'let ml_inserted = vec![' + names.join(', ') + ']; let mut ' + values + ' = ' + owner + '.borrow_mut(); '
        + 'let ml_length = ' + values + '.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); '
        + changes + ' ml_length as f64 }';
    }
    if (operation === 'pop') return prefix + 'let ml_result = ' + owner + '.borrow_mut().pop(); ml_result }';
    if (operation === 'shift') return prefix + 'let mut ' + values + ' = ' + owner + '.borrow_mut(); if ' + values + '.is_empty() { None } else { Some(' + values + '.remove(0)) } }';
    return prefix + owner + '.borrow_mut().reverse(); ' + owner + ' }';
  }
  if (operation === 'clear') return prefix + owner + '.borrow_mut().clear(); () }';
  const key = 'ml_method_key' + suffix;
  prefix += 'let ' + key + ' = ' + (floatKey ? 'if ' + first + ' == 0.0 { 0.0 } else { ' + first + ' }' : first) + '; ';
  const pattern = kind === 'map' ? '(ml_key, _)' : 'ml_key';
  const equality = floatKey ? '(ml_key == &' + key + ' || (ml_key.is_nan() && ' + key + '.is_nan()))' : 'ml_key == &' + key;
  const position = values + '.iter().position(|' + pattern + '| ' + equality + ')';
  if (operation === 'has') return prefix + 'let ' + values + ' = ' + owner + '.borrow(); ' + position + '.is_some() }';
  if (operation === 'get') return prefix + 'let ' + values + ' = ' + owner + '.borrow(); ' + position + '.map(|index| ' + values + '[index].1.clone()) }';
  if (operation === 'delete') return prefix + 'let mut ' + values + ' = ' + owner + '.borrow_mut(); match ' + position + ' { Some(index) => { ' + values + '.remove(index); true }, None => false } }';
  const insertion = kind === 'map' ? values + '.push((' + key + ', ' + second + '));' : values + '.push(' + key + ');';
  const replacement = kind === 'map' ? values + '[index].1 = ' + second + ';' : 'let _ = index;';
  return prefix + '{ let mut ' + values + ' = ' + owner + '.borrow_mut(); match ' + position
    + ' { Some(index) => { ' + replacement + ' }, None => { ' + insertion + ' } } } ' + owner + ' }';
}
