// Generated from complete js/src/translation/rust-array-length-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_exact_array_length(method: &str, binary64: bool, ordinary: bool) -> bool {
    ((method == "len") && !binary64) && ordinary
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_exact_array_length_result(
    expression: &str,
    result: &str,
    bits: f64,
    signed: bool,
    operand: &str,
) -> bool {
    ((((expression == "length") && (result == "fixed")) && (bits == 64f64)) && !signed)
        && (operand == "array")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_exact_array_length(target: &str, operand: &str) -> String {
    if target == "Rust" {
        format!("(({}).len() as u64)", operand.to_owned())
    } else if target == "JavaScript" {
        format!("BigInt(({}).length)", operand.to_owned())
    } else if target == "Lean" {
        format!("({}).size", operand.to_owned())
    } else if target == "Rocq" {
        format!("(N.of_nat (List.length {}))", operand.to_owned())
    } else {
        String::new()
    }
}
