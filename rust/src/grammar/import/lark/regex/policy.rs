// Generated from the JavaScript Lark regex policy.
pub(super) fn accept_lark_regex_flag(flag: &str) -> bool {
    flag == "i" || flag == "s"
}

pub(super) fn accept_lark_regex_quantifier(character: &str) -> bool {
    character == "+" || character == "*" || character == "?"
}

pub(super) fn accept_lark_regex_meta_character(character: &str) -> bool {
    character == "("
        || character == ")"
        || character == "|"
        || character == "^"
        || character == "$"
        || character == "{"
        || character == "}"
        || character == "]"
        || character == "+"
        || character == "*"
        || character == "?"
}

pub(super) fn accept_lark_regex_literal_escape(point: f64) -> bool {
    point < 128.0
        && (point < 48.0 || point > 57.0)
        && (point < 65.0 || point > 90.0)
        && (point < 97.0 || point > 122.0)
}
