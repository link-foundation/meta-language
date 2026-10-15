// Generated from the checked JavaScript cast syntax decisions. Do not edit.
/// Read the shared Rust cast syntax boundary.
#[must_use]
pub fn accept_rust_cast_following_token(following_token: &str, parenthesized_type: bool) -> bool {
    (following_token != "<") || parenthesized_type
}

/// Read the shared Rust cast syntax boundary.
#[must_use]
pub fn read_rust_cast_syntax_message() -> String {
    String::from("a cast before < requires parentheses")
}
