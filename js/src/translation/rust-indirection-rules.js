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

/** @param {string} layer @param {boolean} initial @returns {boolean} */
export function acceptRustSourceBoxCoercionLayer(layer, initial) {
  return initial && layer === 'box';
}

/** @param {string} kind @param {string} operation @returns {string} */
export function readRustSourceExpressionOrigin(kind, operation) {
  if (kind === 'abort') return 'diverging';
  if (kind === 'toString' && operation === 'from') return 'unboxed-conversion';
  if (kind === 'cast') return 'numeric-cast';
  if (kind === 'unary') return 'primitive-operation';
  if (kind === 'binary') {
    if (operation === 'eq' || operation === 'ne' || operation === 'lt' || operation === 'le' || operation === 'gt' || operation === 'ge') return 'comparison';
    if (operation === 'and' || operation === 'or') return 'boolean-operation';
    return 'primitive-operation';
  }
  if (kind === 'num' || kind === 'bool' || kind === 'unit' || kind === 'str') return 'value';
  if (kind === 'toString' || kind === 'show' || kind === 'stringMap' || kind === 'stringTest') return 'derived-value';
  return 'unknown';
}
