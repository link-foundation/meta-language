// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::*;

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
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), object.to_owned())),
                    (String::from(").map("))
                )),
                callback.to_owned()
            )),
            (String::from(")"))
        )
    } else if target == "Rust" {
        {
            let values = format!("{}{}", (String::from("ml_values")), suffix.to_owned());
            {
                let fn_ = format!("{}{}", (String::from("ml_callback")), suffix.to_owned());
                {
                    let index = format!("{}{}", (String::from("ml_index")), suffix.to_owned());
                    {
                        let value = format!("{}{}", (String::from("ml_value")), suffix.to_owned());
                        {
                            let available = vec![
                                value.clone(),
                                format!(
                                    "{}{}",
                                    (format!("{}{}", (String::from("(")), index)),
                                    (String::from(" as f64)"))
                                ),
                                format!("{}{}", values, (String::from(".clone()"))),
                            ];
                            {
                                let call = format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!("{}{}", fn_, (String::from("(")))),
                                        ((self::ml_array::slice(
                                            &available,
                                            0f64,
                                            parameter_count
                                        ))
                                        .join((String::from(", ")).as_str()))
                                    )),
                                    (String::from(")"))
                                );
                                {
                                    let head = format!(
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
                                                                    (String::from("let ")),
                                                                    values
                                                                )),
                                                                (String::from(" = "))
                                                            )),
                                                            object.to_owned()
                                                        )),
                                                        (String::from("; let "))
                                                    )),
                                                    fn_
                                                )),
                                                (String::from(" = "))
                                            )),
                                            callback.to_owned()
                                        )),
                                        (String::from(";"))
                                    );
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
                                                                    (format!(
                                                                        "{}{}",
                                                                        (format!(
                                                                            "{}{}",
                                                                            (String::from("{ ")),
                                                                            head
                                                                        )),
                                                                        (String::from(" "))
                                                                    )),
                                                                    values
                                                                )),
                                                                (String::from(
                                                                    ".iter().cloned().enumerate().map(|("
                                                                ))
                                                            )),
                                                            index
                                                        )),
                                                        (String::from(", "))
                                                    )),
                                                    value
                                                )),
                                                (String::from(")| "))
                                            )),
                                            call
                                        )),
                                        (String::from(").collect::<Vec<_>>() }"))
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
                let call_2 = if {
                    let ml_value = parameter_count;
                    ml_value != 0f64 && !ml_value.is_nan()
                } {
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("(ml_callback ")),
                            ((self::ml_array::slice(&available_2, 0f64, parameter_count))
                                .join((String::from(" ")).as_str()))
                        )),
                        (String::from(")"))
                    )
                } else {
                    String::from("ml_callback")
                };
                {
                    let head_2 = format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (String::from("(let ml_values := ")),
                                    object.to_owned()
                                )),
                                (String::from("; let ml_callback := "))
                            )),
                            callback.to_owned()
                        )),
                        (String::from("; "))
                    );
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                head_2,
                                (String::from(" (ml_values.toList.zipIdx.map (fun ml_pair => "))
                            )),
                            call_2
                        )),
                        (String::from(")).toArray)"))
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
                let call_3 = if {
                    let ml_value = parameter_count;
                    ml_value != 0f64 && !ml_value.is_nan()
                } {
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("(ml_callback ")),
                            ((self::ml_array::slice(&available_3, 0f64, parameter_count))
                                .join((String::from(" ")).as_str()))
                        )),
                        (String::from(")"))
                    )
                } else {
                    String::from("ml_callback")
                };
                {
                    let head_3 = format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (String::from("(let ml_values := ")),
                                    object.to_owned()
                                )),
                                (String::from(" in let ml_callback := "))
                            )),
                            callback.to_owned()
                        )),
                        (String::from(" in "))
                    );
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!("{}{}", head_3, (String::from(" List.map (fun ml_pair => ")))),
                            call_3
                        )),
                        (String::from(
                            ") (List.combine ml_values (List.seq 0 (List.length ml_values))))"
                        ))
                    )
                }
            }
        }
    } else {
        String::from("")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadArrayFromFieldLoop1Result {
    MlReadArrayFromFieldLoop1Done,
    MlReadArrayFromFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_array_from_field_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlReadArrayFromFieldLoop1Result {
    loop {
        return if at < (start + count) {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlReadArrayFromFieldLoop1Result::MlReadArrayFromFieldLoop1Return(
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
            self::MlReadArrayFromFieldLoop1Result::MlReadArrayFromFieldLoop1Done
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
pub fn ml_plan_array_from_callback_constraints_loop2(
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
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::read_array_from_element(
                                starts,
                                counts,
                                targets,
                                declared_params,
                                index,
                            ),
                            self::read_array_from_element(starts, counts, targets, params, index),
                        ],
                        pairs,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
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
pub fn ml_plan_array_from_map_callback_loop3(
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
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::read_array_from_element(starts, counts, targets, declared, index),
                            self::read_array_from_element(starts, counts, targets, params, index),
                        ],
                        plan,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
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
