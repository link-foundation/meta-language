// Generated from linked Rust completion policy. Do not edit by hand.
/** @param {string} model @returns {boolean} */
export function acceptRustCompletionModel(model) {
  return model === "" || model === "closed guarded function returns 1";
}

/** @param {boolean} proven @param {boolean} indirected @returns {boolean} */
export function acceptRustReturnOwnership(proven, indirected) {
  return proven && !indirected;
}

/** @param {string} thenKind @param {string} otherwiseKind @param {boolean} hasElse @param {boolean} continuation @returns {string} */
export function readRustConditionalCompletion(thenKind, otherwiseKind, hasElse, continuation) {
  if (thenKind === "returned" && otherwiseKind === "" && hasElse === false && continuation === true) return "guarded continuation";
  if (thenKind === "returned" && otherwiseKind === "returned" && hasElse === true && continuation === false) return "returned conditional";
  if (thenKind === "value" && otherwiseKind === "value" && hasElse === true && continuation === false) return "conditional value";
  if (thenKind === "returned" && otherwiseKind === "returned" && hasElse === true && continuation === true) return "unreachable continuation";
  if (thenKind === "value" && otherwiseKind === "value" && hasElse === true && continuation === true) return "discarded conditional value";
  return "unsupported completion";
}

/** @param {string} model @param {string} macroName @param {string} firstKind @param {string} firstValue @param {string} secondKind @param {string} count @returns {boolean} */
export function acceptRustCompletionPanicArguments(model, macroName, firstKind, firstValue, secondKind, count) {
  return model === "closed guarded function returns 1" && macroName === "panic" && firstKind === "str" && firstValue === "{}" && secondKind === "str" && count === "2";
}
