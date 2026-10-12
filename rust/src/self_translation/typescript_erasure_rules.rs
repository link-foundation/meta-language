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
