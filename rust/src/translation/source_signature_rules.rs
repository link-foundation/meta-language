// Generated through the checked JavaScript translator. Do not edit.
/// Read the shared owned source signature admission.
#[must_use]
pub fn accept_rust_source_signature_model(model: &str) -> bool {
    model.is_empty() || model == "Rust source function signature 1"
}

/// Read the shared owned source signature admission.
#[must_use]
pub fn accept_rust_source_signature_wrapper(wrapper: &str) -> bool {
    wrapper == "reference" || wrapper == "box"
}

/// Read the shared owned source signature admission.
#[must_use]
pub fn accept_rust_source_signature_owner_kind(kind: &str) -> bool {
    kind == "primitive"
        || kind == "declaration"
        || kind == "library"
        || kind == "tuple"
        || kind == "unit"
        || kind == "slice"
}

/// Read the shared owned source signature admission.
#[must_use]
pub fn accept_rust_source_signature_catalog_owner(kind: &str) -> bool {
    kind == "data" || kind == "module" || kind == "externalCrate" || kind == "ctor"
}

/// Read the shared owned source signature admission.
#[must_use]
pub fn accept_rust_source_signature_primitive(
    kind: &str,
    path: &str,
    bits: &str,
    signed: bool,
) -> bool {
    if path == "f64" {
        kind == "float"
    } else if path == "bool" {
        kind == "bool"
    } else if path == "str" {
        kind == "string"
    } else if path == "i8" {
        kind == "fixed" && bits == "8" && signed
    } else if path == "u8" {
        kind == "fixed" && bits == "8" && !signed
    } else if path == "i16" {
        kind == "fixed" && bits == "16" && signed
    } else if path == "u16" {
        kind == "fixed" && bits == "16" && !signed
    } else if path == "i32" {
        kind == "fixed" && bits == "32" && signed
    } else if path == "u32" {
        kind == "fixed" && bits == "32" && !signed
    } else if path == "i64" {
        kind == "fixed" && bits == "64" && signed
    } else if path == "u64" {
        kind == "fixed" && bits == "64" && !signed
    } else if path == "i128" {
        kind == "fixed" && bits == "128" && signed
    } else if path == "u128" {
        kind == "fixed" && bits == "128" && !signed
    } else if path == "usize" {
        kind == "fixed" && bits == "64" && !signed
    } else if path == "isize" {
        kind == "fixed" && bits == "64" && signed
    } else {
        false
    }
}
