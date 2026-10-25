// Generated from complete js/src/translation/named-default-callable-rules.js. Do not edit by hand.
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
pub fn accept_body_entry_optional_parameter(optional: bool, kind: &str, absence: &str) -> bool {
    (optional && (kind == "none")) && (absence == "undefined")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_optional_callable_arguments(
    language: &str,
    kinds: &[String],
    absences: &[String],
    provided: f64,
) -> Vec<f64> {
    if (((((language != "JavaScript") || ((kinds.len() as f64) != (absences.len() as f64)))
        || ((kinds.len() as f64) > 1024f64))
        || !(self::ml_math::is_safe_integer(provided)))
        || (provided < 0f64))
        || (provided > (kinds.len() as f64))
    {
        Vec::<f64>::new()
    } else {
        {
            let result = vec![(kinds.len() as f64)];
            {
                let at = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_plan_optional_callable_arguments_loop1(
                            kinds, absences, provided, result, at,
                        );
                    match source_ml_source1_s1 {
                        self::MlTemporaryPlanOptionalCallableArgumentsLoop1Result::MlTemporaryPlanOptionalCallableArgumentsLoop1Done(result_2) => {
                            result_2
                        }
                        self::MlTemporaryPlanOptionalCallableArgumentsLoop1Result::MlTemporaryPlanOptionalCallableArgumentsLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_optional_callable_parameter_bridge(
    language: &str,
    supplied: f64,
    exact: &[bool],
    lifted: &[bool],
) -> Vec<f64> {
    if ((((language != "JavaScript") || ((exact.len() as f64) != (lifted.len() as f64)))
        || !(self::ml_math::is_safe_integer(supplied)))
        || (supplied < (exact.len() as f64)))
        || ((exact.len() as f64) > 1024f64)
    {
        Vec::<f64>::new()
    } else {
        {
            let result = vec![(exact.len() as f64)];
            {
                let needed = false;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s2 =
                            self::source_ml_temporary_plan_optional_callable_parameter_bridge_loop2(
                                exact, lifted, result, needed, at,
                            );
                        match source_ml_source1_s2 {
                            self::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Result::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Done(result_2, needed_2) => {
                                if needed_2 {
                                    result_2
                                } else {
                                    Vec::<f64>::new()
                                }
                            }
                            self::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Result::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Return(source_ml_temporary_result2) => {
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
pub fn require_named_callable_default_initialization(
    present: bool,
    optional: bool,
    kind: &str,
    absence: &str,
) -> bool {
    present && !(self::accept_body_entry_optional_parameter(optional, kind, absence))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_named_callable_default_reference(
    language: &str,
    defaults: &[bool],
    guards: &[bool],
) -> Vec<f64> {
    if language == "JavaScript" {
        if (defaults.len() as f64) == (guards.len() as f64) {
            {
                let needed = false;
                {
                    let at = 0f64;
                    {
                        let needed_2 =
                            self::source_ml_temporary_plan_named_callable_default_reference_loop3(
                                defaults, needed, at,
                            );
                        if needed_2 {
                            if (defaults.len() as f64) > 1024f64 {
                                vec![-1f64]
                            } else {
                                {
                                    let result = vec![(defaults.len() as f64)];
                                    {
                                        let at_2 = 0f64;
                                        {
                                            self::source_ml_temporary_plan_named_callable_default_reference_loop4(defaults, guards, result, at_2)
                                        }
                                    }
                                }
                            }
                        } else {
                            Vec::<f64>::new()
                        }
                    }
                }
            }
        } else {
            vec![-1f64]
        }
    } else {
        Vec::<f64>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_named_callable_default_reference_absence(kind: &str, absence: &str) -> String {
    if (kind != "option") || (absence == "undefined") {
        String::from("undefined")
    } else if (absence == "null") || (absence == "both") {
        String::from("both")
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanOptionalCallableArgumentsLoop1Result {
    MlTemporaryPlanOptionalCallableArgumentsLoop1Done(Vec<f64>),
    MlTemporaryPlanOptionalCallableArgumentsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_optional_callable_arguments_loop1(
    kinds: &[String],
    absences: &[String],
    mut provided: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> self::MlTemporaryPlanOptionalCallableArgumentsLoop1Result {
    loop {
        return if at < (kinds.len() as f64) {
            if at < provided {
                {
                    let result_2 = {
                        let source_ml_source1_concat_value3 = result;
                        {
                            let source_ml_source1_concat_value4 = vec![at];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source1_concat_value3,
                                ),
                                source_ml_source1_concat_value4,
                            )
                        }
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let absence = self::read_required_text_fact(absences, at);
                    if ((self::read_required_text_fact(kinds, at)) != "option")
                        || ((absence != "undefined") && (absence != "both"))
                    {
                        self::MlTemporaryPlanOptionalCallableArgumentsLoop1Result::MlTemporaryPlanOptionalCallableArgumentsLoop1Return(Vec::<f64>::new())
                    } else {
                        {
                            let result_3 = {
                                let source_ml_source1_concat_value5 = result;
                                {
                                    let source_ml_source1_concat_value6 = vec![-1f64];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value5,
                                        ),
                                        source_ml_source1_concat_value6,
                                    )
                                }
                            };
                            {
                                let at_3 = at + 1f64;
                                {
                                    (result, at) = (result_3, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanOptionalCallableArgumentsLoop1Result::MlTemporaryPlanOptionalCallableArgumentsLoop1Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanOptionalCallableParameterBridgeLoop2Result {
    MlTemporaryPlanOptionalCallableParameterBridgeLoop2Done(Vec<f64>, bool),
    MlTemporaryPlanOptionalCallableParameterBridgeLoop2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_optional_callable_parameter_bridge_loop2(
    exact: &[bool],
    lifted: &[bool],
    mut result: Vec<f64>,
    mut needed: bool,
    mut at: f64,
) -> self::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Result {
    loop {
        return if at < (exact.len() as f64) {
            if self::read_required_boolean_fact(exact, at) {
                {
                    let result_2 = {
                        let source_ml_source1_concat_value7 = result;
                        {
                            let source_ml_source1_concat_value8 = vec![0f64];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source1_concat_value7,
                                ),
                                source_ml_source1_concat_value8,
                            )
                        }
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                if self::read_required_boolean_fact(lifted, at) {
                    {
                        let needed_2 = true;
                        {
                            let result_3 = {
                                let source_ml_source1_concat_value9 = result;
                                {
                                    let source_ml_source1_concat_value10 = vec![1f64];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value9,
                                        ),
                                        source_ml_source1_concat_value10,
                                    )
                                }
                            };
                            {
                                let at_3 = at + 1f64;
                                {
                                    (result, needed, at) = (result_3, needed_2, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    self::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Result::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Return(Vec::<f64>::new())
                }
            }
        } else {
            self::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Result::MlTemporaryPlanOptionalCallableParameterBridgeLoop2Done(result, needed)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_named_callable_default_reference_loop3(
    defaults: &[bool],
    mut needed: bool,
    mut at: f64,
) -> bool {
    loop {
        if at < (defaults.len() as f64) {
            if self::read_required_boolean_fact(defaults, at) {
                {
                    let needed_2 = true;
                    {
                        let at_2 = at + 1f64;
                        {
                            (needed, at) = (needed_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { needed };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_named_callable_default_reference_loop4(
    defaults: &[bool],
    guards: &[bool],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (defaults.len() as f64) {
            {
                let initialized = self::read_required_boolean_fact(defaults, at);
                {
                    let guarded = self::read_required_boolean_fact(guards, at);
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value11 = result;
                            {
                                let source_ml_source1_concat_value12 = vec![
                                    (if initialized { 1f64 } else { 0f64 }
                                        + if guarded { 2f64 } else { 0f64 }),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value11,
                                    ),
                                    source_ml_source1_concat_value12,
                                )
                            }
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (result, at) = (result_2, at_2);
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
