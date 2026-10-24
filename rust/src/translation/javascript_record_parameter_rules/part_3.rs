// Generated from complete js/src/translation/javascript-record-parameter-rules.js. Do not edit by hand.
use super::accept_java_script_record_parameter_bound_name;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_binding_names;
use super::read_array_binding_initializer_end;
use super::read_required_number_fact;
use super::read_required_text_fact;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result {
    MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Done,
    MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_array_binding_name_loop4(
    name: &self::ml_string::Text,
    reserved: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result {
    loop {
        return if index < (reserved.len() as f64) {
            if (self::read_required_text_fact(reserved, index)) == (*name) {
                self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Return(false)
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
            self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingFactFieldLoop5Result {
    MlSource1TemporaryReadArrayBindingFactFieldLoop5Done,
    MlSource1TemporaryReadArrayBindingFactFieldLoop5Return(f64),
}

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
                            let source_ml_source2_s14 = if kind
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
                                            let source_ml_source2_concat_value15 = closes;
                                            {
                                                let source_ml_source2_concat_value16 =
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
                                                        source_ml_source2_concat_value15,
                                                    ),
                                                    source_ml_source2_concat_value16,
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
                            match source_ml_source2_s14 {
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
                            let source_ml_source2_concat_value17 = vec![index];
                            {
                                let source_ml_source2_concat_value18 = positions;
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value17), source_ml_source2_concat_value18)
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
                                    let source_ml_source2_s19 = if (kind
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
                                    match source_ml_source2_s19 {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(index_4, rest_3) => {
                                            if (index_4 >= (values.len() as f64)) || (index_4 >= (kinds.len() as f64)) {
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
                                            } else {
                                                {
                                                    let source_ml_source2_s24 = if ((self::read_required_text_fact(kinds, index_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_4)) == (self::ml_string::Text::from_units(vec![91]))) {
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
                                                                            let source_ml_source2_concat_value25 = positions;
                                                                            {
                                                                                let source_ml_source2_concat_value26 = self::ml_array::slice(&nested, 1f64, f64::INFINITY);
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value25), source_ml_source2_concat_value26)
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
                                                                    let source_ml_source2_concat_value27 = positions;
                                                                    {
                                                                        let source_ml_source2_concat_value28 = vec![index_4];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value27), source_ml_source2_concat_value28)
                                                                    }
                                                                };
                                                                {
                                                                    let index_6 = index_4 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(index_6, positions_3)
                                                                }
                                                            }
                                                        }
};
                                                    match source_ml_source2_s24 {
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(index_7, positions_4) => {
                                                            if (index_7 >= (values.len() as f64)) || (index_7 >= (kinds.len() as f64)) {
                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
                                                            } else if rest_3 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![93]))) {
                                                                        {
                                                                            let source_ml_source2_concat_value29 = vec![index_7];
                                                                            {
                                                                                let source_ml_source2_concat_value30 = positions_4;
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value29), source_ml_source2_concat_value30)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        vec![-1f64]
                                                                    })
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s31 = if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![61]))) {
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
                                                                        match source_ml_source2_s31 {
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
                                                                                                let source_ml_source2_concat_value32 = vec![index_9];
                                                                                                {
                                                                                                    let source_ml_source2_concat_value33 = positions_4;
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value32), source_ml_source2_concat_value33)
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
pub enum MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result {
    MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Done,
    MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_record_parameter_pattern_loop12(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut binding: bool,
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, at);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Return(-1f64)
                } else {
                    if kind != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) {
                        {
                            let binding_3 = if depth == 0f64 { { false } } else { binding };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (binding, at) = (binding_3, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let value = self::read_required_text_fact(values, at);
                            if (depth == 0f64)
                                && (value == (self::ml_string::Text::from_units(vec![41])))
                            {
                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Return(-1f64)
                            } else if ((depth == 0f64) && binding)
                                && (value == (self::ml_string::Text::from_units(vec![123])))
                            {
                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Return(at)
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
                                                let at_3 = at + 1f64;
                                                {
                                                    (depth, binding, at) =
                                                        (depth_2, binding_4, at_3);
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
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Return(-1f64)
                                            } else {
                                                {
                                                    let at_4 = at + 1f64;
                                                    {
                                                        (depth, at) = (depth_3, at_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (depth == 0f64)
                                            && (value
                                                == (self::ml_string::Text::from_units(vec![44])))
                                        {
                                            {
                                                let binding_5 = true;
                                                {
                                                    let at_5 = at + 1f64;
                                                    {
                                                        (binding, at) = (binding_5, at_5);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            if (depth == 0f64)
                                                && (value
                                                    != (self::ml_string::Text::from_units(vec![
                                                        44,
                                                    ])))
                                            {
                                                {
                                                    let binding_6 = false;
                                                    {
                                                        let at_6 = at + 1f64;
                                                        {
                                                            (binding, at) = (binding_6, at_6);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                            {
                                                {
                                                    let at_7 = at + 1f64;
                                                    {
                                                        at = at_7;
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
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_record_parameter_binding_loop13(
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    mut duplicate: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (names.len() as f64) {
            if (self::read_required_text_fact(names, index)) == (*name) {
                {
                    let duplicate_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (duplicate, index) = (duplicate_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { duplicate };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop14(
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    mut duplicate: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (names.len() as f64) {
            if (self::read_required_text_fact(names, index)) == (*name) {
                {
                    let duplicate_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (duplicate, index) = (duplicate_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { duplicate };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16 {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Done(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18 {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18Next(f64, f64),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop17(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut ordinary: bool,
    mut at: f64,
    mut names: Vec<self::ml_string::Text>,
    mut pairs: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result {
    loop {
        return if (at < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![125]))))
        {
            if ((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![46, 46, 46])))
            {
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(vec![-6f64, at])
            } else {
                let ml_library_inequality_1 = (self::read_required_text_fact(kinds, at))
                    != (self::ml_string::Text::from_units(vec![
                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                    ]));
                if ml_library_inequality_1 {
                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(vec![-7f64, at])
                } else {
                    {
                        let property = at;
                        {
                            let at_2 = at + 1f64;
                            if at_2 >= (kinds.len() as f64) {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(vec![-1f64, at_2])
                            } else {
                                {
                                    let binding = property;
                                    {
                                        let source_ml_source2_s34 =
                                            if ((self::read_required_text_fact(kinds, at_2))
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ])))
                                                && ((self::read_required_text_fact(values, at_2))
                                                    == (self::ml_string::Text::from_units(vec![
                                                        58,
                                                    ])))
                                            {
                                                {
                                                    let at_3 = at_2 + 1f64;
                                                    if at_3 >= (kinds.len() as f64) {
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18Return(vec![-1f64, at_3])
                                                } else if ((self::read_required_text_fact(kinds, at_3)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![123]))) || ((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![91])))) {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18Return(vec![-5f64, at_3])
                                                    } else {
                                                        {
                                                            let binding_2 = at_3;
                                                            {
                                                                let at_4 = at_3 + 1f64;
                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18Next(at_4, binding_2)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18Next(at_2, binding)
                                            };
                                        match source_ml_source2_s34 {
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18Next(at_5, binding_3) => {
                                                {
                                                    let name = self::read_required_text_fact(values, binding_3);
                                                    if (self::accept_java_script_record_parameter_bound_name(&(self::read_required_text_fact(kinds, binding_3)), &(name), &(names), reserved, ordinary)) {
                                                        {
                                                            let names_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone()], names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            {
                                                                let pairs_2 = {
                                                                    let source_ml_source2_concat_value35 = pairs;
                                                                    {
                                                                        let source_ml_source2_concat_value36 = vec![property, binding_3];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value35), source_ml_source2_concat_value36)
                                                                    }
                                                                };
                                                                if at_5 >= (kinds.len() as f64) {
                                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(vec![-1f64, at_5])
                                                                } else if ((self::read_required_text_fact(kinds, at_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_5)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(vec![-2f64, at_5])
                                                                    } else {
                                                                        if ((self::read_required_text_fact(kinds, at_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_5)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                                            {
                                                                                let at_6 = at_5 + 1f64;
                                                                                {
                                                                                    (at, names, pairs) = (at_6, names_2, pairs_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
{
                                                                            if (((self::read_required_text_fact(kinds, at_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_5)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                                                {
                                                                                    (at, names, pairs) = (at_5, names_2, pairs_2);
                                                                                    continue;
                                                                                }
                                                                            }
{
                                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(vec![-1f64, at_5])
                                                                            }
                                                                        }
                                                                    }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(vec![-8f64, binding_3])
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin18Return(source_ml_source1_temporary_result18) => {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(source_ml_source1_temporary_result18)
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
            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Done(at, names, pairs)
        };
    }
}
