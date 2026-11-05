// Generated from complete js/src/translation/rust-scalar-source-interface-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_interface_model(model: &str) -> bool {
    model == "retained scalar Rust function interface 1"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_value_type(kind: &str, bits: f64, signed: bool) -> String {
    if kind == "unit" {
        String::from("()")
    } else if kind == "bool" {
        String::from("bool")
    } else if kind == "float" {
        String::from("f64")
    } else if kind == "fixed" {
        if bits == 8f64 {
            if signed {
                String::from("i8")
            } else {
                String::from("u8")
            }
        } else if bits == 16f64 {
            if signed {
                String::from("i16")
            } else {
                String::from("u16")
            }
        } else if bits == 32f64 {
            if signed {
                String::from("i32")
            } else {
                String::from("u32")
            }
        } else if bits == 64f64 {
            if signed {
                String::from("i64")
            } else {
                String::from("u64")
            }
        } else if bits == 128f64 {
            if signed {
                String::from("i128")
            } else {
                String::from("u128")
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_type(
    value_type: &str,
    owner_kind: &str,
    owner: &str,
    paths: f64,
    children: f64,
) -> String {
    if children == 0f64 {
        if value_type == "()" {
            if (owner_kind == "unit") && (paths == 0f64) {
                String::from("()")
            } else {
                String::new()
            }
        } else if (((owner_kind != "primitive") || (paths != 1f64)) || ((value_type).is_empty()))
            || ((((((((((((value_type != "bool") && (value_type != "f64"))
                && (value_type != "i8"))
                && (value_type != "u8"))
                && (value_type != "i16"))
                && (value_type != "u16"))
                && (value_type != "i32"))
                && (value_type != "u32"))
                && (value_type != "i64"))
                && (value_type != "u64"))
                && (value_type != "i128"))
                && (value_type != "u128"))
        {
            String::new()
        } else if value_type == owner {
            value_type.to_owned()
        } else if (value_type == "u64") && (owner == "usize") {
            String::from("usize")
        } else if (value_type == "i64") && (owner == "isize") {
            String::from("isize")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_context(language: &str, forbidden: bool) -> bool {
    (language == "Rust") && !forbidden
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_declaration(kind: &str, generated: bool, owned: bool) -> bool {
    ((kind == "fn") && !generated) && owned
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_declaration_field(field: &str, ordinary_allowed: bool) -> bool {
    ordinary_allowed && (field != "sourceLoop")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_position(
    signature_model: &str,
    source_type: &str,
    invalid_wrappers: bool,
    borrowed_result: bool,
) -> bool {
    (((signature_model == "Rust source function signature 2") && (!(source_type).is_empty()))
        && !invalid_wrappers)
        && !borrowed_result
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_expression(kind: &str) -> bool {
    ((((((((kind == "lit") || (kind == "unit")) || (kind == "var")) || (kind == "let"))
        || (kind == "if"))
        || (kind == "unary"))
        || (kind == "binary"))
        || (kind == "cast"))
        || (kind == "call")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_expression_field(kind: &str, field: &str) -> bool {
    if ((field == "k") || (field == "type")) || (field == "span") {
        self::accept_rust_scalar_source_expression(kind)
    } else if kind == "lit" {
        field == "value"
    } else if kind == "var" {
        field == "name"
    } else if kind == "let" {
        ((field == "name") || (field == "value")) || (field == "body")
    } else if kind == "if" {
        ((field == "cond") || (field == "then")) || (field == "else")
    } else if kind == "unary" {
        ((field == "op") || (field == "arg")) || (field == "semantics")
    } else if kind == "binary" {
        ((((((field == "op") || (field == "left")) || (field == "right")) || (field == "domain"))
            || (field == "semantics"))
            || (field == "rounding"))
            || (field == "byZero")
    } else if kind == "call" {
        (field == "fn") || (field == "args")
    } else if kind == "cast" {
        (((field == "arg") || (field == "from")) || (field == "to")) || (field == "flavor")
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_scalar_source_call(
    owned: bool,
    argument_types: bool,
    result_type: bool,
    static_borrow: bool,
) -> bool {
    ((owned && argument_types) && result_type) && !static_borrow
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_source_budget(visits: f64) -> bool {
    ((0f64..=100_000f64).contains(&visits)) && ((visits % 1f64) == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_scalar_source_conversion(
    value: &str,
    source_type: &str,
    into_source: bool,
) -> String {
    if source_type == "usize" {
        format!(
            "({}) as {}",
            value.to_owned(),
            if into_source {
                String::from("usize")
            } else {
                String::from("u64")
            }
        )
    } else if source_type == "isize" {
        format!(
            "({}) as {}",
            value.to_owned(),
            if into_source {
                String::from("isize")
            } else {
                String::from("i64")
            }
        )
    } else {
        value.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_argument_name() -> String {
    String::from("ml_source_argument")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_target_guard() -> String {
    String::from(
        "#[cfg(not(target_pointer_width = \"64\"))]\ncompile_error!(\"retained scalar Rust function interface 1 requires target_pointer_width=64\");",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_encoding() -> String {
    String::from("retained-scalar-source-interface")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_source_statement() -> String {
    String::from(
        "owned version-two scalar headers retain source primitive identities; calls evaluate each argument once in order before borrowing copies; usize/isize use exact 64-bit boundary conversions and other pointer widths refuse",
    )
}
