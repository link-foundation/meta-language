// Generated from complete js/src/grammar-scanner-inference-rules.js. Do not edit by hand.
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
pub fn read_grammar_scanner_observation_limit() -> f64 {
    256f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_scanner_observation_byte_limit() -> f64 {
    262_144f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_grammar_scanner_transitions(
    modes: &[String],
    spellings: &[String],
    tokens: &[String],
    next_modes: &[String],
) -> Vec<f64> {
    if (((((modes.len() as f64) == 0f64)
        || ((modes.len() as f64) > (self::read_grammar_scanner_observation_limit())))
        || ((modes.len() as f64) != (spellings.len() as f64)))
        || ((modes.len() as f64) != (tokens.len() as f64)))
        || ((modes.len() as f64) != (next_modes.len() as f64))
    {
        vec![-1f64]
    } else {
        {
            let unique = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_plan_grammar_scanner_transitions_loop1(
                            modes, spellings, tokens, next_modes, unique, index,
                        );
                    match source_ml_source1_s1 {
                        self::MlTemporaryPlanGrammarScannerTransitionsLoop1Result::MlTemporaryPlanGrammarScannerTransitionsLoop1Done(unique_2) => {
                            {
                                let plan = Vec::<f64>::new();
                                {
                                    let position = 0f64;
                                    {
                                        self::source_ml_temporary_plan_grammar_scanner_transitions_loop2(spellings, &(unique_2), plan, position)
                                    }
                                }
                            }
                        }
                        self::MlTemporaryPlanGrammarScannerTransitionsLoop1Result::MlTemporaryPlanGrammarScannerTransitionsLoop1Return(source_ml_temporary_result3) => {
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
pub fn read_grammar_scanner_inference_error(code: f64) -> String {
    if code == -1f64 {
        String::from("expected bounded nonempty scanner trace observations")
    } else if code == -2f64 {
        String::from(
            "conflicting scanner outputs for the same state and spelling require unsupported context sensitivity",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_grammar_scanner_definition(
    modes: &[String],
    spellings: &[String],
    tokens: &[String],
    next_modes: &[String],
    plan: &[f64],
    scanner: &str,
) -> String {
    {
        let declared_modes = vec![String::from("default")];
        {
            let declared_tokens = Vec::<String>::new();
            {
                let declarations = String::new();
                {
                    let operations = String::new();
                    {
                        let position = 0f64;
                        {
                            let source_ml_source1_s2 =
                                self::source_ml_temporary_render_grammar_scanner_definition_loop5(
                                    modes,
                                    spellings,
                                    tokens,
                                    next_modes,
                                    plan,
                                    declared_modes,
                                    declared_tokens,
                                    declarations,
                                    operations,
                                    position,
                                );
                            match source_ml_source1_s2 {
                                self::MlTemporaryRenderGrammarScannerDefinitionLoop5Result::MlTemporaryRenderGrammarScannerDefinitionLoop5Done(declared_tokens_2, declarations_2, operations_2) => {
                                    format!("{declarations_2}(scanner {} (tokens {}) (operations {operations_2}fail))\n", scanner.to_owned(), declared_tokens_2.join((String::from(" ")).as_str()))
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
pub fn source_ml_temporary_plan_grammar_scanner_transitions_loop3(
    spellings: &[String],
    unique: &[f64],
    plan: &[f64],
    mut selected: f64,
    mut length: f64,
    mut candidate: f64,
) -> f64 {
    loop {
        if candidate < (unique.len() as f64) {
            {
                let index = self::read_required_number_fact(unique, candidate);
                {
                    let size = (self::read_required_text_fact(spellings, index))
                        .encode_utf16()
                        .count() as f64;
                    if (size > length)
                        && !(self::ml_array::includes(
                            plan,
                            &index,
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        ))
                    {
                        {
                            let selected_2 = index;
                            {
                                let length_2 = size;
                                {
                                    let candidate_2 = candidate + 1f64;
                                    {
                                        (selected, length, candidate) =
                                            (selected_2, length_2, candidate_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    {
                        {
                            let candidate_3 = candidate + 1f64;
                            {
                                candidate = candidate_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { selected };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_grammar_scanner_transitions_loop2(
    spellings: &[String],
    unique: &[f64],
    mut plan: Vec<f64>,
    mut position: f64,
) -> Vec<f64> {
    loop {
        if position < (unique.len() as f64) {
            {
                let selected = -1f64;
                {
                    let length = 0f64;
                    {
                        let candidate = 0f64;
                        {
                            let selected_2 =
                                self::source_ml_temporary_plan_grammar_scanner_transitions_loop3(
                                    spellings,
                                    unique,
                                    &(plan),
                                    selected,
                                    length,
                                    candidate,
                                );
                            {
                                let plan_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![selected_2], plan);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let position_2 = position + 1f64;
                                    {
                                        (plan, position) = (plan_2, position_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { plan };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanGrammarScannerTransitionsLoop1Result {
    MlTemporaryPlanGrammarScannerTransitionsLoop1Done(Vec<f64>),
    MlTemporaryPlanGrammarScannerTransitionsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_grammar_scanner_transitions_loop4(
    modes: &[String],
    spellings: &[String],
    unique: &[f64],
    mode: &str,
    spelling: &str,
    mut previous: f64,
    mut position: f64,
) -> f64 {
    loop {
        if (position < (unique.len() as f64)) && (previous < 0f64) {
            {
                let candidate = self::read_required_number_fact(unique, position);
                if (mode == (self::read_required_text_fact(modes, candidate)))
                    && (spelling == (self::read_required_text_fact(spellings, candidate)))
                {
                    {
                        let previous_2 = candidate;
                        {
                            let position_2 = position + 1f64;
                            {
                                (previous, position) = (previous_2, position_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let position_3 = position + 1f64;
                        {
                            position = position_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { previous };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_grammar_scanner_transitions_loop1(
    modes: &[String],
    spellings: &[String],
    tokens: &[String],
    next_modes: &[String],
    mut unique: Vec<f64>,
    mut index: f64,
) -> self::MlTemporaryPlanGrammarScannerTransitionsLoop1Result {
    loop {
        return if index < (modes.len() as f64) {
            {
                let mode = self::read_required_text_fact(modes, index);
                {
                    let spelling = self::read_required_text_fact(spellings, index);
                    {
                        let token = self::read_required_text_fact(tokens, index);
                        {
                            let next = self::read_required_text_fact(next_modes, index);
                            if ((((((((mode).is_empty()) || (mode == "%"))
                                || ((spelling).is_empty()))
                                || (spelling == "%"))
                                || ((token).is_empty()))
                                || (token == "%"))
                                || ((next).is_empty()))
                                || (next == "%")
                            {
                                self::MlTemporaryPlanGrammarScannerTransitionsLoop1Result::MlTemporaryPlanGrammarScannerTransitionsLoop1Return(vec![-1f64])
                            } else {
                                {
                                    let previous = -1f64;
                                    {
                                        let position = 0f64;
                                        {
                                            let previous_2 = self::source_ml_temporary_plan_grammar_scanner_transitions_loop4(modes, spellings, &(unique), &(mode), &(spelling), previous, position);
                                            if previous_2 < 0f64 {
                                                {
                                                    let unique_2 = {
                                                        let (ml_append_right, ml_append_left) =
                                                            (vec![index], unique);
                                                        self::ml_array::append(
                                                            ml_append_left,
                                                            ml_append_right,
                                                        )
                                                    };
                                                    {
                                                        let index_2 = index + 1f64;
                                                        {
                                                            (unique, index) = (unique_2, index_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                            {
                                                if (token
                                                    != (self::read_required_text_fact(
                                                        tokens, previous_2,
                                                    )))
                                                    || (next
                                                        != (self::read_required_text_fact(
                                                            next_modes, previous_2,
                                                        )))
                                                {
                                                    self::MlTemporaryPlanGrammarScannerTransitionsLoop1Result::MlTemporaryPlanGrammarScannerTransitionsLoop1Return(vec![-2f64])
                                                } else {
                                                    {
                                                        let index_3 = index + 1f64;
                                                        {
                                                            index = index_3;
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
            self::MlTemporaryPlanGrammarScannerTransitionsLoop1Result::MlTemporaryPlanGrammarScannerTransitionsLoop1Done(unique)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderGrammarScannerDefinitionLoop5Result {
    MlTemporaryRenderGrammarScannerDefinitionLoop5Done(Vec<String>, String, String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderGrammarScannerDefinitionJoin7 {
    MlTemporaryRenderGrammarScannerDefinitionJoin7Next(Vec<String>, String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderGrammarScannerDefinitionJoin6 {
    MlTemporaryRenderGrammarScannerDefinitionJoin6Next(Vec<String>, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_grammar_scanner_definition_loop5(
    modes: &[String],
    spellings: &[String],
    tokens: &[String],
    next_modes: &[String],
    plan: &[f64],
    mut declared_modes: Vec<String>,
    mut declared_tokens: Vec<String>,
    mut declarations: String,
    mut operations: String,
    mut position: f64,
) -> self::MlTemporaryRenderGrammarScannerDefinitionLoop5Result {
    loop {
        return if position < (plan.len() as f64) {
            {
                let index = self::read_required_number_fact(plan, position);
                {
                    let mode = self::read_required_text_fact(modes, index);
                    {
                        let spelling = self::read_required_text_fact(spellings, index);
                        {
                            let token = self::read_required_text_fact(tokens, index);
                            {
                                let next = self::read_required_text_fact(next_modes, index);
                                {
                                    let source_ml_source1_s3 = if (self::ml_array::includes(
                                        &declared_modes,
                                        &mode.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    )) {
                                        self::MlTemporaryRenderGrammarScannerDefinitionJoin6::MlTemporaryRenderGrammarScannerDefinitionJoin6Next(declared_modes, declarations.clone())
                                    } else {
                                        {
                                            let declared_modes_2 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![mode.clone()], declared_modes);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let declarations_2 =
                                                    format!("{declarations}(mode {mode})\n");
                                                self::MlTemporaryRenderGrammarScannerDefinitionJoin6::MlTemporaryRenderGrammarScannerDefinitionJoin6Next(declared_modes_2, declarations_2)
                                            }
                                        }
                                    };
                                    match source_ml_source1_s3 {
                                        self::MlTemporaryRenderGrammarScannerDefinitionJoin6::MlTemporaryRenderGrammarScannerDefinitionJoin6Next(declared_modes_3, declarations_3) => {
                                            {
                                                let source_ml_source1_s4 = if (self::ml_array::includes(&declared_modes_3, &next.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::MlTemporaryRenderGrammarScannerDefinitionJoin7::MlTemporaryRenderGrammarScannerDefinitionJoin7Next(declared_modes_3, declarations_3.clone())
                                                } else {
                                                    {
                                                        let declared_modes_4 = { let (ml_append_right, ml_append_left) = (vec![next.clone()], declared_modes_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                        {
                                                            let declarations_4 = format!("{declarations_3}(mode {next})\n");
                                                            self::MlTemporaryRenderGrammarScannerDefinitionJoin7::MlTemporaryRenderGrammarScannerDefinitionJoin7Next(declared_modes_4, declarations_4)
                                                        }
                                                    }
                                                };
                                                match source_ml_source1_s4 {
                                                    self::MlTemporaryRenderGrammarScannerDefinitionJoin7::MlTemporaryRenderGrammarScannerDefinitionJoin7Next(declared_modes_5, declarations_5) => {
                                                        {
                                                            let declared_tokens_3 = if (self::ml_array::includes(&declared_tokens, &token.clone(), 0f64, false, |a, b| a == b)) {
                                                                declared_tokens
                                                            } else {
                                                                {
                                                                    { let (ml_append_right, ml_append_left) = (vec![token.clone()], declared_tokens); self::ml_array::append(ml_append_left, ml_append_right) }
                                                                }
                                                            };
                                                            {
                                                                let operations_2 = format!("{operations}(if (all (valid {token}) (equal mode (text {mode})) (next (literal {spelling}))) (then (consume (literal {spelling})) (setMode {next}) (emit {token}))) ");
                                                                {
                                                                    let position_2 = position + 1f64;
                                                                    {
                                                                        (declared_modes, declared_tokens, declarations, operations, position) = (declared_modes_5, declared_tokens_3, declarations_5.clone(), operations_2, position_2);
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
                    }
                }
            }
        } else {
            self::MlTemporaryRenderGrammarScannerDefinitionLoop5Result::MlTemporaryRenderGrammarScannerDefinitionLoop5Done(declared_tokens, declarations.clone(), operations.clone())
        };
    }
}
