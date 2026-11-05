// Generated from complete js/src/translation/optional-policy-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_policy_widening(
    language: &str,
    source: &str,
    absence: &str,
    target: &str,
    policy: &str,
    same_element: bool,
) -> String {
    if (((((language == "JavaScript") && (source == "option")) && (target == "option"))
        && same_element)
        && (policy == "both"))
        && ((absence == "null") || (absence == "undefined"))
    {
        absence.to_owned()
    } else {
        String::new()
    }
}
