// Generated from complete js/src/translation/source-exception-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_operation(name: &str) -> String {
    if name == "Error" {
        String::from("raiseJavaScriptError")
    } else if name == "TypeError" {
        String::from("raiseJavaScriptTypeError")
    } else if name == "RangeError" {
        String::from("raiseJavaScriptRangeError")
    } else if name == "SyntaxError" {
        String::from("raiseJavaScriptSyntaxError")
    } else if name == "ReferenceError" {
        String::from("raiseJavaScriptReferenceError")
    } else if name == "URIError" {
        String::from("raiseJavaScriptURIError")
    } else if name == "EvalError" {
        String::from("raiseJavaScriptEvalError")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_class(operation: &str) -> String {
    if operation == "raiseJavaScriptError" {
        String::from("Error")
    } else if operation == "raiseJavaScriptTypeError" {
        String::from("TypeError")
    } else if operation == "raiseJavaScriptRangeError" {
        String::from("RangeError")
    } else if operation == "raiseJavaScriptSyntaxError" {
        String::from("SyntaxError")
    } else if operation == "raiseJavaScriptReferenceError" {
        String::from("ReferenceError")
    } else if operation == "raiseJavaScriptURIError" {
        String::from("URIError")
    } else if operation == "raiseJavaScriptEvalError" {
        String::from("EvalError")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_exception_constructor_operation(name: &str) -> String {
    if (self::read_java_script_source_exception_operation(name)) == "" {
        String::new()
    } else {
        format!("constructAndRaiseJavaScript{}", name.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_exception_constructor_class(operation: &str) -> String {
    if operation == "constructAndRaiseJavaScriptError" {
        String::from("Error")
    } else if operation == "constructAndRaiseJavaScriptTypeError" {
        String::from("TypeError")
    } else if operation == "constructAndRaiseJavaScriptRangeError" {
        String::from("RangeError")
    } else if operation == "constructAndRaiseJavaScriptSyntaxError" {
        String::from("SyntaxError")
    } else if operation == "constructAndRaiseJavaScriptReferenceError" {
        String::from("ReferenceError")
    } else if operation == "constructAndRaiseJavaScriptURIError" {
        String::from("URIError")
    } else if operation == "constructAndRaiseJavaScriptEvalError" {
        String::from("EvalError")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_source_exception(kind: &str, operation: &str) -> bool {
    (kind == "math") && (!(self::read_java_script_source_exception_class(operation)).is_empty())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_absent_message(absence: &str) -> String {
    if absence == "null" {
        String::from("null")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_message_form(kind: &str) -> String {
    if kind == "string" {
        String::from("identity")
    } else if kind == "unit" {
        String::from("empty")
    } else if kind == "option" {
        String::from("optional")
    } else if (((kind == "bool") || (kind == "float")) || (kind == "int")) || (kind == "nat") {
        String::from("text")
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_source_exception_expression(
    target: &str,
    operation: &str,
    message: &str,
    present: &str,
    utf16: bool,
) -> String {
    {
        let name = self::read_java_script_source_exception_class(operation);
        if (name).is_empty() {
            String::new()
        } else if target == "JavaScript" {
            format!(
                "((ml_message,ml_present)=>{{throw new {name}(ml_present?ml_message:undefined);}})({},{})",
                message.to_owned(),
                present.to_owned()
            )
        } else if target == "Rust" {
            format!(
                "(crate::ml_source_exception::SourceException {{ class_name: String::from(\"{name}\"), message_units: {}, message_present: {} }}).raise()",
                if utf16 {
                    format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(")), message.to_owned())),
                        (String::from(").0"))
                    )
                } else {
                    format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(")), message.to_owned())),
                        (String::from(").encode_utf16().collect()"))
                    )
                },
                present.to_owned()
            )
        } else {
            String::new()
        }
    }
}
