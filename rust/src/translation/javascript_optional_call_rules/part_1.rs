// Generated from complete js/src/translation/javascript-optional-call-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_call_operation() -> String {
    String::from("invokeJavaScriptOptionalCallable")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_optional_call_head(kind: &str, path_length: f64) -> bool {
    (((((((((((kind == "name") && (path_length >= 1f64)) || (kind == "functionValue"))
        || (kind == "lambda"))
        || (kind == "app"))
        || (kind == "let"))
        || (kind == "if"))
        || (kind == "math"))
        || (kind == "none"))
        || (kind == "unit"))
        || (kind == "field"))
        || (kind == "index")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_call_form(kind: &str, element: &str) -> String {
    if kind == "function" {
        String::from("direct")
    } else if (kind == "unit") || ((kind == "option") && (element == "unit")) {
        String::from("absent")
    } else if (kind == "option") && (element == "function") {
        String::from("guarded")
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_call_result_form(kind: &str) -> String {
    if kind == "unit" {
        String::from("undefined")
    } else if kind == "option" {
        String::from("flatten")
    } else {
        String::from("present")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_call_field() -> String {
    String::from("callable")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_optional_call_chain(suffix: &str) -> bool {
    (((suffix == ".") || (suffix == "?.")) || (suffix == "[")) || (suffix == "(")
}
