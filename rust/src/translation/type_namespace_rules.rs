// Generated through the checked JavaScript translator. Do not edit.
/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_model(model: &str) -> bool {
    model.is_empty() || model == "resolved Rust type namespace 1"
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_declaration(kind: &str) -> bool {
    kind == "data"
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub const fn accept_rust_type_namespace_fields(invalid_fields: bool) -> bool {
    !invalid_fields
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub const fn accept_rust_type_namespace_anchor(above_crate: bool) -> bool {
    !above_crate
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_body_constructor(selected: bool, constructor_name: &str) -> bool {
    !selected
        || constructor_name != "String::from"
            && constructor_name != "i8::from"
            && constructor_name != "u8::from"
            && constructor_name != "i16::from"
            && constructor_name != "u16::from"
            && constructor_name != "i32::from"
            && constructor_name != "u32::from"
            && constructor_name != "i64::from"
            && constructor_name != "u64::from"
            && constructor_name != "i128::from"
            && constructor_name != "u128::from"
            && constructor_name != "usize::from"
            && constructor_name != "isize::from"
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_body_method(
    selected: bool,
    method_name: &str,
    owned_unit_clone: bool,
) -> bool {
    !selected
        || method_name == "clone" && owned_unit_clone
        || method_name != "clone"
            && method_name != "to_owned"
            && method_name != "to_string"
            && method_name != "as_str"
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_method_discovery(discovery: bool, method_name: &str) -> bool {
    discovery && method_name == "clone"
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub const fn accept_rust_type_namespace_unit_clone(
    owned_derivation: bool,
    field_free: bool,
    direct_receiver: bool,
) -> bool {
    owned_derivation && field_free && direct_receiver
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_container(kind: &str, dense: bool, unrepresented: bool) -> bool {
    (kind == "array" && dense && !unrepresented)
        || ((kind == "map" || kind == "record") && !unrepresented)
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_discovery_type(discovery: bool, type_name: &str) -> bool {
    discovery && type_name == "str"
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_position(
    kind: &str,
    invalid_wrappers: bool,
    borrowed_result: bool,
) -> bool {
    (kind == "primitive" || kind == "declaration") && !invalid_wrappers && !borrowed_result
}
