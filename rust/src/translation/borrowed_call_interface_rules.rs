// Generated through the checked JavaScript translator. Do not edit.
/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn accept_rust_borrowed_call_interface_model(model: &str) -> bool {
    model == "closed primitive borrowed Rust call interface 1"
}

/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn accept_rust_borrowed_call_target(owned: bool, leaf: bool, static_lifetime: bool) -> bool {
    (owned && leaf) && !static_lifetime
}

/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn accept_rust_borrowed_call_shape(
    kind: &str,
    unrepresented: bool,
    correct_arity: bool,
) -> bool {
    (kind == "call" && !unrepresented) && correct_arity
}

/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn accept_rust_borrowed_call_inventory(has_caller: bool) -> bool {
    has_caller
}

/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn accept_rust_borrowed_call_type(argument_types: bool, result_types: bool) -> bool {
    argument_types && result_types
}

/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn render_rust_borrowed_call_header_reference(
    lifetime: &str,
    inner: &str,
    inner_reference: bool,
) -> String {
    if lifetime.is_empty() {
        format!(
            "&{}{inner}",
            if inner_reference {
                String::from(" ")
            } else {
                String::new()
            }
        )
    } else {
        format!("&'{lifetime} {inner}")
    }
}

/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn render_rust_borrowed_call_reference(value: &str) -> String {
    format!("&({value})")
}

/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn render_rust_borrowed_call_invocation(head: &str, arguments_text: &str) -> String {
    format!("{head}({arguments_text})")
}

/// Apply the shared closed primitive borrowed call interface policy.
#[must_use]
pub fn render_rust_borrowed_call_block(body: &str) -> String {
    format!("{{\n{body}\n}}")
}
