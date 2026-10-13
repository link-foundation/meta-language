// Generated through the checked JavaScript translator. Do not edit.
/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_call_model(model: &str) -> bool {
    (model.is_empty()) || model == "closed pure Rust unit calls 1"
}

/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_name_character(code: f64, first: bool) -> bool {
    ((((95f64..=95f64).contains(&code)) || ((65f64..=90f64).contains(&code)))
        || ((97f64..=122f64).contains(&code)))
        || ((!first && code >= 48f64) && code <= 57f64)
}

/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_name(name: &str) -> bool {
    (((!name.is_empty()) && name != "_") && !(name.starts_with("ml_")))
        && !(((((((((((((((((((((((((((((((((((((((((((((((((((name == "main" || name == "as")
            || name == "async")
            || name == "await")
            || name == "break")
            || name == "const")
            || name == "continue")
            || name == "crate")
            || name == "dyn")
            || name == "else")
            || name == "enum")
            || name == "extern")
            || name == "false")
            || name == "fn")
            || name == "for")
            || name == "if")
            || name == "impl")
            || name == "in")
            || name == "let")
            || name == "loop")
            || name == "match")
            || name == "mod")
            || name == "move")
            || name == "mut")
            || name == "pub")
            || name == "ref")
            || name == "return")
            || name == "self")
            || name == "Self")
            || name == "static")
            || name == "struct")
            || name == "super")
            || name == "trait")
            || name == "true")
            || name == "type")
            || name == "unsafe")
            || name == "use")
            || name == "where")
            || name == "while")
            || name == "abstract")
            || name == "become")
            || name == "box")
            || name == "do")
            || name == "final")
            || name == "macro")
            || name == "override")
            || name == "priv")
            || name == "try")
            || name == "typeof")
            || name == "unsized")
            || name == "virtual")
            || name == "yield")
}

/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_header_form(form: &str) -> bool {
    form == "implicit unit" || form == "explicit unit"
}

/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_body_form(form: &str) -> bool {
    form == "empty block" || form == "unit expression"
}

/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_path_form(form: &str) -> bool {
    form == "unqualified name" || form == "absolute crate path"
}

/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_context(language: &str, forbidden: bool) -> bool {
    language == "Rust" && !forbidden
}
