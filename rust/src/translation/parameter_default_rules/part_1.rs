// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_parameter_default_value_step(kind: &str, flavor: &str) -> String {
    if (((kind == "lit") || (kind == "none")) || (kind == "unit")) || (kind == "var") {
        String::from("pure")
    } else if (kind == "cast") && (flavor == "exact") {
        String::from("arg")
    } else if kind == "some" {
        String::from("value")
    } else {
        String::from("effect")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_parameter_default_argument_bindings(index: f64, provided: f64, pure: bool) -> bool {
    ((index >= 0f64) && ((index + 1f64) < provided)) && !pure
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_parameter_default_argument_inference(
    defaulted: bool,
    optional: bool,
    kind: &str,
    absence: &str,
) -> String {
    if !defaulted || optional {
        String::from("identity")
    } else if kind == "var" {
        String::from("defer")
    } else if (kind == "option") && (absence == "undefined") {
        String::from("element")
    } else {
        String::from("value")
    }
}
