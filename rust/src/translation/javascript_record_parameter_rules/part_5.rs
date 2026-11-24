// Generated from complete js/src/translation/javascript-record-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result;
use super::accept_java_script_source_binding_name;
use super::find_java_script_formal_default_expression_end;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_binding_names;
use super::read_array_binding_fact_field;
use super::read_array_binding_initializer_end;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_binding_fact_field_loop5(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    field: &self::ml_string::Text,
    mut at: f64,
    mut end: f64,
) -> self::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == (*field) {
                self::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result::MlSource1TemporaryReadArrayBindingFactFieldLoop5Return(self::read_required_number_fact(targets, at))
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
            self::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result::MlSource1TemporaryReadArrayBindingFactFieldLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result {
    MlSource1TemporaryReadArrayBindingInitializerEndLoop6Done,
    MlSource1TemporaryReadArrayBindingInitializerEndLoop6Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingInitializerEndJoin7 {
    MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(Vec<self::ml_string::Text>),
    MlSource1TemporaryReadArrayBindingInitializerEndJoin7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_binding_initializer_end_loop6(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut start: f64,
    mut index: f64,
    mut closes: Vec<self::ml_string::Text>,
) -> self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                        self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Return(-1f64)
                    } else {
                        {
                            let source_ml_source2_s109 = if kind
                                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
                            {
                                if ((closes.len() as f64) == 0f64)
                                    && ((value == (self::ml_string::Text::from_units(vec![44])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Return(if index > start {
                                        index
                                    } else {
                                        -1f64
                                    })
                                } else if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let closes_2 = {
                                            let source_ml_source2_concat_value110 = closes;
                                            {
                                                let source_ml_source2_concat_value111 =
                                                    vec![if value
                                                        == (self::ml_string::Text::from_units(
                                                            vec![40],
                                                        )) {
                                                        self::ml_string::Text::from_units(vec![41])
                                                    } else if value
                                                        == (self::ml_string::Text::from_units(
                                                            vec![91],
                                                        ))
                                                    {
                                                        self::ml_string::Text::from_units(vec![93])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![125])
                                                    }];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        source_ml_source2_concat_value110,
                                                    ),
                                                    source_ml_source2_concat_value111,
                                                )
                                            }
                                        };
                                        self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes_2)
                                    }
                                } else if ((value == (self::ml_string::Text::from_units(vec![41])))
                                    || (value == (self::ml_string::Text::from_units(vec![93]))))
                                    || (value == (self::ml_string::Text::from_units(vec![125])))
                                {
                                    if ((closes.len() as f64) == 0f64)
                                        || ((self::read_required_text_fact(
                                            &(closes),
                                            ((closes.len() as f64) - 1f64),
                                        )) != value)
                                    {
                                        self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Return(-1f64)
                                    } else {
                                        {
                                            let closes_3 = self::ml_array::slice(
                                                &closes,
                                                0f64,
                                                ((closes.len() as f64) - 1f64),
                                            );
                                            self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes_3)
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes)
                                }
                            } else {
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes)
                            };
                            match source_ml_source2_s109 {
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes_4) => {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (index, closes) = (index_2, closes_4);
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Return(source_ml_source1_temporary_result7) => {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Return(source_ml_source1_temporary_result7)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Done,
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Next(f64),
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(f64, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_binding_names_loop8(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut index: f64,
    mut positions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if (kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (value == (self::ml_string::Text::from_units(vec![93])))
                    {
                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return({
                            let source_ml_source2_concat_value112 = vec![index];
                            {
                                let source_ml_source2_concat_value113 = positions;
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value112), source_ml_source2_concat_value113)
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
                                    let source_ml_source2_s114 = if (kind
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
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(index_3, rest_2)
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(index, rest)
                                    };
                                    match source_ml_source2_s114 {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(index_4, rest_3) => {
                                            if (index_4 >= (values.len() as f64)) || (index_4 >= (kinds.len() as f64)) {
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
                                            } else {
                                                {
                                                    let source_ml_source2_s119 = if ((self::read_required_text_fact(kinds, index_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_4)) == (self::ml_string::Text::from_units(vec![91]))) {
                                                        if rest_3 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let nested = self::plan_java_script_array_binding_names(kinds, values, index_4);
                                                                if (self::read_required_number_fact(&(nested), 0f64)) < 0f64 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(vec![-1f64])
                                                                } else {
                                                                    {
                                                                        let positions_2 = {
                                                                            let source_ml_source2_concat_value120 = positions;
                                                                            {
                                                                                let source_ml_source2_concat_value121 = self::ml_array::slice(&nested, 1f64, f64::INFINITY);
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value120), source_ml_source2_concat_value121)
                                                                            }
                                                                        };
                                                                        {
                                                                            let index_5 = self::read_required_number_fact(&(nested), 0f64) + 1f64;
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(index_5, positions_2)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
let ml_library_inequality_3 = (self::read_required_text_fact(kinds, index_4)) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])) ;
if ml_library_inequality_3 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let positions_3 = {
                                                                    let source_ml_source2_concat_value122 = positions;
                                                                    {
                                                                        let source_ml_source2_concat_value123 = vec![index_4];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value122), source_ml_source2_concat_value123)
                                                                    }
                                                                };
                                                                {
                                                                    let index_6 = index_4 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(index_6, positions_3)
                                                                }
                                                            }
                                                        }
};
                                                    match source_ml_source2_s119 {
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(index_7, positions_4) => {
                                                            if (index_7 >= (values.len() as f64)) || (index_7 >= (kinds.len() as f64)) {
                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
                                                            } else if rest_3 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![93]))) {
                                                                        {
                                                                            let source_ml_source2_concat_value124 = vec![index_7];
                                                                            {
                                                                                let source_ml_source2_concat_value125 = positions_4;
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value124), source_ml_source2_concat_value125)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        vec![-1f64]
                                                                    })
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s126 = if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                            {
                                                                                let index_8 = self::read_array_binding_initializer_end(kinds, values, (index_7 + 1f64));
                                                                                if index_8 < 0f64 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Return(vec![-1f64])
                                                                                } else {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Next(index_8)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Next(index_7)
                                                                        };
                                                                        match source_ml_source2_s126 {
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Next(index_9) => {
                                                                                {
let ml_library_inequality_2 = (self::read_required_text_fact(kinds, index_9)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) ;
if ml_library_inequality_2 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
                                                                                } else {
                                                                                    {
                                                                                        let next = self::read_required_text_fact(values, index_9);
                                                                                        if next == (self::ml_string::Text::from_units(vec![93])) {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return({
                                                                                                let source_ml_source2_concat_value127 = vec![index_9];
                                                                                                {
                                                                                                    let source_ml_source2_concat_value128 = positions_4;
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value127), source_ml_source2_concat_value128)
                                                                                                }
                                                                                            })
                                                                                        } else {
let ml_library_inequality_1 = next != (self::ml_string::Text::from_units(vec![44])) ;
if ml_library_inequality_1 {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
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
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Return(source_ml_source1_temporary_result11) => {
                                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(source_ml_source1_temporary_result11)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                        }
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(source_ml_source1_temporary_result11_2) => {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(source_ml_source1_temporary_result11_2)
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
            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Result {
    MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Done(f64, self::ml_string::Text, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_binding_default_term_loop12(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    mut absence: self::ml_string::Text,
    mut kind: f64,
) -> self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Result {
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
            self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Result::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Done(node, absence, kind)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Result {
    MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Done,
    MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_array_formal_pattern_loop13(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut binding: bool,
    mut index: f64,
) -> self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Return(-1f64)
                } else {
                    if kind != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) {
                        {
                            let binding_3 = if depth == 0f64 { { false } } else { binding };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (binding, index) = (binding_3, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let value = self::read_required_text_fact(values, index);
                            if (depth == 0f64)
                                && (value == (self::ml_string::Text::from_units(vec![41])))
                            {
                                self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Return(-1f64)
                            } else if ((depth == 0f64) && binding)
                                && (value == (self::ml_string::Text::from_units(vec![91])))
                            {
                                self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Return(index)
                            } else {
                                if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let depth_2 = depth + 1f64;
                                        {
                                            let binding_4 = false;
                                            {
                                                let index_3 = index + 1f64;
                                                {
                                                    (depth, binding, index) =
                                                        (depth_2, binding_4, index_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                                {
                                    if ((value == (self::ml_string::Text::from_units(vec![41])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                        || (value == (self::ml_string::Text::from_units(vec![125])))
                                    {
                                        {
                                            let depth_3 = depth - 1f64;
                                            if depth_3 < 0f64 {
                                                self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Return(-1f64)
                                            } else {
                                                {
                                                    let index_4 = index + 1f64;
                                                    {
                                                        (depth, index) = (depth_3, index_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if depth == 0f64 {
                                            {
                                                let binding_5 = value
                                                    == (self::ml_string::Text::from_units(vec![
                                                        44,
                                                    ]));
                                                {
                                                    let index_5 = index + 1f64;
                                                    {
                                                        (binding, index) = (binding_5, index_5);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            {
                                                let index_6 = index + 1f64;
                                                {
                                                    index = index_6;
                                                    continue;
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
            self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Result {
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15 {
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result {
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Done(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop17(
    names: &[self::ml_string::Text],
    name: &self::ml_string::Text,
    mut duplicate: bool,
    mut previous: f64,
) -> bool {
    loop {
        if previous < (names.len() as f64) {
            if (self::read_required_text_fact(names, previous)) == (*name) {
                {
                    let duplicate_2 = true;
                    {
                        let previous_2 = previous + 1f64;
                        {
                            (duplicate, previous) = (duplicate_2, previous_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let previous_3 = previous + 1f64;
                    {
                        previous = previous_3;
                        continue;
                    }
                }
            }
        }
        return { duplicate };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18 {
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Next(f64, f64, f64),
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop16(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut defaults: bool,
    mut index: f64,
    mut names: Vec<self::ml_string::Text>,
    mut position: f64,
    mut bindings: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result {
    loop {
        return if (index < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, index))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, index))
                    == (self::ml_string::Text::from_units(vec![93]))))
        {
            if ((self::read_required_text_fact(kinds, index))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, index))
                    == (self::ml_string::Text::from_units(vec![44])))
            {
                {
                    let position_2 = position + 1f64;
                    {
                        let index_2 = index + 1f64;
                        {
                            (index, position) = (index_2, position_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let rest = ((self::read_required_text_fact(kinds, index))
                        == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && ((self::read_required_text_fact(values, index))
                            == (self::ml_string::Text::from_units(vec![46, 46, 46])));
                    {
                        let index_4 = if rest { { index + 1f64 } } else { index };
                        if index_4 >= (kinds.len() as f64) {
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(vec![-1f64, index_4])
                        } else {
                            {
                                let token = index_4;
                                {
                                    let name = self::read_required_text_fact(values, index_4);
                                    {
                                        let token_kind =
                                            self::read_required_text_fact(kinds, index_4);
                                        if (token_kind
                                            == (self::ml_string::Text::from_units(vec![
                                                112, 117, 110, 99, 116,
                                            ])))
                                            && ((name
                                                == (self::ml_string::Text::from_units(vec![91])))
                                                || (name
                                                    == (self::ml_string::Text::from_units(vec![
                                                        123,
                                                    ]))))
                                        {
                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(vec![-2f64, index_4])
                                        } else {
                                            {
                                                let duplicate = false;
                                                {
                                                    let previous = 0f64;
                                                    {
                                                        let duplicate_2 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop17(&(names), &(name), duplicate, previous);
                                                        if (self::accept_java_script_source_binding_name(&(token_kind), &(name), duplicate_2, reserved)) {
                                                            {
                                                                let names_2 = {
                                                                    let source_ml_source2_concat_value129 = names;
                                                                    {
                                                                        let source_ml_source2_concat_value130 = vec![name.clone()];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value129), source_ml_source2_concat_value130)
                                                                    }
                                                                };
                                                                {
                                                                    let index_5 = index_4 + 1f64;
                                                                    {
                                                                        let position_3 = position + 1f64;
                                                                        {
                                                                            let default_start = -1f64;
                                                                            {
                                                                                let default_end = -1f64;
                                                                                if index_5 >= (kinds.len() as f64) {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(vec![-1f64, index_5])
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_s131 = if ((self::read_required_text_fact(kinds, index_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_5)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                            if defaults {if rest {
                                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Return(vec![-5f64, index_5])
                                                                                                } else {
                                                                                                    {
                                                                                                        let default_start_2 = index_5 + 1f64;
                                                                                                        {
                                                                                                            let default_end_2 = self::find_java_script_formal_default_expression_end(kinds, values, default_start_2);
                                                                                                            if default_end_2 < 0f64 {
                                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Return(vec![-1f64, default_start_2])
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let index_6 = default_end_2;
                                                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Next(index_6, default_start_2, default_end_2)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }} else {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Return(vec![-3f64, index_5])
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Next(index_5, default_start, default_end)
                                                                                        };
                                                                                        match source_ml_source2_s131 {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Next(index_7, default_start_3, default_end_3) => {
                                                                                                {
                                                                                                    let bindings_2 = {
                                                                                                        let source_ml_source2_concat_value132 = bindings;
                                                                                                        {
                                                                                                            let source_ml_source2_concat_value133 = if defaults {
                                                                                                                vec![if rest {
                                                                                                                    1f64
                                                                                                                } else {
                                                                                                                    0f64
                                                                                                                }, (position_3 - 1f64), token, default_start_3, default_end_3]
                                                                                                            } else {
                                                                                                                vec![if rest {
                                                                                                                    1f64
                                                                                                                } else {
                                                                                                                    0f64
                                                                                                                }, (position_3 - 1f64), token]
                                                                                                            };
                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value132), source_ml_source2_concat_value133)
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let next_kind = self::read_required_text_fact(kinds, index_7);
                                                                                                        {
                                                                                                            let next = self::read_required_text_fact(values, index_7);
                                                                                                            if (next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(vec![-3f64, index_7])
                                                                                                            } else if rest && !((next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![93])))) {
                                                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(vec![-5f64, index_7])
                                                                                                                } else {
                                                                                                                    if (next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![44]))) {
                                                                                                                        {
                                                                                                                            let index_8 = index_7 + 1f64;
                                                                                                                            {
                                                                                                                                (index, names, position, bindings) = (index_8, names_2, position_3, bindings_2);
                                                                                                                                continue;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
{
                                                                                                                        if ((next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![93])))) {
                                                                                                                            {
                                                                                                                                (index, names, position, bindings) = (index_7, names_2, position_3, bindings_2);
                                                                                                                                continue;
                                                                                                                            }
                                                                                                                        }
{
                                                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(vec![-1f64, index_7])
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin18Return(source_ml_source1_temporary_result18) => {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(source_ml_source1_temporary_result18)
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
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(vec![-4f64, index_4])
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
            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Done(index, names, bindings)
        };
    }
}
