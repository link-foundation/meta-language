// Generated from complete js/src/translation/rust-primitive-inherent-copy-rules.js. Do not edit by hand.
use super::accept_rust_primitive_selection_receiver;
use super::ml_array;
use super::read_rust_checked_integer_arity;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_associated_arity(owner: &str, method: &str) -> f64 {
    if (self::ml_array::includes(
        &vec![
            String::from("bool"),
            String::from("i8"),
            String::from("u8"),
            String::from("i16"),
            String::from("u16"),
            String::from("i32"),
            String::from("u32"),
            String::from("i64"),
            String::from("u64"),
            String::from("i128"),
            String::from("u128"),
            String::from("isize"),
            String::from("usize"),
        ],
        &owner.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )) {
        if (method == "min") || (method == "max") {
            2f64
        } else if owner == "bool" {
            -1f64
        } else {
            {
                let checked = self::read_rust_checked_integer_arity(method, true);
                if (checked >= 0f64) && ((method != "checked_abs") || owner.starts_with("i")) {
                    checked + 1f64
                } else if self::ml_array::includes(
                    &vec![
                        String::from("saturating_add"),
                        String::from("saturating_sub"),
                        String::from("saturating_mul"),
                        String::from("wrapping_add"),
                        String::from("wrapping_sub"),
                    ],
                    &method.to_owned(),
                    0f64,
                    false,
                    |a, b| a == b,
                ) {
                    2f64
                } else if method == "wrapping_neg" {
                    1f64
                } else if (owner.starts_with("i")) {
                    if self::ml_array::includes(
                        &vec![
                            String::from("is_negative"),
                            String::from("is_positive"),
                            String::from("signum"),
                            String::from("saturating_neg"),
                            String::from("saturating_abs"),
                            String::from("wrapping_abs"),
                        ],
                        &method.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    ) {
                        1f64
                    } else {
                        -1f64
                    }
                } else {
                    -1f64
                }
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_associated_literal(expected: &str, form: &str, typed: bool) -> bool {
    (((!(expected).is_empty()) && (expected != "bool")) && !typed)
        && ((form == "numeral") || (form == "negative numeral"))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_associated_receiver(
    expected: &str,
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (expected == path)
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_inherent_copy(
    method: &str,
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
    references_only: bool,
    copy_evidence: bool,
) -> bool {
    (((((((method != "min") && (method != "max"))
        && ((self::read_rust_primitive_associated_arity(path, method)) > 0f64))
        && (kind == "fixed"))
        && (layers >= 1f64))
        && references_only)
        && copy_evidence)
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, 0f64,
        )
}
