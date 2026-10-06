//! The message a source program aborts with, in the words of the source
//! language, so every target aborts where the source does with the message
//! the source prints.
//!
//! Machine integers are Rust's, whose arithmetic panics (overflow checks on,
//! as in a debug build) with these messages; `BigInt` division by zero is
//! JavaScript's `RangeError`; a natural parameter is a JavaScript `BigInt`
//! parameter whose leading guard throws its own message.
//!
//! Mirrors `js/src/translation/aborts.js`.

use super::types::Type;

/// The panic of a machine-integer operation whose result is out of range:
/// `op` is `add`, `sub`, `mul`, `div`, `rem` or `neg`.
///
/// # Panics
/// On any other operation, which cannot overflow.
#[must_use]
pub fn overflow_message(op: &str) -> &'static str {
    match op {
        "add" => "attempt to add with overflow",
        "sub" => "attempt to subtract with overflow",
        "mul" => "attempt to multiply with overflow",
        "div" => "attempt to divide with overflow",
        "rem" => "attempt to calculate the remainder with overflow",
        "neg" => "attempt to negate with overflow",
        other => panic!("{other} does not overflow"),
    }
}

/// The abort of an integer division or remainder by zero.
#[must_use]
pub fn zero_divisor_message(op: &str, ty: &Type) -> &'static str {
    if !matches!(ty, Type::Fixed { .. }) {
        return "Division by zero";
    }
    if op == "rem" {
        "attempt to calculate the remainder with a divisor of zero"
    } else {
        "attempt to divide by zero"
    }
}

/// The abort of a checked conversion of a negative integer to a natural: the guard's own message.
#[must_use]
pub fn cast_message(message: Option<&str>) -> &str {
    message.unwrap_or("conversion of a negative integer to a natural")
}

/// The message of `panic!`, `unreachable!`, `unimplemented!` and `todo!`, with an optional literal message.
#[must_use]
pub fn rust_macro_message(name: &str, message: Option<&str>) -> String {
    let prefix = match name {
        "unreachable" => Some("internal error: entered unreachable code"),
        "unimplemented" => Some("not implemented"),
        "todo" => Some("not yet implemented"),
        _ => None,
    };
    match (prefix, message) {
        (prefix, None) => prefix.unwrap_or("explicit panic").to_owned(),
        (None, Some(message)) => message.to_owned(),
        (Some(prefix), Some(message)) => format!("{prefix}: {message}"),
    }
}
