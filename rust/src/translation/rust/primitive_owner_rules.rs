// Generated through the checked JavaScript translator. Do not edit.
/// Read a shared standard primitive source owner decision.
#[must_use]
pub fn accept_rust_primitive_owner_model(model: &str) -> bool {
    model.is_empty() || model == "standard primitive Rust binary64 owner 1"
}

/// Read a shared standard primitive source owner decision.
#[must_use]
pub fn accept_rust_standard_primitive_owner(path: &str, declared: bool, shadowed: bool) -> bool {
    (!declared && !shadowed) && (path == "std::primitive::f64" || path == "core::primitive::f64")
}

/// Read a shared standard primitive source owner decision.
#[must_use]
pub fn accept_rust_primitive_owner_value_type(kind: &str) -> bool {
    kind == "float" || kind == "bool"
}

/// Read a shared standard primitive source owner decision.
#[must_use]
pub fn accept_rust_primitive_owner_expression(kind: &str) -> bool {
    (((((kind == "lit" || kind == "var") || kind == "let") || kind == "if") || kind == "unary")
        || kind == "binary")
        || kind == "math"
}

/// Read a shared standard primitive source owner decision.
#[must_use]
pub fn read_rust_primitive_owner_constant_value(name: &str) -> String {
    if name == "NAN" {
        String::from("NaN")
    } else if name == "INFINITY" {
        String::from("Infinity")
    } else if name == "NEG_INFINITY" {
        String::from("-Infinity")
    } else {
        String::new()
    }
}
