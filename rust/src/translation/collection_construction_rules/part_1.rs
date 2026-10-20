// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
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
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Number fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required text fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Boolean fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_field(
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
                        let ml_s1 = self::ml_read_collection_construction_field_loop1(
                            labels, targets, field, start, count, at,
                        );
                        match ml_s1 {
                            self::MlReadCollectionConstructionFieldLoop1Result::MlReadCollectionConstructionFieldLoop1Done => {
                                -1f64
                            }
                            self::MlReadCollectionConstructionFieldLoop1Result::MlReadCollectionConstructionFieldLoop1Return(ml_result1) => {
                                ml_result1
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
pub fn read_collection_construction_element(
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
        let kind =
            self::read_collection_construction_field(starts, counts, labels, targets, node, "kind");
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
                    self::read_collection_construction_field(
                        starts, counts, labels, targets, expected, "element",
                    )
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
                    self::read_collection_construction_field(
                        starts, counts, labels, targets, expected, "key",
                    )
                } else {
                    -1f64
                },
                if context {
                    self::read_collection_construction_field(
                        starts, counts, labels, targets, expected, "value",
                    )
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
            let element = self::read_collection_construction_field(
                starts, counts, labels, targets, items, "element",
            );
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
                        let elements = self::read_collection_construction_field(
                            starts, counts, labels, targets, element, "elements",
                        );
                        if ((!array || !tuple) || (elements < 0f64))
                            || ((self::read_required_number_fact(counts, elements)) != 2f64)
                        {
                            vec![2f64, -1f64, -1f64]
                        } else {
                            vec![
                                0f64,
                                self::read_collection_construction_element(
                                    starts, counts, targets, elements, 0f64,
                                ),
                                self::read_collection_construction_element(
                                    starts, counts, targets, elements, 1f64,
                                ),
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadCollectionConstructionFieldLoop1Result {
    MlReadCollectionConstructionFieldLoop1Done,
    MlReadCollectionConstructionFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_collection_construction_field_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlReadCollectionConstructionFieldLoop1Result {
    loop {
        return if at < (start + count) {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlReadCollectionConstructionFieldLoop1Result::MlReadCollectionConstructionFieldLoop1Return(self::read_required_number_fact(targets, at))
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
            self::MlReadCollectionConstructionFieldLoop1Result::MlReadCollectionConstructionFieldLoop1Done
        };
    }
}
