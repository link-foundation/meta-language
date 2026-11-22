// Generated from complete js/src/translation/javascript-class-observation-rules.js. Do not edit by hand.
use super::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2;

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
pub fn dependency_function1(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    field: &str,
) -> f64 {
    if node < 0f64 {
        -1f64
    } else {
        {
            let start = self::read_required_number_fact(starts, node);
            {
                let count = self::read_required_number_fact(counts, node);
                {
                    let at = start;
                    {
                        let source_ml_source2_s1 =
                            self::source_ml_source1_temporary_dependency_function1_loop1(
                                labels, targets, field, start, count, at,
                            );
                        match source_ml_source2_s1 {
                            self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Done => {
                                -1f64
                            }
                            self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1
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
pub fn dependency_function2(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    node: f64,
    index: f64,
) -> f64 {
    if ((node < 0f64) || (index < 0f64))
        || (index >= (self::read_required_number_fact(counts, node)))
    {
        -1f64
    } else {
        self::read_required_number_fact(
            targets,
            (self::read_required_number_fact(starts, node) + index),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_kind(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> String {
    {
        let kind = self::dependency_function1(starts, counts, labels, targets, node, "kind");
        if kind < 0f64 {
            String::new()
        } else {
            self::read_required_text_fact(texts, kind)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_collection_construction_inference(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    kind: &str,
    expected: f64,
) -> Vec<f64> {
    {
        let context = (self::read_collection_construction_kind(
            texts, starts, counts, labels, targets, expected,
        )) == kind;
        if kind == "set" {
            vec![
                1f64,
                if context { 1f64 } else { 0f64 },
                if context {
                    self::dependency_function1(starts, counts, labels, targets, expected, "element")
                } else {
                    -1f64
                },
                -1f64,
            ]
        } else if kind == "map" {
            vec![
                2f64,
                if context { 1f64 } else { 0f64 },
                if context {
                    self::dependency_function1(starts, counts, labels, targets, expected, "key")
                } else {
                    -1f64
                },
                if context {
                    self::dependency_function1(starts, counts, labels, targets, expected, "value")
                } else {
                    -1f64
                },
            ]
        } else {
            vec![0f64, 0f64, -1f64, -1f64]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_collection_construction_checked(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    kind: &str,
    items: f64,
) -> Vec<f64> {
    {
        let array = (self::read_collection_construction_kind(
            texts, starts, counts, labels, targets, items,
        )) == "array";
        {
            let element =
                self::dependency_function1(starts, counts, labels, targets, items, "element");
            if kind == "set" {
                if array {
                    vec![0f64, element, -1f64]
                } else {
                    vec![1f64, -1f64, -1f64]
                }
            } else {
                {
                    let tuple = (self::read_collection_construction_kind(
                        texts, starts, counts, labels, targets, element,
                    )) == "tuple";
                    {
                        let elements = self::dependency_function1(
                            starts, counts, labels, targets, element, "elements",
                        );
                        if ((!array || !tuple) || (elements < 0f64))
                            || ((self::read_required_number_fact(counts, elements)) != 2f64)
                        {
                            vec![2f64, -1f64, -1f64]
                        } else {
                            vec![
                                0f64,
                                self::dependency_function2(starts, counts, targets, elements, 0f64),
                                self::dependency_function2(starts, counts, targets, elements, 1f64),
                            ]
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_key_admission(kind: &str, primitive: bool) -> f64 {
    if primitive {
        0f64
    } else if kind == "set" {
        3f64
    } else {
        4f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_diagnostic(code: f64, part: &str) -> String {
    if (part == "message") && (code == 1f64) {
        String::from("Set constructor needs an iterable array")
    } else if (part == "message") && (code == 2f64) {
        String::from("Map constructor needs an iterable of key/value pairs")
    } else if (part == "message") && (code == 3f64) {
        String::from("set keys need explicit reference identities for structured values")
    } else if (part == "message") && (code == 4f64) {
        String::from("map keys need explicit reference identities for structured values")
    } else if (part == "construct") && (code == 3f64) {
        String::from("Set object identity")
    } else if (part == "construct") && (code == 4f64) {
        String::from("Map object identity")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_binding(target: &str, kind: &str, role: &str) -> String {
    {
        let set = kind == "set";
        if target == "Rocq" {
            if role == "previous" {
                if set {
                    String::from("ml_previous")
                } else {
                    String::from("(fst ml_previous)")
                }
            } else if set {
                String::from("ml_value")
            } else {
                String::from("(fst ml_value)")
            }
        } else if role == "previous" {
            if set {
                String::from("ml_previous")
            } else {
                String::from("ml_previous.1")
            }
        } else if set {
            String::from("ml_value")
        } else {
            String::from("ml_value.1")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_construction_type(
    target: &str,
    kind: &str,
    first: &str,
    second: &str,
) -> String {
    if target == "Lean" {
        if kind == "set" {
            format!("(Array {})", first.to_owned())
        } else {
            format!("(Array ({} × {}))", first.to_owned(), second.to_owned())
        }
    } else if target == "Rocq" {
        if kind == "set" {
            format!("(list {})", first.to_owned())
        } else {
            format!("(list ({} * {}))%type", first.to_owned(), second.to_owned())
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_construction(
    target: &str,
    kind: &str,
    key_kind: &str,
    values: &str,
    collection_type: &str,
    element_type: &str,
    comparison: &str,
) -> String {
    {
        let set = kind == "set";
        if target == "JavaScript" {
            format!(
                "new {}({})",
                if set {
                    String::from("Set")
                } else {
                    String::from("Map")
                },
                values.to_owned()
            )
        } else if target == "Rust" {
            {
                let comparator = if key_kind == "float" {
                    String::from("|a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())")
                } else {
                    String::from("|a, b| a == b")
                };
                {
                    let values_3 = if key_kind == "float" {
                        {
                            format!(
                                "({}).into_iter().map(|ml_value| {}).collect::<Vec<_>>()",
                                values.to_owned(),
                                if set {
                                    String::from("if ml_value == 0f64 { 0f64 } else { ml_value }")
                                } else {
                                    String::from(
                                        "(if ml_value.0 == 0f64 { 0f64 } else { ml_value.0 }, ml_value.1)",
                                    )
                                }
                            )
                        }
                    } else {
                        values.to_owned()
                    };
                    format!(
                        "crate::ml_array::{}({values_3}, {comparator})",
                        kind.to_owned()
                    )
                }
            }
        } else {
            {
                let value = self::read_collection_construction_binding(target, kind, "value");
                {
                    let previous =
                        self::read_collection_construction_binding(target, kind, "previous");
                    if target == "Lean" {
                        {
                            let equal = if key_kind == "float" {
                                format!(
                                    "({previous} == {value} || ({previous}.isNaN && {value}.isNaN))"
                                )
                            } else {
                                format!("({previous} == {value})")
                            };
                            {
                                let normalized = if key_kind == "float" {
                                    if set {
                                        String::from(
                                            "(if ml_original == (0 : Float) then (0 : Float) else ml_original)",
                                        )
                                    } else {
                                        String::from(
                                            "((if ml_original.1 == (0 : Float) then (0 : Float) else ml_original.1), ml_original.2)",
                                        )
                                    }
                                } else {
                                    String::from("ml_original")
                                };
                                {
                                    let existing = if set {
                                        String::from("ml_accumulator")
                                    } else {
                                        format!(
                                            "(ml_accumulator.map (fun ml_previous => if {equal} then (ml_previous.1, ml_value.2) else ml_previous))"
                                        )
                                    };
                                    format!(
                                        "({}.foldl (fun ml_accumulator ml_original => let ml_value := {normalized}; if ml_accumulator.any (fun ml_previous => {equal}) then {existing} else ml_accumulator.push ml_value) (#[] : {}))",
                                        values.to_owned(),
                                        collection_type.to_owned()
                                    )
                                }
                            }
                        }
                    } else {
                        {
                            let equal_2 = comparison.to_owned();
                            {
                                let equal_4 = if key_kind == "float" {
                                    {
                                        format!(
                                            "(orb {equal_2} (andb (PrimFloat.is_nan {previous}) (PrimFloat.is_nan {value})))"
                                        )
                                    }
                                } else {
                                    equal_2
                                };
                                {
                                    let normalized_2 = if key_kind == "float" {
                                        if set {
                                            String::from(
                                                "(if PrimFloat.eqb ml_original 0%float then 0%float else ml_original)",
                                            )
                                        } else {
                                            String::from(
                                                "((if PrimFloat.eqb (fst ml_original) 0%float then 0%float else fst ml_original), snd ml_original)",
                                            )
                                        }
                                    } else {
                                        String::from("ml_original")
                                    };
                                    {
                                        let existing_2 = if set {
                                            String::from("ml_accumulator")
                                        } else {
                                            format!(
                                                "(List.map (fun ml_previous => if {equal_4} then (fst ml_previous, snd ml_value) else ml_previous) ml_accumulator)"
                                            )
                                        };
                                        format!(
                                            "(List.fold_left (fun ml_accumulator ml_original => let ml_value := {normalized_2} in if List.existsb (fun ml_previous => {equal_4}) ml_accumulator then {existing_2} else List.app ml_accumulator (ml_value :: nil)) {} (@nil {}))",
                                            values.to_owned(),
                                            element_type.to_owned()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_observation_operands(kind: &str, proof: &str) -> String {
    if self::ml_array::includes(
        &vec![
            String::from("eq"),
            String::from("ne"),
            String::from("lt"),
            String::from("le"),
            String::from("gt"),
            String::from("ge"),
        ],
        &proof.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("left right")
    } else if proof == "forall" {
        String::from("binders")
    } else if (kind == "toString") || (kind == "optionalText") {
        String::from("arg")
    } else if kind == "strictEqual" {
        String::from("left right")
    } else if (kind == "print") || (kind == "output") {
        String::from("expr")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_observation_operation_operands(
    kind: &str,
    proof: &str,
    operation: &str,
) -> String {
    if (kind == "binary")
        && self::ml_array::includes(
            &vec![String::from("eq"), String::from("ne")],
            &operation.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from("left right")
    } else {
        self::read_java_script_closed_class_observation_operands(kind, proof)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn contains_java_script_closed_class_brand(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    root: f64,
    declarations: f64,
    brands: &[String],
) -> bool {
    if (brands.len() as f64) == 0f64 {
        false
    } else {
        {
            let pending = vec![root];
            {
                let seen = Vec::<f64>::new();
                {
                    let source_ml_source2_s2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(texts, starts, counts, labels, targets, declarations, brands, pending, seen);
                    match source_ml_source2_s2 {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done => {
                            false
                        }
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction1Loop1Result {
    MlSource1TemporaryDependencyFunction1Loop1Done,
    MlSource1TemporaryDependencyFunction1Loop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function1_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryDependencyFunction1Loop1Result {
    loop {
        return if at < (start + count) {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Return(self::read_required_number_fact(targets, at))
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
            self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done,
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(Vec<f64>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    brands: &[String],
    mut pending: Vec<f64>,
    mut fields: f64,
    mut at: f64,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    loop {
        return if at < (self::read_required_number_fact(counts, fields)) {
            {
                let row = self::dependency_function2(starts, counts, targets, fields, at);
                {
                    let name =
                        self::dependency_function1(starts, counts, labels, targets, row, "name");
                    if (name < 0f64)
                        || (self::ml_array::includes(
                            brands,
                            &self::read_required_text_fact(texts, name),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))
                    {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                    } else {
                        {
                            let pending_2 = {
                                let source_ml_source2_concat_value3 = pending;
                                {
                                    let source_ml_source2_concat_value4 =
                                        vec![self::dependency_function1(
                                            starts, counts, labels, targets, row, "type",
                                        )];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value3,
                                        ),
                                        source_ml_source2_concat_value4,
                                    )
                                }
                            };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (pending, at) = (pending_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending)
        };
    }
}
