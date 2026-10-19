// Generated from the linked closed primitive borrowed Rust call interface policy.
/** @param {string} model @returns {boolean} */
export function acceptRustBorrowedCallInterfaceModel(model) { return model === "closed primitive borrowed Rust call interface 1"; }
/** @param {string} model @param {string} kind @param {string} targetKind @param {string} argumentStatus @returns {string} */
export function readRustBorrowedCallTargetDiagnostic(model, kind, targetKind, argumentStatus) { if (model !== "closed primitive borrowed Rust call interface 1" || kind !== "call") return ""; if (argumentStatus === "mismatch") return "the linked program has inconsistent call argument type"; if (argumentStatus === "matched" && targetKind === "call") return "Rust borrowed call interface: every caller must target an owned pure leaf without a static borrowed input"; return ""; }
/** @param {boolean} owned @param {boolean} leaf @param {boolean} staticLifetime @returns {boolean} */
export function acceptRustBorrowedCallTarget(owned, leaf, staticLifetime) { return owned && leaf && !staticLifetime; }
/** @param {string} kind @param {boolean} unrepresented @param {boolean} correctArity @returns {boolean} */
export function acceptRustBorrowedCallShape(kind, unrepresented, correctArity) { return kind === "call" && !unrepresented && correctArity; }
/** @param {boolean} hasCaller @returns {boolean} */
export function acceptRustBorrowedCallInventory(hasCaller) { return hasCaller; }
/** @param {boolean} argumentTypes @param {boolean} resultTypes @returns {boolean} */
export function acceptRustBorrowedCallType(argumentTypes, resultTypes) { return argumentTypes && resultTypes; }
/** @param {string} lifetime @param {string} inner @param {boolean} innerReference @returns {string} */
export function renderRustBorrowedCallHeaderReference(lifetime, inner, innerReference) { return lifetime === "" ? "&" + (innerReference ? " " : "") + inner : "&'" + lifetime + " " + inner; }
/** @param {string} value @returns {string} */
export function renderRustBorrowedCallReference(value) { return "&(" + value + ")"; }
/** @param {string} head @param {string} argumentsText @returns {string} */
export function renderRustBorrowedCallInvocation(head, argumentsText) { return head + "(" + argumentsText + ")"; }
/** @param {string} body @returns {string} */
export function renderRustBorrowedCallBlock(body) { return "{\n" + body + "\n}"; }
