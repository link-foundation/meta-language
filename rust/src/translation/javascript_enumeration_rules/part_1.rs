// Generated from complete js/src/translation/javascript-enumeration-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Result;
use super::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Result;
use super::source_ml_source1_temporary_read_java_script_guarded_record_projection_positions_loop2;
use super::source_ml_source1_temporary_read_java_script_own_key_callback_position_loop3;
use super::source_ml_source1_temporary_read_java_script_own_key_callback_receiver_position_loop4;
use super::source_ml_source1_temporary_render_array_callback_call_loop1;

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
pub fn read_array_callback_result_form(operation: &str) -> String {
    if operation == "map" {
        String::from("mapped array")
    } else if operation == "filter" {
        String::from("receiver")
    } else if operation == "find" {
        String::from("optional element")
    } else if operation == "findIndex" {
        String::from("float")
    } else if (operation == "some") || (operation == "every") {
        String::from("bool")
    } else if operation == "flatMap" {
        String::from("callback array")
    } else if operation == "forEach" {
        String::from("unit")
    } else if operation == "reduce" {
        String::from("accumulator")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_expected_result_form(operation: &str) -> String {
    if ((((operation == "filter") || (operation == "find")) || (operation == "findIndex"))
        || (operation == "some"))
        || (operation == "every")
    {
        String::from("bool")
    } else if operation == "reduce" {
        String::from("accumulator")
    } else if operation == "flatMap" {
        String::from("array")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_maximum_arguments(operation: &str) -> f64 {
    if operation == "reduce" {
        2f64
    } else if (self::read_array_callback_result_form(operation)) == "" {
        -1f64
    } else {
        1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_signature_error(
    operation: &str,
    kind: &str,
    declared_count: f64,
    provided_count: f64,
    parameters_match: bool,
    result_matches: bool,
    expected_result: &str,
) -> String {
    if kind == "function" {
        if (declared_count > provided_count) || !parameters_match {
            format!(".{}() callback parameters differ", operation.to_owned())
        } else if (!(expected_result).is_empty()) && !result_matches {
            format!(
                ".{}() callback result must be {}",
                operation.to_owned(),
                expected_result.to_owned()
            )
        } else {
            String::new()
        }
    } else {
        format!(".{}() requires a function", operation.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_parameter_role(operation: &str, position: f64) -> String {
    if operation == "reduce" {
        if position == 0f64 {
            String::from("accumulator")
        } else if position == 1f64 {
            String::from("element")
        } else if position == 2f64 {
            String::from("index")
        } else if position == 3f64 {
            String::from("receiver")
        } else {
            String::new()
        }
    } else if position == 0f64 {
        String::from("element")
    } else if position == 1f64 {
        String::from("index")
    } else if position == 2f64 {
        String::from("receiver")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_argument(
    operation: &str,
    position: f64,
    suffix: &str,
    clone_element: bool,
) -> String {
    {
        let role = self::read_array_callback_parameter_role(operation, position);
        if role == "accumulator" {
            format!("ml_accumulator{}", suffix.to_owned())
        } else if role == "element" {
            format!(
                "ml_value{}{}",
                suffix.to_owned(),
                if clone_element {
                    String::from(".clone()")
                } else {
                    String::new()
                }
            )
        } else if role == "index" {
            format!("(ml_index{} as f64)", suffix.to_owned())
        } else if role == "receiver" {
            format!("ml_values{}.clone()", suffix.to_owned())
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_call(
    operation: &str,
    parameter_count: f64,
    suffix: &str,
    clone_element: bool,
) -> String {
    {
        let arguments_text = String::new();
        {
            let position = 0f64;
            {
                let maximum = if operation == "reduce" { 4f64 } else { 3f64 };
                {
                    let arguments_text_2 =
                        self::source_ml_source1_temporary_render_array_callback_call_loop1(
                            operation,
                            parameter_count,
                            suffix,
                            clone_element,
                            arguments_text,
                            position,
                            maximum,
                        );
                    format!("ml_callback{}({arguments_text_2})", suffix.to_owned())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_expression(
    operation: &str,
    object: &str,
    callback: &str,
    parameter_count: f64,
    suffix: &str,
    initial: &str,
    has_initial: bool,
) -> String {
    {
        let values = format!("ml_values{}", suffix.to_owned());
        {
            let index = format!("ml_index{}", suffix.to_owned());
            {
                let value = format!("ml_value{}", suffix.to_owned());
                {
                    let call =
                        self::render_array_callback_call(operation, parameter_count, suffix, false);
                    {
                        let head = format!(
                            "let {values} = {}; let ml_callback{} = {};",
                            object.to_owned(),
                            suffix.to_owned(),
                            callback.to_owned()
                        );
                        {
                            let iterator = format!("{values}.iter().cloned().enumerate()");
                            if (operation == "some") || (operation == "every") {
                                format!(
                                    "{{ {head} {iterator}.{}(|({index}, {value})| {call}) }}",
                                    if operation == "some" {
                                        String::from("any")
                                    } else {
                                        String::from("all")
                                    }
                                )
                            } else if operation == "findIndex" {
                                format!(
                                    "{{ {head} {iterator}.position(|({index}, {value})| {call}).map_or(-1f64, |ml_position| ml_position as f64) }}"
                                )
                            } else if operation == "flatMap" {
                                format!(
                                    "{{ {head} {iterator}.flat_map(|({index}, {value})| {call}).collect::<Vec<_>>() }}"
                                )
                            } else if (operation == "filter") || (operation == "find") {
                                {
                                    let predicate = self::render_array_callback_call(
                                        operation,
                                        parameter_count,
                                        suffix,
                                        true,
                                    );
                                    if operation == "filter" {
                                        format!(
                                            "{{ {head} {iterator}.filter_map(|({index}, {value})| if {predicate} {{ Some({value}) }} else {{ None }}).collect::<Vec<_>>() }}"
                                        )
                                    } else {
                                        format!(
                                            "{{ {head} {iterator}.find_map(|({index}, {value})| if {predicate} {{ Some({value}) }} else {{ None }}) }}"
                                        )
                                    }
                                }
                            } else if operation == "forEach" {
                                format!(
                                    "{{ {head} for ({index}, {value}) in {iterator} {{ {call}; }} () }}"
                                )
                            } else if operation == "reduce" {
                                if has_initial {
                                    format!(
                                        "{{ {head} {iterator}.fold({}, |ml_accumulator{}, ({index}, {value})| {call}) }}",
                                        initial.to_owned(),
                                        suffix.to_owned()
                                    )
                                } else {
                                    format!(
                                        "{{ {head} let mut ml_iterator{} = {iterator}; let ml_initial{} = ml_iterator{}.next().map(|(_, ml_first)| ml_first).expect(\"Reduce of empty array with no initial value\"); ml_iterator{}.fold(ml_initial{}, |ml_accumulator{}, ({index}, {value})| {call}) }}",
                                        suffix.to_owned(),
                                        suffix.to_owned(),
                                        suffix.to_owned(),
                                        suffix.to_owned(),
                                        suffix.to_owned(),
                                        suffix.to_owned()
                                    )
                                }
                            } else {
                                String::new()
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_argument_count_error(operation: &str, count: f64) -> String {
    {
        let maximum = self::read_array_callback_maximum_arguments(operation);
        if (maximum < 0f64) || ((count >= 1f64) && (count <= maximum)) {
            String::new()
        } else {
            {
                let expected = if maximum == 1f64 {
                    String::from("1")
                } else {
                    String::from("1 to 2")
                };
                format!(".{}() expects {expected} arguments", operation.to_owned())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_checked_result_context(operation: &str) -> String {
    {
        let expected = self::read_array_callback_expected_result_form(operation);
        if expected == "array" {
            String::new()
        } else {
            expected
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_result_error(operation: &str, result_kind: &str) -> String {
    if (operation == "flatMap") && (result_kind != "array") {
        String::from("flatMap callback must return an array")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_checked_argument_position(
    operation: &str,
    has_initial: bool,
    position: f64,
) -> f64 {
    if (operation == "reduce") && has_initial {
        if position == 0f64 {
            1f64
        } else if position == 1f64 {
            0f64
        } else {
            -1f64
        }
    } else if position == 0f64 {
        0f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_reference_expression(
    operation: &str,
    object: &str,
    callback: &str,
    parameter_count: f64,
    suffix: &str,
    reference_result: bool,
) -> String {
    if (operation != "flatMap") || !reference_result {
        String::new()
    } else {
        {
            let values = format!("ml_values{}", suffix.to_owned());
            {
                let index = format!("ml_index{}", suffix.to_owned());
                {
                    let value = format!("ml_value{}", suffix.to_owned());
                    {
                        let result = format!("ml_callback_values{}", suffix.to_owned());
                        {
                            let call = self::render_array_callback_call(
                                operation,
                                parameter_count,
                                suffix,
                                false,
                            );
                            {
                                let head = format!(
                                    "let {values} = {}; let ml_callback{} = {};",
                                    object.to_owned(),
                                    suffix.to_owned(),
                                    callback.to_owned()
                                );
                                {
                                    let iterator = format!("{values}.iter().cloned().enumerate()");
                                    format!(
                                        "{{ {head} std::rc::Rc::new(std::cell::RefCell::new({iterator}.flat_map(|({index}, {value})| {{ let {result} = {call}.borrow().clone(); {result} }}).collect::<Vec<_>>())) }}"
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
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
                    self::source_ml_source1_temporary_read_java_script_guarded_record_projection_positions_loop2(result, index)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_own_key_callback_position(operation: &str) -> f64 {
    if (self::read_array_callback_result_form(operation)) == "" {
        -1f64
    } else {
        {
            let position = 0f64;
            {
                let source_ml_source2_s1 = self::source_ml_source1_temporary_read_java_script_own_key_callback_position_loop3(operation, position);
                match source_ml_source2_s1 {
                    self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Result::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Done => {
                        -1f64
                    }
                    self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Result::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_own_key_callback_receiver_position(operation: &str) -> f64 {
    if (self::read_array_callback_result_form(operation)) == "" {
        -1f64
    } else {
        {
            let position = 0f64;
            {
                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_java_script_own_key_callback_receiver_position_loop4(operation, position);
                match source_ml_source2_s2 {
                    self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Result::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Done => {
                        -1f64
                    }
                    self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Result::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Return(source_ml_source1_temporary_result4) => {
                        source_ml_source1_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_java_script_own_key_callback_source(
    expression: &str,
    operation: &str,
    count: f64,
    spread: bool,
    local: bool,
    owner: &str,
    open: bool,
    callback: &str,
) -> bool {
    (((((((expression == "math") && (operation == "enumerateJavaScriptObjectKeys"))
        && (count == 1f64))
        && !spread)
        && local)
        && (owner == "record"))
        && !open)
        && (callback == "lambda")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_own_key_callback_binding(
    owner_shadowed: bool,
    receiver_exposed: bool,
    key: &str,
) -> bool {
    (!owner_shadowed && !receiver_exposed) && (key == "string")
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
    } else if (operation == "rebaseJavaScriptRecordPropertyOrder")
        || (operation == "copyJavaScriptRecordPropertyOrder")
    {
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
            || (((operation == "rebaseJavaScriptRecordPropertyOrder")
                || (operation == "copyJavaScriptRecordPropertyOrder"))
                && (index == 1f64))
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
