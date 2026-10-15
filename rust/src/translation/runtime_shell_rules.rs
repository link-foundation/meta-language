// Generated through the checked JavaScript translator. Do not edit.
/// Read the shared closed runtime-shell admission.
#[must_use]
pub fn accept_rust_runtime_shell_model(model: &str) -> bool {
    model.is_empty() || model == "closed pure Rust unit runtime shell 1"
}

/// Read the shared closed runtime-shell admission.
#[must_use]
pub fn accept_rust_runtime_shell_reserved_name(name: &str) -> bool {
    !matches!(
        name,
        "std" | "ml_main" | "main" | "__proto__" | "prototype" | "constructor"
    )
}

/// Read the shared closed runtime-shell admission.
#[must_use]
pub fn accept_rust_runtime_shell_stack_shift(value: f64) -> bool {
    (16f64..=28f64).contains(&value) && value % 1f64 == 0f64
}
