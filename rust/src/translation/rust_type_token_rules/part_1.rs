// Generated from complete js/src/translation/rust-type-token-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_closing_token_parts(token: &str) -> Vec<String> {
    if token == ">>" {
        vec![String::from(">"), String::from(">")]
    } else if token == ">=" {
        vec![String::from(">"), String::from("=")]
    } else if token == ">>=" {
        vec![String::from(">"), String::from(">"), String::from("=")]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_reference_token_parts(token: &str) -> Vec<String> {
    if token == "&&" {
        vec![String::from("&"), String::from("&")]
    } else {
        Vec::<String>::new()
    }
}
