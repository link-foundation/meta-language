// Generated from complete js/src/translation/rust-primitive-selection-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_model(model: &str) -> bool {
    ((model).is_empty()) || (model == "Rust source function signature 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_wrapper(wrapper: &str) -> bool {
    (wrapper == "reference") || (wrapper == "box")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_owner_kind(kind: &str) -> bool {
    (((((kind == "primitive") || (kind == "declaration")) || (kind == "library"))
        || (kind == "tuple"))
        || (kind == "unit"))
        || (kind == "slice")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_catalog_owner(kind: &str) -> bool {
    (((kind == "data") || (kind == "module")) || (kind == "externalCrate")) || (kind == "ctor")
}

/// Execute the checked shared source algorithm.
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
        ((kind == "fixed") && (bits == "8")) && (signed == (true))
    } else if path == "u8" {
        ((kind == "fixed") && (bits == "8")) && (signed == (false))
    } else if path == "i16" {
        ((kind == "fixed") && (bits == "16")) && (signed == (true))
    } else if path == "u16" {
        ((kind == "fixed") && (bits == "16")) && (signed == (false))
    } else if path == "i32" {
        ((kind == "fixed") && (bits == "32")) && (signed == (true))
    } else if path == "u32" {
        ((kind == "fixed") && (bits == "32")) && (signed == (false))
    } else if path == "i64" {
        ((kind == "fixed") && (bits == "64")) && (signed == (true))
    } else if path == "u64" {
        ((kind == "fixed") && (bits == "64")) && (signed == (false))
    } else if path == "i128" {
        ((kind == "fixed") && (bits == "128")) && (signed == (true))
    } else if path == "u128" {
        ((kind == "fixed") && (bits == "128")) && (signed == (false))
    } else if path == "usize" {
        ((kind == "fixed") && (bits == "64")) && (signed == (false))
    } else if path == "isize" {
        ((kind == "fixed") && (bits == "64")) && (signed == (true))
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_call_role(
    path: &str,
    ordinary: bool,
    shadowed: bool,
) -> String {
    if ordinary && !shadowed {
        if path == "std::cmp::min" {
            String::from("min")
        } else if path == "std::cmp::max" {
            String::from("max")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_comparison(method: &str, ordinary: bool) -> String {
    if ordinary {
        if method == "min" {
            String::from("le")
        } else if method == "max" {
            String::from("gt")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_condition(method: &str, kind: &str) -> String {
    if kind == "bool" {
        if method == "min" {
            String::from("not")
        } else if method == "max" {
            String::from("identity")
        } else {
            String::new()
        }
    } else if kind == "fixed" {
        self::read_rust_primitive_selection_comparison(method, true)
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_selection_receiver(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((((kind == "fixed") || (kind == "bool")) && (owner == "primitive")) && (children == 0f64))
        && proven)
        && (layers == 0f64))
        && self::accept_rust_source_signature_primitive(kind, path, bits, signed)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_selection_argument(
    expected: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    actual: &str,
    actual_bits: &str,
    actual_signed: bool,
    actual_owner: &str,
    layers: f64,
    proven: bool,
    known: bool,
    literal: bool,
) -> bool {
    if layers == 0f64 {
        if ((literal && (expected == "fixed")) && ((actual).is_empty()))
            && ((actual_owner).is_empty())
        {
            true
        } else {
            ((((known && proven) && (expected == actual)) && (bits == actual_bits))
                && (signed == actual_signed))
                && (owner == actual_owner)
        }
    } else {
        false
    }
}
