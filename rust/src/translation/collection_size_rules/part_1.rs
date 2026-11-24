// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_collection_size_field(field: &str, kind: &str) -> bool {
    (field == "size") && ((kind == "map") || (kind == "set"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_size(target: &str, value: &str) -> String {
    if target == "JavaScript" {
        format!("({}).size", value.to_owned())
    } else if target == "Rust" {
        format!("({}.len() as f64)", value.to_owned())
    } else if target == "Lean" {
        format!("(Float.ofNat {}.size)", value.to_owned())
    } else {
        format!(
            "(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length {}))))",
            value.to_owned()
        )
    }
}
