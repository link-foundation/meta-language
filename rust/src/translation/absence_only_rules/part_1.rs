// Generated from complete js/src/translation/absence-only-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_absence_only_value(language: &str, kind: &str, element: &str) -> bool {
    (language == "JavaScript") && ((kind == "unit") || ((kind == "option") && (element == "unit")))
}
