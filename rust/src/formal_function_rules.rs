// Generated automatically from js/src/formal-function-rules.js.
// Emitter styles are recorded in parity/self-translation/formal-function-rules-decorators.lino.
// Do not edit by hand.
/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_formal_function_declaration(kind: &str) -> bool {
    kind == "fn"
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_formal_function_dependency(kind: &str) -> bool {
    kind == "fn" || kind == "data"
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn include_formal_function_relation(label: &str) -> bool {
    label != "source span" && label != "provenance"
}
