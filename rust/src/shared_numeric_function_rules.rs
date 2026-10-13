// Generated automatically from js/src/shared-numeric-function-rules.js.
// Emitter styles are recorded in parity/self-translation/shared-numeric-function-rules-decorators.lino.
// Do not edit by hand.
/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_type(kind: &str) -> bool {
    kind == "float" || kind == "bool"
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_expression(kind: &str) -> bool {
    kind == "lit"
        || kind == "var"
        || kind == "let"
        || kind == "if"
        || kind == "call"
        || kind == "unary"
        || kind == "binary"
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_unary(operation: &str, kind: &str, semantics: &str) -> bool {
    operation == "neg" && kind == "float" && semantics == "ieee"
        || operation == "not" && kind == "bool" && semantics.is_empty()
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_binary(operation: &str, domain: &str, semantics: &str) -> bool {
    if operation == "add"
        || operation == "sub"
        || operation == "mul"
        || operation == "div"
        || operation == "rem"
    {
        domain == "float" && semantics == "ieee"
    } else if operation == "eq" || operation == "ne" {
        (domain == "float" || domain == "bool") && semantics.is_empty()
    } else if operation == "lt" || operation == "le" || operation == "gt" || operation == "ge" {
        domain == "float" && semantics.is_empty()
    } else {
        false
    }
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_type_shape(kind: &str, single_field: bool) -> bool {
    accept_shared_numeric_type(kind) && single_field
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_context(language: &str, unsupported_context: bool) -> bool {
    (language == "JavaScript" || language == "Rust") && !unsupported_context
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_declaration(kind: &str, qualified: bool) -> bool {
    kind == "fn" && !qualified
}

/// Read a shared formal function graph decision.
#[must_use]
pub const fn accept_shared_numeric_parameter(guarded: bool, defaulted: bool) -> bool {
    !guarded && !defaulted
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_entry(generated: bool, kind: &str, owned: bool) -> bool {
    generated && kind == "call" && owned
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_expression_shape(kind: &str, qualified: bool) -> bool {
    accept_shared_numeric_expression(kind) && !qualified
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_literal(
    kind: &str,
    literal_flags: (bool, bool, bool, bool, bool),
) -> bool {
    let (boolean_value, decimal_value, finite_value, canonical_value, require_canonical) =
        literal_flags;
    if kind == "bool" {
        boolean_value
    } else {
        kind == "float" && decimal_value && finite_value && (!require_canonical || canonical_value)
    }
}

/// Read a shared formal function graph decision.
#[must_use]
pub const fn accept_shared_numeric_context_flag(
    present: bool,
    boolean_scalar: bool,
    false_value: bool,
) -> bool {
    !present || boolean_scalar && false_value
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_source_signature(
    model: &str,
    language: &str,
    matching_layout: bool,
    qualified: bool,
) -> bool {
    model == "Rust source function signature 1"
        && language == "Rust"
        && matching_layout
        && !qualified
}

/// Read a shared formal function graph decision.
#[must_use]
pub fn accept_shared_numeric_source_type(
    kind: &str,
    owner_kind: &str,
    owner_path: &str,
    qualified: bool,
) -> bool {
    owner_kind == "primitive"
        && !qualified
        && (kind == "float" && owner_path == "f64" || kind == "bool" && owner_path == "bool")
}
