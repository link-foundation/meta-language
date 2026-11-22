// Generated from complete js/src/translation/rust-exclusive-source-compound-rules.js. Do not edit by hand.
use super::accept_rust_primitive_from_source;
use super::ml_array;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_exclusive_source_compound_place_at(
    forms: &[String],
    start: f64,
    depth: f64,
) -> Vec<f64> {
    if (depth >= 256f64) || (start >= (forms.len() as f64)) {
        Vec::<f64>::new()
    } else if {
        let source_ml_source1_comparison_operand23 =
            self::ml_array::optional_at(forms, self::ml_array::number_index(start));
        {
            let source_ml_source1_comparison_operand24 = String::from("*");
            match (
                source_ml_source1_comparison_operand23,
                source_ml_source1_comparison_operand24,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    } {
        {
            let inner = self::read_rust_exclusive_source_compound_place_at(
                forms,
                (start + 1f64),
                (depth + 1f64),
            );
            if (inner.len() as f64) >= 2f64 {
                {
                    let (ml_append_right, ml_append_left) = (vec![-1f64], inner);
                    self::ml_array::append(ml_append_left, ml_append_right)
                }
            } else {
                Vec::<f64>::new()
            }
        }
    } else {
        {
            let end = start + 1f64;
            {
                let root = start;
                {
                    let fields = Vec::<f64>::new();
                    {
                        let source_ml_source1_s31 = if {
                            let source_ml_source1_comparison_operand32 =
                                self::ml_array::optional_at(
                                    forms,
                                    self::ml_array::number_index(start),
                                );
                            {
                                let source_ml_source1_comparison_operand33 = String::from("(");
                                match (
                                    source_ml_source1_comparison_operand32,
                                    source_ml_source1_comparison_operand33,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        } {
                            {
                                let plan = self::read_rust_exclusive_source_compound_place_at(
                                    forms,
                                    (start + 1f64),
                                    (depth + 1f64),
                                );
                                if ((plan.len() as f64) < 2f64) || {
                                    let source_ml_source1_comparison_operand34 =
                                        self::ml_array::optional_at(
                                            forms,
                                            self::ml_array::number_index(
                                                (self::ml_array::optional_at(
                                                    &plan,
                                                    self::ml_array::number_index(0f64),
                                                ))
                                                .unwrap_or(f64::NAN),
                                            ),
                                        );
                                    {
                                        let source_ml_source1_comparison_operand35 =
                                            String::from(")");
                                        !(match (
                                            source_ml_source1_comparison_operand34,
                                            source_ml_source1_comparison_operand35,
                                        ) {
                                            (Some(ml_left), ml_right) => ml_left == ml_right,
                                            _ => false,
                                        })
                                    }
                                } {
                                    self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1Return(Vec::<f64>::new())
                                } else {
                                    {
                                        let end_2 = (self::ml_array::optional_at(
                                            &plan,
                                            self::ml_array::number_index(0f64),
                                        ))
                                        .unwrap_or(f64::NAN)
                                            + 1f64;
                                        {
                                            let root_2 = (self::ml_array::optional_at(
                                                &plan,
                                                self::ml_array::number_index(1f64),
                                            ))
                                            .unwrap_or_else(|| -1f64);
                                            {
                                                let fields_2 = self::ml_array::slice(
                                                    &plan,
                                                    2f64,
                                                    f64::INFINITY,
                                                );
                                                self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1Next(end_2, root_2, fields_2)
                                            }
                                        }
                                    }
                                }
                            }
                        } else if {
                            let source_ml_source1_comparison_operand36 =
                                self::ml_array::optional_at(
                                    forms,
                                    self::ml_array::number_index(start),
                                );
                            {
                                let source_ml_source1_comparison_operand37 = String::from("name");
                                !(match (
                                    source_ml_source1_comparison_operand36,
                                    source_ml_source1_comparison_operand37,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                })
                            }
                        } {
                            self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1Return(Vec::<f64>::new())
                        } else {
                            self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1Next(end, root, fields)
                        };
                        match source_ml_source1_s31 {
                                self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1Next(end_3, root_3, fields_3) => {
                                    {
                                        let field_start = end_3;
                                        {
                                            let source_ml_source1_s38 = self::source_ml_temporary_read_rust_exclusive_source_compound_place_at_loop2(forms, end_3);
                                            match source_ml_source1_s38 {
                                                self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Result::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Done(end_4) => {
                                                    {
                                                        let positions = { let ml_values1 = { let ml_values2 = self::ml_array::slice(forms, field_start, end_4); let ml_callback2 = std::rc::Rc::new(move |x_: String, index: f64| -> bool { (index % 2f64) == 1f64 }); ml_values2.iter().cloned().enumerate().filter_map(|(ml_index2, ml_value2)| if ml_callback2(ml_value2.clone(), (ml_index2 as f64)) { Some(ml_value2) } else { None }).collect::<Vec<_>>() }; let ml_callback1 = { let field_start = field_start; std::rc::Rc::new(move |x_: String, index: f64| -> f64 { (field_start + (2f64 * index)) + 1f64 }) }; ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1, (ml_index1 as f64))).collect::<Vec<_>>() };
                                                        self::ml_array::append(self::ml_array::append(vec![end_4, root_3], fields_3), positions)
                                                    }
                                                }
                                                self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Result::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Return(source_ml_temporary_result2) => {
                                                    source_ml_temporary_result2
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1Return(source_ml_temporary_result2_2) => {
                                    source_ml_temporary_result2_2
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_exclusive_source_compound_primitive(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((kind == "fixed") || (kind == "float"))
        && self::accept_rust_primitive_from_source(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Result {
    MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Done(f64),
    MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_exclusive_source_compound_place_at_loop2(
    forms: &[String],
    mut end: f64,
) -> self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Result {
    loop {
        return if {
            let source_ml_source1_comparison_operand39 =
                self::ml_array::optional_at(forms, self::ml_array::number_index(end));
            {
                let source_ml_source1_comparison_operand40 = String::from(".");
                match (
                    source_ml_source1_comparison_operand39,
                    source_ml_source1_comparison_operand40,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } {
            {
                let field = end + 1f64;
                if {
                    let source_ml_source1_comparison_operand41 =
                        self::ml_array::optional_at(forms, self::ml_array::number_index(field));
                    {
                        let source_ml_source1_comparison_operand42 = String::from("name");
                        !(match (
                            source_ml_source1_comparison_operand41,
                            source_ml_source1_comparison_operand42,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } && {
                    let source_ml_source1_comparison_operand43 =
                        self::ml_array::optional_at(forms, self::ml_array::number_index(field));
                    {
                        let source_ml_source1_comparison_operand44 = String::from("number");
                        !(match (
                            source_ml_source1_comparison_operand43,
                            source_ml_source1_comparison_operand44,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } {
                    self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Result::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Return(Vec::<f64>::new())
                } else {
                    {
                        let end_2 = field + 1f64;
                        {
                            end = end_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Result::MlTemporaryReadRustExclusiveSourceCompoundPlaceAtLoop2Done(end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1 {
    MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1Next(f64, f64, Vec<f64>),
    MlTemporaryReadRustExclusiveSourceCompoundPlaceAtJoin1Return(Vec<f64>),
}
