// Generated from complete js/src/translation/javascript-optional-chain-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_call_operation() -> String {
    String::from("invokeJavaScriptOptionalCallable")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_optional_call_head(kind: &str, path_length: f64) -> bool {
    (((((((((kind == "name") && (path_length >= 1f64)) || (kind == "functionValue"))
        || (kind == "lambda"))
        || (kind == "app"))
        || (kind == "let"))
        || (kind == "if"))
        || (kind == "math"))
        || (kind == "none"))
        || (kind == "unit")
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_chain_operation() -> String {
    String::from("continueJavaScriptOptionalChain")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_chain_field() -> String {
    String::from("receiver")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_chain_form(kind: &str, element: &str) -> String {
    if (kind == "unit") || ((kind == "option") && (element == "unit")) {
        String::from("absent")
    } else if kind == "option" {
        String::from("guarded")
    } else {
        String::from("direct")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_chain_inference_form(kind: &str, element: &str) -> String {
    if kind == "var" {
        String::from("unknown")
    } else {
        self::read_java_script_optional_chain_form(kind, element)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_chain_presence_form(kind: &str) -> String {
    if ((((((kind == "record") || (kind == "array")) || (kind == "tuple")) || (kind == "map"))
        || (kind == "set"))
        || (kind == "function"))
        || (kind == "data")
    {
        String::from("truthy")
    } else {
        String::from("tag")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_optional_chain_result_form(kind: &str) -> String {
    self::read_java_script_optional_call_result_form(kind)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_optional_chain_operands(
    arguments_count: f64,
    spread: bool,
    name: &str,
) -> bool {
    ((arguments_count == 1f64) && !spread) && ((name.encode_utf16().count() as f64) > 0f64)
}
