// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::ml_array;
use super::read_array_from_element;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_from_mapping(
    target: &str,
    object: &str,
    callback: &str,
    parameter_count: f64,
    suffix: &str,
) -> String {
    if target == "JavaScript" {
        format!("({}).map({})", object.to_owned(), callback.to_owned())
    } else if target == "Rust" {
        {
            let values = format!("ml_values{}", suffix.to_owned());
            {
                let fn_ = format!("ml_callback{}", suffix.to_owned());
                {
                    let index = format!("ml_index{}", suffix.to_owned());
                    {
                        let value = format!("ml_value{}", suffix.to_owned());
                        {
                            let available = vec![
                                value.clone(),
                                format!("({index} as f64)"),
                                format!("{values}.clone()"),
                            ];
                            {
                                let call = format!(
                                    "{fn_}({})",
                                    (self::ml_array::slice(&available, 0f64, parameter_count))
                                        .join((String::from(", ")).as_str())
                                );
                                {
                                    let head = format!(
                                        "let {values} = {}; let {fn_} = {};",
                                        object.to_owned(),
                                        callback.to_owned()
                                    );
                                    format!(
                                        "{{ {head} {values}.iter().cloned().enumerate().map(|({index}, {value})| {call}).collect::<Vec<_>>() }}"
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if target == "Lean" {
        {
            let available_2 = vec![
                String::from("ml_pair.1"),
                String::from("(Float.ofNat ml_pair.2)"),
                String::from("ml_values"),
            ];
            {
                let call_2 = {
                    let ml_library_condition_3 = {
                        let ml_value = parameter_count;
                        ml_value != 0f64 && !ml_value.is_nan()
                    };
                    if ml_library_condition_3 {
                        format!(
                            "(ml_callback {})",
                            (self::ml_array::slice(&available_2, 0f64, parameter_count))
                                .join((String::from(" ")).as_str())
                        )
                    } else {
                        String::from("ml_callback")
                    }
                };
                {
                    let head_2 = format!(
                        "(let ml_values := {}; let ml_callback := {}; ",
                        object.to_owned(),
                        callback.to_owned()
                    );
                    format!(
                        "{head_2} (ml_values.toList.zipIdx.map (fun ml_pair => {call_2})).toArray)"
                    )
                }
            }
        }
    } else if target == "Rocq" {
        {
            let available_3 = vec![
                String::from("(fst ml_pair)"),
                String::from("(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (snd ml_pair))))"),
                String::from("ml_values"),
            ];
            {
                let call_3 = {
                    let ml_library_condition_2 = {
                        let ml_value = parameter_count;
                        ml_value != 0f64 && !ml_value.is_nan()
                    };
                    if ml_library_condition_2 {
                        format!(
                            "(ml_callback {})",
                            (self::ml_array::slice(&available_3, 0f64, parameter_count))
                                .join((String::from(" ")).as_str())
                        )
                    } else {
                        String::from("ml_callback")
                    }
                };
                {
                    let head_3 = format!(
                        "(let ml_values := {} in let ml_callback := {} in ",
                        object.to_owned(),
                        callback.to_owned()
                    );
                    format!(
                        "{head_3} List.map (fun ml_pair => {call_3}) (List.combine ml_values (List.seq 0 (List.length ml_values))))"
                    )
                }
            }
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_formal_array_callback_operation(operation: &str) -> bool {
    self::ml_array::includes(
        &vec![
            String::from("filter"),
            String::from("find"),
            String::from("findIndex"),
            String::from("some"),
            String::from("every"),
            String::from("flatMap"),
            String::from("reduce"),
        ],
        &operation.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_array_callback_operation(
    target: &str,
    operation: &str,
    object: &str,
    callback: &str,
    parameter_count: f64,
    initial: &str,
    element: &str,
) -> String {
    {
        let reduction = operation == "reduce";
        {
            let lean = target == "Lean";
            {
                let value = if lean {
                    String::from("ml_pair.1")
                } else {
                    String::from("(fst ml_pair)")
                };
                {
                    let index = if lean {
                        String::from("(Float.ofNat ml_pair.2)")
                    } else {
                        String::from("(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (snd ml_pair))))")
                    };
                    {
                        let arguments_text = (self::ml_array::slice(
                            &if reduction {
                                vec![
                                    String::from("ml_accumulator"),
                                    value.clone(),
                                    index.clone(),
                                    String::from("ml_values"),
                                ]
                            } else {
                                vec![value.clone(), index.clone(), String::from("ml_values")]
                            },
                            0f64,
                            parameter_count,
                        ))
                        .join((String::from(" ")).as_str());
                        {
                            let call = {
                                let ml_library_condition_1 = {
                                    let ml_value = parameter_count;
                                    ml_value != 0f64 && !ml_value.is_nan()
                                };
                                if ml_library_condition_1 {
                                    format!("(ml_callback {arguments_text})")
                                } else {
                                    String::from("ml_callback")
                                }
                            };
                            {
                                let source_ml_source2_s6 = if lean {
                                    {
                                        let head = format!(
                                            "(let ml_values := {}; let ml_callback := {}; ",
                                            object.to_owned(),
                                            callback.to_owned()
                                        );
                                        {
                                            let pairs = String::from("ml_values.toList.zipIdx");
                                            {
                                                let predicate = format!("(fun ml_pair => {call})");
                                                if operation == "flatMap" {
                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Return(format!("{head} ({pairs}.flatMap (fun ml_pair => {call}.toList)).toArray)"))
                                                } else if (operation == "some")
                                                    || (operation == "every")
                                                {
                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Return(format!("{head} {pairs}.{} {predicate})", if operation == "some" {
                                                            String::from("any")
                                                        } else {
                                                            String::from("all")
                                                        }))
                                                } else if operation == "filter" {
                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Return(format!("{head} (({pairs}.filter {predicate}).map (fun ml_pair => ml_pair.1)).toArray)"))
                                                } else if operation == "find" {
                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Return(format!("{head} ({pairs}.find? {predicate}).map (fun ml_pair => ml_pair.1))"))
                                                } else if operation == "findIndex" {
                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Return(format!("{head} (({pairs}.find? {predicate}).map (fun ml_pair => Float.ofNat ml_pair.2)).getD (-1 : Float))"))
                                                } else if reduction {
                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Return(format!("{head} {pairs}.foldl (fun ml_accumulator ml_pair => {call}) {})", initial.to_owned()))
                                                } else {
                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Next
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Next
                                };
                                match source_ml_source2_s6 {
                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Next => {
                                        {
                                            let source_ml_source2_s7 = if target == "Rocq" {
                                                {
                                                    let head_2 = format!("(let ml_values := {} in let ml_callback := {} in ", object.to_owned(), callback.to_owned());
                                                    {
                                                        let pairs_2 = String::from("(List.combine ml_values (List.seq 0 (List.length ml_values)))");
                                                        {
                                                            let predicate_2 = format!("(fun ml_pair => {call})");
                                                            if operation == "flatMap" {
                                                                self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Return(format!("{head_2} List.flat_map {predicate_2} {pairs_2})"))
                                                            } else if (operation == "some") || (operation == "every") {
                                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Return(format!("{head_2} List.{} {predicate_2} {pairs_2})", if operation == "some" {
                                                                        String::from("existsb")
                                                                    } else {
                                                                        String::from("forallb")
                                                                    }))
                                                                } else {
                                                                    {
                                                                        let first = format!("(@fst {} nat)", element.to_owned());
                                                                        if operation == "filter" {
                                                                            self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Return(format!("{head_2} List.map {first} (List.filter {predicate_2} {pairs_2}))"))
                                                                        } else if operation == "find" {
                                                                                self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Return(format!("{head_2} option_map {first} (List.find {predicate_2} {pairs_2}))"))
                                                                            } else if operation == "findIndex" {
                                                                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Return(format!("{head_2} match List.find {predicate_2} {pairs_2} with Some ml_pair => {index} | None => (PrimFloat.opp 1%float) end)"))
                                                                                } else if reduction {
                                                                                        self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Return(format!("{head_2} List.fold_left (fun ml_accumulator ml_pair => {call}) {pairs_2} {})", initial.to_owned()))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Next
                                                                                    }
                                                                    }
                                                                }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Next
                                            };
                                            match source_ml_source2_s7 {
                                                self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Next => {
                                                    String::new()
                                                }
                                                self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Return(source_ml_source1_temporary_result5) => {
                                                    source_ml_source1_temporary_result5
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4::MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Return(source_ml_source1_temporary_result5_2) => {
                                        source_ml_source1_temporary_result5_2
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
pub enum MlSource1TemporaryReadArrayFromFieldLoop1Result {
    MlSource1TemporaryReadArrayFromFieldLoop1Done,
    MlSource1TemporaryReadArrayFromFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_from_field_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadArrayFromFieldLoop1Result {
    loop {
        return if at < (start + count) {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlSource1TemporaryReadArrayFromFieldLoop1Result::MlSource1TemporaryReadArrayFromFieldLoop1Return(self::read_required_number_fact(targets, at))
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
            self::MlSource1TemporaryReadArrayFromFieldLoop1Result::MlSource1TemporaryReadArrayFromFieldLoop1Done
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
pub fn source_ml_source1_temporary_plan_array_from_callback_constraints_loop2(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut params: f64,
    mut declared_params: f64,
    mut count: f64,
    mut pairs: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let pairs_2 = {
                    let source_ml_source2_concat_value8 = pairs;
                    {
                        let source_ml_source2_concat_value9 = vec![
                            self::read_array_from_element(
                                starts,
                                counts,
                                targets,
                                declared_params,
                                index,
                            ),
                            self::read_array_from_element(starts, counts, targets, params, index),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value8,
                            ),
                            source_ml_source2_concat_value9,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (pairs, index) = (pairs_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { pairs };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_array_from_map_callback_loop3(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut params: f64,
    mut declared: f64,
    mut count: f64,
    mut plan: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let plan_2 = {
                    let source_ml_source2_concat_value10 = plan;
                    {
                        let source_ml_source2_concat_value11 = vec![
                            self::read_array_from_element(starts, counts, targets, declared, index),
                            self::read_array_from_element(starts, counts, targets, params, index),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value10,
                            ),
                            source_ml_source2_concat_value11,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (plan, index) = (plan_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { plan };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5 {
    MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Next,
    MlSource1TemporaryRenderFormalArrayCallbackOperationJoin5Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4 {
    MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Next,
    MlSource1TemporaryRenderFormalArrayCallbackOperationJoin4Return(String),
}
