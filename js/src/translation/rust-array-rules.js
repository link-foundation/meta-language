// Pure source array admission decisions, translated through the checked model.

/** @param {string} owner @param {boolean} shadowed @returns {boolean} */
export function acceptRustSourceArrayOwner(owner, shadowed) {
  return !shadowed && (owner === 'Vec' || owner === 'std::vec::Vec');
}

/** @param {string} kind @param {boolean} indirected @returns {boolean} */
export function acceptRustSourceArrayElement(kind, indirected) {
  return !indirected && (kind === 'bool' || kind === 'float' || kind === 'fixed' || kind === 'unit' || kind === 'string');
}

/** @param {string} actual @param {string} expected @param {boolean} borrowed @returns {boolean} */
export function acceptRustSourceArrayCoercion(actual, expected, borrowed) {
  return actual === expected || (borrowed && actual === 'owned vector' && expected === 'immutable slice');
}
