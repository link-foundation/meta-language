// Array absence decisions and syntax are shared by both runtime adapters.
// The host supplies evaluated receivers and their checked scalar kinds.

/** @param {string} language @param {boolean} optional @param {boolean} proven @param {boolean} expectedOptional @returns {boolean} */
export function selectOptionalArrayRead(language, optional, proven, expectedOptional) {
  return optional || language === 'JavaScript' && !proven || expectedOptional;
}

/** @param {string} language @param {string} kind @param {string} element @returns {boolean} */
export function selectOptionalArrayIndexNumber(language, kind, element) {
  return language === 'JavaScript' && kind === 'option' && (element === 'float' || element === 'unit');
}

/** @param {string} kind @param {boolean} reference @returns {string} */
export function readArrayIterableForm(kind, reference) {
  if (kind === 'array') return 'array';
  if (kind === 'string') return 'string';
  if (reference && (kind === 'set' || kind === 'map')) return 'mutable';
  if (kind === 'set') return 'set';
  if (kind === 'map') return 'map';
  return '';
}

/** @param {string} field @returns {string} */
export function readReferenceMutationDiagnostic(field) {
  if (field === 'feature') return 'reference mutation';
  if (field === 'reason') return 'this mutation requires its shared reference and slot operations to be represented';
  return '';
}

/** @param {string} target @param {string} kind @param {string} values @param {string} position @returns {string} */
export function renderOptionalArrayRead(target, kind, values, position) {
  if (target === 'JavaScript') return '(' + values + ')[' + position + ']';
  if (target === 'Rust') {
    const index = kind === 'float' ? 'crate::ml_array::number_index(' + position + ')' : position + '.to_index()';
    return 'crate::ml_array::optional_at(' + values + ', ' + index + ')';
  }
  if (target === 'Lean') {
    if (kind === 'float') return '(let ml_index := ' + position + '; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then ' + values + '[ml_index.toUInt64.toNat]? else none)';
    if (kind === 'nat') return '(' + values + '[' + position + ']?)';
    return '(let ml_index := ' + position + '; if ml_index < 0 then none else ' + values + '[ml_index.toNat]?)';
  }
  const index = kind === 'float' ? '(ml_float_index ' + position + ')' : kind === 'nat' ? '(Z.of_N ' + position + ')' : position;
  return '(let ml_index := ' + index + ' in if Z.ltb ml_index 0 then None else List.nth_error ' + values + ' (Z.to_nat ml_index))';
}

/** @param {string} target @param {boolean} utf16 @param {string} value @returns {string} */
export function renderStringPointIteration(target, utf16, value) {
  if (target === 'JavaScript') return '[...' + value + ']';
  if (target === 'Rust') return utf16 ? value + '.code_points()' : value + '.chars().map(|ml_character| ml_character.to_string()).collect::<Vec<String>>()';
  if (target === 'Lean') return '((' + value + '.toList.map (fun ml_character => String.singleton ml_character)).toArray)';
  return '';
}

/** @param {string} target @param {boolean} map @param {string} operation @param {string} value @param {string} keyType @param {string} valueType @returns {string} */
export function renderCollectionIteration(target, map, operation, value, keyType, valueType) {
  if (target === 'JavaScript') return '[...(' + value + ').' + operation + '()]';
  if (target === 'Rust') {
    if (operation === 'entries') return map ? value : value + '.iter().map(|ml_value| (ml_value.clone(), ml_value.clone())).collect::<Vec<_>>()';
    if (!map) return value;
    return value + '.iter().map(|ml_pair| ml_pair.' + (operation === 'keys' ? '0' : '1') + '.clone()).collect::<Vec<_>>()';
  }
  if (target === 'Lean') {
    if (operation === 'entries') return map ? value : '(' + value + '.map (fun ml_value => (ml_value, ml_value)))';
    return map ? '(' + value + '.map (fun ml_pair => ml_pair.' + (operation === 'keys' ? '1' : '2') + '))' : value;
  }
  if (operation === 'entries') return map ? value : '(List.map (fun ml_value => (ml_value, ml_value)) ' + value + ')';
  return map ? '(List.map (@' + (operation === 'keys' ? 'fst' : 'snd') + ' ' + keyType + ' ' + valueType + ') ' + value + ')' : value;
}
