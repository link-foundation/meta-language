/** @param {string} field @param {string} kind @returns {boolean} */
export function selectCollectionSizeField(field, kind) {
  return field === 'size' && (kind === 'map' || kind === 'set');
}

/** @param {string} target @param {string} value @returns {string} */
export function renderCollectionSize(target, value) {
  if (target === 'JavaScript') return '(' + value + ').size';
  if (target === 'Rust') return '(' + value + '.len() as f64)';
  if (target === 'Lean') return '(Float.ofNat ' + value + '.size)';
  return '(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ' + value + '))))';
}
