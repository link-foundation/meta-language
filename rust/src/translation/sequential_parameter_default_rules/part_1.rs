// Generated from complete js/src/translation/sequential-parameter-default-rules.js. Do not edit by hand.
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
pub fn field(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    name: &str,
) -> f64 {
    if node < 0f64 {
        -1f64
    } else {
        {
            let first = self::read_required_number_fact(starts, node);
            {
                let count = self::read_required_number_fact(counts, node);
                {
                    let at = first;
                    {
                        let source_ml_source1_s1 = self::source_ml_temporary_field_loop1(
                            labels, targets, name, first, count, at,
                        );
                        match source_ml_source1_s1 {
                            self::MlTemporaryFieldLoop1Result::MlTemporaryFieldLoop1Done => -1f64,
                            self::MlTemporaryFieldLoop1Result::MlTemporaryFieldLoop1Return(
                                source_ml_temporary_result1,
                            ) => source_ml_temporary_result1,
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn contains_parameter_default_name_reference(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    names: &[String],
) -> bool {
    {
        let node = 0f64;
        {
            let source_ml_source1_s2 =
                self::source_ml_temporary_contains_parameter_default_name_reference_loop2(
                    forms, texts, starts, counts, labels, targets, names, node,
                );
            match source_ml_source1_s2 {
                self::MlTemporaryContainsParameterDefaultNameReferenceLoop2Result::MlTemporaryContainsParameterDefaultNameReferenceLoop2Done => {
                    false
                }
                self::MlTemporaryContainsParameterDefaultNameReferenceLoop2Result::MlTemporaryContainsParameterDefaultNameReferenceLoop2Return(source_ml_temporary_result2) => {
                    source_ml_temporary_result2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_sequential_parameter_initialization(defaults: &[bool], references: bool) -> bool {
    if references {
        true
    } else {
        {
            let seen = false;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s3 =
                        self::source_ml_temporary_require_sequential_parameter_initialization_loop3(
                            defaults, seen, at,
                        );
                    match source_ml_source1_s3 {
                        self::MlTemporaryRequireSequentialParameterInitializationLoop3Result::MlTemporaryRequireSequentialParameterInitializationLoop3Done => {
                            false
                        }
                        self::MlTemporaryRequireSequentialParameterInitializationLoop3Result::MlTemporaryRequireSequentialParameterInitializationLoop3Return(source_ml_temporary_result3) => {
                            source_ml_temporary_result3
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_sequential_parameter_initialization(parameters: f64, provided: f64) -> Vec<f64> {
    if ((((!(self::ml_math::is_safe_integer(parameters))
        || !(self::ml_math::is_safe_integer(provided)))
        || (parameters < 1f64))
        || (parameters > 1024f64))
        || (provided < 0f64))
        || (provided > parameters)
    {
        Vec::<f64>::new()
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let at = 0f64;
                {
                    let result_2 =
                        self::source_ml_temporary_plan_sequential_parameter_initialization_loop4(
                            provided, result, at,
                        );
                    {
                        let at_2 = 0f64;
                        {
                            self::source_ml_temporary_plan_sequential_parameter_initialization_loop5(
                                parameters, result_2, at_2,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_ordinary_parameter_binding_positions(
    kinds: &[String],
    values: &[String],
    start: f64,
) -> Vec<f64> {
    if (((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_safe_integer(start)))
        || (start < 0f64)
    {
        Vec::<f64>::new()
    } else {
        {
            let positions = Vec::<f64>::new();
            {
                let closes = Vec::<String>::new();
                {
                    let binding = true;
                    {
                        let at = start;
                        {
                            let source_ml_source1_s4 = self::source_ml_temporary_plan_ordinary_parameter_binding_positions_loop6(kinds, values, positions, closes, binding, at);
                            match source_ml_source1_s4 {
                                self::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Result::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Done => {
                                    Vec::<f64>::new()
                                }
                                self::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Result::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Return(source_ml_temporary_result6) => {
                                    source_ml_temporary_result6
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFieldLoop1Result {
    MlTemporaryFieldLoop1Done,
    MlTemporaryFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_field_loop1(
    labels: &[String],
    targets: &[f64],
    name: &str,
    mut first: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlTemporaryFieldLoop1Result {
    loop {
        return if at < (first + count) {
            if (self::read_required_text_fact(labels, at)) == name {
                self::MlTemporaryFieldLoop1Result::MlTemporaryFieldLoop1Return(
                    self::read_required_number_fact(targets, at),
                )
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryFieldLoop1Result::MlTemporaryFieldLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryContainsParameterDefaultNameReferenceLoop2Result {
    MlTemporaryContainsParameterDefaultNameReferenceLoop2Done,
    MlTemporaryContainsParameterDefaultNameReferenceLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_contains_parameter_default_name_reference_loop2(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    names: &[String],
    mut node: f64,
) -> self::MlTemporaryContainsParameterDefaultNameReferenceLoop2Result {
    loop {
        return if node < (forms.len() as f64) {
            if (self::read_required_text_fact(forms, node)) != "object" {
                {
                    let node_2 = node + 1f64;
                    {
                        node = node_2;
                        continue;
                    }
                }
            }
            {
                {
                    let kind = self::field(starts, counts, labels, targets, node, "k");
                    if (kind < 0f64) || ((self::read_required_text_fact(texts, kind)) != "name") {
                        {
                            let node_3 = node + 1f64;
                            {
                                node = node_3;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let path = self::field(starts, counts, labels, targets, node, "path");
                            if (path < 0f64)
                                || ((self::read_required_number_fact(counts, path)) != 1f64)
                            {
                                {
                                    let node_4 = node + 1f64;
                                    {
                                        node = node_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                {
                                    let name = self::read_required_number_fact(
                                        targets,
                                        self::read_required_number_fact(starts, path),
                                    );
                                    if self::ml_array::includes(
                                        names,
                                        &self::read_required_text_fact(texts, name),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ) {
                                        self::MlTemporaryContainsParameterDefaultNameReferenceLoop2Result::MlTemporaryContainsParameterDefaultNameReferenceLoop2Return(true)
                                    } else {
                                        {
                                            let node_5 = node + 1f64;
                                            {
                                                node = node_5;
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryContainsParameterDefaultNameReferenceLoop2Result::MlTemporaryContainsParameterDefaultNameReferenceLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRequireSequentialParameterInitializationLoop3Result {
    MlTemporaryRequireSequentialParameterInitializationLoop3Done,
    MlTemporaryRequireSequentialParameterInitializationLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_sequential_parameter_initialization_loop3(
    defaults: &[bool],
    mut seen: bool,
    mut at: f64,
) -> self::MlTemporaryRequireSequentialParameterInitializationLoop3Result {
    loop {
        return if at < (defaults.len() as f64) {
            if self::read_required_boolean_fact(defaults, at) {
                {
                    let seen_2 = true;
                    {
                        let at_2 = at + 1f64;
                        {
                            (seen, at) = (seen_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                if seen {
                    self::MlTemporaryRequireSequentialParameterInitializationLoop3Result::MlTemporaryRequireSequentialParameterInitializationLoop3Return(true)
                } else {
                    {
                        let at_3 = at + 1f64;
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryRequireSequentialParameterInitializationLoop3Result::MlTemporaryRequireSequentialParameterInitializationLoop3Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_sequential_parameter_initialization_loop4(
    mut provided: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < provided {
            {
                let result_2 = {
                    let source_ml_source1_concat_value5 = result;
                    {
                        let source_ml_source1_concat_value6 = vec![0f64, at];
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
                    let at_2 = at + 1f64;
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_sequential_parameter_initialization_loop5(
    mut parameters: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < parameters {
            {
                let result_2 = {
                    let source_ml_source1_concat_value7 = result;
                    {
                        let source_ml_source1_concat_value8 = vec![1f64, at];
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
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Result {
    MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Done,
    MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanOrdinaryParameterBindingPositionsJoin7 {
    MlTemporaryPlanOrdinaryParameterBindingPositionsJoin7Next(Vec<f64>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_ordinary_parameter_binding_positions_loop6(
    kinds: &[String],
    values: &[String],
    mut positions: Vec<f64>,
    mut closes: Vec<String>,
    mut binding: bool,
    mut at: f64,
) -> self::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, at);
                {
                    let value = self::read_required_text_fact(values, at);
                    if (!({
                        let ml_value = closes.len() as f64;
                        ml_value != 0f64 && !ml_value.is_nan()
                    }) && (kind == "punct"))
                        && (value == ")")
                    {
                        self::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Result::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Return({
                            let source_ml_source1_concat_value9 = vec![(at + 1f64)];
                            {
                                let source_ml_source1_concat_value10 = positions;
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value9), source_ml_source1_concat_value10)
                            }
                        })
                    } else {
                        {
                            let source_ml_source1_s13 = if (!({
                                let ml_value = closes.len() as f64;
                                ml_value != 0f64 && !ml_value.is_nan()
                            }) && binding)
                                && (kind == "identifier")
                            {
                                {
                                    let positions_2 = {
                                        let source_ml_source1_concat_value14 = positions;
                                        {
                                            let source_ml_source1_concat_value15 = vec![at];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source1_concat_value14,
                                                ),
                                                source_ml_source1_concat_value15,
                                            )
                                        }
                                    };
                                    {
                                        let binding_2 = false;
                                        self::MlTemporaryPlanOrdinaryParameterBindingPositionsJoin7::MlTemporaryPlanOrdinaryParameterBindingPositionsJoin7Next(positions_2, binding_2)
                                    }
                                }
                            } else {
                                self::MlTemporaryPlanOrdinaryParameterBindingPositionsJoin7::MlTemporaryPlanOrdinaryParameterBindingPositionsJoin7Next(positions, binding)
                            };
                            match source_ml_source1_s13 {
                                self::MlTemporaryPlanOrdinaryParameterBindingPositionsJoin7::MlTemporaryPlanOrdinaryParameterBindingPositionsJoin7Next(positions_3, binding_3) => {
                                    if kind != "punct" {
                                        {
                                            let at_2 = at + 1f64;
                                            {
                                                (positions, binding, at) = (positions_3, binding_3, at_2);
                                                continue;
                                            }
                                        }
                                    }
{
                                        if ((value == "(") || (value == "[")) || (value == "{") {
                                            {
                                                let closes_2 = {
                                                    let source_ml_source1_concat_value16 = closes;
                                                    {
                                                        let source_ml_source1_concat_value17 = vec![if value == "(" {
                                                            String::from(")")
                                                        } else if value == "[" {
                                                                String::from("]")
                                                            } else {
                                                                String::from("}")
                                                            }];
                                                        self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source1_concat_value16), source_ml_source1_concat_value17)
                                                    }
                                                };
                                                {
                                                    let at_3 = at + 1f64;
                                                    {
                                                        (positions, closes, binding, at) = (positions_3, closes_2, binding_3, at_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
{
                                            if ((value == ")") || (value == "]")) || (value == "}") {
                                                if !({ let ml_value = closes.len() as f64; ml_value != 0f64 && !ml_value.is_nan() }) || ((self::read_required_text_fact(&(closes), ((closes.len() as f64) - 1f64))) != value) {
                                                    self::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Result::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Return(Vec::<f64>::new())
                                                } else {
                                                    {
                                                        let closes_3 = self::ml_array::slice(&closes, 0f64, ((closes.len() as f64) - 1f64));
                                                        {
                                                            let at_4 = at + 1f64;
                                                            {
                                                                (positions, closes, binding, at) = (positions_3, closes_3, binding_3, at_4);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if !({ let ml_value = closes.len() as f64; ml_value != 0f64 && !ml_value.is_nan() }) && (value == ",") {
                                                    {
                                                        let binding_4 = true;
                                                        {
                                                            let at_5 = at + 1f64;
                                                            {
                                                                (positions, binding, at) = (positions_3, binding_4, at_5);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
{
                                                    {
                                                        let at_6 = at + 1f64;
                                                        {
                                                            (positions, binding, at) = (positions_3, binding_3, at_6);
                                                            continue;
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
        } else {
            self::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Result::MlTemporaryPlanOrdinaryParameterBindingPositionsLoop6Done
        };
    }
}
