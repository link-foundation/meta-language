// Pure source-layer admissions translated into Rust by the checked translator.

/** @param {string} layer @returns {boolean} */
export function acceptRustImmutableIndirectionLayer(layer) {
  return layer === 'reference' || layer === 'box';
}

/** @param {string} owner @param {boolean} shadowed @returns {boolean} */
export function acceptRustSourceBoxOwner(owner, shadowed) {
  return !shadowed && (owner === 'Box' || owner === 'std::boxed::Box' || owner === 'alloc::boxed::Box');
}

/** @param {boolean} indirected @param {boolean} exact @param {boolean} borrowed @param {boolean} coerced @returns {boolean} */
export function acceptRustSourceIndirectionArgument(indirected, exact, borrowed, coerced) {
  return !indirected || exact || (borrowed && coerced);
}
