// Generated from complete js/src/translation/source-module-request-rules.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_module_request_term(term: &str) -> bool {
    ((term == "import_statement") || (term == "export_statement")) || (term == "call_expression")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_module_request_type_scope(term: &str) -> bool {
    self::ml_array::includes(
        &vec![
            String::from("type_alias_declaration"),
            String::from("type_annotation"),
            String::from("type_arguments"),
            String::from("type_parameters"),
            String::from("interface_declaration"),
            String::from("type_query"),
            String::from("ambient_declaration"),
        ],
        &term.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_module_request_literal(term: &str) -> bool {
    term == "string"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_module_request_import_callee(term: &str) -> bool {
    term == "import"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_module_request_kind(code: f64) -> String {
    if code == 1f64 {
        String::from("module import")
    } else if code == 2f64 {
        String::from("module export")
    } else if code == 3f64 {
        String::from("dynamic module import")
    } else if code == 4f64 {
        String::from("type module import")
    } else if code == 5f64 {
        String::from("type module export")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_module_request_code(
    language: &str,
    term: &str,
    second: &str,
    third: &str,
    context: bool,
) -> f64 {
    {
        let type_only = (language == "TypeScript")
            && (context
                || (((((second == "type") || (second == "typeof")) && (third != "from"))
                    && (third != ","))
                    && (third != "=")));
        if term == "call_expression" {
            if type_only { 4f64 } else { 3f64 }
        } else if term == "import_statement" {
            if type_only { 4f64 } else { 1f64 }
        } else if term == "export_statement" {
            if type_only { 5f64 } else { 2f64 }
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_module_request_grammar(language: &str, tsx: bool) -> String {
    if language == "JavaScript" {
        String::from("JavaScript")
    } else if language == "TypeScript" {
        if tsx {
            String::from("tsx")
        } else {
            String::from("TypeScript")
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_source_module_request_command(args: &[String]) -> Vec<f64> {
    {
        let file = -1f64;
        {
            let language = 0f64;
            {
                let tsx = false;
                {
                    let binary = false;
                    {
                        let ended = false;
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s1 = self::source_ml_temporary_plan_source_module_request_command_loop1(args, file, language, tsx, binary, ended, at);
                                match source_ml_source1_s1 {
                                    self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Done(file_2, language_2, tsx_2, binary_2) => {
                                        if (file_2 < 0f64) || (tsx_2 && (language_2 == 1f64)) {
                                            vec![-2f64]
                                        } else {
                                            {
                                                let path = (self::ml_array::optional_at(args, self::ml_array::number_index(file_2))).unwrap_or_else(|| String::new());
                                                {
                                                    let tsx_4 = if (language_2 != 1f64) && path.ends_with(".tsx") {
                                                        {
                                                            true
                                                        }
                                                    } else {
                                                        tsx_2
                                                    };
                                                    {
                                                        let language_4 = if language_2 == 0f64 {
                                                            {
                                                                if ((tsx_4 || path.ends_with(".ts")) || path.ends_with(".mts")) || path.ends_with(".cts") {
                                                                    2f64
                                                                } else {
                                                                    1f64
                                                                }
                                                            }
                                                        } else {
                                                            language_2
                                                        };
                                                        vec![file_2, language_4, if tsx_4 {
                                                            1f64
                                                        } else {
                                                            0f64
                                                        }, if binary_2 {
                                                            1f64
                                                        } else {
                                                            0f64
                                                        }]
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Return(source_ml_temporary_result1) => {
                                        source_ml_temporary_result1
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanSourceModuleRequestCommandLoop1Result {
    MlTemporaryPlanSourceModuleRequestCommandLoop1Done(f64, f64, bool, bool),
    MlTemporaryPlanSourceModuleRequestCommandLoop1Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanSourceModuleRequestCommandJoin2 {
    MlTemporaryPlanSourceModuleRequestCommandJoin2Next(f64),
    MlTemporaryPlanSourceModuleRequestCommandJoin2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_source_module_request_command_loop1(
    args: &[String],
    mut file: f64,
    mut language: f64,
    mut tsx: bool,
    mut binary: bool,
    mut ended: bool,
    mut at: f64,
) -> self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result {
    loop {
        return if at < (args.len() as f64) {
            {
                let arg = (self::ml_array::optional_at(args, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| String::new());
                if !ended && (((arg == "--help") || (arg == "-h")) || (arg == "help")) {
                    self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Return(vec![-1f64])
                } else {
                    if !ended && (arg == "--") {
                        {
                            let ended_2 = true;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (ended, at) = (ended_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if !ended && (arg == "--tsx") {
                            if tsx {
                                self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Return(vec![-2f64])
                            } else {
                                {
                                    let tsx_2 = true;
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (tsx, at) = (tsx_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended && (arg == "--binary") {
                            if binary {
                                self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Return(vec![-2f64])
                            } else {
                                {
                                    let binary_2 = true;
                                    {
                                        let at_4 = at + 1f64;
                                        {
                                            (binary, at) = (binary_2, at_4);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended && (arg == "--language") {
                            {
                                let ml_library_inequality_1 = language != 0f64;
                                if ml_library_inequality_1 {
                                    self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Return(vec![-2f64])
                                } else {
                                    {
                                        let at_5 = at + 1f64;
                                        {
                                            let value = (self::ml_array::optional_at(
                                                args,
                                                self::ml_array::number_index(at_5),
                                            ))
                                            .unwrap_or_else(|| String::new());
                                            {
                                                let source_ml_source1_s2 =
                                                    if self::ml_array::includes(
                                                        &vec![
                                                            String::from("JavaScript"),
                                                            String::from("javascript"),
                                                            String::from("js"),
                                                        ],
                                                        &value.clone(),
                                                        0f64,
                                                        false,
                                                        |a, b| a == b,
                                                    ) {
                                                        {
                                                            let language_2 = 1f64;
                                                            self::MlTemporaryPlanSourceModuleRequestCommandJoin2::MlTemporaryPlanSourceModuleRequestCommandJoin2Next(language_2)
                                                        }
                                                    } else if self::ml_array::includes(
                                                        &vec![
                                                            String::from("TypeScript"),
                                                            String::from("typescript"),
                                                            String::from("ts"),
                                                        ],
                                                        &value.clone(),
                                                        0f64,
                                                        false,
                                                        |a, b| a == b,
                                                    ) {
                                                        {
                                                            let language_3 = 2f64;
                                                            self::MlTemporaryPlanSourceModuleRequestCommandJoin2::MlTemporaryPlanSourceModuleRequestCommandJoin2Next(language_3)
                                                        }
                                                    } else {
                                                        self::MlTemporaryPlanSourceModuleRequestCommandJoin2::MlTemporaryPlanSourceModuleRequestCommandJoin2Return(vec![-2f64])
                                                    };
                                                match source_ml_source1_s2 {
                                                        self::MlTemporaryPlanSourceModuleRequestCommandJoin2::MlTemporaryPlanSourceModuleRequestCommandJoin2Next(language_4) => {
                                                            {
                                                                let at_6 = at_5 + 1f64;
                                                                {
                                                                    (language, at) = (language_4, at_6);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::MlTemporaryPlanSourceModuleRequestCommandJoin2::MlTemporaryPlanSourceModuleRequestCommandJoin2Return(source_ml_temporary_result2) => {
                                                            self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Return(source_ml_temporary_result2)
                                                        }
                                                    }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (file >= 0f64) || (!ended && arg.starts_with("-")) {
                            self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Return(vec![-2f64])
                        } else {
                            {
                                let file_2 = at;
                                {
                                    let at_7 = at + 1f64;
                                    {
                                        (file, at) = (file_2, at_7);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanSourceModuleRequestCommandLoop1Result::MlTemporaryPlanSourceModuleRequestCommandLoop1Done(file, language, tsx, binary)
        };
    }
}
