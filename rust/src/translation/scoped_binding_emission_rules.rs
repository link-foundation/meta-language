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
pub fn accept_java_script_scoped_binding_own_field(role: &str, kind: &str, field: &str) -> bool {
    (((((((((((((((((role == "program" && kind == "")
        && ((((((((((field == "schemaVersion" || field == "sourceLanguage")
            || field == "items")
            || field == "declarations")
            || field == "main")
            || field == "imports")
            || field == "externals")
            || field == "outputThreaded")
            || field == "abortsThreaded")
            || field == "referenceSemantics")
            || field == "utf16Strings"))
        || ((role == "main" && kind == "")
            && ((field == "effects" || field == "span")
                || field == "sequentialAsync")))
        || ((role == "declaration" && kind == "fn")
            && ((((((((((((((field == "k" || field == "name")
                || field == "fullName")
                || field == "modulePath")
                || field == "params")
                || field == "ret")
                || field == "body")
                || field == "span")
                || field == "recursive")
                || field == "decreasing")
                || field == "mutual")
                || field == "generated")
                || field == "sourceSignature")
                || field == "sourceLoop")
                || field == "sourcePrimitiveOwners")))
        || ((role == "parameter" && kind == "")
            && ((((field == "name" || field == "type") || field == "span")
                || field == "guard")
                || field == "default")))
        || ((role == "effect" && kind == "let")
            && ((((field == "k" || field == "name") || field == "value")
                || field == "constant")
                || field == "span")))
        || ((role == "effect" && kind == "print")
            && ((field == "k" || field == "expr") || field == "span")))
        || ((role == "expression" && kind == "lit")
            && (((field == "k" || field == "type") || field == "span")
                || field == "value")))
        || ((role == "expression" && kind == "unit")
            && ((field == "k" || field == "type") || field == "span")))
        || ((role == "expression" && kind == "none")
            && (((field == "k" || field == "type") || field == "span")
                || field == "absence")))
        || ((role == "expression" && kind == "var")
            && (((field == "k" || field == "type") || field == "span") || field == "name")))
        || ((role == "expression" && kind == "let")
            && (((((field == "k" || field == "type") || field == "span")
                || field == "name")
                || field == "value")
                || field == "body")))
        || ((role == "expression" && kind == "if")
            && (((((field == "k" || field == "type") || field == "span") || field == "cond")
                || field == "then")
                || field == "else")))
        || ((role == "expression" && kind == "unary")
            && (((((field == "k" || field == "type") || field == "span") || field == "op")
                || field == "arg")
                || field == "semantics")))
        || ((role == "expression" && kind == "binary")
            && (((((((((field == "k" || field == "type") || field == "span")
                || field == "op")
                || field == "left")
                || field == "right")
                || field == "domain")
                || field == "semantics")
                || field == "rounding")
                || field == "byZero")))
        || ((role == "expression" && kind == "call")
            && ((((field == "k" || field == "type") || field == "span") || field == "fn")
                || field == "args")))
        || ((role == "expression" && kind == "print")
            && ((((field == "k" || field == "type") || field == "span") || field == "text")
                || field == "body")))
        || ((role == "expression" && kind == "math")
            && ((((field == "k" || field == "type") || field == "span") || field == "op")
                || field == "args"))
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_source_facet(field: &str, record: bool) -> bool {
    ((field == "sourceSignature" || field == "sourceLoop") || field == "sourcePrimitiveOwners")
        && record
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_boolean_field(
    role: &str,
    kind: &str,
    field: &str,
) -> bool {
    (((role == "main" && kind == "") && field == "sequentialAsync")
        || ((role == "declaration" && kind == "fn")
            && ((field == "recursive" || field == "mutual") || field == "generated")))
        || ((role == "effect" && kind == "let") && field == "constant")
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_empty_sequence(
    field: &str,
    present: bool,
    sequence: bool,
    empty: bool,
) -> bool {
    (!(field == "imports" || field == "externals") || !present) || (sequence && empty)
}

/// Apply the shared owned scoped binding emission policy.
#[must_use]
pub fn accept_java_script_scoped_binding_defined_field(undefined_value: bool) -> bool {
    !undefined_value
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
