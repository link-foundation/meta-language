// Existing immutable collection behavior, shared by all target constructors.

/** @param {string} kind @param {string} operation @returns {string} */
export function readCollectionAccessResult(kind, operation) {
  if (kind !== 'map' && kind !== 'set') return '';
  if (operation === 'has') return 'boolean';
  if (kind === 'map' && operation === 'get') return 'optional value';
  if (operation === 'keys') return 'keys';
  if (operation === 'values') return 'values';
  if (operation === 'entries') return 'entries';
  return '';
}

/** @param {string} operation @returns {number} */
export function readCollectionAccessArgumentCount(operation) {
  return operation === 'get' || operation === 'has' ? 1 : 0;
}

/** @param {string} target @param {string} object @returns {string} */
export function renderCollectionSizeExpression(target, object) {
  if (target === 'JavaScript') return '(' + object + ').size';
  if (target === 'Rust') return '(' + object + '.len() as f64)';
  if (target === 'Lean') return '(Float.ofNat ' + object + '.size)';
  if (target === 'Rocq') return '(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ' + object + '))))';
  return '';
}

/** Equality and target type spellings are derived from the actual checked
 * primitive owners by the existing target type algebra.
 * @param {string} target @param {string} kind @param {string} operation
 * @param {string} object @param {string} search @param {boolean} floatKey
 * @param {string} comparison @param {string} keyType @param {string} valueType
 * @returns {string}
 */
export function renderCollectionReadExpression(target, kind, operation, object, search, floatKey, comparison, keyType, valueType) {
  if (target === 'JavaScript') return '(' + object + ').' + operation + '(' + search + ')';
  if (target === 'Rust') {
    const equal = floatKey ? '(ml_key == &ml_search || (ml_key.is_nan() && ml_search.is_nan()))' : 'ml_key == &ml_search';
    const bindings = kind === 'map' ? '(ml_key, ml_value)' : 'ml_key';
    const action = operation === 'get' ? 'find_map(|' + bindings + '| if ' + equal + ' { Some(ml_value.clone()) } else { None })'
      : 'any(|' + bindings + '| ' + equal + ')';
    return '{ let ml_collection = ' + object + '; let ml_search = ' + search + '; ml_collection.iter().' + action + ' }';
  }
  if (target === 'Lean') {
    const key = kind === 'map' ? 'ml_value.1' : 'ml_value';
    const equal = floatKey ? '(' + key + ' == ml_search || (' + key + '.isNaN && ml_search.isNaN))' : '(' + key + ' == ml_search)';
    const result = operation === 'get' ? '((ml_values.toList.find? (fun ml_value => ' + equal + ')).map (fun ml_pair => ml_pair.2))'
      : '(ml_values.any (fun ml_value => ' + equal + '))';
    return '(let ml_values := ' + object + '; let ml_search := ' + search + '; ' + result + ')';
  }
  if (target === 'Rocq') {
    const key = kind === 'map' ? '(fst ml_value)' : 'ml_value';
    const equal = floatKey ? '(orb ' + comparison + ' (andb (PrimFloat.is_nan ' + key + ') (PrimFloat.is_nan ml_search)))' : comparison;
    const result = operation === 'get' ? '(option_map (@snd ' + keyType + ' ' + valueType + ') (List.find (fun ml_value => ' + equal + ') ml_values))'
      : '(List.existsb (fun ml_value => ' + equal + ') ml_values)';
    return '(let ml_values := ' + object + ' in let ml_search := ' + search + ' in ' + result + ')';
  }
  return '';
}
