// Generated from complete js/src/translation/record-value-order-template.js. Do not edit by hand.
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

#[derive(Clone, Debug, PartialEq)]
pub enum OpaqueValue {
    WrappedValue(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum WrappedEntry {
    WrappedEntry(String, Box<self::OpaqueValue>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_order_position() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("TypeError"),
        message_units: (String::from("invalid ordered record property order"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_opaque_value() -> self::OpaqueValue {
    (self::ml_source_exception::SourceException {
        class_name: String::from("TypeError"),
        message_units: (String::from("invalid ordered record property order"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_order_position(order: &[f64], index: f64) -> f64 {
    (self::ml_array::optional_at(order, self::ml_array::number_index(index)))
        .unwrap_or_else(self::refuse_order_position)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_opaque_value(values: Vec<self::OpaqueValue>, index: f64) -> self::OpaqueValue {
    (self::ml_array::optional_at(&values, self::ml_array::number_index(index)))
        .unwrap_or_else(self::refuse_opaque_value)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_entry_key() -> String {
    (self::ml_source_exception::SourceException {
        class_name: String::from("TypeError"),
        message_units: (String::from("invalid own entry key and value sequence"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_entry_key(keys: &[String], index: f64) -> String {
    (self::ml_array::optional_at(keys, self::ml_array::number_index(index)))
        .unwrap_or_else(self::refuse_entry_key)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn reorder_wrapped_record_property_values(
    values: Vec<self::OpaqueValue>,
    order: &[f64],
) -> Vec<self::OpaqueValue> {
    if (values.len() as f64) == (order.len() as f64) {
        {
            let result = Vec::<self::OpaqueValue>::new();
            {
                let seen = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        self::source_ml_temporary_reorder_wrapped_record_property_values_loop1(
                            values, order, result, seen, index,
                        )
                    }
                }
            }
        }
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (String::from("invalid ordered record property order"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn pair_wrapped_own_property_values(
    keys: &[String],
    values: Vec<self::OpaqueValue>,
) -> Vec<self::WrappedEntry> {
    if (keys.len() as f64) == (values.len() as f64) {
        {
            let result = Vec::<self::WrappedEntry>::new();
            {
                let index = 0f64;
                {
                    self::source_ml_temporary_pair_wrapped_own_property_values_loop2(
                        keys, values, result, index,
                    )
                }
            }
        }
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (String::from("invalid own entry key and value sequence"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_reorder_wrapped_record_property_values_loop1(
    mut values: Vec<self::OpaqueValue>,
    order: &[f64],
    mut result: Vec<self::OpaqueValue>,
    mut seen: Vec<f64>,
    mut index: f64,
) -> Vec<self::OpaqueValue> {
    loop {
        return if index < (order.len() as f64) {
            {
                let position = self::read_order_position(order, index);
                if (((!(self::ml_math::is_integer(position)) || (position < 0f64))
                    || (position >= (values.len() as f64)))
                    || ((position == 0f64) && ((1f64 / position) < 0f64)))
                    || self::ml_array::includes(
                        &seen,
                        &position,
                        0f64,
                        false,
                        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                    )
                {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (String::from("invalid ordered record property order"))
                            .encode_utf16()
                            .collect(),
                        message_present: true,
                    })
                    .raise()
                } else {
                    {
                        let seen_2 = {
                            let source_ml_source1_concat_value1 = seen;
                            {
                                let source_ml_source1_concat_value2 = vec![position];
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
                            let result_2 = {
                                let source_ml_source1_concat_value3 = result;
                                {
                                    let source_ml_source1_concat_value4 =
                                        vec![self::read_opaque_value(values.clone(), position)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::OpaqueValue>::new(),
                                            source_ml_source1_concat_value3,
                                        ),
                                        source_ml_source1_concat_value4,
                                    )
                                }
                            };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (result, seen, index) = (result_2, seen_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_pair_wrapped_own_property_values_loop2(
    keys: &[String],
    mut values: Vec<self::OpaqueValue>,
    mut result: Vec<self::WrappedEntry>,
    mut index: f64,
) -> Vec<self::WrappedEntry> {
    loop {
        if index < (keys.len() as f64) {
            {
                let key = self::read_entry_key(keys, index);
                {
                    let value = self::read_opaque_value(values.clone(), index);
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value5 = result;
                            {
                                let source_ml_source1_concat_value6 =
                                    vec![self::WrappedEntry::WrappedEntry(key, Box::new(value))];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::WrappedEntry>::new(),
                                        source_ml_source1_concat_value5,
                                    ),
                                    source_ml_source1_concat_value6,
                                )
                            }
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}
