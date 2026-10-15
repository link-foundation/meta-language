// Generated from the additive linked owned Rust literal-edit policy. Do not edit.
/** @param {string} model @returns {boolean} */
export function acceptGeneratedOwnedRustValueEditModel(model){return model === "closed generated owned Rust literal value edits 1";}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptGeneratedOwnedRustLiteralEditOwner(kind,operation){return kind === "string" && operation === "owned standard String literal construction" || kind === "fixed" && operation === "owned signed i32 literal";}
/** @param {boolean} negative @param {boolean} owned @returns {boolean} */
export function acceptGeneratedOwnedRustSignOwner(negative,owned){return negative === owned;}
