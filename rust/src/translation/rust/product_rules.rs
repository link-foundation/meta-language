// Generated from rust-product-rules.js by the checked translator.
// Do not edit by hand.
/// Admit the corresponding immutable Rust source product.
#[must_use]
pub fn accept_rust_immutable_product_type(
    kind: &str,
    children_accepted: bool,
    indirected: bool,
) -> bool {
    !indirected
        && (kind == "bool"
            || kind == "float"
            || kind == "fixed"
            || kind == "unit"
            || (kind == "tuple" && children_accepted))
}

/// Admit the corresponding immutable Rust source product.
#[must_use]
pub fn accept_rust_tuple_index_spelling(text: &str) -> bool {
    if text.is_empty() || (text.len() > 1 && text.starts_with("0")) {
        false
    } else {
        text.replace(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], "")
            .is_empty()
    }
}
