// Generated through the checked JavaScript translator. Do not edit.
/// Apply a shared closed const predicate source decision.
#[must_use]
pub fn accept_rust_const_function_model(model: &str) -> bool {
    model.is_empty() || model == "closed pure Rust const predicates 1"
}

/// Apply a shared closed const predicate source decision.
#[must_use]
pub fn accept_rust_const_function_type(kind: &str) -> bool {
    kind == "bool" || kind == "float"
}

/// Apply a shared closed const predicate source decision.
#[must_use]
pub fn accept_rust_const_function_binary(kind: &str, operation: &str) -> bool {
    (kind == "bool"
        && (((operation == "and" || operation == "or") || operation == "eq") || operation == "ne"))
        || (kind == "float"
            && (((((operation == "eq" || operation == "ne") || operation == "lt")
                || operation == "le")
                || operation == "gt")
                || operation == "ge"))
}

/// Apply a shared closed const predicate source decision.
#[must_use]
pub fn accept_rust_const_function_container(kind: &str, dense: bool, unrepresented: bool) -> bool {
    !unrepresented && ((kind == "record" || kind == "map") || (kind == "array" && dense))
}

/// Apply a shared closed const predicate source decision.
#[must_use]
pub fn accept_rust_const_function_offset(value: f64, negative_zero: bool) -> bool {
    (((0f64..=9_007_199_254_740_991f64).contains(&value)) && ((value % 1f64) == 0f64))
        && !negative_zero
}

/// Apply a shared closed const predicate source decision.
#[must_use]
pub const fn accept_rust_const_function_interval(start: f64, end: f64, nonempty: bool) -> bool {
    start <= end && (!nonempty || start < end)
}

/// Apply a shared closed const predicate source decision.
#[must_use]
pub fn accept_rust_const_function_name_character(code: f64, first: bool) -> bool {
    ((((95f64..=95f64).contains(&code)) || ((65f64..=90f64).contains(&code)))
        || ((97f64..=122f64).contains(&code)))
        || (!first && (48f64..=57f64).contains(&code))
}

/// Apply a shared closed const predicate source decision.
#[must_use]
pub fn accept_rust_const_function_name(name: &str) -> bool {
    !name.is_empty()
        && name != "_"
        && !matches!(
            name,
            "as" | "async"
                | "await"
                | "break"
                | "const"
                | "continue"
                | "crate"
                | "dyn"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
                | "abstract"
                | "become"
                | "box"
                | "do"
                | "final"
                | "gen"
                | "macro"
                | "override"
                | "priv"
                | "try"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
        )
}

/// Apply a shared closed const predicate source decision.
#[must_use]
pub const fn accept_rust_const_function_finite_literal(value: f64) -> bool {
    value.is_finite()
}
