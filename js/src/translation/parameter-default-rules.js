// Shared decisions for delaying parameter initializers until actual arguments
// have evaluated. Hosts traverse the original checked child selected here.

/** @param {string} kind @param {string} flavor @returns {string} */
export function readParameterDefaultValueStep(kind, flavor) {
  if (kind === 'lit' || kind === 'none' || kind === 'unit' || kind === 'var') return 'pure';
  if (kind === 'cast' && flavor === 'exact') return 'arg';
  return kind === 'some' ? 'value' : 'effect';
}

/** @param {number} index @param {number} provided @param {boolean} pure @returns {boolean} */
export function requireParameterDefaultArgumentBindings(index, provided, pure) {
  return index >= 0 && index + 1 < provided && !pure;
}

/** @param {boolean} defaulted @param {boolean} optional @param {string} kind @param {string} absence @returns {string} */
export function readParameterDefaultArgumentInference(defaulted, optional, kind, absence) {
  if (!defaulted || optional) return 'identity';
  if (kind === 'var') return 'defer';
  return kind === 'option' && absence === 'undefined' ? 'element' : 'value';
}
