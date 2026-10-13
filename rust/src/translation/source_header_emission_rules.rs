// Generated through the checked JavaScript translator. Do not edit.
/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn accept_rust_source_header_emission_model(model: &str) -> bool {
    model == "primitive borrowed Rust function header 1"
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn accept_rust_source_header_emission_context(language: &str, forbidden: bool) -> bool {
    language == "Rust" && !forbidden
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn accept_rust_source_header_emission_declaration(
    kind: &str,
    non_flat: bool,
    generated: bool,
) -> bool {
    (kind == "fn" && !non_flat) && !generated
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn accept_rust_source_header_emission_position(
    kind: &str,
    owner_kind: &str,
    owner_path: &str,
    invalid_arguments: bool,
    invalid_wrappers: bool,
    borrowed_result: bool,
) -> bool {
    (((!invalid_arguments && !invalid_wrappers) && !borrowed_result) && owner_kind == "primitive")
        && ((kind == "float" && owner_path == "f64") || (kind == "bool" && owner_path == "bool"))
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn accept_rust_source_header_emission_expression(kind: &str) -> bool {
    ((((kind == "lit" || kind == "var") || kind == "let") || kind == "if") || kind == "unary")
        || kind == "binary"
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn accept_rust_source_header_lifetime_character(character: f64, first: bool) -> bool {
    ((((95f64..=95f64).contains(&character)) || ((65f64..=90f64).contains(&character)))
        || ((97f64..=122f64).contains(&character)))
        || (!first && ((48f64..=57f64).contains(&character)))
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn read_rust_source_header_primitive(kind: &str) -> String {
    if kind == "float" {
        String::from("f64")
    } else if kind == "bool" {
        String::from("bool")
    } else {
        String::new()
    }
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn render_rust_source_header_reference(lifetime: &str, inner: &str) -> String {
    if lifetime.is_empty() {
        format!("&{inner}")
    } else {
        format!("&'{lifetime} {inner}")
    }
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn render_rust_source_header_parameter(name: &str, type_: &str) -> String {
    format!("{name}: {type_}")
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn render_rust_source_header_lifetime(name: &str) -> String {
    format!("'{name}")
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn render_rust_source_header_dereference(inner: &str) -> String {
    format!("*{inner}")
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn render_rust_source_header_copy(name: &str, dereferenced: &str) -> String {
    format!("let {name} = {dereferenced};")
}

/// Apply the shared primitive borrowed header emission policy.
#[must_use]
pub fn render_rust_source_header_function(
    name: &str,
    lifetimes: &str,
    parameters: &str,
    result: &str,
    body: &str,
) -> String {
    format!(
        "pub fn {name}{}({parameters}) -> {result} {{\n{body}\n}}",
        if lifetimes.is_empty() {
            String::new()
        } else {
            format!("<{lifetimes}>")
        }
    )
}
