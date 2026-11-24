// Generated from complete js/src/translation/rust-copy-closure-rules.js. Do not edit by hand.
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
pub fn read_rust_copy_closure_parameter_form(role: &str) -> String {
    if role == "_" {
        String::from("wildcard")
    } else if role == "(" {
        String::from("product")
    } else if role == "binding" {
        String::from("binding")
    } else {
        String::from("unsupported")
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
    {
        let error = self::read_rust_copy_closure_parameter_header_error(ordinary, names, copies);
        if error.is_empty() {
            if result_written {
                if result_copy {
                    String::new()
                } else {
                    String::from("the closure result requires a direct immutable Copy source type")
                }
            } else {
                String::from("the closure result requires its actual explicit type")
            }
        } else {
            error
        }
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
                let source_ml_source1_s2 = if collecting {
                    self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Next
                } else {
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_s3 = self::source_ml_temporary_read_rust_copy_closure_capture_error_loop2(copies, index);
                            match source_ml_source1_s3 {
                            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Done => {
                                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Next
                            }
                            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Return(source_ml_temporary_result2) => {
                                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Return(source_ml_temporary_result2)
                            }
                        }
                        }
                    }
                };
                match source_ml_source1_s2 {
                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Next => {
                    String::new()
                }
                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Return(source_ml_temporary_result1) => {
                    source_ml_temporary_result1
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_parameter_header_error(
    ordinary: bool,
    names: &[String],
    copies: &[bool],
) -> String {
    if ordinary {
        if ((names.len() as f64) != (copies.len() as f64)) || ((names.len() as f64) > 1024f64) {
            String::from("the complete typed closure header exceeds its source budget")
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s4 = self::source_ml_temporary_read_rust_copy_closure_parameter_header_error_loop3(names, copies, index);
                    match source_ml_source1_s4 {
                        self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Done => {
                            String::new()
                        }
                        self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Return(source_ml_temporary_result3) => {
                            source_ml_temporary_result3
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
pub const fn test_rust_copy_closure_deferred_result(
    requested: bool,
    written: bool,
    known: bool,
) -> bool {
    (requested && !written) && !known
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_deferred_source_error(
    kind: &str,
    parameters: f64,
    layers: f64,
    identity: bool,
) -> String {
    if (((kind == "lambda") && (parameters == 0f64)) && (layers == 0f64)) && identity {
        String::new()
    } else {
        String::from(
            "a deferred fallback needs its actual zero-argument closure source and identity",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_deferred_result_error(
    kind: &str,
    pending: bool,
    known: bool,
    copy: bool,
) -> String {
    if (((kind == "lambda") && pending) && known) && copy {
        String::new()
    } else {
        String::from("a deferred closure needs its actual complete immutable Copy result context")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_result_context_role(kind: &str, operation: &str) -> String {
    if kind == "let" {
        String::from("binding")
    } else if kind == "name" {
        String::from("name")
    } else if kind == "if" {
        String::from("branches")
    } else if kind == "match" {
        String::from("rows")
    } else if (kind == "unary") && (operation == "neg") {
        String::from("operand")
    } else if (kind == "binary")
        && self::ml_array::includes(
            &vec![
                String::from("add"),
                String::from("sub"),
                String::from("mul"),
                String::from("div"),
                String::from("rem"),
            ],
            &operation.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from("operands")
    } else {
        String::from("leaf")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_literal_result_error(
    kind: &str,
    negative: bool,
    typed: bool,
    result: &str,
    signed: bool,
) -> String {
    if (negative && (result == "fixed")) && !signed {
        String::from("an unsigned Rust result cannot use unary negation")
    } else if ((kind == "num") && !typed) && (result != "fixed") {
        String::from("an unsuffixed Rust integer literal needs an actual integer result context")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_copy_closure_result_binding(path: &[String], name: &str) -> bool {
    ((path.len() as f64) == 1f64) && {
        let source_ml_source1_comparison_operand5 =
            self::ml_array::optional_at(path, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand6 = name.to_owned();
            match (
                source_ml_source1_comparison_operand5,
                source_ml_source1_comparison_operand6,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_reference_pattern_depth(token: &str) -> f64 {
    if token == "&" {
        1f64
    } else if token == "&&" {
        2f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_reference_pattern_error(
    depth: f64,
    layers: f64,
    references: bool,
    parameter: bool,
) -> String {
    if (depth == 0f64)
        || (((((((self::ml_math::is_integer(depth) && (depth >= 1f64)) && (depth <= 256f64))
            && self::ml_math::is_integer(layers))
            && (layers >= depth))
            && (layers <= 256f64))
            && references)
            && parameter)
    {
        String::new()
    } else {
        String::from(
            "a closure reference pattern needs its actual immutable Copy scalar reference layers",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_copy_closure_pattern_binding_error(names: &[String]) -> String {
    if (names.len() as f64) > 1024f64 {
        String::from("the complete typed closure header exceeds its source budget")
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s7 =
                    self::source_ml_temporary_read_rust_copy_closure_pattern_binding_error_loop5(
                        names, index,
                    );
                match source_ml_source1_s7 {
                    self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Done => {
                        String::new()
                    }
                    self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Return(source_ml_temporary_result5) => {
                        source_ml_temporary_result5
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureCaptureErrorJoin1 {
    MlTemporaryReadRustCopyClosureCaptureErrorJoin1Next,
    MlTemporaryReadRustCopyClosureCaptureErrorJoin1Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result {
    MlTemporaryReadRustCopyClosureCaptureErrorLoop2Done,
    MlTemporaryReadRustCopyClosureCaptureErrorLoop2Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_copy_closure_capture_error_loop2(
    copies: &[bool],
    mut index: f64,
) -> self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result {
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
                self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Return(String::from("closure captures require proven direct immutable Copy source values"))
            }
        } else {
            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Result {
    MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Done,
    MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Result {
    MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Done,
    MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_copy_closure_parameter_header_error_loop4(
    names: &[String],
    mut index: f64,
    mut before: f64,
) -> self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Result {
    loop {
        return if before < index {
            if (self::read_required_text_fact(names, before))
                == (self::read_required_text_fact(names, index))
            {
                self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Return(String::from("closure parameters require distinct actual bindings"))
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
            self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_copy_closure_parameter_header_error_loop3(
    names: &[String],
    copies: &[bool],
    mut index: f64,
) -> self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Result {
    loop {
        return if index < (names.len() as f64) {
            if (self::read_required_boolean_fact(copies, index)) {
                {
                    let before = 0f64;
                    {
                        let source_ml_source1_s8 = self::source_ml_temporary_read_rust_copy_closure_parameter_header_error_loop4(names, index, before);
                        match source_ml_source1_s8 {
                            self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Done => {
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        index = index_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop4Return(source_ml_temporary_result4) => {
                                self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Return(source_ml_temporary_result4)
                            }
                        }
                    }
                }
            } else {
                self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Return(String::from("closure parameters require direct immutable Copy source types"))
            }
        } else {
            self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Result {
    MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Done,
    MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Result {
    MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Done,
    MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_copy_closure_pattern_binding_error_loop6(
    names: &[String],
    mut index: f64,
    mut before: f64,
) -> self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Result {
    loop {
        return if before < index {
            if (self::read_required_text_fact(names, before))
                == (self::read_required_text_fact(names, index))
            {
                self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Return(String::from("closure parameters require distinct actual bindings"))
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
            self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_copy_closure_pattern_binding_error_loop5(
    names: &[String],
    mut index: f64,
) -> self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Result {
    loop {
        return if index < (names.len() as f64) {
            {
                let before = 0f64;
                {
                    let source_ml_source1_s9 = self::source_ml_temporary_read_rust_copy_closure_pattern_binding_error_loop6(names, index, before);
                    match source_ml_source1_s9 {
                        self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Done => {
                            {
                                let index_2 = index + 1f64;
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop6Return(source_ml_temporary_result6) => {
                            self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Return(source_ml_temporary_result6)
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop5Done
        };
    }
}
