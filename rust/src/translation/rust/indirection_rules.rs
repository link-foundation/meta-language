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

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_source_box_coercion_layer(layer: &str, initial: bool) -> bool {
    initial && layer == "box"
}

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn read_rust_source_expression_origin(kind: &str, operation: &str) -> String {
    if kind == "abort" {
        String::from("diverging")
    } else {
        if kind == "toString" && (operation == "from") {
            String::from("unboxed-conversion")
        } else {
            if kind == "cast" {
                String::from("numeric-cast")
            } else {
                if kind == "unary" {
                    String::from("primitive-operation")
                } else {
                    if kind == "binary" {
                        if operation == "eq"
                            || operation == "ne"
                            || operation == "lt"
                            || operation == "le"
                            || operation == "gt"
                            || operation == "ge"
                        {
                            String::from("comparison")
                        } else {
                            if operation == "and" || operation == "or" {
                                String::from("boolean-operation")
                            } else {
                                String::from("primitive-operation")
                            }
                        }
                    } else {
                        if ((kind == "num" || kind == "bool") || kind == "unit") || kind == "str" {
                            String::from("value")
                        } else {
                            if ((kind == "toString" || kind == "show") || kind == "stringMap")
                                || kind == "stringTest"
                            {
                                String::from("derived-value")
                            } else {
                                String::from("unknown")
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_source_box_availability(owner: &str, declared: bool) -> bool {
    owner != "alloc::boxed::Box" || declared
}

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_source_text_equality(
    operation: &str,
    left_direct_string: bool,
    right_direct_string: bool,
    left_borrowed_slice: bool,
    right_borrowed_slice: bool,
) -> bool {
    (operation == "eq" || operation == "ne")
        && ((left_direct_string && right_borrowed_slice)
            || (right_direct_string && left_borrowed_slice))
}

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_source_lifetime_binding(name: &str, duplicate: bool, bounded: bool) -> bool {
    !duplicate
        && !bounded
        && name != "static"
        && name != "_"
        && name != "as"
        && name != "break"
        && name != "const"
        && name != "continue"
        && name != "crate"
        && name != "else"
        && name != "enum"
        && name != "extern"
        && name != "false"
        && name != "fn"
        && name != "for"
        && name != "if"
        && name != "impl"
        && name != "in"
        && name != "let"
        && name != "loop"
        && name != "match"
        && name != "mod"
        && name != "move"
        && name != "mut"
        && name != "pub"
        && name != "ref"
        && name != "return"
        && name != "self"
        && name != "Self"
        && name != "struct"
        && name != "super"
        && name != "trait"
        && name != "true"
        && name != "type"
        && name != "unsafe"
        && name != "use"
        && name != "where"
        && name != "while"
        && name != "async"
        && name != "await"
        && name != "dyn"
        && name != "abstract"
        && name != "become"
        && name != "box"
        && name != "do"
        && name != "final"
        && name != "macro"
        && name != "override"
        && name != "priv"
        && name != "typeof"
        && name != "unsized"
        && name != "virtual"
        && name != "yield"
        && name != "try"
        && name != "gen"
}

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_source_lifetime_reference(name: &str, declared: bool) -> bool {
    name == "static" || name == "_" || declared
}

/// Admit the corresponding immutable Rust source projection.
#[must_use]
pub fn accept_rust_source_lifetime_result(named_bindings: bool, reference_result: bool) -> bool {
    !named_bindings || !reference_result
}
