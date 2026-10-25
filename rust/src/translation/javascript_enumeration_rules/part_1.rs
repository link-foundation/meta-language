// Generated from complete js/src/translation/javascript-enumeration-rules.js. Do not edit by hand.
/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
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
}

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_own_property_builtin_operation(name: &str) -> String {
    if name == "Object.hasOwn" {
        String::from("testJavaScriptOwnedProperty")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_own_property_receiver_form(kind: &str, element: &str) -> String {
    if (kind == "unit") || ((kind == "option") && (element == "unit")) {
        String::from("absent")
    } else if kind == "option" {
        String::from("guarded")
    } else if kind == "record" {
        String::from("record")
    } else if ((kind == "array") || (kind == "tuple")) || (kind == "string") {
        String::from("indexed")
    } else if (((((kind == "bool") || (kind == "float")) || (kind == "int")) || (kind == "nat"))
        || (kind == "map"))
        || (kind == "set")
    {
        String::from("empty")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_coercion_error() -> String {
    String::from("Cannot convert undefined or null to object")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_property_index_limit() -> String {
    String::from("4294967295")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_own_property_guard_arm(kind: &str, operation: &str, negated: bool) -> f64 {
    if (kind == "math") && (operation == "testJavaScriptOwnedProperty") {
        if negated { -1f64 } else { 1f64 }
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_own_property_guard_bindings(
    count: f64,
    spread: bool,
    local_object: bool,
    local_key: bool,
    object: &str,
    key: &str,
    open: bool,
) -> bool {
    ((((((count == 2f64) && !spread) && local_object) && local_key) && (object == "record"))
        && (key == "string"))
        && !open
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_guarded_record_index(
    object: &str,
    open: bool,
    key: &str,
    guarded: bool,
) -> bool {
    (((object == "record") && !open) && (key == "string")) && guarded
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_guarded_record_projection_positions(count: f64) -> Vec<f64> {
    if (!(self::ml_math::is_safe_integer(count)) || (count < 0f64)) || (count > 100_000f64) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid actual guarded record field count"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let index = count - 1f64;
                {
                    self::source_ml_temporary_read_java_script_guarded_record_projection_positions_loop1(result, index)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_own_enumeration_receiver_form(kind: &str, element: &str) -> String {
    if kind == "fixed" {
        String::from("empty")
    } else {
        self::read_java_script_own_property_receiver_form(kind, element)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_enumeration_builtin_operation(name: &str) -> String {
    if name == "Object.keys" {
        String::from("enumerateJavaScriptObjectKeys")
    } else if name == "Object.values" {
        String::from("enumerateJavaScriptObjectValues")
    } else if name == "Object.entries" {
        String::from("enumerateJavaScriptObjectEntries")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_enumeration_form(operation: &str) -> String {
    if operation == "enumerateJavaScriptObjectKeys" {
        String::from("keys")
    } else if operation == "enumerateJavaScriptObjectValues" {
        String::from("values")
    } else if operation == "enumerateJavaScriptObjectEntries" {
        String::from("entries")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_enumeration_value_domain(form: &str) -> bool {
    (form == "values") || (form == "entries")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_entry_pairing_operation() -> String {
    String::from("pairJavaScriptObjectEntryValues")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_kernel_arity(operation: &str) -> f64 {
    if (((operation == "readJavaScriptRecordPropertyIndex")
        || (operation == (self::read_java_script_object_entry_pairing_operation())))
        || (operation == "reorderJavaScriptRecordPropertyValues"))
        || (((((operation == "createJavaScriptRecordPropertyOrder")
            || (operation == "mergeJavaScriptRecordPropertyOrder"))
            || (operation == "readJavaScriptRecordPropertyKeys"))
            || (operation == "acceptJavaScriptRecordPropertyOrder"))
            || (operation == "readJavaScriptRecordPropertyKeyTexts"))
    {
        2f64
    } else if operation == "rebaseJavaScriptRecordPropertyOrder" {
        3f64
    } else if (operation == "createJavaScriptDensePropertyKeyTexts")
        || (operation == "createJavaScriptStringOwnPropertyValues")
    {
        1f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_kernel_parameter(operation: &str, index: f64) -> String {
    {
        let count = self::read_java_script_record_order_kernel_arity(operation);
        if (!(self::ml_math::is_integer(index)) || (index < 0f64)) || (index >= count) {
            String::new()
        } else if operation == "readJavaScriptRecordPropertyIndex" {
            if index == 0f64 {
                String::from("name rows")
            } else {
                String::from("text")
            }
        } else if operation == (self::read_java_script_object_entry_pairing_operation()) {
            if index == 0f64 {
                String::from("key texts")
            } else {
                String::from("values")
            }
        } else if operation == "reorderJavaScriptRecordPropertyValues" {
            if index == 0f64 {
                String::from("values")
            } else {
                String::from("positions")
            }
        } else if operation == "createJavaScriptDensePropertyKeyTexts" {
            String::from("number")
        } else if operation == "createJavaScriptStringOwnPropertyValues" {
            String::from("text")
        } else if ((index == 0f64) || (operation == "mergeJavaScriptRecordPropertyOrder"))
            || ((operation == "rebaseJavaScriptRecordPropertyOrder") && (index == 1f64))
        {
            String::from("name rows")
        } else {
            String::from("positions")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_kernel_result(operation: &str) -> String {
    {
        let ml_product_condition_1 = {
            let ml_value = self::read_java_script_record_order_kernel_arity(operation);
            ml_value != 0f64 && !ml_value.is_nan()
        };
        if ml_product_condition_1 {
            if operation == "readJavaScriptRecordPropertyIndex" {
                String::from("number")
            } else if operation == (self::read_java_script_object_entry_pairing_operation()) {
                String::from("entries")
            } else if operation == "reorderJavaScriptRecordPropertyValues" {
                String::from("values")
            } else if operation == "readJavaScriptRecordPropertyKeys" {
                String::from("name rows")
            } else if operation == "acceptJavaScriptRecordPropertyOrder" {
                String::from("boolean")
            } else if ((operation == "readJavaScriptRecordPropertyKeyTexts")
                || (operation == "createJavaScriptDensePropertyKeyTexts"))
                || (operation == "createJavaScriptStringOwnPropertyValues")
            {
                String::from("key texts")
            } else {
                String::from("positions")
            }
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_ordered_record_order_read(
    expression: &str,
    operation: &str,
    kind: &str,
    ordered: bool,
    reference: bool,
) -> bool {
    ((((expression == "method") && (operation == "readJavaScriptRecordPropertyOrder"))
        && (kind == "record"))
        && ordered)
        && !reference
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_guarded_record_projection_positions_loop1(
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index >= 0f64 {
            {
                let result_2 = {
                    let source_ml_source1_concat_value1 = result;
                    {
                        let source_ml_source1_concat_value2 = vec![index];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source1_concat_value1,
                            ),
                            source_ml_source1_concat_value2,
                        )
                    }
                };
                {
                    let index_2 = index - 1f64;
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}
