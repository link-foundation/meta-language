// Generated from complete js/src/translation/rust-panic-value-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_value_operation() -> String {
    String::from("raiseRustPanicValue")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_rust_panic_value(kind: &str, operation: &str) -> bool {
    (kind == "math") && (operation == (self::read_rust_panic_value_operation()))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_value_argument_error(
    language: &str,
    count: f64,
    spread: bool,
    kind: &str,
) -> String {
    if (((language == "Rust") && (count == 1f64)) && !spread) && (kind == "string") {
        String::new()
    } else {
        String::from("a Rust panic value needs one checked source String message")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_panic_value_expression(target: &str, message: &str) -> String {
    if target == "JavaScript" {
        format!(
            "((ml_message)=>{{throw new Error(ml_message);}})({})",
            message.to_owned()
        )
    } else if target == "Rust" {
        format!("panic!(\"{{}}\", {})", message.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_panic_formatting_macro(name: &str, ordinary: bool) -> bool {
    ordinary
        && ((((name == "panic") || (name == "unreachable")) || (name == "unimplemented"))
            || (name == "todo"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_panic_formatting_value(
    name: &str,
    ordinary: bool,
    kind: &str,
    count: f64,
    braces: bool,
) -> bool {
    (self::accept_rust_panic_formatting_macro(name, ordinary) && (count > 0f64))
        && ((count > 1f64) || ((kind == "str") && braces))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_formatting_prefix(name: &str) -> String {
    if name == "unreachable" {
        String::from("internal error: entered unreachable code: ")
    } else if name == "unimplemented" {
        String::from("not implemented: ")
    } else if name == "todo" {
        String::from("not yet implemented: ")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_completion_kind(kind: &str, operation: &str) -> String {
    if self::accept_linked_rust_panic_value(kind, operation) {
        String::from("abort")
    } else {
        kind.to_owned()
    }
}
