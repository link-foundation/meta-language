// Generated from complete js/src/translation/lexical-declaration-rules.js. Do not edit by hand.
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
pub fn find_java_script_lexical_declaration_conflict(
    parents: &[f64],
    boundaries: &[f64],
    parameter_scopes: &[f64],
    parameter_names: &[String],
    declaration_scopes: &[f64],
    declaration_names: &[String],
    declaration_kinds: &[f64],
) -> Vec<f64> {
    if ((((parents.len() as f64) != (boundaries.len() as f64))
        || ((parameter_scopes.len() as f64) != (parameter_names.len() as f64)))
        || ((declaration_scopes.len() as f64) != (declaration_names.len() as f64)))
        || ((declaration_kinds.len() as f64) != (declaration_names.len() as f64))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("inconsistent actual lexical declaration columns"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s1 =
                    self::source_ml_temporary_find_java_script_lexical_declaration_conflict_loop1(
                        parents,
                        boundaries,
                        parameter_scopes,
                        parameter_names,
                        declaration_scopes,
                        declaration_names,
                        declaration_kinds,
                        index,
                    );
                match source_ml_source1_s1 {
                    self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Done => {
                        Vec::<f64>::new()
                    }
                    self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Return(source_ml_temporary_result1) => {
                        source_ml_temporary_result1
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Result {
    MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Done,
    MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Result {
    MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Done,
    MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_lexical_declaration_conflict_loop3(
    parameter_scopes: &[f64],
    parameter_names: &[String],
    mut index: f64,
    name: &str,
    mut parent: f64,
    mut parameter: f64,
) -> self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Result {
    loop {
        return if parameter < (parameter_names.len() as f64) {
            if ((self::read_required_number_fact(parameter_scopes, parameter)) == parent)
                && ((self::read_required_text_fact(parameter_names, parameter)) == name)
            {
                self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Return(vec![index, 1f64])
            } else {
                {
                    let parameter_2 = parameter + 1f64;
                    {
                        parameter = parameter_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Result {
    MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Done,
    MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_find_java_script_lexical_declaration_conflict_loop2(
    declaration_scopes: &[f64],
    declaration_names: &[String],
    declaration_kinds: &[f64],
    mut index: f64,
    mut scope: f64,
    name: &str,
    mut kind: f64,
    mut previous: f64,
) -> self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Result {
    loop {
        return if previous < index {
            if (((self::read_required_number_fact(declaration_scopes, previous)) == scope)
                && ((self::read_required_text_fact(declaration_names, previous)) == name))
                && ((kind == 1f64)
                    || ((self::read_required_number_fact(declaration_kinds, previous)) == 1f64))
            {
                self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Return(vec![index, 0f64])
            } else {
                {
                    let previous_2 = previous + 1f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_find_java_script_lexical_declaration_conflict_loop1(
    parents: &[f64],
    boundaries: &[f64],
    parameter_scopes: &[f64],
    parameter_names: &[String],
    declaration_scopes: &[f64],
    declaration_names: &[String],
    declaration_kinds: &[f64],
    mut index: f64,
) -> self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Result {
    loop {
        return if index < (declaration_names.len() as f64) {
            {
                let scope = self::read_required_number_fact(declaration_scopes, index);
                {
                    let name = self::read_required_text_fact(declaration_names, index);
                    {
                        let kind = self::read_required_number_fact(declaration_kinds, index);
                        if (((scope < 0f64) || ((scope % 1f64) != 0f64))
                            || (scope >= (parents.len() as f64)))
                            || ((kind != 0f64) && (kind != 1f64))
                        {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("RangeError"),
                                message_units: (String::from(
                                    "invalid actual lexical declaration frame or kind",
                                ))
                                .encode_utf16()
                                .collect(),
                                message_present: true,
                            })
                            .raise()
                        } else {
                            {
                                let previous = 0f64;
                                {
                                    let source_ml_source1_s2 = self::source_ml_temporary_find_java_script_lexical_declaration_conflict_loop2(declaration_scopes, declaration_names, declaration_kinds, index, scope, &(name), kind, previous);
                                    match source_ml_source1_s2 {
                                        self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Done => {
                                            {
                                                let parent = self::read_required_number_fact(parents, scope);
                                                if (((kind == 1f64) && ((self::read_required_number_fact(boundaries, scope)) == 0f64)) && (parent >= 0f64)) && ((self::read_required_number_fact(boundaries, parent)) > 0f64) {
                                                    {
                                                        let parameter = 0f64;
                                                        {
                                                            let source_ml_source1_s3 = self::source_ml_temporary_find_java_script_lexical_declaration_conflict_loop3(parameter_scopes, parameter_names, index, &(name), parent, parameter);
                                                            match source_ml_source1_s3 {
                                                                self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Done => {
                                                                    {
                                                                        let index_2 = index + 1f64;
                                                                        {
                                                                            index = index_2;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                                self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop3Return(source_ml_temporary_result3) => {
                                                                    self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Return(source_ml_temporary_result3)
                                                                }
                                                            }
                                                        }
                                                    }
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
                                        self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop2Return(source_ml_temporary_result3_2) => {
                                            self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Return(source_ml_temporary_result3_2)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Result::MlTemporaryFindJavaScriptLexicalDeclarationConflictLoop1Done
        };
    }
}
