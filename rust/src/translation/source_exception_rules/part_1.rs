// Generated from complete js/src/translation/source-exception-rules.js. Do not edit by hand.
/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_exception_ignored_pattern_fields(fields: f64) -> String {
    if fields == 4f64 {
        String::from(" _ _")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_exception_payload_fields(
    output: bool,
    names: &[String],
    kinds: &[String],
) -> bool {
    (((((((((output && ((names.len() as f64) == 4f64)) && ((kinds.len() as f64) == 4f64))
        && match (
            self::ml_array::optional_at(names, self::ml_array::number_index(0f64)),
            String::from("output"),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(0f64)),
            String::from("output"),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(names, self::ml_array::number_index(1f64)),
            String::from("message"),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(1f64)),
            String::from("string"),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(names, self::ml_array::number_index(2f64)),
            String::from("className"),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(2f64)),
            String::from("string"),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(names, self::ml_array::number_index(3f64)),
            String::from("messagePresent"),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(kinds, self::ml_array::number_index(3f64)),
            String::from("bool"),
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_exception_observation_statement() -> String {
    String::from(
        "ml_source_exception_observation retains the source exception constructor class, exact UTF-16 message and own-message property presence; main projects the display message at its ordinary process boundary",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_exception_rocq_output_prelude() -> String {
    String::from(
        "Definition ml_emit_source_exception {E : Type} (lines : list string) (next : list string * option E) : list string * option E := (List.app lines (fst next), snd next).",
    )
}
