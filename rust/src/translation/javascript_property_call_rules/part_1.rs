// Generated from complete js/src/translation/javascript-property-call-rules.js. Do not edit by hand.
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
pub fn read_java_script_source_property_operation_kind(operation: &str) -> String {
    if (operation == "indexOf") || (operation == "lastIndexOf") {
        String::from("index")
    } else if ((operation == "includes") || (operation == "startsWith"))
        || (operation == "endsWith")
    {
        String::from("test")
    } else if ((((operation == "trim") || (operation == "trimStart")) || (operation == "trimEnd"))
        || (operation == "toLowerCase"))
        || (operation == "toUpperCase")
    {
        String::from("map")
    } else if operation == "toString" {
        String::from("text")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_resolved_property_call_form(receiver: &str, operation: &str) -> String {
    {
        let kind = self::read_java_script_source_property_operation_kind(operation);
        if (kind).is_empty() {
            String::new()
        } else if receiver == "record" {
            String::from("record")
        } else if kind == "text" {
            String::from("text")
        } else if receiver == "string" {
            kind
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_method_inference_step(
    kind: &str,
    parameters: f64,
    arguments_count: f64,
) -> String {
    if kind == "var" {
        String::from("infer")
    } else if (kind == "function")
        && ((parameters).partial_cmp(&(arguments_count)) == Some(std::cmp::Ordering::Equal))
    {
        String::from("apply")
    } else {
        String::from("checked error")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_call_fallback_kind(operation: &str, selected: &str) -> String {
    if operation == "includes" {
        String::from("string")
    } else if operation == "toString" {
        String::from("scalar")
    } else {
        selected.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_concatenation_fallback_kind(operation: &str, selected: &str) -> String {
    if operation == "concat" {
        String::from("array")
    } else {
        self::read_java_script_property_call_fallback_kind(operation, selected)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_concatenation_argument_role(
    receiver: &str,
    argument: &str,
    force: bool,
) -> String {
    if (argument == "var") && !force {
        String::from("defer")
    } else if receiver == "array" {
        if argument == "array" {
            String::from("spread")
        } else {
            String::from("item")
        }
    } else {
        let ml_library_inequality_1 = receiver != "string";
        if ml_library_inequality_1 {
            String::new()
        } else if self::ml_array::includes(
            &vec![
                String::from("string"),
                String::from("float"),
                String::from("bool"),
                String::from("int"),
                String::from("nat"),
                String::from("fixed"),
                String::from("unit"),
                String::from("option"),
            ],
            &argument.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ) {
            String::from("text")
        } else {
            String::from("unsupported")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_text_form(kind: &str) -> String {
    if kind == "string" {
        String::from("identity")
    } else if kind == "unit" {
        String::from("undefined")
    } else if kind == "option" {
        String::from("optional")
    } else if ((((kind == "float") || (kind == "bool")) || (kind == "int")) || (kind == "nat"))
        || (kind == "fixed")
    {
        String::from("convert")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_search_default_position(operation: &str) -> String {
    if operation == "endsWith" {
        String::from("Infinity")
    } else if operation == "lastIndexOf" {
        String::from("NaN")
    } else {
        String::from("0")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_string_search_window(operation: &str) -> Vec<String> {
    if operation == "endsWith" {
        vec![String::from("0"), String::from("position")]
    } else {
        vec![String::from("position"), String::from("Infinity")]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_search_result_kind(operation: &str) -> String {
    if (operation == "indexOf") || (operation == "lastIndexOf") {
        String::from("float")
    } else {
        String::from("bool")
    }
}
