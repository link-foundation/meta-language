// Generated through the checked JavaScript translator. Do not edit.
/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_model(model: &str) -> bool {
    model == "owned JavaScript scoped bindings 1"
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_context(language: &str, forbidden: bool) -> bool {
    (language == "Rust" || language == "JavaScript") && !forbidden
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_reference_context(
    present: bool,
    form: &str,
    owned_snapshot: bool,
) -> bool {
    (!present || form == "false") || (form == "true" && owned_snapshot)
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_declaration(
    kind: &str,
    non_flat: bool,
    generated: bool,
) -> bool {
    (kind == "fn" && !non_flat) && !generated
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_value_type(
    kind: &str,
    single_field: bool,
) -> bool {
    (((kind == "float" || kind == "bool") || kind == "string") || kind == "unit") && single_field
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_absent_value_type(
    kind: &str,
    element: &str,
    absence: &str,
    exact: bool,
) -> bool {
    ((kind == "option" && element == "float") && absence == "undefined") && exact
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_absent_expression(
    kind: &str,
    absence: &str,
) -> bool {
    kind == "option" && absence == "undefined"
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_expression(kind: &str) -> bool {
    (((((((((kind == "lit" || kind == "unit") || kind == "none") || kind == "var")
        || kind == "let")
        || kind == "if")
        || kind == "unary")
        || kind == "binary")
        || kind == "call")
        || kind == "print")
        || kind == "math"
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_effect(kind: &str) -> bool {
    kind == "let" || kind == "print"
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_emission_role(role: &str, owner: &str) -> bool {
    ((role == "function binding" && owner == "function declaration")
        || (role == "main binding" && owner == "main effect container"))
        || (role == "scalar loop binding" && owner == "scalar loop declaration")
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn reserve_java_script_scoped_binding_declaration_names(preserve: bool) -> bool {
    !preserve
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn render_java_script_scoped_binding_identifier(name: &str, preserve: bool) -> String {
    if preserve {
        name.to_owned()
    } else {
        format!("binding_{name}")
    }
}
