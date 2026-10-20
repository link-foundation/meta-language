// Generated from js/src/typescript-erasure-rules.js by
// js/scripts/generate-typescript-erasure-rules.mjs. Do not edit by hand.
/// Retain the first expression child for a trailing compile time annotation.
#[must_use]
pub fn retain_first_erasure_child(operation: &str) -> bool {
    operation == "suffix"
}

/// Erase the surrounding export of a compile time declaration.
#[must_use]
pub fn erase_exported_declaration(operation: &str, exported: bool) -> bool {
    operation == "declaration" && exported
}

/// Erase a question mark only when it belongs to an optional parameter.
#[must_use]
pub fn erase_optional_parameter_marker(term: &str, parent_term: &str) -> bool {
    term == "?" && parent_term == "optional_parameter"
}

/// Preserve carriage returns and line feeds while masking every other scalar.
#[must_use]
pub fn render_erased_type_script_scalar(carriage_return: bool, line_feed: bool) -> String {
    if carriage_return {
        String::from("\r")
    } else if line_feed {
        String::from("\n")
    } else {
        String::from(" ")
    }
}

/// Escape a comment terminator only in supplemental documentation pattern text.
/// Original executable patterns remain authoritative; this display has no inverse claim.
#[must_use]
pub fn render_type_script_documentation_pattern(pattern: &str) -> String {
    pattern.replace("*/", "*\\/")
}

/// Erase a type specifier and its following comma; retain runtime imports and evaluation.
#[must_use]
pub fn erase_type_script_named_import(
    type_specifier: bool,
    comma: bool,
    after_type_specifier: bool,
) -> bool {
    type_specifier || comma && after_type_specifier
}

/// Read the shared compile-time receiver parameter decision.
#[must_use]
pub fn accept_type_script_receiver_parameter(
    parameter_flags: (bool, bool, bool, bool, bool),
) -> bool {
    let (first, required, annotated, initialized, owner) = parameter_flags;
    first && required && annotated && !initialized && owner
}

/// Read the shared compile-time receiver parameter decision.
#[must_use]
pub fn erase_type_script_receiver_separator(comma: bool, after_receiver: bool) -> bool {
    comma && after_receiver
}
