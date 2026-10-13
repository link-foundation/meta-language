// Generated from rust-scalar-parameter-rules.js by the checked translator.
// Do not edit by hand.
/// Evaluate the selected unwritten scalar parameter premise.
#[must_use]
pub fn accept_rust_unwritten_scalar_parameter(
    kind: &str,
    direct_copy: bool,
    shadowed: bool,
    closed: bool,
    possible_write: bool,
) -> bool {
    direct_copy
        && !shadowed
        && closed
        && !possible_write
        && (kind == "bool" || kind == "float" || kind == "fixed")
}

/// Evaluate the selected unwritten scalar parameter premise.
#[must_use]
pub fn detect_rust_source_assignment_token(kind: &str, text: &str) -> bool {
    kind == "punct"
        && (text == "="
            || text == "+="
            || text == "-="
            || text == "*="
            || text == "/="
            || text == "%="
            || text == "&="
            || text == "|="
            || text == "^="
            || text == "<<="
            || text == ">>=")
}
