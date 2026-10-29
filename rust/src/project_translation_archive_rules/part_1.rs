// Generated from complete js/src/project-translation-archive-rules.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_project_translation_archive_vocabulary(role: &str) -> String {
    if role == "draft" {
        String::from("meta-language/project-translation/draft")
    } else if role == "configuration" {
        String::from("meta-language/project-translation/configuration")
    } else if role == "sources" {
        String::from("meta-language/project-translation/sources")
    } else if role == "interpretation" {
        String::from("independent source semantic translation")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_project_translation_archive_source_language(language: &str, path: &str) -> String {
    if (language == "TypeScript") && path.ends_with(".tsx") {
        String::from("TSX")
    } else {
        language.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_project_translation_archive_header_error(
    version: f64,
    interpretation: &str,
    inventory: f64,
    modules: f64,
    sources: f64,
) -> String {
    if ((((version == 1f64)
        && (interpretation
            == (self::read_project_translation_archive_vocabulary("interpretation"))))
        && (inventory > 0f64))
        && ((inventory).partial_cmp(&(modules)) == Some(std::cmp::Ordering::Equal)))
        && ((inventory).partial_cmp(&(sources)) == Some(std::cmp::Ordering::Equal))
    {
        String::new()
    } else {
        String::from("incomplete or unknown project translation archive")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_project_translation_archive_source_error(
    identifier: &str,
    language: &str,
    bytes: f64,
    digest: &str,
    path: &str,
    actual_language: &str,
    actual_bytes: f64,
    actual_digest: &str,
) -> String {
    if (((((identifier == path)
        && ((self::read_project_translation_archive_source_language(language, identifier))
            == actual_language))
        && self::ml_math::is_safe_integer(bytes))
        && (bytes >= 0f64))
        && ((bytes).partial_cmp(&(actual_bytes)) == Some(std::cmp::Ordering::Equal)))
        && (digest == actual_digest)
    {
        String::new()
    } else {
        String::from("project source identity, language, byte count or digest changed")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_project_translation_archive_product_error(
    draft: bool,
    executable: bool,
    gaps: f64,
    identical: bool,
    digest: bool,
) -> String {
    if executable && ((!draft || (gaps != 0f64)) || !identical) {
        String::from("an unresolved or changed project product cannot be executable")
    } else if digest {
        String::new()
    } else {
        String::from("project draft product digest changed")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_project_translation_archive_binding_error(
    issues: &[String],
    actual_issues: &[String],
    gap_issues: &[String],
    specifiers: &[String],
    providers: &[String],
    actual_specifiers: &[String],
    actual_providers: &[String],
) -> String {
    if (((((issues.len() as f64) != (actual_issues.len() as f64))
        || ((issues.len() as f64) != (gap_issues.len() as f64)))
        || ((specifiers.len() as f64) != (providers.len() as f64)))
        || ((specifiers.len() as f64) != (actual_specifiers.len() as f64)))
        || ((specifiers.len() as f64) != (actual_providers.len() as f64))
    {
        String::from("project source issues or dependency directory changed")
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s1 =
                    self::source_ml_temporary_read_project_translation_archive_binding_error_loop1(
                        issues,
                        actual_issues,
                        gap_issues,
                        index,
                    );
                match source_ml_source1_s1 {
                    self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Done => {
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source1_s2 = self::source_ml_temporary_read_project_translation_archive_binding_error_loop2(specifiers, providers, actual_specifiers, actual_providers, index_2);
                                match source_ml_source1_s2 {
                                    self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Done => {
                                        String::new()
                                    }
                                    self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Return(source_ml_temporary_result2) => {
                                        source_ml_temporary_result2
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Return(source_ml_temporary_result2_2) => {
                        source_ml_temporary_result2_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_project_translation_archive_command(args: &[String]) -> Vec<f64> {
    {
        let file = -1f64;
        {
            let target = -1f64;
            {
                let abstract_ = false;
                {
                    let binary = false;
                    {
                        let ended = false;
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s3 = self::source_ml_temporary_plan_project_translation_archive_command_loop3(args, file, target, abstract_, binary, ended, at);
                                match source_ml_source1_s3 {
                                    self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Done(file_2, target_2, abstract_2, binary_2) => {
                                        if (file_2 >= 0f64) && (target_2 >= 0f64) {
                                            vec![file_2, target_2, if abstract_2 {
                                                1f64
                                            } else {
                                                0f64
                                            }, if binary_2 {
                                                1f64
                                            } else {
                                                0f64
                                            }]
                                        } else {
                                            vec![-2f64]
                                        }
                                    }
                                    self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(source_ml_temporary_result3) => {
                                        source_ml_temporary_result3
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
pub fn read_project_translation_archive_configuration_error(
    version: f64,
    count: f64,
    inventory: f64,
    target: &str,
    selected: &str,
) -> String {
    if (((version == 1f64)
        && ((count).partial_cmp(&(inventory)) == Some(std::cmp::Ordering::Equal)))
        && (!(target).is_empty()))
        && (target == selected)
    {
        String::new()
    } else {
        String::from("project translation configuration changed")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Result {
    MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Done,
    MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_project_translation_archive_binding_error_loop2(
    specifiers: &[String],
    providers: &[String],
    actual_specifiers: &[String],
    actual_providers: &[String],
    mut index: f64,
) -> self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Result {
    loop {
        return if index < (specifiers.len() as f64) {
            if {
                let source_ml_source1_comparison_operand4 =
                    self::ml_array::optional_at(specifiers, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand5 = self::ml_array::optional_at(
                        actual_specifiers,
                        self::ml_array::number_index(index),
                    );
                    !(match (
                        source_ml_source1_comparison_operand4,
                        source_ml_source1_comparison_operand5,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } || {
                let source_ml_source1_comparison_operand6 =
                    self::ml_array::optional_at(providers, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand7 = self::ml_array::optional_at(
                        actual_providers,
                        self::ml_array::number_index(index),
                    );
                    !(match (
                        source_ml_source1_comparison_operand6,
                        source_ml_source1_comparison_operand7,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Return(String::from("project source issues or dependency directory changed"))
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result {
    MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Done,
    MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_project_translation_archive_binding_error_loop1(
    issues: &[String],
    actual_issues: &[String],
    gap_issues: &[String],
    mut index: f64,
) -> self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result {
    loop {
        return if index < (issues.len() as f64) {
            if {
                let source_ml_source1_comparison_operand8 =
                    self::ml_array::optional_at(issues, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand9 = self::ml_array::optional_at(
                        actual_issues,
                        self::ml_array::number_index(index),
                    );
                    !(match (
                        source_ml_source1_comparison_operand8,
                        source_ml_source1_comparison_operand9,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } || {
                let source_ml_source1_comparison_operand10 =
                    self::ml_array::optional_at(issues, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand11 = self::ml_array::optional_at(
                        gap_issues,
                        self::ml_array::number_index(index),
                    );
                    !(match (
                        source_ml_source1_comparison_operand10,
                        source_ml_source1_comparison_operand11,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Return(String::from("project source issues or dependency directory changed"))
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Result::MlTemporaryReadProjectTranslationArchiveBindingErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result {
    MlTemporaryPlanProjectTranslationArchiveCommandLoop3Done(f64, f64, bool, bool),
    MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_project_translation_archive_command_loop3(
    args: &[String],
    mut file: f64,
    mut target: f64,
    mut abstract_: bool,
    mut binary: bool,
    mut ended: bool,
    mut at: f64,
) -> self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result {
    loop {
        return if at < (args.len() as f64) {
            {
                let arg = (self::ml_array::optional_at(args, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| String::new());
                if !ended && (((arg == "--help") || (arg == "-h")) || (arg == "help")) {
                    self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-1f64])
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
                        if !ended && (arg == "--abstract") {
                            if abstract_ {
                                self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-2f64])
                            } else {
                                {
                                    let abstract_2 = true;
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (abstract_, at) = (abstract_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended && (arg == "--binary") {
                            if binary {
                                self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-2f64])
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
                        } else if !ended && (arg == "--to") {
                            if (target >= 0f64) || ((at + 1f64) >= (args.len() as f64)) {
                                self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-2f64])
                            } else {
                                {
                                    let at_5 = at + 1f64;
                                    {
                                        let target_2 = at_5;
                                        {
                                            let at_6 = at_5 + 1f64;
                                            {
                                                (target, at) = (target_2, at_6);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (file >= 0f64) || (!ended && arg.starts_with("-")) {
                            self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Return(vec![-2f64])
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
            self::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Result::MlTemporaryPlanProjectTranslationArchiveCommandLoop3Done(file, target, abstract_, binary)
        };
    }
}
