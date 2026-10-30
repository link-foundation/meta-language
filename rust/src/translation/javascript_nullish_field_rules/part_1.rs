// Generated from complete js/src/translation/javascript-nullish-field-rules.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_nullish_field_form(kind: &str, element: &str) -> String {
    self::read_java_script_optional_chain_form(kind, element)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_nullish_field_message(absence: &str, field: &str) -> String {
    format!(
        "{}{}{}",
        self::read_java_script_nullish_property_message_prefix(absence, true),
        field.to_owned(),
        self::read_java_script_nullish_property_message_suffix(true)
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_nullish_property_message_prefix(absence: &str, named: bool) -> String {
    format!(
        "Cannot read properties of {}{}",
        absence.to_owned(),
        if named {
            String::from(" (reading '")
        } else {
            String::new()
        }
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_nullish_property_message_suffix(named: bool) -> String {
    if named {
        String::from("')")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_nullish_key_description(
    kind: &str,
    owns_to_string: bool,
    owns_constructor: bool,
) -> String {
    if ((((((kind == "string") || (kind == "float")) || (kind == "int")) || (kind == "nat"))
        || (kind == "bool"))
        || (kind == "unit"))
        || (kind == "option")
    {
        String::from("text")
    } else if (kind == "array") || (kind == "tuple") {
        String::from("omitted")
    } else if kind == "record" {
        if owns_to_string {
            String::from("omitted")
        } else if owns_constructor {
            String::from("unsupported")
        } else {
            String::from("object")
        }
    } else if kind == "map" {
        String::from("map")
    } else if kind == "set" {
        String::from("set")
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_nullish_index_key_field() -> String {
    String::from("sourceKey")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_nullish_key_description_origin(form: &str, kind: &str) -> bool {
    if form == "text" {
        true
    } else if form == "object" {
        kind == "record"
    } else if form == "map" {
        kind == "map"
    } else if form == "set" {
        kind == "set"
    } else {
        (form == "omitted") && (((kind == "record") || (kind == "array")) || (kind == "tuple"))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_nullish_key_description_text(form: &str) -> String {
    if form == "object" {
        String::from("#<Object>")
    } else if form == "map" {
        String::from("#<Map>")
    } else if form == "set" {
        String::from("#<Set>")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_property_description_shift() -> String {
    String::from("6400")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_property_description_overflow() -> String {
    String::from("<a very large BigInt>")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_nullish_field_capture() -> String {
    String::from("receiver")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_slice_reference_copy(
    language: &str,
    kind: &str,
    reference: bool,
) -> bool {
    ((language == "JavaScript") && (kind == "array")) && reference
}
