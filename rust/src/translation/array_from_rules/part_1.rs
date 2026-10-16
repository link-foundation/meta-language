// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::*;

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
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
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
    fn integer(value: f64) -> f64 {
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
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Number fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required text fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Boolean fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_optional_array_read(
    language: &str,
    optional: bool,
    proven: bool,
    expected_optional: bool,
) -> bool {
    (optional || ((language == "JavaScript") && !proven)) || expected_optional
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_optional_array_index_number(language: &str, kind: &str, element: &str) -> bool {
    ((language == "JavaScript") && (kind == "option"))
        && ((element == "float") || (element == "unit"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_iterable_form(kind: &str, reference: bool) -> String {
    if kind == "array" {
        String::from("array")
    } else if kind == "string" {
        String::from("string")
    } else if reference && ((kind == "set") || (kind == "map")) {
        String::from("mutable")
    } else if kind == "set" {
        String::from("set")
    } else if kind == "map" {
        String::from("map")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_mutation_diagnostic(field: &str) -> String {
    if field == "feature" {
        String::from("reference mutation")
    } else if field == "reason" {
        String::from(
            "this mutation requires its shared reference and slot operations to be represented",
        )
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_array_read(
    target: &str,
    kind: &str,
    values: &str,
    position: &str,
) -> String {
    if target == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), values.to_owned())),
                    (String::from(")["))
                )),
                position.to_owned()
            )),
            (String::from("]"))
        )
    } else if target == "Rust" {
        {
            let index = if kind == "float" {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("self::ml_array::number_index(")),
                        position.to_owned()
                    )),
                    (String::from(")"))
                )
            } else {
                format!("{}{}", position.to_owned(), (String::from(".to_index()")))
            };
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("self::ml_array::optional_at(")),
                            values.to_owned()
                        )),
                        (String::from(", "))
                    )),
                    index
                )),
                (String::from(")"))
            )
        }
    } else if target == "Lean" {
        if kind == "float" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("(let ml_index := ")),
                            position.to_owned()
                        )),
                        (String::from(
                            "; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then "
                        ))
                    )),
                    values.to_owned()
                )),
                (String::from("[ml_index.toUInt64.toNat]? else none)"))
            )
        } else if kind == "nat" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(")), values.to_owned())),
                        (String::from("["))
                    )),
                    position.to_owned()
                )),
                (String::from("]?)"))
            )
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("(let ml_index := ")),
                            position.to_owned()
                        )),
                        (String::from("; if ml_index < 0 then none else "))
                    )),
                    values.to_owned()
                )),
                (String::from("[ml_index.toNat]?)"))
            )
        }
    } else {
        {
            let index_2 = if kind == "float" {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("(ml_float_index ")),
                        position.to_owned()
                    )),
                    (String::from(")"))
                )
            } else if kind == "nat" {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(Z.of_N ")), position.to_owned())),
                    (String::from(")"))
                )
            } else {
                position.to_owned()
            };
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(let ml_index := ")), index_2)),
                        (String::from(" in if Z.ltb ml_index 0 then None else List.nth_error "))
                    )),
                    values.to_owned()
                )),
                (String::from(" (Z.to_nat ml_index))"))
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_string_point_iteration(target: &str, utf16: bool, value: &str) -> String {
    if target == "JavaScript" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("[...")), value.to_owned())),
            (String::from("]"))
        )
    } else if target == "Rust" {
        if utf16 {
            format!("{}{}", value.to_owned(), (String::from(".code_points()")))
        } else {
            format!(
                "{}{}",
                value.to_owned(),
                (String::from(
                    ".chars().map(|ml_character| ml_character.to_string()).collect::<Vec<String>>()"
                ))
            )
        }
    } else if target == "Lean" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("((")), value.to_owned())),
            (String::from(
                ".toList.map (fun ml_character => String.singleton ml_character)).toArray)"
            ))
        )
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_iteration(
    target: &str,
    map: bool,
    operation: &str,
    value: &str,
    key_type: &str,
    value_type: &str,
) -> String {
    if target == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("[...(")), value.to_owned())),
                    (String::from(")."))
                )),
                operation.to_owned()
            )),
            (String::from("()]"))
        )
    } else if target == "Rust" {
        if operation == "entries" {
            if map {
                value.to_owned()
            } else {
                format!(
                    "{}{}",
                    value.to_owned(),
                    (String::from(
                        ".iter().map(|ml_value| (ml_value.clone(), ml_value.clone())).collect::<Vec<_>>()"
                    ))
                )
            }
        } else if map {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        value.to_owned(),
                        (String::from(".iter().map(|ml_pair| ml_pair."))
                    )),
                    (if operation == "keys" {
                        String::from("0")
                    } else {
                        String::from("1")
                    })
                )),
                (String::from(".clone()).collect::<Vec<_>>()"))
            )
        } else {
            value.to_owned()
        }
    } else if target == "Lean" {
        if operation == "entries" {
            if map {
                value.to_owned()
            } else {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), value.to_owned())),
                    (String::from(".map (fun ml_value => (ml_value, ml_value)))"))
                )
            }
        } else if map {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(")), value.to_owned())),
                        (String::from(".map (fun ml_pair => ml_pair."))
                    )),
                    (if operation == "keys" {
                        String::from("1")
                    } else {
                        String::from("2")
                    })
                )),
                (String::from("))"))
            )
        } else {
            value.to_owned()
        }
    } else if operation == "entries" {
        if map {
            value.to_owned()
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(List.map (fun ml_value => (ml_value, ml_value)) ")),
                    value.to_owned()
                )),
                (String::from(")"))
            )
        }
    } else if map {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (String::from("(List.map (@")),
                                        (if operation == "keys" {
                                            String::from("fst")
                                        } else {
                                            String::from("snd")
                                        })
                                    )),
                                    (String::from(" "))
                                )),
                                key_type.to_owned()
                            )),
                            (String::from(" "))
                        )),
                        value_type.to_owned()
                    )),
                    (String::from(") "))
                )),
                value.to_owned()
            )),
            (String::from(")"))
        )
    } else {
        value.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_from_field(
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
                        let ml_s1 = self::ml_read_array_from_field_loop1(
                            labels, targets, field, start, count, at,
                        );
                        match ml_s1 {
                            self::MlReadArrayFromFieldLoop1Result::MlReadArrayFromFieldLoop1Done => {
                                -1f64
                            }
                            self::MlReadArrayFromFieldLoop1Result::MlReadArrayFromFieldLoop1Return(ml_result1) => {
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
pub fn read_array_from_element(
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
pub fn read_array_from_kind(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> String {
    {
        let kind = self::read_array_from_field(starts, counts, labels, targets, node, "kind");
        if kind < 0f64 {
            String::from("")
        } else {
            self::read_required_text_fact(texts, kind)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_from_argument_error(count: f64) -> String {
    if (count < 1f64) || (count > 2f64) {
        String::from("Array.from expects one or two arguments")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_from_receiver_error(kind: &str) -> String {
    if (self::read_array_iterable_form(kind, false)).is_empty() {
        String::from("this iterable needs an operational iterator encoding")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_array_from_iterable_type(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    object: f64,
) -> Vec<f64> {
    {
        let form = self::read_array_iterable_form(
            &(self::read_array_from_kind(texts, starts, counts, labels, targets, object)),
            false,
        );
        if form == "array" {
            vec![
                0f64,
                self::read_array_from_field(starts, counts, labels, targets, object, "element"),
            ]
        } else if form == "string" {
            vec![1f64]
        } else if form == "set" {
            vec![
                2f64,
                self::read_array_from_field(starts, counts, labels, targets, object, "element"),
            ]
        } else if form == "map" {
            vec![
                3f64,
                self::read_array_from_field(starts, counts, labels, targets, object, "key"),
                self::read_array_from_field(starts, counts, labels, targets, object, "value"),
            ]
        } else {
            Vec::<f64>::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_array_from_expected_element(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    expected: f64,
) -> Vec<f64> {
    if (self::read_array_from_kind(texts, starts, counts, labels, targets, expected)) == "array" {
        vec![
            1f64,
            self::read_array_from_field(starts, counts, labels, targets, expected, "element"),
        ]
    } else {
        vec![0f64, -1f64]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_array_from_inference_inputs(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    args: f64,
    expected: f64,
) -> Vec<f64> {
    self::ml_array::append(
        self::plan_array_from_expected_element(texts, starts, counts, labels, targets, expected),
        vec![
            self::read_array_from_element(starts, counts, targets, args, 0f64),
            self::read_array_from_element(starts, counts, targets, args, 1f64),
        ],
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_from_callback_error(declared: f64, provided: f64) -> String {
    if declared > provided {
        String::from("a callback declares more parameters than its operation supplies")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_array_from_callback_constraints(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    term: f64,
    declared: f64,
    params: f64,
    ret: f64,
) -> Vec<f64> {
    if (self::read_array_from_kind(texts, starts, counts, labels, targets, declared)) != "function"
    {
        vec![1f64, term, params, ret]
    } else {
        {
            let declared_params =
                self::read_array_from_field(starts, counts, labels, targets, declared, "params");
            {
                let count = self::read_required_number_fact(counts, declared_params);
                if (self::read_array_from_callback_error(
                    count,
                    self::read_required_number_fact(counts, params),
                ))
                .is_empty()
                {
                    {
                        let pairs = vec![0f64];
                        {
                            let index = 0f64;
                            {
                                let pairs_2 = self::ml_plan_array_from_callback_constraints_loop2(
                                    starts,
                                    counts,
                                    targets,
                                    params,
                                    declared_params,
                                    count,
                                    pairs,
                                    index,
                                );
                                {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![
                                            self::read_array_from_field(
                                                starts, counts, labels, targets, declared, "ret",
                                            ),
                                            ret,
                                        ],
                                        pairs_2,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                }
                            }
                        }
                    }
                } else {
                    vec![-1f64]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_array_from_map_callback(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    callback: f64,
    params: f64,
) -> Vec<f64> {
    if (self::read_array_from_kind(texts, starts, counts, labels, targets, callback)) != "function"
    {
        vec![0f64, -1f64]
    } else {
        {
            let declared =
                self::read_array_from_field(starts, counts, labels, targets, callback, "params");
            {
                let count = self::read_required_number_fact(counts, declared);
                if (self::read_array_from_callback_error(
                    count,
                    self::read_required_number_fact(counts, params),
                ))
                .is_empty()
                {
                    {
                        let plan = vec![
                            2f64,
                            self::read_array_from_field(
                                starts, counts, labels, targets, callback, "ret",
                            ),
                        ];
                        {
                            let index = 0f64;
                            {
                                self::ml_plan_array_from_map_callback_loop3(
                                    starts, counts, targets, params, declared, count, plan, index,
                                )
                            }
                        }
                    }
                } else {
                    vec![1f64, -1f64]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_from_map_callback_error(status: f64, parameters_equal: bool) -> String {
    if status == 0f64 {
        String::from(".map() requires a function")
    } else if (status == 1f64) || !parameters_equal {
        String::from(".map() callback parameters differ")
    } else {
        String::from("")
    }
}
