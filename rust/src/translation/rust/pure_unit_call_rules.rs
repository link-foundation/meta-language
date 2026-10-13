// Generated through the checked JavaScript translator. Do not edit.
/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_call_model(model: &str) -> bool {
    model.is_empty() || model == "closed pure Rust unit calls 1"
}

/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_name_character(code: f64, first: bool) -> bool {
    ((((95f64..=95f64).contains(&code)) || ((65f64..=90f64).contains(&code)))
        || ((97f64..=122f64).contains(&code)))
        || (!first && (48f64..=57f64).contains(&code))
}

/// Read a shared pure Rust Unit-call admission.
#[must_use]
pub fn accept_rust_pure_unit_name(name: &str) -> bool {
    !name.is_empty()
        && name != "_"
        && !name.starts_with("ml_")
        && !matches!(
            name,
            "main"
                | "as"
                | "async"
                | "await"
                | "break"
                | "const"
                | "continue"
                | "crate"
                | "dyn"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
                | "abstract"
                | "become"
                | "box"
                | "do"
                | "final"
                | "gen"
                | "macro"
                | "override"
                | "priv"
                | "try"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
        )
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
