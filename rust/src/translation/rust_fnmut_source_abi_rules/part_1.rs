// Generated from complete js/src/translation/rust-fnmut-source-abi-rules.js. Do not edit by hand.
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
pub fn read_rust_fn_mut_source_packet_error(
    modes: &[String],
    formal_count: f64,
    callback: f64,
    captures: f64,
    packet: &[f64],
) -> String {
    if ((((((((!(self::ml_math::is_integer(formal_count)) || (formal_count < 1f64))
        || (formal_count > 256f64))
        || !(self::ml_math::is_integer(captures)))
        || (captures < 0f64))
        || (captures > 256f64))
        || !(self::ml_math::is_integer(callback)))
        || (callback < 0f64))
        || (callback >= formal_count))
        || ((formal_count + captures) != (modes.len() as f64))
    {
        String::from(
            "the expanded source formal inventory adds exactly the actual callback state captures",
        )
    } else {
        {
            let packet_index = 0f64;
            {
                let parameter = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_read_rust_fn_mut_source_packet_error_loop1(
                            modes,
                            formal_count,
                            callback,
                            packet,
                            packet_index,
                            parameter,
                        );
                    match source_ml_source1_s1 {
                        self::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Done(packet_index_2) => {
                            if packet_index_2 == (packet.len() as f64) {
                                String::new()
                            } else {
                                String::from("a FnMut completion publishes the original result then every actual exclusive environment root")
                            }
                        }
                        self::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result {
    MlTemporaryReadRustFnMutSourcePacketErrorLoop1Done(f64),
    MlTemporaryReadRustFnMutSourcePacketErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_fn_mut_source_packet_error_loop1(
    modes: &[String],
    mut formal_count: f64,
    mut callback: f64,
    packet: &[f64],
    mut packet_index: f64,
    mut parameter: f64,
) -> self::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result {
    loop {
        return if parameter < (modes.len() as f64) {
            {
                let mode = self::read_required_text_fact(modes, parameter);
                {
                    let ml_library_inequality_1 = (mode == "callback")
                        != ((parameter).partial_cmp(&(callback))
                            == Some(std::cmp::Ordering::Equal));
                    if ml_library_inequality_1 {
                        self::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Return(String::from("the concrete callback ABI binds exactly its actual original formal"))
                    } else if (parameter >= formal_count) && (mode != "exclusive") {
                        self::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Return(String::from("the callback state ABI retains distinct actual captured lexical roots and their exact packet positions"))
                    } else if (((mode != "callback") && (mode != "value")) && (mode != "shared"))
                        && (mode != "exclusive")
                    {
                        self::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Return(String::from("concrete callback formals retain actual value, shared, exclusive or callback roles"))
                    } else if mode == "exclusive" {
                        if (packet_index >= (packet.len() as f64))
                            || ((self::read_required_number_fact(packet, packet_index))
                                != parameter)
                        {
                            self::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Return(String::from("a FnMut completion publishes the original result then every actual exclusive environment root"))
                        } else {
                            {
                                let packet_index_2 = packet_index + 1f64;
                                {
                                    let parameter_2 = parameter + 1f64;
                                    {
                                        (packet_index, parameter) = (packet_index_2, parameter_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let parameter_3 = parameter + 1f64;
                            {
                                parameter = parameter_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Result::MlTemporaryReadRustFnMutSourcePacketErrorLoop1Done(packet_index)
        };
    }
}
