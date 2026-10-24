// Generated from complete js/src/translation/rust-unit-value-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_unit_statement_disposition(
    kind: &str,
    collecting: bool,
    proven: bool,
    type_: &str,
    indirected: bool,
) -> String {
    if kind == "abort" {
        String::from("abort")
    } else if ((proven && (type_ == "unit")) && !indirected) || (collecting && (kind == "app")) {
        String::from("bind")
    } else {
        String::from("refuse")
    }
}
