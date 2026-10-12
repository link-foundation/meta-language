// Generated from rust-indirection-rules.js by the checked translator.
// Do not edit by hand.
/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_immutable_indirection_layer(layer: &str) -> bool {
    layer == "reference" || layer == "box"
}

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_source_box_owner(owner: &str, shadowed: bool) -> bool {
    !shadowed && (owner == "Box" || owner == "std::boxed::Box" || owner == "alloc::boxed::Box")
}

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_source_indirection_argument(
    indirected: bool,
    exact: bool,
    borrowed: bool,
    coerced: bool,
) -> bool {
    !indirected || exact || (borrowed && coerced)
}
