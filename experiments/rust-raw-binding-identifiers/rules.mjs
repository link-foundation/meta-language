// Generated experimental raw binding decisions. Do not edit.
/** @param {string} model @returns {boolean} */
export function acceptRustRawBindingIdentifierModel(model) { return model === "Rust raw binding identifier interpretation 1"; }
/** @param {string} name @param {boolean} raw @param {boolean} charactersAccepted @returns {boolean} */
export function acceptRustRawBindingIdentifierName(name, raw, charactersAccepted) {
  if (!charactersAccepted || name === "" || name === "_" || name === "crate" || name === "self" || name === "super" || name === "Self") return false;
  return raw || !(name === "as" || name === "async" || name === "await" || name === "break" || name === "const" || name === "continue" || name === "crate" || name === "dyn" || name === "else" || name === "enum" || name === "extern" || name === "false" || name === "fn" || name === "for" || name === "if" || name === "impl" || name === "in" || name === "let" || name === "loop" || name === "match" || name === "mod" || name === "move" || name === "mut" || name === "pub" || name === "ref" || name === "return" || name === "self" || name === "Self" || name === "static" || name === "struct" || name === "super" || name === "trait" || name === "true" || name === "type" || name === "unsafe" || name === "use" || name === "where" || name === "while" || name === "abstract" || name === "become" || name === "box" || name === "do" || name === "final" || name === "gen" || name === "macro" || name === "override" || name === "priv" || name === "try" || name === "typeof" || name === "unsized" || name === "virtual" || name === "yield");
}
/** @param {string} kind @param {boolean} signed @param {number} bits @returns {boolean} */
export function acceptRustRawBindingIdentifierType(kind, signed, bits) { return kind === "bool" || kind === "fixed" && signed && bits === 32; }
/** @param {boolean} recursive @param {boolean} mutual @returns {boolean} */
export function acceptRustRawBindingIdentifierClosure(recursive, mutual) { return !recursive && !mutual; }
