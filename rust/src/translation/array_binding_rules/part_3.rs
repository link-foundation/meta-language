// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6;
use super::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7;
use super::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8;
use super::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_binding_names;
use super::read_array_binding_fact_field;
use super::read_array_binding_initializer_end;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_binding_names_loop5(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut index: f64,
    mut positions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if (kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (value == (self::ml_string::Text::from_units(vec![93])))
                    {
                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return({
                            let source_ml_source2_concat_value21 = vec![index];
                            {
                                let source_ml_source2_concat_value22 = positions;
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value21), source_ml_source2_concat_value22)
                            }
                        })
                    } else {
                        if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (value == (self::ml_string::Text::from_units(vec![44])))
                        {
                            {
                                let index_2 = index + 1f64;
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let rest = false;
                                {
                                    let source_ml_source2_s23 = if (kind
                                        == (self::ml_string::Text::from_units(vec![
                                            112, 117, 110, 99, 116,
                                        ])))
                                        && (value
                                            == (self::ml_string::Text::from_units(vec![
                                                46, 46, 46,
                                            ]))) {
                                        {
                                            let rest_2 = true;
                                            {
                                                let index_3 = index + 1f64;
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6Next(index_3, rest_2)
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6Next(index, rest)
                                    };
                                    match source_ml_source2_s23 {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6Next(index_4, rest_3) => {
                                            if (index_4 >= (values.len() as f64)) || (index_4 >= (kinds.len() as f64)) {
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                            } else {
                                                {
                                                    let source_ml_source2_s28 = if ((self::read_required_text_fact(kinds, index_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_4)) == (self::ml_string::Text::from_units(vec![91]))) {
                                                        if rest_3 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let nested = self::plan_java_script_array_binding_names(kinds, values, index_4);
                                                                if (self::read_required_number_fact(&(nested), 0f64)) < 0f64 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                                } else {
                                                                    {
                                                                        let positions_2 = {
                                                                            let source_ml_source2_concat_value29 = positions;
                                                                            {
                                                                                let source_ml_source2_concat_value30 = self::ml_array::slice(&nested, 1f64, f64::INFINITY);
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value29), source_ml_source2_concat_value30)
                                                                            }
                                                                        };
                                                                        {
                                                                            let index_5 = self::read_required_number_fact(&(nested), 0f64) + 1f64;
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Next(index_5, positions_2)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
let ml_library_inequality_3 = (self::read_required_text_fact(kinds, index_4)) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])) ;
if ml_library_inequality_3 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let positions_3 = {
                                                                    let source_ml_source2_concat_value31 = positions;
                                                                    {
                                                                        let source_ml_source2_concat_value32 = vec![index_4];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value31), source_ml_source2_concat_value32)
                                                                    }
                                                                };
                                                                {
                                                                    let index_6 = index_4 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Next(index_6, positions_3)
                                                                }
                                                            }
                                                        }
};
                                                    match source_ml_source2_s28 {
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Next(index_7, positions_4) => {
                                                            if (index_7 >= (values.len() as f64)) || (index_7 >= (kinds.len() as f64)) {
                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                            } else if rest_3 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![93]))) {
                                                                        {
                                                                            let source_ml_source2_concat_value33 = vec![index_7];
                                                                            {
                                                                                let source_ml_source2_concat_value34 = positions_4;
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value33), source_ml_source2_concat_value34)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        vec![-1f64]
                                                                    })
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s35 = if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                            {
                                                                                let index_8 = self::read_array_binding_initializer_end(kinds, values, (index_7 + 1f64));
                                                                                if index_8 < 0f64 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Return(vec![-1f64])
                                                                                } else {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Next(index_8)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Next(index_7)
                                                                        };
                                                                        match source_ml_source2_s35 {
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Next(index_9) => {
                                                                                {
let ml_library_inequality_2 = (self::read_required_text_fact(kinds, index_9)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) ;
if ml_library_inequality_2 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                                                } else {
                                                                                    {
                                                                                        let next = self::read_required_text_fact(values, index_9);
                                                                                        if next == (self::ml_string::Text::from_units(vec![93])) {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return({
                                                                                                let source_ml_source2_concat_value36 = vec![index_9];
                                                                                                {
                                                                                                    let source_ml_source2_concat_value37 = positions_4;
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value36), source_ml_source2_concat_value37)
                                                                                                }
                                                                                            })
                                                                                        } else {
let ml_library_inequality_1 = next != (self::ml_string::Text::from_units(vec![44])) ;
if ml_library_inequality_1 {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                                                            } else {
                                                                                                {
                                                                                                    let index_10 = index_9 + 1f64;
                                                                                                    {
                                                                                                        (index, positions) = (index_10, positions_4);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
}
                                                                                    }
                                                                                }
}
                                                                            }
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Return(source_ml_source1_temporary_result8) => {
                                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(source_ml_source1_temporary_result8)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                        }
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(source_ml_source1_temporary_result8_2) => {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(source_ml_source1_temporary_result8_2)
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
            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Result {
    MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Done(f64, self::ml_string::Text, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_binding_default_term_loop9(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    mut absence: self::ml_string::Text,
    mut kind: f64,
) -> self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Result {
    loop {
        if (kind >= 0f64)
            && ((self::read_required_text_fact(texts, kind))
                == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
        {
            {
                let field = self::read_array_binding_fact_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 99, 101])),
                );
                {
                    let current = if field < 0f64 {
                        self::ml_string::Text::from_units(vec![98, 111, 116, 104])
                    } else {
                        self::read_required_text_fact(texts, field)
                    };
                    {
                        let absence_2 = if (absence == (self::ml_string::Text::from_units(vec![])))
                            || (absence == current)
                        {
                            current
                        } else {
                            self::ml_string::Text::from_units(vec![98, 111, 116, 104])
                        };
                        {
                            let node_2 = self::read_array_binding_fact_field(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![
                                    101, 108, 101, 109, 101, 110, 116,
                                ])),
                            );
                            {
                                let kind_2 = self::read_array_binding_fact_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node_2,
                                    &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
                                );
                                {
                                    (node, absence, kind) = (node_2, absence_2, kind_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Result::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Done(node, absence, kind)
        };
    }
}
