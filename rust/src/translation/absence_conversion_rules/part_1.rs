// Generated from complete js/src/translation/absence-conversion-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_absence_only_value(language: &str, kind: &str, element: &str) -> bool {
    (language == "JavaScript") && ((kind == "unit") || ((kind == "option") && (element == "unit")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_absence_only_conversion(
    language: &str,
    kind: &str,
    element: &str,
    absence: &str,
    target: &str,
    policy: &str,
) -> String {
    if !(self::require_java_script_absence_only_value(language, kind, element))
        || (target != "option")
    {
        String::new()
    } else if (kind == "unit") || (absence == "undefined") {
        if (policy == "undefined") || (policy == "both") {
            String::from("undefined")
        } else {
            String::new()
        }
    } else if ((absence == "null") || (absence == "both")) && (policy == "both") {
        String::from("tagged")
    } else {
        String::new()
    }
}
