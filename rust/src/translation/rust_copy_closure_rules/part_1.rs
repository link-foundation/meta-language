// Generated from complete js/src/translation/rust-copy-closure-rules.js. Do not edit by hand.
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
pub fn read_rust_copy_closure_header_error(
    ordinary: bool,
    names: &[String],
    copies: &[bool],
    result_written: bool,
    result_copy: bool,
) -> String {
    if ordinary {
        if ((names.len() as f64) != (copies.len() as f64)) || ((names.len() as f64) > 1024f64) {
            String::from("the complete typed closure header exceeds its source budget")
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_read_rust_copy_closure_header_error_loop1(
                            names, copies, index,
                        );
                    match source_ml_source1_s1 {
                        self::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Done => {
                            if result_written {if result_copy {
                                    String::new()
                                } else {
                                    String::from("the closure result requires a direct immutable Copy source type")
                                }} else {
                                String::from("the closure result requires its actual explicit type")
                            }
                        }
                        self::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    } else {
        String::from("closure ownership requires its separate active source profile")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_capture_error(
    collecting: bool,
    copies: &[bool],
    assigned: f64,
) -> String {
    {
        let ml_library_inequality_1 = assigned != 0f64;
        if ml_library_inequality_1 {
            String::from("mutable closure captures require their shared ownership interpretation")
        } else {
            {
                let source_ml_source1_s3 = if collecting {
                    self::MlTemporaryReadRustCopyClosureCaptureErrorJoin3::MlTemporaryReadRustCopyClosureCaptureErrorJoin3Next
                } else {
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_s4 = self::source_ml_temporary_read_rust_copy_closure_capture_error_loop4(copies, index);
                            match source_ml_source1_s4 {
                            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Done => {
                                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin3::MlTemporaryReadRustCopyClosureCaptureErrorJoin3Next
                            }
                            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Return(source_ml_temporary_result4) => {
                                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin3::MlTemporaryReadRustCopyClosureCaptureErrorJoin3Return(source_ml_temporary_result4)
                            }
                        }
                        }
                    }
                };
                match source_ml_source1_s3 {
                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin3::MlTemporaryReadRustCopyClosureCaptureErrorJoin3Next => {
                    String::new()
                }
                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin3::MlTemporaryReadRustCopyClosureCaptureErrorJoin3Return(source_ml_temporary_result3) => {
                    source_ml_temporary_result3
                }
            }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_copy_closure_invocation(kind: &str) -> bool {
    kind == "function"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_join_error(first: bool, second: bool, same: bool) -> String {
    if (first || second) && !same {
        String::from("distinct anonymous closure types require an actual source coercion")
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureHeaderErrorLoop1Result {
    MlTemporaryReadRustCopyClosureHeaderErrorLoop1Done,
    MlTemporaryReadRustCopyClosureHeaderErrorLoop1Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureHeaderErrorLoop2Result {
    MlTemporaryReadRustCopyClosureHeaderErrorLoop2Done,
    MlTemporaryReadRustCopyClosureHeaderErrorLoop2Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_copy_closure_header_error_loop2(
    names: &[String],
    mut index: f64,
    mut before: f64,
) -> self::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Result {
    loop {
        return if before < index {
            if (self::read_required_text_fact(names, before))
                == (self::read_required_text_fact(names, index))
            {
                self::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Return(String::from("closure parameters require distinct actual bindings"))
            } else {
                {
                    let before_2 = before + 1f64;
                    {
                        before = before_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_copy_closure_header_error_loop1(
    names: &[String],
    copies: &[bool],
    mut index: f64,
) -> self::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Result {
    loop {
        return if index < (names.len() as f64) {
            if (self::read_required_boolean_fact(copies, index)) {
                {
                    let before = 0f64;
                    {
                        let source_ml_source1_s5 =
                            self::source_ml_temporary_read_rust_copy_closure_header_error_loop2(
                                names, index, before,
                            );
                        match source_ml_source1_s5 {
                            self::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Done => {
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        index = index_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop2Return(source_ml_temporary_result2) => {
                                self::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Return(source_ml_temporary_result2)
                            }
                        }
                    }
                }
            } else {
                self::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Return(String::from("closure parameters require direct immutable Copy source types"))
            }
        } else {
            self::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Result::MlTemporaryReadRustCopyClosureHeaderErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureCaptureErrorJoin3 {
    MlTemporaryReadRustCopyClosureCaptureErrorJoin3Next,
    MlTemporaryReadRustCopyClosureCaptureErrorJoin3Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureCaptureErrorLoop4Result {
    MlTemporaryReadRustCopyClosureCaptureErrorLoop4Done,
    MlTemporaryReadRustCopyClosureCaptureErrorLoop4Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_copy_closure_capture_error_loop4(
    copies: &[bool],
    mut index: f64,
) -> self::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Result {
    loop {
        return if index < (copies.len() as f64) {
            if (self::read_required_boolean_fact(copies, index)) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Return(String::from("closure captures require proven direct immutable Copy source values"))
            }
        } else {
            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop4Done
        };
    }
}
