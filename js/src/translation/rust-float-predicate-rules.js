// Generated from linked Rust float predicate policy. Do not edit by hand.
/** @param {string} method @returns {boolean} */
export function acceptRustFloatPredicateMethod(method) {
  return method === "is_nan" || method === "is_finite" || method === "is_infinite";
}

/** @param {string} method @returns {boolean} */
export function acceptRustFloatOperationMethod(method) {
  return method === "abs" || method === "floor" || method === "ceil" || method === "trunc";
}

/** @param {string} kind @param {string} model @param {boolean} proven @param {boolean} indirected @param {boolean} shadowed @param {string} argumentsCount @returns {boolean} */
export function acceptRustFloatPredicateReceiver(kind, model, proven, indirected, shadowed, argumentsCount) {
  return proven && !indirected && !shadowed && kind === "float" && model === "binary64 scalar" && argumentsCount === "0";
}
