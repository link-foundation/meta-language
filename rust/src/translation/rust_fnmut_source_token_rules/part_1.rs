// Generated from complete js/src/translation/rust-fnmut-source-token-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustFnMutSourceTokenPlanJoin4;
use super::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Result;
use super::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Result;
use super::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Result;
use super::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Result;
use super::source_ml_temporary_delimited_positions_loop2;
use super::source_ml_temporary_read_rust_fn_mut_source_token_plan_loop3;
use super::source_ml_temporary_read_rust_fn_mut_source_token_plan_loop5;
use super::source_ml_temporary_read_rust_fn_mut_source_token_plan_loop6;
use super::source_ml_temporary_read_rust_fn_mut_source_token_plan_loop7;

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
pub fn read_rust_fn_mut_source_model_error(model: &str, minor: f64) -> String {
    if model == "Rust closed source FnMut specializations 1" {
        if (!(self::ml_math::is_integer(minor)) || (minor < 90f64)) || (minor >= 100f64) {
            String::from(
                "closed source FnMut specialization is uncovered outside Rust >=1.90 <1.100",
            )
        } else {
            String::new()
        }
    } else {
        String::from("select the actual closed source FnMut specialization model")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_fn_mut_source_declaration_budget_error(
    tokens: f64,
    parameters: f64,
    generics: f64,
    callbacks: f64,
) -> String {
    if ((((((((!(self::ml_math::is_safe_integer(tokens)) || (tokens < 0f64))
        || (tokens > 1_000_000f64))
        || !(self::ml_math::is_safe_integer(parameters)))
        || (parameters < 0f64))
        || (parameters > 256f64))
        || !(self::ml_math::is_safe_integer(generics)))
        || (generics < 0f64))
        || (generics > 32f64))
        || (callbacks != 1f64)
    {
        String::from(
            "a complete FnMut source declaration needs one actual callback and bounded tokens, parameters and concrete generic arguments",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn token_fact(values: &[String], index: f64) -> String {
    if (index >= 0f64) && (index < (values.len() as f64)) {
        self::read_required_text_fact(values, index)
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn is_punctuation(kinds: &[String], values: &[String], index: f64, value: &str) -> bool {
    ((self::token_fact(kinds, index)) == "punct") && ((self::token_fact(values, index)) == value)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn is_identifier(kinds: &[String], values: &[String], index: f64, value: &str) -> bool {
    ((self::token_fact(kinds, index)) == "identifier")
        && ((self::token_fact(values, index)) == value)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn closing_position(
    kinds: &[String],
    values: &[String],
    start: f64,
    opening: &str,
    closing: &str,
) -> f64 {
    if (self::is_punctuation(kinds, values, start, opening)) {
        {
            let depth = 1f64;
            {
                let at = start + 1f64;
                {
                    let source_ml_source1_s1 = self::source_ml_temporary_closing_position_loop1(
                        kinds, values, start, opening, closing, depth, at,
                    );
                    match source_ml_source1_s1 {
                        self::MlTemporaryClosingPositionLoop1Result::MlTemporaryClosingPositionLoop1Done => {
                            -1f64
                        }
                        self::MlTemporaryClosingPositionLoop1Result::MlTemporaryClosingPositionLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn delimited_positions(kinds: &[String], values: &[String], start: f64, end: f64) -> Vec<f64> {
    {
        let ranges = Vec::<f64>::new();
        {
            let begin = start;
            {
                let angles = 0f64;
                {
                    let at = start;
                    {
                        let source_ml_source1_s2 =
                            self::source_ml_temporary_delimited_positions_loop2(
                                kinds, values, end, ranges, begin, angles, at,
                            );
                        match source_ml_source1_s2 {
                            self::MlTemporaryDelimitedPositionsLoop2Result::MlTemporaryDelimitedPositionsLoop2Done(ranges_2, begin_2, angles_2) => {
                                {
let ml_library_inequality_1 = angles_2 != 0f64 ;
if ml_library_inequality_1 {
                                    vec![-1f64, end]
                                } else {
                                    {
                                        if begin_2 < end {
                                            {
                                                { let (ml_append_right, ml_append_left) = (vec![begin_2, end], ranges_2); self::ml_array::append(ml_append_left, ml_append_right) }
                                            }
                                        } else {
                                            ranges_2
                                        }
                                    }
                                }
}
                            }
                            self::MlTemporaryDelimitedPositionsLoop2Result::MlTemporaryDelimitedPositionsLoop2Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
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
pub fn callback_positions(kinds: &[String], values: &[String], start: f64, end: f64) -> Vec<f64> {
    {
        let mutable = self::is_identifier(kinds, values, start, "mut");
        {
            let binder = if mutable { start + 1f64 } else { start };
            if ((self::token_fact(kinds, binder)) != "identifier")
                || !(self::is_punctuation(kinds, values, (binder + 1f64), ":"))
            {
                Vec::<f64>::new()
            } else {
                {
                    let type_start = binder + 2f64;
                    {
                        let exclusive = self::is_punctuation(kinds, values, type_start, "&");
                        if exclusive
                            && !(self::is_identifier(kinds, values, (type_start + 1f64), "mut"))
                        {
                            Vec::<f64>::new()
                        } else {
                            {
                                let implementation = if exclusive {
                                    type_start + 2f64
                                } else {
                                    type_start
                                };
                                if !(self::is_identifier(kinds, values, implementation, "impl"))
                                    || !(self::is_identifier(
                                        kinds,
                                        values,
                                        (implementation + 1f64),
                                        "FnMut",
                                    ))
                                {
                                    Vec::<f64>::new()
                                } else {
                                    {
                                        let arguments_start = implementation + 2f64;
                                        {
                                            let arguments_end = self::closing_position(
                                                kinds,
                                                values,
                                                arguments_start,
                                                "(",
                                                ")",
                                            );
                                            if ((arguments_end < 0f64)
                                                || ((arguments_end + 2f64) >= end))
                                                || !(self::is_punctuation(
                                                    kinds,
                                                    values,
                                                    (arguments_end + 1f64),
                                                    "->",
                                                ))
                                            {
                                                vec![-1f64, arguments_start]
                                            } else {
                                                {
                                                    let arguments_ranges =
                                                        self::delimited_positions(
                                                            kinds,
                                                            values,
                                                            (arguments_start + 1f64),
                                                            arguments_end,
                                                        );
                                                    if ((arguments_ranges.len() as f64) > 0f64)
                                                        && ((self::read_required_number_fact(
                                                            &(arguments_ranges),
                                                            0f64,
                                                        )) < 0f64)
                                                    {
                                                        arguments_ranges
                                                    } else {
                                                        self::ml_array::append(
                                                            vec![
                                                                binder,
                                                                if exclusive { 1f64 } else { 0f64 },
                                                                if mutable { 1f64 } else { 0f64 },
                                                                type_start,
                                                                end,
                                                                (arguments_end + 2f64),
                                                                end,
                                                            ],
                                                            arguments_ranges,
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_fn_mut_source_token_plan(
    kinds: &[String],
    values: &[String],
    start: f64,
) -> Vec<f64> {
    if (((kinds.len() as f64) != (values.len() as f64))
        || !(self::is_identifier(kinds, values, start, "fn")))
        || ((self::token_fact(kinds, (start + 1f64))) != "identifier")
    {
        vec![-1f64, 1f64, start]
    } else {
        {
            let header_end = start + 2f64;
            {
                let selected = false;
                {
                    let source_ml_source1_s3 =
                        self::source_ml_temporary_read_rust_fn_mut_source_token_plan_loop3(
                            kinds, values, start, header_end, selected,
                        );
                    match source_ml_source1_s3 {
                        self::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Done(header_end_2, selected_2) => {
                            if selected_2 {
                                {
                                    let generics = Vec::<f64>::new();
                                    {
                                        let generic_names = Vec::<String>::new();
                                        {
                                            let formal_start = start + 2f64;
                                            {
                                                let source_ml_source1_s5 = if self::is_punctuation(kinds, values, formal_start, "<") {
                                                    {
                                                        let end = self::closing_position(kinds, values, formal_start, "<", ">");
                                                        if (end < 0f64) || (end >= header_end_2) {
                                                            self::MlTemporaryReadRustFnMutSourceTokenPlanJoin4::MlTemporaryReadRustFnMutSourceTokenPlanJoin4Return(vec![-1f64, 3f64, formal_start])
                                                        } else {
                                                            {
                                                                let ranges = self::delimited_positions(kinds, values, (formal_start + 1f64), end);
                                                                if ((ranges.len() as f64) == 0f64) || ((self::read_required_number_fact(&(ranges), 0f64)) < 0f64) {
                                                                    self::MlTemporaryReadRustFnMutSourceTokenPlanJoin4::MlTemporaryReadRustFnMutSourceTokenPlanJoin4Return(vec![-1f64, 3f64, formal_start])
                                                                } else {
                                                                    {
                                                                        let at = 0f64;
                                                                        {
                                                                            let source_ml_source1_s6 = self::source_ml_temporary_read_rust_fn_mut_source_token_plan_loop7(kinds, values, generics, generic_names, &(ranges), at);
                                                                            match source_ml_source1_s6 {
                                                                                self::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Done(generics_2) => {
                                                                                    {
                                                                                        let formal_start_2 = end + 1f64;
                                                                                        self::MlTemporaryReadRustFnMutSourceTokenPlanJoin4::MlTemporaryReadRustFnMutSourceTokenPlanJoin4Next(generics_2, formal_start_2)
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop7Return(source_ml_temporary_result7) => {
                                                                                    self::MlTemporaryReadRustFnMutSourceTokenPlanJoin4::MlTemporaryReadRustFnMutSourceTokenPlanJoin4Return(source_ml_temporary_result7)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::MlTemporaryReadRustFnMutSourceTokenPlanJoin4::MlTemporaryReadRustFnMutSourceTokenPlanJoin4Next(generics, formal_start)
                                                };
                                                match source_ml_source1_s5 {
                                                    self::MlTemporaryReadRustFnMutSourceTokenPlanJoin4::MlTemporaryReadRustFnMutSourceTokenPlanJoin4Next(generics_3, formal_start_3) => {
                                                        {
                                                            let formal_end = self::closing_position(kinds, values, formal_start_3, "(", ")");
                                                            if (formal_end < 0f64) || (formal_end >= header_end_2) {
                                                                vec![-1f64, 4f64, formal_start_3]
                                                            } else {
                                                                {
                                                                    let formals = self::delimited_positions(kinds, values, (formal_start_3 + 1f64), formal_end);
                                                                    if ((formals.len() as f64) > 0f64) && ((self::read_required_number_fact(&(formals), 0f64)) < 0f64) {
                                                                        vec![-1f64, 4f64, self::read_required_number_fact(&(formals), 1f64)]
                                                                    } else {
                                                                        {
                                                                            let callback = Vec::<f64>::new();
                                                                            {
                                                                                let callback_index = -1f64;
                                                                                {
                                                                                    let callbacks = 0f64;
                                                                                    {
                                                                                        let at_2 = 0f64;
                                                                                        {
                                                                                            let source_ml_source1_s7 = self::source_ml_temporary_read_rust_fn_mut_source_token_plan_loop5(kinds, values, &(formals), callback, callback_index, callbacks, at_2);
                                                                                            match source_ml_source1_s7 {
                                                                                                self::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Done(callback_2, callback_index_2, callbacks_2) => {
                                                                                                    {
                                                                                                        let at_3 = formal_end + 1f64;
                                                                                                        {
                                                                                                            let source_ml_source1_s8 = self::source_ml_temporary_read_rust_fn_mut_source_token_plan_loop6(kinds, values, header_end_2, at_3);
                                                                                                            match source_ml_source1_s8 {
                                                                                                                self::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Done => {
                                                                                                                    {
                                                                                                                        let body_end = self::closing_position(kinds, values, header_end_2, "{", "}");
                                                                                                                        if body_end < 0f64 {
                                                                                                                            vec![-1f64, 6f64, header_end_2]
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let formal_count = (formals.len() as f64) / 2f64;
                                                                                                                                {
let ml_library_inequality_1 = (self::read_rust_fn_mut_source_declaration_budget_error(((body_end - start) + 1f64), formal_count, (generics_3.len() as f64), callbacks_2)) != "" ;
if ml_library_inequality_1 {
                                                                                                                                    vec![-1f64, 7f64, start]
                                                                                                                                } else {
                                                                                                                                    {
                                                                                                                                        let has_result = self::is_punctuation(kinds, values, (formal_end + 1f64), "->") && ((formal_end + 2f64) < header_end_2);
                                                                                                                                        self::ml_array::append(self::ml_array::append(vec![header_end_2, body_end, if has_result {
                                                                                                                                            formal_end + 2f64
                                                                                                                                        } else {
                                                                                                                                            -1f64
                                                                                                                                        }, if has_result {
                                                                                                                                            header_end_2
                                                                                                                                        } else {
                                                                                                                                            -1f64
                                                                                                                                        }, formal_count, callback_index_2, (generics_3.len() as f64)], generics_3), callback_2)
                                                                                                                                    }
                                                                                                                                }
}
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                self::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop6Return(source_ml_temporary_result6) => {
                                                                                                                    source_ml_temporary_result6
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop5Return(source_ml_temporary_result6_2) => {
                                                                                                    source_ml_temporary_result6_2
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadRustFnMutSourceTokenPlanJoin4::MlTemporaryReadRustFnMutSourceTokenPlanJoin4Return(source_ml_temporary_result6_3) => {
                                                        source_ml_temporary_result6_3
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                Vec::<f64>::new()
                            }
                        }
                        self::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Result::MlTemporaryReadRustFnMutSourceTokenPlanLoop3Return(source_ml_temporary_result7_2) => {
                            source_ml_temporary_result7_2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_fn_mut_source_token_error(reason: f64) -> String {
    if reason == 1f64 {
        String::from("the selected source function needs its actual lexical token and owner path")
    } else if reason == 2f64 {
        String::from("the original function header exceeds its actual source token budget")
    } else if reason == 3f64 {
        String::from(
            "this closed specialization requires distinct unconstrained actual type binders",
        )
    } else if reason == 4f64 {
        String::from("an actual source formal has an inconsistent complete delimiter inventory")
    } else if reason == 5f64 {
        String::from("the actual FnMut bound needs its complete explicit result source position")
    } else if reason == 6f64 {
        String::from(
            "where predicates and absent generic bodies need their actual separate source interpretation",
        )
    } else {
        String::from(
            "a complete FnMut source declaration needs one actual callback and bounded tokens, parameters and concrete generic arguments",
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryClosingPositionLoop1Result {
    MlTemporaryClosingPositionLoop1Done,
    MlTemporaryClosingPositionLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_closing_position_loop1(
    kinds: &[String],
    values: &[String],
    mut start: f64,
    opening: &str,
    closing: &str,
    mut depth: f64,
    mut at: f64,
) -> self::MlTemporaryClosingPositionLoop1Result {
    loop {
        return if (at < (kinds.len() as f64)) && ((at - start) < 1_000_000f64) {
            {
                let depth_3 = if self::is_punctuation(kinds, values, at, opening) {
                    { depth + 1f64 }
                } else {
                    depth
                };
                if self::is_punctuation(kinds, values, at, closing) {
                    {
                        let depth_4 = depth_3 - 1f64;
                        if depth_4 == 0f64 {
                            self::MlTemporaryClosingPositionLoop1Result::MlTemporaryClosingPositionLoop1Return(at)
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    (depth, at) = (depth_4, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = at + 1f64;
                        {
                            (depth, at) = (depth_3, at_3);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryClosingPositionLoop1Result::MlTemporaryClosingPositionLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDelimitedPositionsLoop2Result {
    MlTemporaryDelimitedPositionsLoop2Done(Vec<f64>, f64, f64),
    MlTemporaryDelimitedPositionsLoop2Return(Vec<f64>),
}
