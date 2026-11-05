// Generated from complete js/src/translation/ordered-record-layout-rules.js. Do not edit by hand.
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
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Number fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required text fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Boolean fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Number fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required text fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Boolean fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn join_record_items(items: &[String], separator: &str) -> String {
    {
        let text = String::new();
        {
            let index = 0f64;
            {
                self::source_ml_source1_temporary_join_record_items_loop1(
                    items, separator, text, index,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_value(target: &str, labels: &[String], items: &[String]) -> String {
    if target == "JavaScript" {
        {
            let entries = Vec::<String>::new();
            {
                let index = 0f64;
                {
                    let entries_2 = self::source_ml_source1_temporary_render_record_value_loop2(
                        labels, items, entries, index,
                    );
                    format!("({{{}}})", self::join_record_items(&(entries_2), ", "))
                }
            }
        }
    } else if target == "Rust" {
        format!(
            "({}{})",
            self::join_record_items(items, ", "),
            if (items.len() as f64) == 1f64 {
                String::from(",")
            } else {
                String::new()
            }
        )
    } else if target == "Lean" {
        if (items.len() as f64) > 0f64 {
            format!("({})", self::join_record_items(items, ", "))
        } else {
            String::from("()")
        }
    } else if target == "Rocq" {
        if (items.len() as f64) > 0f64 {
            format!("({})", self::join_record_items(items, ", "))
        } else {
            String::from("tt")
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_field(target: &str, object: &str, field: &str, index: &str) -> String {
    if target == "JavaScript" {
        format!("({})[{}]", object.to_owned(), field.to_owned())
    } else if target == "Rust" {
        format!("({}).{}", object.to_owned(), index.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_product_field(target: &str, object: &str, index: f64, count: f64) -> String {
    if target == "Lean" {
        if count == 1f64 {
            object.to_owned()
        } else {
            {
                let suffix = String::new();
                {
                    let position = 0f64;
                    {
                        let suffix_2 =
                            self::source_ml_source1_temporary_render_record_product_field_loop3(
                                index, suffix, position,
                            );
                        format!(
                            "({}){suffix_2}{}",
                            object.to_owned(),
                            if index < (count - 1f64) {
                                String::from(".1")
                            } else {
                                String::new()
                            }
                        )
                    }
                }
            }
        }
    } else if target == "Rocq" {
        {
            let value = object.to_owned();
            {
                let position_2 = count - 1f64;
                {
                    let value_2 =
                        self::source_ml_source1_temporary_render_record_product_field_loop4(
                            index, value, position_2,
                        );
                    if index == 0f64 {
                        value_2
                    } else {
                        format!("(snd {value_2})")
                    }
                }
            }
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_type(target: &str, fields: &[String]) -> String {
    if target == "Lean" {
        if (fields.len() as f64) > 0f64 {
            format!("({})", self::join_record_items(fields, " × "))
        } else {
            String::from("Unit")
        }
    } else if target == "Rocq" {
        if (fields.len() as f64) > 0f64 {
            format!("({})%type", self::join_record_items(fields, " * "))
        } else {
            String::from("unit")
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_ordered_record_number_storage(kind: &str, ordered: bool) -> bool {
    (kind == "record") && ordered
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_ordered_record_type(target: &str, payload: &str, order: &str) -> String {
    if target == "Rust" {
        format!("({}, {})", payload.to_owned(), order.to_owned())
    } else if target == "Lean" {
        format!("({} × {})", payload.to_owned(), order.to_owned())
    } else if target == "Rocq" {
        format!("(({} * {})%type)", payload.to_owned(), order.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_ordered_record_payload(target: &str, object: &str) -> String {
    if target == "JavaScript" {
        object.to_owned()
    } else if target == "Rust" {
        format!("({}).0", object.to_owned())
    } else if target == "Lean" {
        format!("({}).1", object.to_owned())
    } else if target == "Rocq" {
        format!("(fst {})", object.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_ordered_record_order(target: &str, object: &str) -> String {
    if target == "Rust" {
        format!("({}).1.clone()", object.to_owned())
    } else if target == "Lean" {
        format!("({}).2", object.to_owned())
    } else if target == "Rocq" {
        format!("(snd {})", object.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_ordered_record_value(
    target: &str,
    labels: &[String],
    values: &[String],
    order: &str,
) -> String {
    {
        let payload = self::render_record_value(target, labels, values);
        if ((target == "Rust") || (target == "Lean")) || (target == "Rocq") {
            format!("({payload}, {})", order.to_owned())
        } else if target == "JavaScript" {
            format!(
                "((ml_fields, ml_order) => {{ const ml_object = {{}}; const ml_names = [{}]; for (const ml_field of ml_order) {{ Object.defineProperty(ml_object, ml_names[ml_field], {{ value: ml_fields[ml_field], writable: true, enumerable: true, configurable: true }}); }} return ml_object; }})([{}], {})",
                labels.join((String::from(", ")).as_str()),
                values.join((String::from(", ")).as_str()),
                order.to_owned()
            )
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_ordered_record_observed_order(
    target: &str,
    object: &str,
    labels: &[String],
) -> String {
    if target == "JavaScript" {
        format!(
            "((ml_object) => {{ const ml_names = [{}]; return Object.keys(ml_object).map(ml_name => ml_names.indexOf(ml_name)); }})({})",
            labels.join((String::from(", ")).as_str()),
            object.to_owned()
        )
    } else {
        self::render_ordered_record_order(target, object)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_property_order_kernel_entry(operation: &str) -> String {
    if operation == "createJavaScriptRecordPropertyOrder" {
        String::from("create")
    } else if operation == "mergeJavaScriptRecordPropertyOrder" {
        String::from("merge")
    } else if operation == "rebaseJavaScriptRecordPropertyOrder" {
        String::from("rebase")
    } else if operation == "readJavaScriptRecordPropertyKeys" {
        String::from("keys")
    } else if operation == "acceptJavaScriptRecordPropertyOrder" {
        String::from("valid")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_property_order_kernel_call(
    target: &str,
    operation: &str,
    args: &[String],
) -> String {
    {
        let entry = self::read_record_property_order_kernel_entry(operation);
        if (!entry.is_empty()) {
            if target == "JavaScript" {
                format!(
                    "ml_record_property_order.{entry}({})",
                    args.join((String::from(", ")).as_str())
                )
            } else if target == "Lean" {
                format!(
                    "(MLRecordPropertyOrder.{entry} {})",
                    args.join((String::from(" ")).as_str())
                )
            } else if target == "Rocq" {
                format!(
                    "(MLRecordPropertyOrder.{entry} {})",
                    args.join((String::from(" ")).as_str())
                )
            } else {
                let ml_library_inequality_1 = target != "Rust";
                if ml_library_inequality_1 {
                    String::new()
                } else {
                    {
                        let name = if entry == "create" {
                            String::from("create_java_script_record_property_order")
                        } else if entry == "merge" {
                            String::from("merge_java_script_record_property_order")
                        } else if entry == "rebase" {
                            String::from("rebase_java_script_record_property_order")
                        } else if entry == "keys" {
                            String::from("read_java_script_record_property_keys")
                        } else {
                            String::from("accept_java_script_record_property_order")
                        };
                        format!(
                            "crate::ml_record_property_order::{name}({})",
                            ({
                                let ml_values1 = args.to_vec();
                                let ml_callback1 =
                                    std::rc::Rc::new(move |value: String| -> String {
                                        format!(
                                            "{}{}",
                                            (format!("{}{}", (String::from("&(")), value)),
                                            (String::from(")"))
                                        )
                                    });
                                ml_values1
                                    .iter()
                                    .cloned()
                                    .enumerate()
                                    .map(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
                                    .collect::<Vec<_>>()
                            })
                            .join((String::from(", ")).as_str())
                        )
                    }
                }
            }
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_join_record_items_loop1(
    items: &[String],
    separator: &str,
    mut text: String,
    mut index: f64,
) -> String {
    loop {
        if index < (items.len() as f64) {
            {
                let text_3 = if index > 0f64 {
                    { format!("{text}{}", separator.to_owned()) }
                } else {
                    text
                };
                {
                    let text_4 = format!("{text_3}{}", self::read_required_text_fact(items, index));
                    {
                        let index_2 = index + 1f64;
                        {
                            (text, index) = (text_4, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { text };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_record_value_loop2(
    labels: &[String],
    items: &[String],
    mut entries: Vec<String>,
    mut index: f64,
) -> Vec<String> {
    loop {
        if index < (items.len() as f64) {
            {
                let entries_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![format!(
                            "[{}]: {}",
                            self::read_required_text_fact(labels, index),
                            self::read_required_text_fact(items, index)
                        )],
                        entries,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (entries, index) = (entries_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { entries };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_record_product_field_loop3(
    mut index: f64,
    mut suffix: String,
    mut position: f64,
) -> String {
    loop {
        if position < index {
            {
                let suffix_2 = format!("{suffix}.2");
                {
                    let position_2 = position + 1f64;
                    {
                        (suffix, position) = (suffix_2, position_2);
                        continue;
                    }
                }
            }
        }
        return { suffix };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_record_product_field_loop4(
    mut index: f64,
    mut value: String,
    mut position: f64,
) -> String {
    loop {
        if position > index {
            {
                let value_2 = format!("(fst {value})");
                {
                    let position_2 = position - 1f64;
                    {
                        (value, position) = (value_2, position_2);
                        continue;
                    }
                }
            }
        }
        return { value };
    }
}
