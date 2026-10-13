// Generated from the linked resolved Rust type namespace policy.
/** @param {string} model @returns {boolean} */
export function acceptRustTypeNamespaceModel(model) { return model === "" || model === "resolved Rust type namespace 1"; }
/** @param {string} kind @returns {boolean} */
export function acceptRustTypeNamespaceDeclaration(kind) { return kind === "data"; }
/** @param {boolean} invalidFields @returns {boolean} */
export function acceptRustTypeNamespaceFields(invalidFields) { return !invalidFields; }
/** @param {boolean} aboveCrate @returns {boolean} */
export function acceptRustTypeNamespaceAnchor(aboveCrate) { return !aboveCrate; }
/** @param {string} kind @param {boolean} invalidWrappers @param {boolean} borrowedResult @returns {boolean} */
export function acceptRustTypeNamespacePosition(kind, invalidWrappers, borrowedResult) { return (kind === "primitive" || kind === "declaration") && !invalidWrappers && !borrowedResult; }
