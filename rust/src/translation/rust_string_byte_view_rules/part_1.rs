// Generated from complete js/src/translation/rust-string-byte-view-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_byte_view_candidate(method: &str, ordinary: bool) -> bool {
    ordinary && (((method == "as_bytes") || (method == "into_bytes")) || (method == "bytes"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_string_byte_view_error(
    method: &str,
    count: f64,
    kind: &str,
    storage: &str,
    layers: f64,
    known: bool,
    references: bool,
) -> String {
    if count == 0f64 {
        if ((!known || (kind != "string")) || !references)
            || !(((storage == "owned string") && (layers >= 0f64))
                || ((storage == "string slice") && (layers >= 1f64)))
        {
            String::from("a byte view needs the actual standard String or str source owner")
        } else if (method == "into_bytes") && !((storage == "owned string") && (layers == 0f64)) {
            String::from("consuming bytes need an actual owned standard String")
        } else {
            String::new()
        }
    } else {
        String::from("a standard string byte view needs no arguments")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_string_byte_view_storage(method: &str) -> String {
    if method == "into_bytes" {
        String::from("owned vector")
    } else {
        String::from("immutable slice")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_string_byte_view_layers(method: &str) -> f64 {
    if method == "into_bytes" { 0f64 } else { 1f64 }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_string_byte_iterator_error(method: &str, proven: bool) -> String {
    if (method == "bytes") && !proven {
        String::from("byte iteration needs its actual source string view")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_byte_kernel_unit_requirement(
    language: &str,
    kind: &str,
    operation: &str,
    receiver: &str,
) -> bool {
    (((language == "Rust") && (kind == "method")) && (operation == "codePointAt"))
        && (receiver == "string")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_string_byte_scalar_bridge(target: &str, value: &str, units: bool) -> String {
    if units && (target == "Rust") {
        format!("format!(\"{{}}\", {})", value.to_owned())
    } else {
        value.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_byte_scalar_bridge(target: &str, units: bool) -> bool {
    (((!units || (target == "JavaScript")) || (target == "Rust")) || (target == "Lean"))
        || (target == "Rocq")
}
