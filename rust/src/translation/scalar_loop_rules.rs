// Generated through the checked JavaScript translator. Do not edit.
/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_rust_scalar_loop_model(model: &str) -> bool {
    model.is_empty()
        || model == "closed primitive scalar loop 1"
        || model == "closed declaration scalar loop 1"
}

/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_scalar_loop_declaration_model(model: &str) -> bool {
    model == "closed declaration scalar loop 1"
}

/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_scalar_loop_language(language: &str) -> bool {
    language == "Rust" || language == "JavaScript"
}

/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_scalar_loop_value_kind(kind: &str) -> bool {
    kind == "float" || kind == "bool"
}

/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_scalar_loop_expression_type_annotation(
    present: bool,
    null_value: bool,
    kind: &str,
) -> bool {
    !present || null_value || kind == "float" || kind == "bool"
}

/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_scalar_loop_binary_operation(operation: &str) -> bool {
    operation == "add"
        || operation == "sub"
        || operation == "mul"
        || operation == "div"
        || operation == "rem"
        || operation == "eq"
        || operation == "ne"
        || operation == "lt"
        || operation == "le"
        || operation == "gt"
        || operation == "ge"
        || operation == "and"
        || operation == "or"
        || operation == "plus"
}

/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_scalar_loop_unary_operation(operation: &str) -> bool {
    operation == "neg" || operation == "not"
}

/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_scalar_loop_math_operation(operation: &str) -> bool {
    operation == "abs"
        || operation == "floor"
        || operation == "ceil"
        || operation == "trunc"
        || operation == "isNaN"
        || operation == "isFinite"
}

/// Read the shared closed scalar loop admission.
#[must_use]
pub fn accept_scalar_loop_primitive_owner(kind: &str, owner: &str) -> bool {
    kind == "float" && owner == "f64" || kind == "bool" && owner == "bool"
}
