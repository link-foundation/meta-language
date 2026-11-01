// Ownership-specific admission only; Rust method/branch semantics remain in
// the existing shared predicate policy consumed by both source adapters.
/** @param {string} role @param {boolean} identity @returns {string} */
export function readRustOwnedStandardSumPredicateResultError(role,identity){return (role==='copy'||role==='owned')&&identity?'':'an eager default or first callback must establish its actual complete Copy or supported owned result type';}
