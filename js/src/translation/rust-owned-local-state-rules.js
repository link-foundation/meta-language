// Generated from linked owned local state policy. Do not edit.
/** @param {string} model @returns {boolean} */
export function acceptRustOwnedLocalStateModel(model){return model === "" || model === "closed owned text local state 1" || model === "canonical owned text local copies 1";}
/** @param {string} kind @param {string} bits @param {boolean} signed @returns {boolean} */
export function acceptRustOwnedLocalValueType(kind,bits,signed){return kind === "string" || kind === "bool" || (kind === "fixed" && bits === "32" && signed);}
/** @param {string} kind @returns {boolean} */
export function acceptRustOwnedLocalExpression(kind){return kind === "lit" || kind === "var" || kind === "tuple" || kind === "tupleIndex" || kind === "let" || kind === "if" || kind === "binary" || kind === "toString";}
/** @param {string} operation @returns {boolean} */
export function acceptRustOwnedLocalBinaryOperation(operation){return operation === "eq";}
